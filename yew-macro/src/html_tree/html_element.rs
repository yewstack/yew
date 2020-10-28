use super::{HtmlChildrenTree, HtmlDashedName, TagTokens};
use crate::props::{ClassesForm, ElementProps, Prop};
use crate::stringify::Stringify;
use crate::{non_capitalized_ascii, Peek, PeekValue};
use boolinator::Boolinator;
use proc_macro2::{Delimiter, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Block, Ident, Token};

pub struct HtmlElement {
    name: TagName,
    props: ElementProps,
    children: HtmlChildrenTree,
}

impl PeekValue<()> for HtmlElement {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlElementOpen::peek(cursor)
            .or_else(|| HtmlElementClose::peek(cursor))
            .map(|_| ())
    }
}

impl Parse for HtmlElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if HtmlElementClose::peek(input.cursor()).is_some() {
            return match input.parse::<HtmlElementClose>() {
                Ok(close) => Err(syn::Error::new_spanned(
                    close.to_spanned(),
                    "this closing tag has no corresponding opening tag",
                )),
                Err(err) => Err(err),
            };
        }

        let open = input.parse::<HtmlElementOpen>()?;
        // Return early if it's a self-closing tag
        if open.is_self_closing() {
            return Ok(HtmlElement {
                name: open.name,
                props: open.props,
                children: HtmlChildrenTree::new(),
            });
        }

        if let TagName::Lit(name) = &open.name {
            // Void elements should not have children.
            // See https://html.spec.whatwg.org/multipage/syntax.html#void-elements
            //
            // For dynamic tags this is done at runtime!
            match name.to_ascii_lowercase_string().as_str() {
                "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
                | "meta" | "param" | "source" | "track" | "wbr" => {
                    return Err(syn::Error::new_spanned(open.to_spanned(), format!("the tag `<{}>` is a void element and cannot have children (hint: rewrite this as `<{0}/>`)", name)));
                }
                _ => {}
            }
        }

        let open_key = open.name.get_key();
        let mut children = HtmlChildrenTree::new();
        loop {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    open.to_spanned(),
                    "this opening tag has no corresponding closing tag",
                ));
            }
            if let Some(close_key) = HtmlElementClose::peek(input.cursor()) {
                if open_key == close_key {
                    break;
                }
            }

            children.parse_child(input)?;
        }

        input.parse::<HtmlElementClose>()?;

        Ok(Self {
            name: open.name,
            props: open.props,
            children,
        })
    }
}

