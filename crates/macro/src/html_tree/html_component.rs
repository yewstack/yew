use super::HtmlProp;
use super::HtmlPropSuffix;
use super::HtmlTreeNested;
use crate::PeekValue;
use boolinator::Boolinator;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use std::cmp::Ordering;
use syn::buffer::Cursor;
use syn::parse;
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
    children: Vec<HtmlTreeNested>,
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
                    "this close tag has no corresponding open tag",
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
                children: Vec::new(),
            });
        }

        let mut children: Vec<HtmlTreeNested> = vec![];
        loop {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    open,
                    "this open tag has no corresponding close tag",
                ));
            }
            if let Some(ty) = HtmlComponentClose::peek(input.cursor()) {
                if open.ty == ty {
                    break;
                }
            }

            children.push(input.parse()?);
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

        let validate_props = if let Props::List(list_props) = props {
            let check_props = list_props.props.iter().map(|HtmlProp { label, .. }| {
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
                .children(::yew::html::ChildrenRenderer::new({
                    let mut v = ::std::vec::Vec::new();
                    #(v.extend(::yew::utils::NodeSeq::from(#children));)*
                    v
                }))
            }
        } else {
            quote! {}
        };

        let init_props = match props {
            Props::List(list_props) => {
                let set_props = list_props.props.iter().map(|HtmlProp { label, value }| {
                    quote_spanned! { value.span()=> .#label(
                        <::yew::virtual_dom::vcomp::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(
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
            Props::With(with_props) => {
                let props = &with_props.props;
                quote! { #props }
            }
            Props::None => quote! {
                <<#ty as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder()
                    #set_children
                    .build()
            },
        };

        let validate_comp = quote_spanned! { ty.span()=>
            trait __yew_validate_comp: ::yew::html::Component {}
            impl __yew_validate_comp for #ty {}
        };

        let node_ref = if let Some(node_ref) = props.node_ref() {
            quote_spanned! { node_ref.span()=> #node_ref }
        } else {
            quote! { ::yew::html::NodeRef::default() }
        };

        tokens.extend(quote! {{
            // These validation checks show a nice error message to the user.
            // They do not execute at runtime
            if false {
                #validate_comp
                #validate_props
            }

            ::yew::virtual_dom::VChild::<#ty>::new(#init_props, #node_ref)
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
        let props = parse(stream)?;

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
    List,
    With,
}

enum Props {
    List(Box<ListProps>),
    With(Box<WithProps>),
    None,
}

impl Props {
    fn node_ref(&self) -> Option<&Expr> {
        match self {
            Props::List(list_props) => list_props.node_ref.as_ref(),
            Props::With(with_props) => with_props.node_ref.as_ref(),
            Props::None => None,
        }
    }
}

impl PeekValue<PropType> for Props {
    fn peek(cursor: Cursor) -> Option<PropType> {
        let (ident, _) = cursor.ident()?;
        let prop_type = if ident == "with" {
            PropType::With
        } else {
            PropType::List
        };

        Some(prop_type)
    }
}

impl Parse for Props {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        match Props::peek(input.cursor()) {
            Some(PropType::List) => input.parse().map(|l| Props::List(Box::new(l))),
            Some(PropType::With) => input.parse().map(|w| Props::With(Box::new(w))),
            None => Ok(Props::None),
        }
    }
}

struct ListProps {
    props: Vec<HtmlProp>,
    node_ref: Option<Expr>,
}

impl Parse for ListProps {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut props: Vec<HtmlProp> = Vec::new();
        while HtmlProp::peek(input.cursor()).is_some() {
            props.push(input.parse::<HtmlProp>()?);
        }

        let ref_position = props.iter().position(|p| p.label.to_string() == "ref");
        let node_ref = ref_position.map(|i| props.remove(i).value);
        for prop in &props {
            if prop.label.to_string() == "ref" {
                return Err(syn::Error::new_spanned(&prop.label, "too many refs set"));
            }
            if prop.label.to_string() == "type" {
                return Err(syn::Error::new_spanned(&prop.label, "expected identifier"));
            }
            if !prop.label.extended.is_empty() {
                return Err(syn::Error::new_spanned(&prop.label, "expected identifier"));
            }
        }

        // alphabetize
        props.sort_by(|a, b| {
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

        Ok(ListProps { props, node_ref })
    }
}

struct WithProps {
    props: Ident,
    node_ref: Option<Expr>,
}

impl Parse for WithProps {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let with = input.parse::<Ident>()?;
        if with != "with" {
            return Err(input.error("expected to find `with` token"));
        }
        let props = input.parse::<Ident>()?;
        let _ = input.parse::<Token![,]>();

        // Check for the ref tag after `with`
        let mut node_ref = None;
        if let Some(ident) = input.cursor().ident() {
            let prop = input.parse::<HtmlProp>()?;
            if ident.0 == "ref" {
                node_ref = Some(prop.value);
            } else {
                return Err(syn::Error::new_spanned(&prop.label, "unexpected token"));
            }
        }

        Ok(WithProps { props, node_ref })
    }
}
