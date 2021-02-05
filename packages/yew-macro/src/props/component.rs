use super::{Prop, Props, SpecialProps};
use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use std::convert::TryFrom;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Expr, Token,
};

mod kw {
    syn::custom_keyword!(with);
}

pub struct WithProps {
    pub special: SpecialProps,
    pub with: kw::with,
    pub expr: Expr,
}
impl WithProps {
    /// Check if the `ParseStream` contains a `with expr` expression.
    /// This function advances the given `ParseStream`!
    fn contains_with_expr(input: ParseStream) -> bool {
        while !input.is_empty() {
            if input.peek(kw::with) && !input.peek2(Token![=]) {
                return true;
            }
            input.parse::<TokenTree>().ok();
        }

        false
    }
}
impl Parse for WithProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut special = SpecialProps::default();
        let mut with_expr: Option<(kw::with, Expr)> = None;
        while !input.is_empty() {
            // no need to check if it's followed by `=` because `with` isn't a special prop
            if input.peek(kw::with) {
                if let Some((with, expr)) = with_expr {
                    return Err(syn::Error::new_spanned(
                        quote! { #with#expr },
                        "there are two `with <props>` definitions for this component (note: you can only define `with <props>` once)"
                    ));
                }
                let with = input.parse::<kw::with>()?;
                if input.is_empty() {
                    return Err(syn::Error::new_spanned(
                        with,
                        "expected expression following this `with`",
                    ));
                }
                with_expr = Some((with, input.parse()?));
            } else {
                let prop = input.parse::<Prop>()?;

                if let Some(slot) = special.get_slot_mut(&prop.label.to_string()) {
                    if slot.is_some() {
                        return Err(syn::Error::new_spanned(
                            &prop.label,
                            &format!("`{}` can only be set once", prop.label),
                        ));
                    }
                    slot.replace(prop);
                } else {
                    return Err(syn::Error::new_spanned(
                        prop.label,
                        "Using the `with props` syntax in combination with named props is not allowed (note: this does not apply to special props like `ref` and `key`)",
                    ));
                }
            }
        }

        let (with, expr) =
            with_expr.ok_or_else(|| input.error("missing `with props` expression"))?;

        Ok(Self {
            special,
            with,
            expr,
        })
    }
}

pub enum ComponentProps {
    List(Props),
    With(Box<WithProps>),
}
impl ComponentProps {
    /// Get the special props supported by both variants
    pub fn special(&self) -> &SpecialProps {
        match self {
            Self::List(props) => &props.special,
            Self::With(props) => &props.special,
        }
    }

    fn prop_validation_tokens(&self, props_ty: impl ToTokens, has_children: bool) -> TokenStream {
        let check_children = if has_children {
            Some(quote_spanned! {props_ty.span()=> __yew_props.children; })
        } else {
            None
        };

        let check_props = match self {
            Self::List(props) => props
                .iter()
                .map(|Prop { label, .. }| {
                    quote_spanned! {label.span()=> __yew_props.#label; }
                })
                .collect(),
            Self::With(with_props) => {
                let expr = &with_props.expr;
                quote_spanned! {props_ty.span()=>
                    let _: #props_ty = #expr;
                }
            }
        };

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
    ) -> TokenStream {
        let validate_props = self.prop_validation_tokens(&props_ty, children_renderer.is_some());
        let build_props = match self {
            Self::List(props) => {
                let set_props = props.iter().map(|Prop { label, value, .. }| {
                        quote_spanned! {value.span()=> .#label(
                            #[allow(unused_braces)]
                            <::yew::virtual_dom::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(
                                #value
                            )
                        )}
                    });

                let set_children = if let Some(children) = children_renderer {
                    Some(quote_spanned! {props_ty.span()=>
                        .children(#children)
                    })
                } else {
                    None
                };

                quote_spanned! {props_ty.span()=>
                    <#props_ty as ::yew::html::Properties>::builder()
                        #(#set_props)*
                        #set_children
                        .build()
                }
            }
            Self::With(with_props) => {
                let ident = Ident::new("__yew_props", props_ty.span());
                let set_children = if let Some(children) = children_renderer {
                    Some(quote_spanned! {props_ty.span()=>
                        #ident.children = #children;
                    })
                } else {
                    None
                };

                let expr = &with_props.expr;
                quote! {
                    let mut #ident = #expr;
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
        if WithProps::contains_with_expr(&input.fork()) {
            input.parse().map(Self::With)
        } else {
            input.parse::<Props>().and_then(Self::try_from)
        }
    }
}

impl TryFrom<Props> for ComponentProps {
    type Error = syn::Error;

    fn try_from(props: Props) -> Result<Self, Self::Error> {
        props.check_no_duplicates()?;
        props.check_all(|prop| {
            if prop.question_mark.is_some() {
                Err(syn::Error::new_spanned(
                    &prop.label,
                    "optional attributes are only supported on elements. Components can use `Option<T>` properties to accomplish the same thing.",
                ))
            } else if !prop.label.extended.is_empty() {
                Err(syn::Error::new_spanned(
                    &prop.label,
                    "expected a valid Rust identifier",
                ))
            } else {
                Ok(())
            }
        })?;

        Ok(Self::List(props))
    }
}
