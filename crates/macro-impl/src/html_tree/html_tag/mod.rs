mod tag_attributes;

use super::HtmlProp as TagAttribute;
use super::HtmlPropSuffix as TagSuffix;
use super::HtmlTree;
use crate::Peek;
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
    ident: Ident,
    attributes: TagAttributes,
    children: Vec<HtmlTree>,
}

impl Peek<()> for HtmlTag {
    fn peek(cursor: Cursor) -> Option<()> {
        HtmlTagOpen::peek(cursor)
            .or(HtmlTagClose::peek(cursor))
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
                if open.ident.to_string() == next_close_ident.to_string() {
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
        let attr_names = attributes.iter().map(|attr| attr.name.to_string());
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
            #vtag.add_attributes(vec![#((#attr_names.to_owned(), (#attr_values).to_string())),*]);
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
            if let Some(next_open_ident) = HtmlTagOpen::peek(cursor) {
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
                        name: Ident::new("value", Span::call_site()),
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
