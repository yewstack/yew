use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Token, Type};

use super::{HtmlChildrenTree, TagTokens};
use crate::props::ComponentProps;

pub struct HtmlComponent {
    ty: Type,
    props: ComponentProps,
    children: HtmlChildrenTree,
    close: Option<HtmlComponentClose>,
}

impl Parse for HtmlComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // check if the next tokens are </
        let trying_to_close = || {
            let lt = input.peek(Token![<]);
            let div = input.peek2(Token![/]);
            lt && div
        };

        if trying_to_close() {
            return match input.parse::<HtmlComponentClose>() {
                Ok(close) => Err(syn::Error::new_spanned(
                    close.to_spanned(),
                    "this closing tag has no corresponding opening tag",
                )),
                Err(err) => Err(err),
            };
        }

        let open = input.parse::<HtmlComponentOpen>()?;
        // Return early if it's a self-closing tag
        if open.is_self_closing() {
            return Ok(HtmlComponent {
                ty: open.ty,
                props: open.props,
                children: HtmlChildrenTree::new(),
                close: None,
            });
        }

        let mut children = HtmlChildrenTree::new();
        loop {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    open.to_spanned(),
                    "this opening tag has no corresponding closing tag",
                ));
            }

            if trying_to_close() {
                break;
            }
            children.parse_child(input)?;
        }

        let close = input.parse::<HtmlComponentClose>()?;

        if !children.is_empty() {
            if let Some(children_prop) = open.props.children() {
                return Err(syn::Error::new_spanned(
                    &children_prop.label,
                    "cannot specify the `children` prop when the component already has children",
                ));
            }
        }

        Ok(HtmlComponent {
            ty: open.ty,
            props: open.props,
            children,
            close: Some(close),
        })
    }
}

impl ToTokens for HtmlComponent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            ty,
            props,
            children,
            close,
        } = self;

        let ty_span = ty.span().resolved_at(Span::call_site());
        let props_ty = quote_spanned!(ty_span=> <#ty as ::yew::html::BaseComponent>::Properties);
        let children_renderer = if children.is_empty() {
            None
        } else {
            Some(quote! { ::yew::html::ChildrenRenderer::new(#children) })
        };
        let build_props = props.build_properties_tokens(&props_ty, children_renderer);
        let key = props.special().wrap_key_attr();
        let use_close_tag = close
            .as_ref()
            .map(|close| {
                let close_ty = &close.ty;
                quote_spanned! {close_ty.span()=>
                    let _ = |_:#close_ty| {};
                }
            })
            .unwrap_or_default();

        tokens.extend(quote_spanned! {ty_span=>
            {
                #use_close_tag
                let __yew_props = #build_props;
                ::yew::virtual_dom::VChild::<#ty>::new(__yew_props, #key)
            }
        });
    }
}

struct HtmlComponentOpen {
    tag: TagTokens,
    ty: Type,
    props: ComponentProps,
}
impl HtmlComponentOpen {
    fn is_self_closing(&self) -> bool {
        self.tag.div.is_some()
    }

    fn to_spanned(&self) -> impl ToTokens {
        self.tag.to_spanned()
    }
}

impl Parse for HtmlComponentOpen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_start_content(input, |input, tag| {
            let ty = input.parse()?;
            let props: ComponentProps = input.parse()?;

            if let Some(ref node_ref) = props.special().node_ref {
                return Err(syn::Error::new_spanned(
                    &node_ref.label,
                    "cannot use `ref` with components. If you want to specify a property, use \
                     `r#ref` here instead.",
                ));
            }

            Ok(Self { tag, ty, props })
        })
    }
}

struct HtmlComponentClose {
    tag: TagTokens,
    ty: Type,
}
impl HtmlComponentClose {
    fn to_spanned(&self) -> impl ToTokens {
        self.tag.to_spanned()
    }
}

impl Parse for HtmlComponentClose {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_end_content(input, |input, tag| {
            let ty = input.parse()?;
            Ok(Self { tag, ty })
        })
    }
}
