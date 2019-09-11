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
use syn::{Ident, Path, PathArguments, PathSegment, Token, Type, TypePath};

pub struct HtmlComponent {
    ty: Type,
    props: Option<Props>,
    children: Vec<HtmlTreeNested>,
}

impl PeekValue<()> for HtmlComponent {
    fn peek(cursor: Cursor) -> Option<()> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        HtmlComponent::peek_type(cursor)?;
        Some(())
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
            if HtmlComponentClose::peek(input.cursor()).is_some() {
                break;
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
        let vcomp_scope = Ident::new("__yew_vcomp_scope", Span::call_site());

        let validate_props = if let Some(Props::List(ListProps(vec_props))) = props {
            let prop_ref = Ident::new("__yew_prop_ref", Span::call_site());
            let check_props = vec_props.iter().map(|HtmlProp { label, .. }| {
                quote! { #prop_ref.#label; }
            });

            let check_children = if !children.is_empty() {
                quote! { #prop_ref.children; }
            } else {
                quote! {}
            };

            // This is a hack to avoid allocating memory but still have a reference to a props
            // struct so that attributes can be checked against it

            #[cfg(has_maybe_uninit)]
            let unallocated_prop_ref = quote! {
                let #prop_ref: <#ty as ::yew::html::Component>::Properties = unsafe { ::std::mem::MaybeUninit::uninit().assume_init() };
            };

            #[cfg(not(has_maybe_uninit))]
            let unallocated_prop_ref = quote! {
                let #prop_ref: <#ty as ::yew::html::Component>::Properties = unsafe { ::std::mem::uninitialized() };
            };

            quote! {
                #unallocated_prop_ref
                #check_children
                #(#check_props)*
            }
        } else {
            quote! {}
        };

        let set_children = if !children.is_empty() {
            let children_len = children.len();
            quote! {
                .children(::yew::html::ChildrenRenderer::new(
                    #children_len,
                    ::std::boxed::Box::new(move || {
                        #[allow(unused_must_use)]
                        || -> ::std::vec::Vec<_> {
                            vec![#(#children.into(),)*]
                        }
                    }()),
                ))
            }
        } else {
            quote! {}
        };

        let init_props = if let Some(props) = props {
            match props {
                Props::List(ListProps(vec_props)) => {
                    let set_props = vec_props.iter().map(|HtmlProp { label, value }| {
                        quote_spanned! { value.span()=>
                            .#label(<::yew::virtual_dom::vcomp::VComp<_> as ::yew::virtual_dom::vcomp::Transformer<_, _, _>>::transform(#vcomp_scope.clone(), #value))
                        }
                    });

                    quote! {
                        <<#ty as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder()
                            #(#set_props)*
                            #set_children
                            .build()
                    }
                }
                Props::With(WithProps(props)) => quote! { #props },
            }
        } else {
            quote! {
                <<#ty as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder()
                    #set_children
                    .build()
            }
        };

        let validate_comp = quote_spanned! { ty.span()=>
            trait __yew_validate_comp {
                type C: ::yew::html::Component;
            }
            impl __yew_validate_comp for () {
                type C = #ty;
            }
        };

        tokens.extend(quote! {{
            // Validation nevers executes at runtime
            if false {
                #validate_comp
                #validate_props
            }

            let #vcomp_scope: ::yew::virtual_dom::vcomp::ScopeHolder<_> = ::std::default::Default::default();
            ::yew::virtual_dom::VChild::<#ty, _>::new(#init_props, #vcomp_scope)
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

    fn peek_type(mut cursor: Cursor) -> Option<Type> {
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
                segments.push(PathSegment {
                    ident,
                    arguments: PathArguments::None,
                });
            } else {
                break;
            }

            // only first `::` is optional
            colons_optional = false;
        }

        let type_str = last_ident?.to_string();
        type_str.is_ascii().as_option()?;
        type_str.bytes().next()?.is_ascii_uppercase().as_option()?;

        Some(Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon,
                segments,
            },
        }))
    }
}

struct HtmlComponentOpen {
    lt: Token![<],
    ty: Type,
    props: Option<Props>,
    div: Option<Token![/]>,
    gt: Token![>],
}

impl PeekValue<Type> for HtmlComponentOpen {
    fn peek(cursor: Cursor) -> Option<Type> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;
        HtmlComponent::peek_type(cursor)
    }
}

impl Parse for HtmlComponentOpen {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lt = input.parse::<Token![<]>()?;
        let ty = input.parse()?;
        // backwards compat
        let _ = input.parse::<Token![:]>();
        let HtmlPropSuffix { stream, div, gt } = input.parse()?;
        let props: Option<Props> = parse(stream).ok();

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

        HtmlComponent::peek_type(cursor)
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
    List(ListProps),
    With(WithProps),
}

impl PeekValue<PropType> for Props {
    fn peek(cursor: Cursor) -> Option<PropType> {
        let (ident, _) = cursor.ident()?;
        let prop_type = if ident.to_string() == "with" {
            PropType::With
        } else {
            PropType::List
        };

        Some(prop_type)
    }
}

impl Parse for Props {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let prop_type = Props::peek(input.cursor())
            .ok_or_else(|| syn::Error::new(Span::call_site(), "ignore - no props found"))?;
        match prop_type {
            PropType::List => input.parse().map(Props::List),
            PropType::With => input.parse().map(Props::With),
        }
    }
}

struct ListProps(Vec<HtmlProp>);
impl Parse for ListProps {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut props: Vec<HtmlProp> = Vec::new();
        while HtmlProp::peek(input.cursor()).is_some() {
            props.push(input.parse::<HtmlProp>()?);
        }

        for prop in &props {
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

        Ok(ListProps(props))
    }
}

struct WithProps(Ident);
impl Parse for WithProps {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let with = input.parse::<Ident>()?;
        if with.to_string() != "with" {
            return Err(input.error("expected to find `with` token"));
        }
        let props = input.parse::<Ident>()?;
        let _ = input.parse::<Token![,]>();
        Ok(WithProps(props))
    }
}
