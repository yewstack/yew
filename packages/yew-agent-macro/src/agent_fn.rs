use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, Generics, Ident, Item, ItemFn, Signature, Visibility};

pub trait AgentFnType {
    type RecvType;
    type OutputType;

    fn attr_name() -> &'static str;
    fn agent_type_name() -> &'static str;
    fn agent_type_name_plural() -> &'static str;
    fn parse_recv_type(sig: &Signature) -> syn::Result<Self::RecvType>;
    fn parse_output_type(sig: &Signature) -> syn::Result<Self::OutputType>;
}

#[derive(Clone)]
pub struct AgentFn<F>
where
    F: AgentFnType + 'static,
{
    pub recv_type: F::RecvType,
    pub output_type: F::OutputType,
    pub generics: Generics,
    pub vis: Visibility,
    pub attrs: Vec<Attribute>,
    pub name: Ident,
    pub agent_name: Option<Ident>,

    pub func: ItemFn,
}

impl<F> Parse for AgentFn<F>
where
    F: AgentFnType + 'static,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Item = input.parse()?;

        let func = match parsed {
            Item::Fn(m) => m,

            item => {
                return Err(syn::Error::new_spanned(
                    item,
                    format!(
                        "`{}` attribute can only be applied to functions",
                        F::attr_name()
                    ),
                ))
            }
        };

        let ItemFn {
            attrs, vis, sig, ..
        } = func.clone();

        if sig.generics.lifetimes().next().is_some() {
            return Err(syn::Error::new_spanned(
                sig.generics,
                format!(
                    "{} can't have generic lifetime parameters",
                    F::agent_type_name_plural()
                ),
            ));
        }

        if sig.asyncness.is_none() {
            return Err(syn::Error::new_spanned(
                sig.asyncness,
                format!("{} functions must be async", F::agent_type_name()),
            ));
        }

        if sig.constness.is_some() {
            return Err(syn::Error::new_spanned(
                sig.constness,
                format!("const functions can't be {}", F::agent_type_name_plural()),
            ));
        }

        if sig.abi.is_some() {
            return Err(syn::Error::new_spanned(
                sig.abi,
                format!("extern functions can't be {}", F::agent_type_name_plural()),
            ));
        }
        let recv_type = F::parse_recv_type(&sig)?;
        let output_type = F::parse_output_type(&sig)?;

        Ok(Self {
            recv_type,
            output_type,
            generics: sig.generics,
            vis,
            attrs,
            name: sig.ident,
            agent_name: None,
            func,
        })
    }
}

impl<F> AgentFn<F>
where
    F: AgentFnType + 'static,
{
    /// Filters attributes that should be copied to agent definition.
    pub fn filter_attrs_for_agent_struct(&self) -> Vec<Attribute> {
        self.attrs
            .iter()
            .filter_map(|m| {
                m.path
                    .get_ident()
                    .and_then(|ident| match ident.to_string().as_str() {
                        "doc" | "allow" => Some(m.clone()),
                        _ => None,
                    })
            })
            .collect()
    }

    /// Filters attributes that should be copied to the agent impl block.
    pub fn filter_attrs_for_agent_impl(&self) -> Vec<Attribute> {
        self.attrs
            .iter()
            .filter_map(|m| {
                m.path
                    .get_ident()
                    .and_then(|ident| match ident.to_string().as_str() {
                        "allow" => Some(m.clone()),
                        _ => None,
                    })
            })
            .collect()
    }

    pub fn phantom_generics(&self) -> Punctuated<Ident, Comma> {
        self.generics
            .type_params()
            .map(|ty_param| ty_param.ident.clone()) // create a new Punctuated sequence without any type bounds
            .collect::<Punctuated<_, Comma>>()
    }

    pub fn merge_agent_name(&mut self, name: AgentName) -> syn::Result<()> {
        if let Some(ref m) = name.agent_name {
            if m == &self.name {
                return Err(syn::Error::new_spanned(
                    m,
                    format!(
                        "the {} must not have the same name as the function",
                        F::agent_type_name()
                    ),
                ));
            }
        }

        self.agent_name = name.agent_name;

        Ok(())
    }

    pub fn inner_fn_ident(&self) -> Ident {
        if self.agent_name.is_some() {
            self.name.clone()
        } else {
            Ident::new("inner", Span::mixed_site())
        }
    }

    pub fn agent_name(&self) -> Ident {
        self.agent_name.clone().unwrap_or_else(|| self.name.clone())
    }

    pub fn print_inner_fn(&self) -> ItemFn {
        let mut func = self.func.clone();
        func.sig.ident = self.inner_fn_ident();

        func.vis = Visibility::Inherited;

        func
    }
}

pub struct AgentName {
    agent_name: Option<Ident>,
}

impl Parse for AgentName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self { agent_name: None });
        }

        let agent_name = input.parse()?;

        Ok(Self {
            agent_name: Some(agent_name),
        })
    }
}
