mod tag_attributes;

use super::HtmlDashedName as TagName;
use super::HtmlProp as TagAttribute;
use super::HtmlPropSuffix as TagSuffix;
use super::HtmlTree;
use crate::{non_capitalized_ascii, Peek, PeekValue};
use boolinator::Boolinator;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Ident, Token};
use tag_attributes::{ClassesForm, TagAttributes};

pub struct HtmlTag {
    tag_name: TagName,
    attributes: TagAttributes,
    children: Vec<HtmlTree>,
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
                    "this close tag has no corresponding open tag",
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
                children: Vec::new(),
            });
        }

        let mut children: Vec<HtmlTree> = vec![];
        loop {
            if input.is_empty() {
                return Err(syn::Error::new_spanned(
                    open,
                    "this open tag has no corresponding close tag",
                ));
            }
            if let Some(next_close_tag_name) = HtmlTagClose::peek(input.cursor()) {
                if open.tag_name == next_close_tag_name {
                    break;
                }
            }

            children.push(input.parse()?);
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

        let name = tag_name.to_string();

        let TagAttributes {
            classes,
            attributes,
            booleans,
            kind,
            value,
            checked,
            node_ref,
            key,
            href,
            listeners,
        } = &attributes;

        let vtag = Ident::new("__yew_vtag", tag_name.span());
        let attr_pairs = attributes.iter().map(|TagAttribute { label, value }| {
            let label_str = label.to_string();
            quote_spanned! {value.span() => (#label_str.to_owned(), (#value).to_string()) }
        });
        let set_booleans = booleans.iter().map(|TagAttribute { label, value }| {
            let label_str = label.to_string();
            quote_spanned! {value.span() =>
                if #value {
                    #vtag.add_attribute(&#label_str, &#label_str);
                }
            }
        });
        let set_kind = kind.iter().map(|kind| {
            quote_spanned! {kind.span()=> #vtag.set_kind(&(#kind)); }
        });
        let set_value = value.iter().map(|value| {
            quote_spanned! {value.span()=> #vtag.set_value(&(#value)); }
        });
        let add_href = href.iter().map(|href| {
            quote_spanned! {href.span()=>
                let __yew_href: ::yew::html::Href = (#href).into();
                #vtag.add_attribute("href", &__yew_href);
            }
        });
        let set_checked = checked.iter().map(|checked| {
            quote_spanned! {checked.span()=> #vtag.set_checked(#checked); }
        });
        let set_classes = classes.iter().map(|classes_form| match classes_form {
            ClassesForm::Tuple(classes) => quote! {
                #vtag.add_classes(vec![#(&(#classes)),*]);
            },
            ClassesForm::Single(classes) => quote! {
                #vtag.set_classes(#classes);
            },
        });
        let set_node_ref = node_ref.iter().map(|node_ref| {
            quote! {
                #vtag.node_ref = #node_ref;
            }
        });
        let set_key = key.iter().map(|key| {
            quote! {
                #vtag.key = Some(#key);
            }
        });
        let listeners = listeners.iter().map(|listener| {
            let name = &listener.label.name;
            let callback = &listener.value;

            quote_spanned! {name.span()=> {
                ::yew::html::#name::Wrapper::new(
                    <::yew::virtual_dom::VTag as ::yew::virtual_dom::Transformer<_, _>>::transform(
                        #callback
                    )
                )
            }}
        });

        tokens.extend(quote! {{
            let mut #vtag = ::yew::virtual_dom::VTag::new(#name);
            #(#set_kind)*
            #(#set_value)*
            #(#add_href)*
            #(#set_checked)*
            #(#set_booleans)*
            #(#set_classes)*
            #(#set_node_ref)*
            #(#set_key)*
            #vtag.add_attributes(vec![#(#attr_pairs),*]);
            #vtag.add_listeners(vec![#(::std::rc::Rc::new(#listeners)),*]);
            #vtag.add_children(vec![#(#children),*]);
            ::yew::virtual_dom::VNode::from(#vtag)
        }});
    }
}

struct HtmlTagOpen {
    lt: Token![<],
    tag_name: TagName,
    attributes: TagAttributes,
    div: Option<Token![/]>,
    gt: Token![>],
}

impl PeekValue<TagName> for HtmlTagOpen {
    fn peek(cursor: Cursor) -> Option<TagName> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (name, cursor) = TagName::peek(cursor)?;
        if name.to_string() == "key" {
            let (punct, _) = cursor.punct()?;
            if punct.as_char() == '=' {
                None
            } else {
                Some(name)
            }
        } else {
            non_capitalized_ascii(&name.to_string()).as_option()?;
            Some(name)
        }
    }
}

impl Parse for HtmlTagOpen {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lt = input.parse::<Token![<]>()?;
        let tag_name = input.parse::<TagName>()?;
        let TagSuffix { stream, div, gt } = input.parse()?;
        let mut attributes: TagAttributes = parse(stream)?;

        // Don't treat value as special for non input / textarea fields
        match tag_name.to_string().as_str() {
            "input" | "textarea" => {}
            _ => {
                if let Some(value) = attributes.value.take() {
                    attributes.attributes.push(TagAttribute {
                        label: TagName::new(Ident::new("value", Span::call_site())),
                        value,
                    });
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

impl PeekValue<TagName> for HtmlTagClose {
    fn peek(cursor: Cursor) -> Option<TagName> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '/').as_option()?;

        let (name, cursor) = TagName::peek(cursor)?;
        non_capitalized_ascii(&name.to_string()).as_option()?;

        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()?;

        Some(name)
    }
}

impl Parse for HtmlTagClose {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(HtmlTagClose {
            lt: input.parse()?,
            div: input.parse()?,
            tag_name: input.parse()?,
            gt: input.parse()?,
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
