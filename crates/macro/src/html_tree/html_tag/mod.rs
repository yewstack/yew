mod tag_attributes;

use super::HtmlDashedName;
use super::HtmlProp as TagAttribute;
use super::HtmlPropSuffix as TagSuffix;
use super::HtmlTree;
use crate::Peek;
use boolinator::Boolinator;
use proc_macro2::{Delimiter, Span};
use quote::{quote, quote_spanned, ToTokens};
use syn::buffer::Cursor;
use syn::parse;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{Ident, Token};
use tag_attributes::{ClassesForm, TagAttributes};

pub struct HtmlTag {
    tag_name: HtmlDashedName,
    attributes: TagAttributes,
    children: Vec<HtmlTree>,
}

impl Peek<()> for HtmlTag {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlTagOpen::peek(cursor).map(|_| ()).or_else(|| {
            HtmlTagClose::peek(cursor)?;
            Some(())
        })
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

        if !HtmlTag::verify_end(input.cursor(), &open.tag_name) {
            return Err(syn::Error::new_spanned(
                open,
                "this open tag has no corresponding close tag",
            ));
        }

        let mut children: Vec<HtmlTree> = vec![];
        loop {
            if let Some(next_close_tag_name) = HtmlTagClose::peek(input.cursor()) {
                if open.tag_name.to_string() == next_close_tag_name.to_string() {
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
            kind,
            value,
            checked,
            disabled,
            selected,
            href,
            listeners,
        } = &attributes;

        let vtag = Ident::new("__yew_vtag", tag_name.span());
        let attr_labels = attributes.iter().map(|attr| attr.label.to_string());
        let attr_values = attributes.iter().map(|attr| &attr.value);
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
        let add_disabled = disabled.iter().map(|disabled| {
            quote_spanned! {disabled.span()=>
                if #disabled {
                    #vtag.add_attribute("disabled", &"true");
                }
            }
        });
        let add_selected = selected.iter().map(|selected| {
            quote_spanned! {selected.span()=>
                if #selected {
                    #vtag.add_attribute("selected", &"selected");
                }
            }
        });
        let set_classes = classes.iter().map(|classes_form| match classes_form {
            ClassesForm::Tuple(classes) => quote! {
                #vtag.add_classes(vec![#(&(#classes)),*]);
            },
            ClassesForm::Single(classes) => quote! {
                #vtag.set_classes(&(#classes));
            },
        });

        tokens.extend(quote! {{
            let mut #vtag = ::yew::virtual_dom::vtag::VTag::new(#name);
            #(#set_kind)*
            #(#set_value)*
            #(#add_href)*
            #(#set_checked)*
            #(#add_disabled)*
            #(#add_selected)*
            #(#set_classes)*
            #vtag.add_attributes(vec![#((#attr_labels.to_owned(), (#attr_values).to_string())),*]);
            #vtag.add_listeners(vec![#(::std::boxed::Box::new(#listeners)),*]);
            #vtag.add_children(vec![#(#children),*]);
            ::yew::virtual_dom::VNode::VTag(#vtag)
        }});
    }
}

impl HtmlTag {
    fn verify_end(mut cursor: Cursor, open_tag_name: &HtmlDashedName) -> bool {
        let mut tag_stack_count = 1;
        loop {
            if HtmlSelfClosingTag::peek(cursor).is_some() {
                // Do nothing
            } else if let Some(next_open_tag_name) = HtmlTagOpen::peek(cursor) {
                if open_tag_name.to_string() == next_open_tag_name.to_string() {
                    tag_stack_count += 1;
                }
            } else if let Some(next_close_tag_name) = HtmlTagClose::peek(cursor) {
                if open_tag_name.to_string() == next_close_tag_name.to_string() {
                    tag_stack_count -= 1;
                    if tag_stack_count == 0 {
                        break;
                    }
                }
            }
            if let Some((_, next)) = cursor.token_tree() {
                cursor = next;
            } else {
                break;
            }
        }

        tag_stack_count == 0
    }
}

