use super::HtmlChildrenTree;
use super::HtmlProp;
use super::HtmlPropSuffix;
use crate::PeekValue;
use boolinator::Boolinator;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use std::cmp::Ordering;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    AngleBracketedGenericArguments, Expr, GenericArgument, Ident, Path, PathArguments, PathSegment,
    Token, Type, TypePath,
};

pub struct HtmlComponent {
    ty: Type,
    props: Props,
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
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if HtmlComponentClose::peek(input.cursor()).is_some() {
            return match input.parse::<HtmlComponentClose>() {
                Ok(close) => Err(syn::Error::new_spanned(
                    close,
                    "this closing tag has no corresponding opening tag",
                )),
                Err(err) => Err(err),
            };
        }

        let open = input.parse::<HtmlComponentOpen>()?;
        // Return early if it's a self-closing tag
        if open.div.is_some() {
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
                    open,
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

        let validate_props = if let PropType::List(list_props) = &props.prop_type {
            let check_props = list_props.iter().map(|HtmlProp { label, .. }| {
                quote! { props.#label; }
            });

            let check_children = if !children.is_empty() {
                quote! { props.children; }
            } else {
                quote! {}
            };

            quote! {
                let _ = |props: <#ty as ::yew::html::Component>::Properties| {
                    #check_children
                    #(#check_props)*
                };
            }
        } else {
            quote! {}
        };

        let set_children = if !children.is_empty() {
            quote! {
                .children(::yew::html::ChildrenRenderer::new(#children))
            }
        } else {
            quote! {}
        };

        let init_props = match &props.prop_type {
            PropType::List(list_props) => {
                let set_props = list_props.iter().map(|HtmlProp { label, value, .. }| {
                    quote_spanned! { value.span()=> .#label(
                        <::yew::virtual_dom::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(
                            #value
                        )
                    )}
                });

                quote! {
                    <<#ty as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder()
                        #(#set_props)*
                        #set_children
                        .build()
                }
            }
            PropType::With(props) => {
                quote! { #props }
            }
            PropType::None => quote! {
                <<#ty as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder()
                    #set_children
                    .build()
            },
        };

        let node_ref = if let Some(node_ref) = &props.node_ref {
            quote_spanned! { node_ref.span()=> #node_ref }
        } else {
            quote! { ::yew::html::NodeRef::default() }
        };

        let key = if let Some(key) = &props.key {
            quote_spanned! { key.span()=> Some(::yew::virtual_dom::Key::from(#key)) }
        } else {
            quote! {None}
        };

        tokens.extend(quote! {{
            // These validation checks show a nice error message to the user.
            // They do not execute at runtime
            if false {
                #validate_props
            }

            ::yew::virtual_dom::VChild::<#ty>::new(#init_props, #node_ref, #key)
        }});
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
    lt: Token![<],
    ty: Type,
    props: Props,
    div: Option<Token![/]>,
    gt: Token![>],
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
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lt = input.parse::<Token![<]>()?;
        let ty = input.parse()?;
        // backwards compat
        let _ = input.parse::<Token![:]>();
        let HtmlPropSuffix { stream, div, gt } = input.parse()?;
        let props = syn::parse2(stream)?;

        Ok(HtmlComponentOpen {
            lt,
            ty,
            props,
            div,
            gt,
        })
    }
}

impl ToTokens for HtmlComponentOpen {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlComponentOpen { lt, gt, .. } = self;
        tokens.extend(quote! {#lt#gt});
    }
}

struct HtmlComponentClose {
    lt: Token![<],
    div: Token![/],
    ty: Type,
    gt: Token![>],
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
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(HtmlComponentClose {
            lt: input.parse()?,
            div: input.parse()?,
            ty: input.parse()?,
            gt: input.parse()?,
        })
    }
}

impl ToTokens for HtmlComponentClose {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlComponentClose { lt, div, ty, gt } = self;
        tokens.extend(quote! {#lt#div#ty#gt});
    }
}

enum PropType {
    List(Vec<HtmlProp>),
    With(Ident),
    None,
}

struct Props {
    node_ref: Option<Expr>,
    key: Option<Expr>,
    prop_type: PropType,
}

const COLLISION_MSG: &str = "Using the `with props` syntax in combination with named props is not allowed (note: this does not apply to the `ref` prop).";

impl Parse for Props {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut props = Props {
            node_ref: None,
            key: None,
            prop_type: PropType::None,
        };
        while let Some((token, _)) = input.cursor().ident() {
            if token == "with" {
                match props.prop_type {
                    PropType::None => Ok(()),
                    PropType::With(_) => Err(input.error("too many `with` tokens used")),
                    PropType::List(_) => Err(syn::Error::new_spanned(&token, COLLISION_MSG)),
                }?;

                input.parse::<Ident>()?;
                props.prop_type = PropType::With(input.parse::<Ident>().map_err(|_| {
                    syn::Error::new_spanned(&token, "`with` must be followed by an identifier")
                })?);

                // Handle optional comma
                let _ = input.parse::<Token![,]>();
                continue;
            }

            if (HtmlProp::peek(input.cursor())).is_none() {
                break;
            }

            let prop = input.parse::<HtmlProp>()?;
            if prop.label.to_string() == "ref" {
                match props.node_ref {
                    None => Ok(()),
                    Some(_) => Err(syn::Error::new_spanned(&prop.label, "too many refs set")),
                }?;

                props.node_ref = Some(prop.value);
                continue;
            }
            if prop.label.to_string() == "key" {
                match props.key {
                    None => Ok(()),
                    Some(_) => Err(syn::Error::new_spanned(&prop.label, "too many keys set")),
                }?;

                props.key = Some(prop.value);
                continue;
            }

            if prop.label.to_string() == "type" {
                return Err(syn::Error::new_spanned(&prop.label, "expected identifier"));
            }

            if !prop.label.extended.is_empty() {
                return Err(syn::Error::new_spanned(&prop.label, "expected identifier"));
            }

            if prop.question_mark.is_some() {
                return Err(syn::Error::new_spanned(
                    &prop.label,
                    "optional attributes are only supported on HTML tags. Yew components can use `Option<T>` properties to accomplish the same thing.",
                ));
            }

            match props.prop_type {
                ref mut prop_type @ PropType::None => {
                    *prop_type = PropType::List(vec![prop]);
                }
                PropType::With(_) => return Err(syn::Error::new_spanned(&token, COLLISION_MSG)),
                PropType::List(ref mut list) => {
                    list.push(prop);
                }
            };
        }

        if let PropType::List(list) = &mut props.prop_type {
            // sort alphabetically
            list.sort_by(|a, b| {
                if a.label == b.label {
                    Ordering::Equal
                } else if a.label.to_string() == "children" {
                    Ordering::Greater
                } else if b.label.to_string() == "children" {
                    Ordering::Less
                } else {
                    a.label
                        .to_string()
                        .partial_cmp(&b.label.to_string())
                        .unwrap()
                }
            });
        }

        Ok(props)
    }
}
