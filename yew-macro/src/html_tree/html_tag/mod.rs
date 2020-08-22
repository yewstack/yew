mod tag_attributes;

use super::HtmlChildrenTree;
use super::HtmlDashedName;
use super::HtmlProp as TagAttribute;
use super::HtmlPropSuffix as TagSuffix;
use crate::stringify;
use crate::{non_capitalized_ascii, Peek, PeekValue};
use boolinator::Boolinator;
use proc_macro2::{Delimiter, Span};
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Block, Ident, Token};
use tag_attributes::{ClassesForm, TagAttributes};

pub struct HtmlTag {
    tag_name: TagName,
    attributes: TagAttributes,
    children: HtmlChildrenTree,
}

impl PeekValue<()> for HtmlTag {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlTagOpen::peek(cursor)
            .or_else(|| HtmlTagClose::peek(cursor))
            .map(|_| ())
    }
}

impl Parse for HtmlTag {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if HtmlTagClose::peek(input.cursor()).is_some() {
            return match input.parse::<HtmlTagClose>() {
                Ok(close) => Err(syn::Error::new_spanned(
                    close,
                    "this closing tag has no corresponding opening tag",
                )),
                Err(err) => Err(err),
            };
        }

        let open = input.parse::<HtmlTagOpen>()?;
        // Return early if it's a self-closing tag
        if open.div.is_some() {
            return Ok(HtmlTag {
                tag_name: open.tag_name,
                attributes: open.attributes,
                children: HtmlChildrenTree::new(),
            });
        }

        if let TagName::Lit(name) = &open.tag_name {
            // Void elements should not have children.
            // See https://html.spec.whatwg.org/multipage/syntax.html#void-elements
            //
            // For dynamic tags this is done at runtime!
            match name.to_ascii_lowercase_string().as_str() {
                "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
                | "meta" | "param" | "source" | "track" | "wbr" => {
                    return Err(syn::Error::new_spanned(&open, format!("the tag `<{}>` is a void element and cannot have children (hint: rewrite this as `<{0}/>`)", name)));
                }
                _ => {}
            }
        }

        let open_key = open.tag_name.get_key();
        let mut children = HtmlChildrenTree::new();
        loop {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    open,
                    "this opening tag has no corresponding closing tag",
                ));
            }
            if let Some(close_key) = HtmlTagClose::peek(input.cursor()) {
                if open_key == close_key {
                    break;
                }
            }

            children.parse_child(input)?;
        }

        input.parse::<HtmlTagClose>()?;

        Ok(HtmlTag {
            tag_name: open.tag_name,
            attributes: open.attributes,
            children,
        })
    }
}

impl ToTokens for HtmlTag {
    #[allow(clippy::cognitive_complexity)]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlTag {
            tag_name,
            attributes,
            children,
        } = self;

        let name = match &tag_name {
            TagName::Lit(name) => {
                let name_str = name.to_string();
                quote! { ::std::borrow::Cow::<'static, str>::Borrowed(#name_str) }
            }
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

        let TagAttributes {
            classes,
            attributes,
            booleans,
            kind,
            value,
            checked,
            node_ref,
            key,
            listeners,
        } = &attributes;

        let vtag = Ident::new("__yew_vtag", tag_name.span());
        let mut attr_pairs: Vec<_> = attributes
            .iter()
            .map(|TagAttribute { label, value }| {
                let label_str = label.to_string();
                let sr = stringify::Constructor::from(value);
                quote! { (#label_str, #sr) }
            })
            .collect();
        let set_booleans = booleans.iter().map(|TagAttribute { label, value }| {
            let label_str = label.to_string();
            quote_spanned! {value.span()=> {
                if #value {
                    #vtag.push_attribute(
                        #label_str,
                        ::std::borrow::Cow::<'static, str>::Borrowed(#label_str),
                    );
                };
            }}
        });
        let set_kind = kind.iter().map(|kind| {
            let sr = stringify::Constructor::from(kind);
            quote_spanned! {kind.span()=> #vtag.set_kind(#sr); }
        });
        let set_value = value.iter().map(|value| {
            quote_spanned! {value.span()=> #vtag.set_value(&(#value)); }
        });
        let set_checked = checked.iter().map(|checked| {
            quote_spanned! {checked.span()=> #vtag.set_checked(#checked); }
        });

        let set_classes = match classes {
            Some(ClassesForm::Tuple(classes)) => Some(quote! {
                let __yew_classes
                    = ::yew::virtual_dom::Classes::default()#(.extend(#classes))*;
                if !__yew_classes.is_empty() {
                    #vtag.push_attribute("class", __yew_classes.to_string());
                };
            }),
            Some(ClassesForm::Single(classes)) => match stringify::try_stringify_expr(classes) {
                Some(s) => {
                    if !s.is_empty() {
                        let sr = stringify::Constructor::from(s);
                        attr_pairs.push(quote! { ("class", #sr) });
                    }
                    None
                }
                None => Some(quote! {
                    let __yew_classes
                        = ::std::convert::Into::<::yew::virtual_dom::Classes>::into(#classes);
                    if !__yew_classes.is_empty() {
                        #vtag.push_attribute(
                            "class",
                            ::std::string::ToString::to_string(&__yew_classes),
                        );
                    };
                }),
            },
            None => None,
        };
        let set_classes_it = set_classes.iter();

        let set_node_ref = node_ref.iter().map(|node_ref| {
            quote! {
                #vtag.node_ref = #node_ref;
            }
        });
        let set_key = key.iter().map(|key| {
            quote! {
                #vtag.key = Some(::std::convert::Into::<::yew::virtual_dom::Key>::into(#key));
            }
        });
        let listeners: Vec<_> = listeners
            .iter()
            .map(|listener| {
                let name = &listener.label.name;
                let callback = &listener.value;

                quote_spanned! {name.span()=> ::yew::html::#name::Wrapper::new(
                    <::yew::virtual_dom::VTag as ::yew::virtual_dom::Transformer<_, _>>
                        ::transform(#callback),
                )}
            })
            .collect();

        // These are the runtime-checks exclusive to dynamic tags.
        // For literal tags this is already done at compile-time.
        let dyn_tag_runtime_checks = if matches!(&tag_name, TagName::Expr(_)) {
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
                        if let ::std::option::Option::Some(value) = #vtag.value.take() {
                            #vtag.push_attribute("value", value);
                        };
                    }
                }
            })
        } else {
            None
        };

        // Constant ifs - easy for the compiler to optimise out
        // Attribute setting ordered to reduce reallocation on collection expansion
        let has_attrs = !attr_pairs.is_empty();
        let has_listeners = !listeners.is_empty();
        let has_children = !children.is_empty();
        tokens.extend(quote_spanned! {tag_name.span()=>
            {
                #[allow(unused_braces)]
                let mut #vtag = ::yew::virtual_dom::VTag::new(#name);
                #(#set_node_ref)*
                #(#set_key)*
                #(#set_kind)*

                if #has_attrs {
                    #vtag.attributes = ::yew::virtual_dom::Attributes::Vec(vec![#(#attr_pairs),*]);
                };
                #(#set_booleans)*
                #(#set_classes_it)*
                #(#set_checked)*
                #(#set_value)*

                if #has_listeners {
                    #vtag.add_listeners(vec![#(::std::rc::Rc::new(#listeners)),*]);
                };
                if #has_children {
                    #[allow(clippy::redundant_clone, unused_braces)]
                    #vtag.add_children(#children);
                };

                #dyn_tag_runtime_checks
                #[allow(unused_braces)]
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
    fn parse(input: ParseStream) -> ParseResult<Self> {
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if DynamicName::peek(input.cursor()).is_some() {
            DynamicName::parse(input).map(Self::Expr)
        } else {
            HtmlDashedName::parse(input).map(Self::Lit)
        }
    }
}

impl ToTokens for TagName {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            TagName::Lit(name) => name.to_tokens(tokens),
            TagName::Expr(name) => name.to_tokens(tokens),
        }
    }
}

