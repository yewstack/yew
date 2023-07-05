use std::convert::TryFrom;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::DotDot;
use syn::Expr;

use super::{Prop, Props, SpecialProps, CHILDREN_LABEL};

struct BaseExpr {
    pub dot_dot: DotDot,
    pub expr: Expr,
}

impl Parse for BaseExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let dot_dot = input.parse()?;
        let expr = input.parse().map_err(|expr_error| {
            let mut error =
                syn::Error::new_spanned(dot_dot, "expected base props expression after `..`");
            error.combine(expr_error);
            error
        })?;
        Ok(Self { dot_dot, expr })
    }
}

impl ToTokens for BaseExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let BaseExpr { dot_dot, expr } = self;
        tokens.extend(quote! { #dot_dot #expr });
    }
}

pub struct ComponentProps {
    props: Props,
    base_expr: Option<Expr>,
}
impl ComponentProps {
    /// Get the special props supported by both variants
    pub fn special(&self) -> &SpecialProps {
        &self.props.special
    }

    // check if the `children` prop is given explicitly
    pub fn children(&self) -> Option<&Prop> {
        self.props.get_by_label(CHILDREN_LABEL)
    }

    fn prop_validation_tokens(&self, props_ty: impl ToTokens, has_children: bool) -> TokenStream {
        let props_ident = Ident::new("__yew_props", props_ty.span());
        let check_children = if has_children {
            Some(quote_spanned! {props_ty.span()=>
                let _ = #props_ident.children;
            })
        } else {
            None
        };

        let check_props: TokenStream = self
            .props
            .iter()
            .map(|Prop { label, .. }| {
                quote_spanned! {Span::call_site().located_at(label.span())=>
                    let _ = &#props_ident.#label;
                }
            })
            .collect();

        quote_spanned! {props_ty.span()=>
            #[allow(clippy::no_effect)]
            if false {
                let _ = |#props_ident: #props_ty| {
                    #check_children
                    #check_props
                };
            };
        }
    }

    pub fn build_properties_tokens<CR: ToTokens>(
        &self,
        props_ty: impl ToTokens,
        children_renderer: Option<CR>,
    ) -> TokenStream {
        let has_children = children_renderer.is_some();
        let validate_props = self.prop_validation_tokens(&props_ty, has_children);
        let build_props = match &self.base_expr {
            None => {
                let builder_ident = Ident::new("__yew_props", props_ty.span());
                let token_ident = Ident::new(
                    "__yew_required_props_token",
                    props_ty.span().resolved_at(Span::mixed_site()),
                );

                let init_builder = quote_spanned! {props_ty.span()=>
                    let mut #builder_ident = <#props_ty as ::yew::html::Properties>::builder();
                    let #token_ident = ::yew::html::AssertAllProps;
                };
                let set_props = self.props.iter().map(|Prop { label, value, .. }| {
                    quote_spanned! {value.span()=>
                        let #token_ident = #builder_ident.#label(#token_ident, #value);
                    }
                });
                let set_children = children_renderer.map(|children| {
                    quote_spanned! {props_ty.span()=>
                        let #token_ident = #builder_ident.children(#token_ident, #children);
                    }
                });
                let build_builder = quote_spanned! {props_ty.span()=>
                    ::yew::html::Buildable::prepare_build(#builder_ident, &#token_ident).build()
                };

                quote! {
                    #init_builder
                    #( #set_props )*
                    #set_children
                    #build_builder
                }
            }
            // Builder pattern is unnecessary in this case, since the base expression guarantees
            // all values are initialized
            Some(expr) => {
                let ident = Ident::new("__yew_props", props_ty.span());
                let set_props = self.props.iter().map(|Prop { label, value, .. }| {
                    quote_spanned! {value.span().resolved_at(Span::call_site())=>
                        #ident.#label = ::yew::html::IntoPropValue::into_prop_value(#value);
                    }
                });
                let set_children = children_renderer.map(|children| {
                    quote_spanned! {props_ty.span()=>
                        #ident.children = ::yew::html::IntoPropValue::into_prop_value(#children);
                    }
                });
                let init_base = quote_spanned! {expr.span().resolved_at(Span::call_site())=>
                    let mut #ident: #props_ty = #expr;
                };

                quote! {
                    #init_base
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

impl TryFrom<Props> for ComponentProps {
    type Error = syn::Error;

    fn try_from(props: Props) -> Result<Self, Self::Error> {
        Ok(Self {
            props: validate(props)?,
            base_expr: None,
        })
    }
}

fn validate(props: Props) -> Result<Props, syn::Error> {
    props.check_no_duplicates()?;
    props.check_all(|prop| {
        if !prop.label.extended.is_empty() {
            Err(syn::Error::new_spanned(
                &prop.label,
                "expected a valid Rust identifier",
            ))
        } else {
            Ok(())
        }
    })?;

    Ok(props)
}
