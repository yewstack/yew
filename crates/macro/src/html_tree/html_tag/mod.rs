mod tag_attributes;

use super::HtmlProp as TagAttribute;
use super::HtmlPropLabel as TagLabel;
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
    ident: Ident,
    attributes: TagAttributes,
    children: Vec<HtmlTree>,
}

impl Peek<()> for HtmlTag {
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
                ident: open.ident,
                attributes: open.attributes,
                children: Vec::new(),
            });
        }

        if !HtmlTag::verify_end(input.cursor(), &open.ident) {
            return Err(syn::Error::new_spanned(
                open,
                "this open tag has no corresponding close tag",
            ));
        }

        let mut children: Vec<HtmlTree> = vec![];
        loop {
            if let Some(next_close_ident) = HtmlTagClose::peek(input.cursor()) {
                if open.ident == next_close_ident {
                    break;
                }
            }

            children.push(input.parse()?);
        }

        input.parse::<HtmlTagClose>()?;

        Ok(HtmlTag {
            ident: open.ident,
            attributes: open.attributes,
            children,
        })
    }
}

impl ToTokens for HtmlTag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlTag {
            ident,
            attributes,
            children,
        } = self;

        let name = ident.to_string();

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

        let vtag = Ident::new("__yew_vtag", ident.span());
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
    fn verify_end(mut cursor: Cursor, open_ident: &Ident) -> bool {
        let mut tag_stack_count = 1;
        loop {
            if HtmlSelfClosingTag::peek(cursor).is_some() {
                // Do nothing
            } else if let Some(next_open_ident) = HtmlTagOpen::peek(cursor) {
                if open_ident.to_string() == next_open_ident.to_string() {
                    tag_stack_count += 1;
                }
            } else if let Some(next_close_ident) = HtmlTagClose::peek(cursor) {
                if open_ident.to_string() == next_close_ident.to_string() {
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

impl Peek<Ident> for HtmlSelfClosingTag {
    fn peek(cursor: Cursor) -> Option<Ident> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (ident, cursor) = cursor.ident()?;
        (ident.to_string().to_lowercase() == ident.to_string()).as_option()?;

        let mut cursor = cursor;
        let mut after_slash = false;
        loop {
            if let Some((punct, next_cursor)) = cursor.punct() {
                match punct.as_char() {
                    '/' => after_slash = true,
                    '>' if after_slash => return Some(ident),
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
    ident: Ident,
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
        let ident = input.parse::<Ident>()?;
        let TagSuffix { stream, div, gt } = input.parse()?;
        let mut attributes: TagAttributes = parse(stream)?;

        // Don't treat value as special for non input / textarea fields
        match ident.to_string().as_str() {
            "input" | "textarea" => {}
            _ => {
                if let Some(value) = attributes.value.take() {
                    attributes.attributes.push(TagAttribute {
                        label: TagLabel::new(Ident::new("value", Span::call_site())),
                        value,
                    });
                }
            }
        }

        Ok(HtmlTagOpen {
            lt,
            ident,
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
    ident: Ident,
    gt: Token![>],
}

impl Peek<Ident> for HtmlTagClose {
    fn peek(cursor: Cursor) -> Option<Ident> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '<').as_option()?;

        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '/').as_option()?;

        let (ident, cursor) = cursor.ident()?;
        (ident.to_string().to_lowercase() == ident.to_string()).as_option()?;

        let (punct, _) = cursor.punct()?;
        (punct.as_char() == '>').as_option()?;

        Some(ident)
    }
}

impl Parse for HtmlTagClose {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(HtmlTagClose {
            lt: input.parse()?,
            div: input.parse()?,
            ident: input.parse()?,
            gt: input.parse()?,
        })
    }
}

impl ToTokens for HtmlTagClose {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlTagClose { lt, div, ident, gt } = self;
        tokens.extend(quote! {#lt#div#ident#gt});
    }
}