impl ToTokens for HtmlElement {
    #[allow(clippy::cognitive_complexity)]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            props,
            children,
        } = self;

        let name_sr = match &name {
            TagName::Lit(name) => name.stringify(),
            TagName::Expr(name) => {
                let expr = &name.expr;
                let vtag_name = Ident::new("__yew_vtag_name", expr.span());
                // this way we get a nice error message (with the correct span) when the expression doesn't return a valid value
                quote_spanned! {expr.span()=> {
                    #[allow(unused_braces)]
                    let mut #vtag_name = ::std::convert::Into::<::std::borrow::Cow::<'static, str>>::into(#expr);
                    if !#vtag_name.is_ascii() {
                        ::std::panic!("a dynamic tag returned a tag name containing non ASCII characters: `{}`", #vtag_name);
                    };
                    // convert to lowercase because the runtime checks rely on it.
                    #vtag_name.to_mut().make_ascii_lowercase();
                    #vtag_name
                }}
            }
        };

        let ElementProps {
            classes,
            attributes,
            booleans,
            kind,
            value,
            checked,
            node_ref,
            key,
            listeners,
        } = &props;

        let vtag = Ident::new("__yew_vtag", name.span());

        // attributes with special treatment

        let set_node_ref = node_ref.as_ref().map(|attr| {
            let value = &attr.value;
            quote! {
                #vtag.node_ref = #value;
            }
        });
        let set_key = key.as_ref().map(|attr| {
            let value = attr.value.optimize_literals();
            quote! {
                #vtag.__macro_set_key(#value);
            }
        });
        let set_value = value.as_ref().map(|attr| {
            let value = attr.value.optimize_literals();
            quote! {
                #vtag.set_value(#value);
            }
        });
        let set_kind = kind.as_ref().map(|attr| {
            let value = attr.value.optimize_literals();
            quote! {
                #vtag.set_kind(#value);
            }
        });
        let set_checked = checked.as_ref().map(|attr| {
            let value = &attr.value;
            quote! {
                #vtag.set_checked(#value);
            }
        });

        // other attributes

        let set_attributes = {
            let normal_attrs = attributes.iter().map(|Prop { label, value, .. }| {
                let key = label.to_lit_str();
                let value = value.optimize_literals();
                quote! {
                    ::yew::virtual_dom::PositionalAttr::new(#key, #value)
                }
            });
            let boolean_attrs = booleans.iter().map(|Prop { label, value, .. }| {
                let key = label.to_lit_str();
                quote! {
                    ::yew::virtual_dom::PositionalAttr::new_boolean(#key, #value)
                }
            });
            let class_attr = classes.as_ref().and_then(|classes| match classes {
                ClassesForm::Tuple(classes) => {
                    let n = classes.len();
                    Some(quote! {
                        {
                            let mut __yew_classes = ::yew::virtual_dom::Classes::with_capacity(#n);
                            #(__yew_classes.push(#classes);)*

                            ::yew::virtual_dom::PositionalAttr::new("class", __yew_classes)
                        }
                    })
                }
                ClassesForm::Single(classes) => match classes.try_into_lit() {
                    Some(lit) => {
                        if lit.value().is_empty() {
                            None
                        } else {
                            let sr = lit.stringify();
                            Some(quote! {
                                ::yew::virtual_dom::PositionalAttr::new("class", #sr)
                            })
                        }
                    }
                    None => {
                        Some(quote! {
                            ::yew::virtual_dom::PositionalAttr::new("class", ::std::convert::Into::<::yew::virtual_dom::Classes>::into(#classes))
                        })
                    }
                }
            });

            let attrs = normal_attrs.chain(boolean_attrs).chain(class_attr);
            quote! {
                #vtag.attributes = ::yew::virtual_dom::Attributes::Vec(::std::vec![#(#attrs),*]);
            }
        };

        let set_listeners = if listeners.is_empty() {
            None
        } else {
            let listeners_it = listeners.iter().map(|Prop { label, value, .. }| {
                let name = &label.name;
                quote! {
                    ::yew::html::#name::Wrapper::__macro_new(#value)
                }
            });

            Some(quote! {
                #vtag.__macro_set_listeners(::std::vec![#(#listeners_it),*]);
            })
        };

        let add_children = if children.is_empty() {
            None
        } else {
            Some(quote! {
                #[allow(clippy::redundant_clone, unused_braces)]
                #vtag.add_children(#children);
            })
        };

        // These are the runtime-checks exclusive to dynamic tags.
        // For literal tags this is already done at compile-time.
        let dyn_tag_runtime_checks = if matches!(&name, TagName::Expr(_)) {
            // when Span::source_file Span::start get stabilised or yew-macro introduces a nightly feature flag
            // we should expand the panic message to contain the exact location of the dynamic tag.
            Some(quote! {
                // check void element
                if !#vtag.children.is_empty() {
                    match #vtag.tag() {
                        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
                        | "meta" | "param" | "source" | "track" | "wbr" => {
                            ::std::panic!("a dynamic tag tried to create a `<{0}>` tag with children. `<{0}>` is a void element which can't have any children.", #vtag.tag());
                        }
                        _ => {}
                    }
                };

                // handle special attribute value
                match #vtag.tag() {
                    "input" | "textarea" => {}
                    _ => {
                        let __yew_v = #vtag.value.take();
                        #vtag.__macro_push_attr(::yew::virtual_dom::PositionalAttr::new("value", __yew_v));
                    }
                }
            })
        } else {
            None
        };

        tokens.extend(quote_spanned! {name.span()=>
            {
                #[allow(unused_braces)]
                let mut #vtag = ::yew::virtual_dom::VTag::new(#name_sr);

                #set_node_ref
                #set_key
                #set_value
                #set_kind
                #set_checked
                #set_attributes
                #set_listeners

                #add_children

                #dyn_tag_runtime_checks
                ::yew::virtual_dom::VNode::from(#vtag)
            }
        });
    }
}

struct DynamicName {
    at: Token![@],
    expr: Option<Block>,
}

impl Peek<'_, ()> for DynamicName {
    fn peek(cursor: Cursor) -> Option<((), Cursor)> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '@').as_option()?;

        // move cursor past block if there is one
        let cursor = cursor
            .group(Delimiter::Brace)
            .map(|(_, _, cursor)| cursor)
            .unwrap_or(cursor);

        Some(((), cursor))
    }
}