struct HtmlTagOpen {
    lt: Token![<],
    tag_name: TagName,
    attributes: TagAttributes,
    div: Option<Token![/]>,
    gt: Token![>],
}

impl PeekValue<TagKey> for HtmlTagOpen {
    fn peek(cursor: Cursor) -> Option<TagKey> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (tag_key, cursor) = TagName::peek(cursor)?;
        if let TagKey::Lit(name) = &tag_key {
            // Avoid parsing `<key=[...]>` as an HtmlTag. It needs to be parsed as an HtmlList.
            if name.to_string() == "key" {
                let (punct, _) = cursor.punct()?;
                // ... unless it isn't followed by a '='. `<key></key>` is a valid HtmlTag!
                (punct.as_char() != '=').as_option()?;
            } else {
                non_capitalized_ascii(&name.to_string()).as_option()?;
            }
        }

        Some(tag_key)
    }
}

impl Parse for HtmlTagOpen {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lt = input.parse::<Token![<]>()?;
        let tag_name = input.parse::<TagName>()?;
        let TagSuffix { stream, div, gt } = input.parse()?;
        let mut attributes: TagAttributes = syn::parse2(stream)?;

        match &tag_name {
            TagName::Lit(name) => {
                // Don't treat value as special for non input / textarea fields
                // For dynamic tags this is done at runtime!
                match name.to_ascii_lowercase_string().as_str() {
                    "input" | "textarea" => {}
                    _ => {
                        if let Some(value) = attributes.value.take() {
                            attributes.attributes.push(TagAttribute {
                                label: HtmlDashedName::new(Ident::new("value", Span::call_site())),
                                value,
                            });
                        }
                    }
                }
            }
            TagName::Expr(name) => {
                if name.expr.is_none() {
                    return Err(syn::Error::new_spanned(
                        tag_name,
                        "this dynamic tag is missing an expression block defining its value",
                    ));
                }
            }
        }

        Ok(HtmlTagOpen {
            lt,
            tag_name,
            attributes,
            div,
            gt,
        })
    }
}

impl ToTokens for HtmlTagOpen {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlTagOpen { lt, gt, .. } = self;
        tokens.extend(quote! {#lt#gt});
    }
}

struct HtmlTagClose {
    lt: Token![<],
    div: Option<Token![/]>,
    tag_name: TagName,
    gt: Token![>],
}

impl PeekValue<TagKey> for HtmlTagClose {
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

impl Parse for HtmlTagClose {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lt = input.parse()?;
        let div = input.parse()?;
        let tag_name = input.parse()?;
        let gt = input.parse()?;

        if let TagName::Expr(name) = &tag_name {
            if let Some(expr) = &name.expr {
                return Err(syn::Error::new_spanned(
                    expr,
                    "dynamic closing tags must not have a body (hint: replace it with just `</@>`)",
                ));
            }
        }

        Ok(HtmlTagClose {
            lt,
            div,
            tag_name,
            gt,
        })
    }
}

impl ToTokens for HtmlTagClose {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlTagClose {
            lt,
            div,
            tag_name,
            gt,
        } = self;
        tokens.extend(quote! {#lt#div#tag_name#gt});
    }
}
