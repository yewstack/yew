use super::{HtmlChildrenTree, TagTokens};
use crate::{
    props::{Prop, Props, SpecialProps},
    PeekValue,
};
use boolinator::Boolinator;
use proc_macro2::{Span, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    AngleBracketedGenericArguments, Expr, GenericArgument, Path, PathArguments, PathSegment, Token,
    Type, TypePath,
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

        let validate_props = if let ComponentProps::List(props) = props {
            let check_props = props.props.iter().map(|Prop { label, .. }| {
                quote_spanned! {label.span()=> __yew_props.#label; }
            });

            let check_children = if !children.is_empty() {
                quote_spanned! {ty.span()=> __yew_props.children; }
            } else {
                quote! {}
            };

            quote_spanned! {ty.span()=>
                #[allow(clippy::no_effect)]
                {
                    let _ = |__yew_props: <#ty as ::yew::html::Component>::Properties| {
                        #check_children
                        #(#check_props)*
                    };
                }
            }
        } else {
            quote! {}
        };

        let set_children = if !children.is_empty() {
            // using span of type because the error message goes something like "children method not found".
            // If we could control the message, it should say "component X doesn't accept children" and point at the children.
            quote_spanned! {ty.span()=>
                .children(::yew::html::ChildrenRenderer::new(#children))
            }
        } else {
            quote! {}
        };

        let init_props = match props {
            ComponentProps::List(props) => {
                let set_props = props.props.iter().map(|Prop { label, value, .. }| {
                    quote_spanned! {value.span()=> .#label(
                        #[allow(unused_braces)]
                        <::yew::virtual_dom::VComp as ::yew::virtual_dom::Transformer<_, _>>::transform(
                            #value
                        )
                    )}
                });

                quote_spanned! {ty.span()=>
                    <<#ty as ::yew::html::Component>::Properties as ::yew::html::Properties>::builder()
                        #(#set_props)*
                        #set_children
                        .build()
                }
            }
            ComponentProps::With(props) => {
                let expr = &props.expr;
                quote! { #expr }
            }
        };

        let special_props = props.get_special();
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
            quote! {None}
        };

        tokens.extend(quote_spanned! {ty.span()=>
            {
                // These validation checks show a nice error message to the user.
                // They do not execute at runtime
                if false {
                    #validate_props
                }

                #[allow(clippy::unit_arg)]
                ::yew::virtual_dom::VChild::<#ty>::new(#init_props, #node_ref, #key)
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

mod kw {
    syn::custom_keyword!(with);
}

struct WithProps {
    special: SpecialProps,
    _with: kw::with,
    expr: Expr,
}
impl WithProps {
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
            if input.peek(kw::with) {
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
                "Using the `with props` syntax in combination with named props is not allowed \
                            (note: this does not apply to special props like `ref` and `key`)"
                    ));
                }
            }
        }

        let (with, expr) =
            with_expr.ok_or_else(|| input.error("missing `with props` expression"))?;

        Ok(Self {
            special,
            _with: with,
            expr,
        })
    }
}

enum ComponentProps {
    List(Props),
    With(Box<WithProps>),
}
impl ComponentProps {
    fn get_special(&self) -> &SpecialProps {
        match self {
            Self::List(props) => &props.special,
            Self::With(props) => &props.special,
        }
    }
}
impl Parse for ComponentProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if WithProps::contains_with_expr(&input.fork()) {
            input.parse().map(Self::With)
        } else {
            let props = input.parse::<Props>()?;
            for prop in props.props.iter() {
                if prop.question_mark.is_some() {
                    return Err(syn::Error::new_spanned(
                        &prop.label,
                        "optional attributes are only supported on HTML tags. Components can use `Option<T>` properties to accomplish the same thing.",
                    ));
                }
                if !prop.label.extended.is_empty() {
                    return Err(syn::Error::new_spanned(
                        &prop.label,
                        "expected a valid Rust identifier",
                    ));
                }
            }

            Ok(Self::List(props))
        }
    }
}
