use super::{Prop, Props, SpecialProps, CHILDREN_LABEL};
use crate::props::PropLabel;
use crate::typed_vdom::all_shared_attributes_as_string;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use std::convert::TryFrom;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::Dot2,
    Expr,
};

struct BaseExpr {
    pub dot2: Dot2,
    pub expr: Expr,
}

impl Parse for BaseExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let dot2 = input.parse()?;
        let expr = input.parse().map_err(|expr_error| {
            let mut error =
                syn::Error::new_spanned(dot2, "expected base props expression after `..`");
            error.combine(expr_error);
            error
        })?;
        Ok(Self { dot2, expr })
    }
}

impl ToTokens for BaseExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BaseExpr { dot2, expr } = self;
        tokens.extend(quote! { #dot2 #expr });
    }
}

pub struct ComponentProps {
    pub props: Props<true>,
    base_expr: Option<Expr>,
}
impl ComponentProps {
    /// Get the special props supported by both variants
    pub fn special(&self) -> &SpecialProps<true> {
        &self.props.special
    }

    // check if the `children` prop is given explicitly
    pub fn children(&self) -> Option<&Prop<true>> {
        self.props.get_by_label(CHILDREN_LABEL)
    }

    fn prop_validation_tokens(
        &self,
        props_ty: impl ToTokens,
        has_children: bool,
        is_element_component: bool,
    ) -> TokenStream {
        let shared = all_shared_attributes_as_string();

        let check_children = if has_children {
            Some(quote_spanned! {props_ty.span()=> __yew_props.children; })
        } else {
            None
        };

        let check_props: TokenStream = self
            .props
            .iter()
            .map(|Prop { label, .. }| {
                let should_check = {
                    let label = label.to_string();
                    label == "__globals" || (is_element_component && shared.contains(&label))
                };
                if should_check {
                    quote! {}
                } else {
                    quote_spanned!( label.span()=> __yew_props.#label; )
                }
            })
            .chain(self.base_expr.iter().map(|expr| {
                quote_spanned! {props_ty.span()=>
                    let _: #props_ty = #expr;
                }
            }))
            .collect();

        quote_spanned! {props_ty.span()=>
            #[allow(clippy::no_effect)]
            if false {
                let _ = |__yew_props: #props_ty| {
                    #check_children
                    #check_props
                };
            }
        }
    }

    pub fn build_properties_tokens<CR: ToTokens>(
        &self,
        props_ty: impl ToTokens,
        children_renderer: Option<CR>,
        is_element: bool,
    ) -> TokenStream {
        let validate_props =
            self.prop_validation_tokens(&props_ty, children_renderer.is_some(), is_element);
        let shared = all_shared_attributes_as_string();
        let build_props = match &self.base_expr {
            None => {
                let mut set_props = vec![];
                let mut global_props = vec![];
                for Prop { label, value, .. } in self.props.iter() {
                    if is_element && shared.contains(&label.to_string()) {
                        global_props.push(quote_spanned! {value.span()=>
                            .#label(#value)
                        })
                    } else {
                        set_props.push(quote_spanned! {value.span()=>
                            .#label(#value)
                        })
                    }
                }
                let globals_setter = is_element.then(|| quote! {
                    .__globals(
                        <::yew::virtual_dom::typings::globals::Globals as ::yew::html::Properties>::builder()
                        #(#global_props)*
                        .build()
                    )
                });
                let set_children = children_renderer.map(|children| {
                    quote_spanned! {props_ty.span()=>
                        .children(#children)
                    }
                });

                quote_spanned! {props_ty.span()=>
                    <#props_ty as ::yew::html::Properties>::builder()
                        #globals_setter
                        #(#set_props)*
                        #set_children
                        .build()
                }
            }
            // Builder pattern is unnecessary in this case, since the base expression guarantees
            // all values are initialized
            Some(expr) => {
                let ident = Ident::new("__yew_props", props_ty.span());
                let set_props = self.props.iter().map(|Prop { label, value, .. }| {
                    quote_spanned! {value.span()=>
                        #ident.#label = ::yew::html::IntoPropValue::into_prop_value(#value);
                    }
                });
                let set_children = children_renderer.map(|children| {
                    quote_spanned! {props_ty.span()=>
                        #ident.children = #children;
                    }
                });

                quote! {
                    let mut #ident = #expr;
                    #(#set_props)*
                    #set_children
                    #ident
                }
            }
        };

        quote! {
            {
                #validate_props
                #build_props
            }
        }
    }
}

impl Parse for ComponentProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let props = validate(input.parse()?)?;
        let base_expr = if input.is_empty() {
            None
        } else {
            Some(input.parse::<BaseExpr>()?)
        };

        if input.is_empty() {
            let base_expr = base_expr.map(|base| base.expr);
            Ok(Self { props, base_expr })
        } else {
            Err(syn::Error::new_spanned(
                base_expr,
                "base props expression must appear last in list of props",
            ))
        }
    }
}

impl TryFrom<Props<true>> for ComponentProps {
    type Error = syn::Error;

    fn try_from(props: Props<true>) -> Result<Self, Self::Error> {
        Ok(Self {
            props: validate(props)?,
            base_expr: None,
        })
    }
}

fn validate(props: Props<true>) -> Result<Props<true>, syn::Error> {
    props.check_no_duplicates()?;
    props.check_all(|prop| match &prop.label {
        PropLabel::HtmlDashedName(name)
            if syn::parse2::<Ident>(name.to_token_stream()).is_err() =>
        {
            Err(syn::Error::new_spanned(
                &name,
                "expected a valid Rust identifier",
            ))
        }
        _ => Ok(()),
    })?;

    Ok(props)
}