/// This struct is only used for its Peek implementation in verify_end. Parsing
/// is done with HtmlTagOpen with `div` set to true.
struct HtmlSelfClosingTag;

impl Peek<HtmlDashedName> for HtmlSelfClosingTag {
    fn peek(cursor: Cursor) -> Option<HtmlDashedName> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (name, cursor) = cursor.ident()?;
        (name.to_string().to_lowercase() == name.to_string()).as_option()?;

        let mut extended = Vec::new();
        let mut cursor = cursor;
        loop {
            if let Some((punct, p_cursor)) = cursor.punct() {
                if punct.as_char() == '-' {
                    let (ident, i_cursor) = p_cursor.ident()?;
                    cursor = i_cursor;
                    extended.push((Token![-](Span::call_site()), ident));
                    continue;
                }
            }
            break;
        }

        let tag_name = HtmlDashedName { name, extended };
        let mut after_slash = false;
        loop {
            if let Some((punct, next_cursor)) = cursor.punct() {
                match punct.as_char() {
                    '/' => after_slash = true,
                    '>' if after_slash => return Some(tag_name),
                    '>' if !after_slash => {
                        // We need to read after the '>' for cases like this:
                        // <div onblur=|_| 2 > 1 />
                        //                   ^ in order to handle this
                        //
                        // Because those cases are NOT handled by the html!
                        // macro, so we want nice error messages.
                        //
                        // This idea here is that, in valid "JSX", after a tag,
                        // only '<' or '{ ... }' can follow. (that should be
                        // enough for reasonable cases)
                        //
                        let is_next_lt = next_cursor
                            .punct()
                            .map(|(p, _)| p.as_char() == '<')
                            .unwrap_or(false);
                        let is_next_brace = next_cursor.group(Delimiter::Brace).is_some();
                        let no_next = next_cursor.token_tree().is_none();
                        if is_next_lt || is_next_brace || no_next {
                            return None;
                        } else {
                            // TODO: Use proc-macro's Diagnostic when stable
                            eprintln!(
                                "HELP: You must wrap expressions containing \
                                 '>' in braces or parenthesis. See #523."
                            );
                        }
                    }
                    _ => after_slash = false,
                }
                cursor = next_cursor;
            } else if let Some((_, next)) = cursor.token_tree() {
                after_slash = false;
                cursor = next;
            } else {
                return None;
            }
        }
    }
}

struct HtmlTagOpen {
    lt: Token![<],
    tag_name: HtmlDashedName,
    attributes: TagAttributes,
    div: Option<Token![/]>,
    gt: Token![>],
}

impl Peek<Ident> for HtmlTagOpen {
    fn peek(cursor: Cursor) -> Option<Ident> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (ident, _) = cursor.ident()?;
        (ident.to_string().to_lowercase() == ident.to_string()).as_option()?;

        Some(ident)
    }
}

impl Parse for HtmlTagOpen {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lt = input.parse::<Token![<]>()?;
        let tag_name = input.parse::<HtmlDashedName>()?;
        let TagSuffix { stream, div, gt } = input.parse()?;
        let mut attributes: TagAttributes = parse(stream)?;

        // Don't treat value as special for non input / textarea fields
        match tag_name.to_string().as_str() {
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
    tag_name: HtmlDashedName,
    gt: Token![>],
}

impl Peek<HtmlDashedName> for HtmlTagClose {
    fn peek(cursor: Cursor) -> Option<HtmlDashedName> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '/').as_option()?;

        let (name, cursor) = cursor.ident()?;
        (name.to_string().to_lowercase() == name.to_string()).as_option()?;

        let mut extended = Vec::new();
        let mut cursor = cursor;
        loop {
            if let Some((punct, p_cursor)) = cursor.punct() {
                if punct.as_char() == '-' {
                    let (ident, i_cursor) = p_cursor.ident()?;
                    cursor = i_cursor;
                    extended.push((Token![-](Span::call_site()), ident));
                    continue;
                }
            }
            break;
        }

        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()?;

        Some(HtmlDashedName { name, extended })
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
