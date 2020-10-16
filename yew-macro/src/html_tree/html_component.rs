use super::{HtmlChildrenTree, TagTokens};
use crate::{props::ComponentProps, PeekValue};
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

pub struct HtmlComponent {
    ty: Type,
    props: ComponentProps,
    children: HtmlChildrenTree,
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

        input.parse::<HtmlComponentClose>()?;

        if !children.is_empty() {
            // check if the `children` prop is given explicitly
            if let ComponentProps::List(props) = &open.props {
                if let Some(children_prop) = props.get_by_label("children") {
                    return Err(syn::Error::new_spanned(
                        &children_prop.label,
                        "cannot specify the `children` prop when the component already has children",
                    ));
                }
            }
        }

        Ok(HtmlComponent {
            ty: open.ty,
            props: open.props,
            children,
        })
    }
}

impl ToTokens for HtmlComponent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            ty,
            props,
            children,
        } = self;

        let props_ty = quote_spanned!(ty.span()=> <#ty as ::yew::html::Component>::Properties);
        let children_renderer = if children.is_empty() {
            None
        } else {
            Some(quote! { ::yew::html::ChildrenRenderer::new(#children) })
        };
        let build_props = props.build_properties_tokens(&props_ty, children_renderer);

        let special_props = props.special();
        let node_ref = if let Some(node_ref) = &special_props.node_ref {
            let value = &node_ref.value;
            quote_spanned! {value.span()=> #value }
        } else {
            quote! { ::yew::html::NodeRef::default() }
        };

        let key = if let Some(key) = &special_props.key {
            let value = &key.value;
            quote_spanned! {value.span()=>
                #[allow(clippy::useless_conversion)]
                Some(::std::convert::Into::<::yew::virtual_dom::Key>::into(#value))
            }
        } else {
            quote! { None }
        };

        tokens.extend(quote_spanned! {ty.span()=>
            {
                #[allow(clippy::unit_arg)]
                ::yew::virtual_dom::VChild::<#ty>::new(#build_props, #node_ref, #key)
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

    fn path_arguments(cursor: Cursor) -> Option<(PathArguments, Cursor)> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (ty, cursor) = Self::peek_type(cursor)?;

        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '>').as_option()?;

        Some((
            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Token![<](Span::call_site()),
                args: vec![GenericArgument::Type(ty)].into_iter().collect(),
                gt_token: Token![>](Span::call_site()),
            }),
            cursor,
        ))
    }

    fn peek_type(mut cursor: Cursor) -> Option<(Type, Cursor)> {
        let mut colons_optional = true;
        let mut last_ident = None;
        let mut leading_colon = None;
        let mut segments = Punctuated::new();

        loop {
            let mut post_colons_cursor = cursor;
            if let Some(c) = Self::double_colon(post_colons_cursor) {
                if colons_optional {
                    leading_colon = Some(Token![::](Span::call_site()));
                }
                post_colons_cursor = c;
            } else if !colons_optional {
                break;
            }

            if let Some((ident, c)) = post_colons_cursor.ident() {
                cursor = c;
                last_ident = Some(ident.clone());
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

        let type_str = last_ident?.to_string();
        type_str.is_ascii().as_option()?;
        type_str.bytes().next()?.is_ascii_uppercase().as_option()?;

        Some((
            Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon,
                    segments,
                },
            }),
            cursor,
        ))
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
        let (typ, _) = HtmlComponent::peek_type(cursor)?;
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
    _ty: Type,
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

        let (typ, cursor) = HtmlComponent::peek_type(cursor)?;

        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()?;

        Some(typ)
    }
}
impl Parse for HtmlComponentClose {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_end_content(input, |input, tag| {
            let ty = input.parse()?;
            Ok(Self { tag, _ty: ty })
        })
    }
}
