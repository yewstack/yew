mod tag_attributes;

use super::{
    HtmlChildrenTree, HtmlDashedName, HtmlProp as TagAttribute, HtmlPropSuffix as TagSuffix,
};
use crate::{non_capitalized_ascii, stringify, Peek, PeekValue};
use boolinator::Boolinator;
use proc_macro2::{Delimiter, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use stringify::Stringify;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Block, Expr, Ident, Token};
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            tag_name,
            attributes,
            children,
        } = self;

        let name = match &tag_name {
            TagName::Lit(name) => Stringify::from(&name.to_string()).into_token_stream(),
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

        let set_attributes = if attributes.is_empty() {
            None
        } else if attributes.iter().any(|attr| attr.question_mark.is_some()) {
            let map = Ident::new("__yew_map", tag_name.span());

            let insert_attr_it = attributes.iter().map(
                |TagAttribute {
                     label,
                     value,
                     question_mark,
                 }| {
                    let label = label.to_string();

                    if question_mark.is_some() {
                        let ident = Ident::new("__yew_attr", value.span());
                        with_optional_attr_runtime_value(
                            &ident,
                            value,
                            &quote! { #map.insert(#label, #ident); },
                        )
                    } else {
                        let sr = Stringify::from(value);
                        quote_spanned! {value.span()=>
                            #map.insert(#label, #sr);
                        }
                    }
                },
            );

            Some(quote! {
                let mut #map = ::yew::__macro::IndexMap::default();
                #(#insert_attr_it)*

                #vtag.attributes = ::yew::virtual_dom::Attributes::IndexMap(#map);
            })
        } else {
            let attr_pairs = attributes.iter().map(|TagAttribute { label, value, .. }| {
                let label = label.to_string();
                let sr = Stringify::from(value);
                quote! { (#label, #sr) }
            });
            Some(quote! {
                #vtag.attributes = ::yew::virtual_dom::Attributes::Vec(::std::vec![#(#attr_pairs),*]);
            })
        };

        let set_booleans = if booleans.is_empty() {
            None
        } else {
            let tokens = booleans
                .iter()
                .map(|TagAttribute { label, value, .. }| {
                    let label_str = label.to_string();
                    let sr = stringify::stringify_static(&label_str);
                    quote_spanned! {value.span()=> {
                        if #value {
                            #vtag.push_attribute(#label_str, #sr);
                        };
                    }}
                })
                .collect::<TokenStream>();
            Some(tokens)
        };

        let set_kind = kind.as_ref().map(|attr| {
            let value = &attr.value;
            if attr.question_mark.is_some() {
                let ident = Ident::new("__yew_v", value.span());
                with_optional_attr_runtime_value(&ident, value, &quote! { #vtag.set_kind(#ident); })
            } else {
                let sr = Stringify::from(value);
                quote_spanned! {value.span()=>
                    #vtag.set_kind(#sr);
                }
            }
        });
        let set_value = value.as_ref().map(|attr| {
            let value = &attr.value;
            if attr.question_mark.is_some() {
                quote_spanned! {value.span()=>
                    if let ::std::option::Option::Some(__yew_v) = ::std::option::Option::as_ref(&(#value)) {
                        #vtag.set_value(__yew_v);
                    };
                }
            } else {
                quote_spanned! {value.span()=>
                    #vtag.set_value(&(#value));
                }
            }
        });
        let set_checked = checked.as_ref().map(|value| {
            quote_spanned! {value.span()=>
                #vtag.set_checked(#value);
            }
        });

        let set_classes = match classes {
            Some(ClassesForm::Tuple(classes)) => Some(quote! {
                let __yew_classes = ::yew::virtual_dom::Classes::default()
                    #(.extend(#classes))*;

                if !__yew_classes.is_empty() {
                    #vtag.push_attribute("class", __yew_classes.to_string());
                };
            }),
            Some(ClassesForm::Single(classes)) => match stringify::try_stringify_expr(classes) {
                Some(s) => {
                    if s.is_empty() {
                        None
                    } else {
                        let sr = Stringify::from(&s);
                        Some(quote! {
                            #vtag.push_attribute("class", #sr);
                        })
                    }
                }
                None => Some(quote! {
                    let __yew_classes = ::std::convert::Into::<::yew::virtual_dom::Classes>::into(#classes);
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

        let set_node_ref = node_ref.as_ref().map(|node_ref| {
            quote! {
                #vtag.node_ref = #node_ref;
            }
        });
        let set_key = key.as_ref().map(|key| {
            quote! {
                #vtag.key = Some(::std::convert::Into::<::yew::virtual_dom::Key>::into(#key));
            }
        });

        let add_listeners = if listeners.is_empty() {
            None
        } else if listeners.iter().any(|attr| attr.question_mark.is_some()) {
            let add_listeners = listeners.iter().map(
                |TagAttribute {
                     label,
                     question_mark,
                     value,
                 }| {
                    let name = &label.name;

                    if question_mark.is_some() {
                        let ident = Ident::new("__yew_listener", name.span());
                        quote_spanned! {value.span()=>
                            let #ident = ::std::option::Option::map(#value, |#ident| {
                                ::std::rc::Rc::new(::yew::html::#name::Wrapper::new(
                                    <::yew::virtual_dom::VTag as ::yew::virtual_dom::Transformer<_, _>>::transform(#ident)
                                ))
                            });
                            if let ::std::option::Option::Some(#ident) = #ident {
                                #vtag.add_listener(#ident);
                            };
                        }
                    } else {
                        quote_spanned! {name.span()=>
                            #vtag.add_listener(::std::rc::Rc::new(::yew::html::#name::Wrapper::new(
                                <::yew::virtual_dom::VTag as ::yew::virtual_dom::Transformer<_, _>>::transform(#value),
                            )));
                        }
                    }
                },
            ).collect();

            Some(add_listeners)
        } else {
            let listeners_it = listeners.iter().map(|TagAttribute {
                label,
                value,
                ..
            }| {
                let name = &label.name;
                quote_spanned! {label.span()=>
                    ::std::rc::Rc::new(::yew::html::#name::Wrapper::new(
                        <::yew::virtual_dom::VTag as ::yew::virtual_dom::Transformer<_, _>>::transform(#value)
                    ))
                }
            });

            Some(quote! {
                #vtag.add_listeners(::std::vec![#(#listeners_it),*]);
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

        tokens.extend(quote_spanned! {tag_name.span()=>
            {
                #[allow(unused_braces)]
                let mut #vtag = ::yew::virtual_dom::VTag::new(#name);

                #set_node_ref
                #set_key
                #set_kind
                #set_attributes
                #set_booleans
                #set_classes
                #set_checked
                #set_value

                #add_listeners
                #add_children

                #dyn_tag_runtime_checks
                #[allow(unused_braces)]
                ::yew::virtual_dom::VNode::from(#vtag)
            }
        });
    }
}

fn with_optional_attr_runtime_value(
    ident: &Ident,
    value: &Expr,
    tokens: &TokenStream,
) -> TokenStream {
    let sr = stringify::stringify_at_runtime(&ident);
    quote_spanned! {value.span()=>
        let #ident = ::std::option::Option::map(#value, |#ident| {
            #sr
        });
        if let ::std::option::Option::Some(#ident) = #ident {
            #tokens
        };
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
    fn parse(input: ParseStream) -> ParseResult<Self> {
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
                        if let Some(attr) = attributes.value.take() {
                            attributes.attributes.push(TagAttribute {
                                label: HtmlDashedName::new(Ident::new("value", Span::call_site())),
                                question_mark: attr.question_mark,
                                value: attr.value,
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let HtmlTagClose {
            lt,
            div,
            tag_name,
            gt,
        } = self;
        tokens.extend(quote! {#lt#div#tag_name#gt});
    }
}