impl Parse for DynamicName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let at = input.parse()?;
        // the expression block is optional, closing tags don't have it.
        let expr = if input.cursor().group(Delimiter::Brace).is_some() {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self { at, expr })
    }
}

impl ToTokens for DynamicName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { at, expr } = self;
        tokens.extend(quote! {#at#expr});
    }
}

#[derive(PartialEq)]
enum TagKey {
    Lit(HtmlDashedName),
    Expr,
}

enum TagName {
    Lit(HtmlDashedName),
    Expr(DynamicName),
}

impl TagName {
    fn get_key(&self) -> TagKey {
        match self {
            TagName::Lit(name) => TagKey::Lit(name.clone()),
            TagName::Expr(_) => TagKey::Expr,
        }
    }
}

impl Peek<'_, TagKey> for TagName {
    fn peek(cursor: Cursor) -> Option<(TagKey, Cursor)> {
        if let Some((_, cursor)) = DynamicName::peek(cursor) {
            Some((TagKey::Expr, cursor))
        } else {
            HtmlDashedName::peek(cursor).map(|(name, cursor)| (TagKey::Lit(name), cursor))
        }
    }
}

impl Parse for TagName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if DynamicName::peek(input.cursor()).is_some() {
            DynamicName::parse(input).map(Self::Expr)
        } else {
            HtmlDashedName::parse(input).map(Self::Lit)
        }
    }
}

impl ToTokens for TagName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TagName::Lit(name) => name.to_tokens(tokens),
            TagName::Expr(name) => name.to_tokens(tokens),
        }
    }
}

struct HtmlElementOpen {
    tag: TagTokens,
    name: TagName,
    props: ElementProps,
}
impl HtmlElementOpen {
    fn is_self_closing(&self) -> bool {
        self.tag.div.is_some()
    }

    fn to_spanned(&self) -> impl ToTokens {
        self.tag.to_spanned()
    }
}

impl PeekValue<TagKey> for HtmlElementOpen {
    fn peek(cursor: Cursor) -> Option<TagKey> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (tag_key, cursor) = TagName::peek(cursor)?;
        if let TagKey::Lit(name) = &tag_key {
            // Avoid parsing `<key=[...]>` as an element. It needs to be parsed as an `HtmlList`.
            if name.to_string() == "key" {
                let (punct, _) = cursor.punct()?;
                // ... unless it isn't followed by a '='. `<key></key>` is a valid element!
                (punct.as_char() != '=').as_option()?;
            } else {
                non_capitalized_ascii(&name.to_string()).as_option()?;
            }
        }

        Some(tag_key)
    }
}

impl Parse for HtmlElementOpen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_start_content(input, |input, tag| {
            let name = input.parse::<TagName>()?;
            let mut props = input.parse::<ElementProps>()?;

            match &name {
                TagName::Lit(name) => {
                    // Don't treat value as special for non input / textarea fields
                    // For dynamic tags this is done at runtime!
                    match name.to_ascii_lowercase_string().as_str() {
                        "input" | "textarea" => {}
                        _ => {
                            if let Some(attr) = props.value.take() {
                                props.attributes.push(attr);
                            }
                        }
                    }
                }
                TagName::Expr(name) => {
                    if name.expr.is_none() {
                        return Err(syn::Error::new_spanned(
                            name,
                            "this dynamic tag is missing an expression block defining its value",
                        ));
                    }
                }
            }

            Ok(Self { tag, name, props })
        })
    }
}

struct HtmlElementClose {
    tag: TagTokens,
    _name: TagName,
}
impl HtmlElementClose {
    fn to_spanned(&self) -> impl ToTokens {
        self.tag.to_spanned()
    }
}

impl PeekValue<TagKey> for HtmlElementClose {
    fn peek(cursor: Cursor) -> Option<TagKey> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '/').as_option()?;

        let (tag_key, cursor) = TagName::peek(cursor)?;
        if let TagKey::Lit(name) = &tag_key {
            non_capitalized_ascii(&name.to_string()).as_option()?;
        }

        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()?;

        Some(tag_key)
    }
}

impl Parse for HtmlElementClose {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        TagTokens::parse_end_content(input, |input, tag| {
            let name = input.parse()?;

            if let TagName::Expr(name) = &name {
                if let Some(expr) = &name.expr {
                    return Err(syn::Error::new_spanned(
                    expr,
                    "dynamic closing tags must not have a body (hint: replace it with just `</@>`)",
                ));
                }
            }

            Ok(Self { tag, _name: name })
        })
    }
}
