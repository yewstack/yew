use boolinator::Boolinator;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, Token, Type,
    TypePath,
};

use super::{HtmlChildrenTree, TagTokens};
use crate::props::ComponentProps;
use crate::PeekValue;

pub struct HtmlComponent {
    ty: Type,
    props: ComponentProps,
    children: HtmlChildrenTree,
    close: Option<HtmlComponentClose>,
}

impl PeekValue<()> for HtmlComponent {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlComponentOpen::peek(cursor)
            .or_else(|| HtmlComponentClose::peek(cursor))
            .map(|_| ())
    }
}

impl Parse for HtmlComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if HtmlComponentClose::peek(input.cursor()).is_some() {
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
            if let Some(ty) = HtmlComponentClose::peek(input.cursor()) {
                if open.ty == ty {
                    break;
                }
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

        let special_props = props.special();
        let node_ref = if let Some(node_ref) = &special_props.node_ref {
            let value = &node_ref.value;
            quote! { #value }
        } else {
            quote! { <::yew::html::NodeRef as ::std::default::Default>::default() }
        };

        let key = if let Some(key) = &special_props.key {
            let value = &key.value;
            quote_spanned! {value.span().resolved_at(Span::call_site())=>
                #[allow(clippy::useless_conversion)]
                Some(::std::convert::Into::<::yew::virtual_dom::Key>::into(#value))
            }
        } else {
            quote! { ::std::option::Option::None }
        };
        let use_close_tag = if let Some(close) = close {
            let close_ty = &close.ty;
            quote_spanned! {close_ty.span()=>
                let _ = |_:#close_ty| {};
            }
        } else {
            Default::default()
        };

        tokens.extend(quote_spanned! {ty_span=>
            {
                #use_close_tag
                let __yew_props = #build_props;
                ::yew::virtual_dom::VChild::<#ty>::new(__yew_props, #node_ref, #key)
            }
        });
    }
}

impl HtmlComponent {
    fn double_colon(mut cursor: Cursor) -> Option<Cursor> {
        for _ in 0..2 {
            let (punct, c) = cursor.punct()?;
            (punct.as_char() == ':').as_option()?;
            cursor = c;
        }

        Some(cursor)
    }

    /// Refer to the [`syn::parse::Parse`] implementation for [`AngleBracketedGenericArguments`].
    fn path_arguments(mut cursor: Cursor) -> Option<(PathArguments, Cursor)> {
        let (punct, c) = cursor.punct()?;
        cursor = c;
        (punct.as_char() == '<').as_option()?;

        let mut args = Punctuated::new();

        loop {
            let punct = cursor.punct();
            if let Some((punct, c)) = punct {
                if punct.as_char() == '>' {
                    cursor = c;
                    break;
                }
            }

            let (ty, c) = Self::peek_type(cursor);
            cursor = c;

            args.push_value(GenericArgument::Type(ty));

            let punct = cursor.punct();
            if let Some((punct, c)) = punct {
                cursor = c;
                if punct.as_char() == '>' {
                    break;
                } else if punct.as_char() == ',' {
                    args.push_punct(Token![,](Span::mixed_site()))
                }
            }
        }

        Some((
            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Token![<](Span::mixed_site()),
                args,
                gt_token: Token![>](Span::mixed_site()),
            }),
            cursor,
        ))
    }

    fn peek_type(mut cursor: Cursor) -> (Type, Cursor) {
        let mut colons_optional = true;
        let mut leading_colon = None;
        let mut segments = Punctuated::new();

        loop {
            let mut post_colons_cursor = cursor;
            if let Some(c) = Self::double_colon(post_colons_cursor) {
                if colons_optional {
                    leading_colon = Some(Token![::](Span::mixed_site()));
                }
                post_colons_cursor = c;
            } else if !colons_optional {
                break;
            }

            if let Some((ident, c)) = post_colons_cursor.ident() {
                cursor = c;
                let arguments = if let Some((args, c)) = Self::path_arguments(cursor) {
                    cursor = c;
                    args
                } else {
                    PathArguments::None
                };

                segments.push(PathSegment { ident, arguments });
            } else {
                break;
            }

            // only first `::` is optional
            colons_optional = false;
        }

        (
            Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon,
                    segments,
                },
            }),
            cursor,
        )
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

impl PeekValue<Type> for HtmlComponentOpen {
    fn peek(cursor: Cursor) -> Option<Type> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;
        let (typ, _) = HtmlComponent::peek_type(cursor);
        Some(typ)
    }
}

impl Parse for HtmlComponentOpen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_start_content(input, |input, tag| {
            let ty = input.parse()?;
            let props = input.parse()?;

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

impl PeekValue<Type> for HtmlComponentClose {
    fn peek(cursor: Cursor) -> Option<Type> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '/').as_option()?;

        let (typ, cursor) = HtmlComponent::peek_type(cursor);

        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()?;

        Some(typ)
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
