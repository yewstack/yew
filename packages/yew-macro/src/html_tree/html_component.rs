use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Token, Type};

use super::{HtmlChildrenTree, TagTokens};
use crate::is_ide_completion;
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
            let close = input.parse::<HtmlComponentClose>();
            if !is_ide_completion() {
                return match close {
                    Ok(close) => Err(syn::Error::new_spanned(
                        close.to_spanned(),
                        "this closing tag has no corresponding opening tag",
                    )),
                    Err(err) => Err(err),
                };
            }
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
        let close = loop {
            if input.is_empty() {
                if is_ide_completion() {
                    break None;
                }
                return Err(syn::Error::new_spanned(
                    open.to_spanned(),
                    "this opening tag has no corresponding closing tag",
                ));
            }

            if trying_to_close() {
                fn format_token_stream(ts: impl ToTokens) -> String {
                    let string = ts.to_token_stream().to_string();
                    // remove unnecessary spaces
                    string.replace(' ', "")
                }

                let fork = input.fork();
                let close = TagTokens::parse_end_content(&fork, |i_fork, tag| {
                    let ty = i_fork.parse().map_err(|e| {
                        syn::Error::new(
                            e.span(),
                            format!(
                                "expected a valid closing tag for component\nnote: found opening \
                                 tag `{lt}{0}{gt}`\nhelp: try `{lt}/{0}{gt}`",
                                format_token_stream(&open.ty),
                                lt = open.tag.lt.to_token_stream(),
                                gt = open.tag.gt.to_token_stream(),
                            ),
                        )
                    })?;

                    if ty != open.ty && !is_ide_completion() {
                        let open_ty = &open.ty;
                        Err(syn::Error::new_spanned(
                            quote!(#open_ty #ty),
                            format!(
                                "mismatched closing tags: expected `{}`, found `{}`",
                                format_token_stream(open_ty),
                                format_token_stream(ty)
                            ),
                        ))
                    } else {
                        let close = HtmlComponentClose { tag, ty };
                        input.advance_to(&fork);
                        Ok(close)
                    }
                })?;
                break Some(close);
            }
            children.parse_child(input)?;
        };

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
            close,
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
        let children_renderer = children.to_children_renderer_tokens();
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
                #[allow(clippy::let_unit_value)]
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
