use super::HtmlProp as TagAttribute;
use super::HtmlPropSuffix as TagSuffix;
use super::HtmlTree;
use crate::Peek;
use boolinator::Boolinator;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Expr, ExprTuple, Ident, Token};

pub struct HtmlTag {
    open: HtmlTagOpen,
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
                open,
                children: Vec::new(),
            });
        }

        let mut cursor = input.cursor();
        let mut tag_stack_count = 1;
        loop {
            if let Some(next_open_ident) = HtmlTagOpen::peek(cursor) {
                if open.ident.to_string() == next_open_ident.to_string() {
                    tag_stack_count += 1;
                }
            } else if let Some(next_close_ident) = HtmlTagClose::peek(cursor) {
                if open.ident.to_string() == next_close_ident.to_string() {
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

        if tag_stack_count > 0 {
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

        Ok(HtmlTag { open, children })
    }
}

impl ToTokens for HtmlTag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let HtmlTag { open, children } = self;
        let tag_name = open.ident.to_string();
        let TagAttributes {
            classes,
            attributes,
            kind,
            value,
            checked,
            disabled,
            selected,
        } = &open.attributes;
        let attr_names = attributes.iter().map(|attr| attr.name.to_string());
        let attr_values = attributes.iter().map(|attr| &attr.value);
        let set_kind = kind.iter().map(|kind| {
            quote! { __yew_vtag.set_kind(&(#kind)); }
        });
        let set_value = value.iter().map(|value| {
            quote! { __yew_vtag.set_value(&(#value)); }
        });
        let set_checked = checked.iter().map(|checked| {
            quote! { __yew_vtag.set_checked(#checked); }
        });
        let add_disabled = disabled.iter().map(|disabled| {
            quote! {
                if #disabled {
                    __yew_vtag.add_attribute("disabled", &"true");
                }
            }
        });
        let add_selected = selected.iter().map(|selected| {
            quote! {
                if #selected {
                    __yew_vtag.add_attribute("selected", &"selected");
                }
            }
        });

        tokens.extend(quote! {{
            let mut __yew_vtag = $crate::virtual_dom::vtag::VTag::new(#tag_name);
            #(__yew_vtag.add_class(&(#classes));)*
            #(__yew_vtag.add_attribute(#attr_names, &(#attr_values));)*
            #(#set_kind)*
            #(#set_value)*
            #(#set_checked)*
            #(#add_disabled)*
            #(#add_selected)*
            #(__yew_vtag.add_child(#children);)*
            __yew_vtag
        }});
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

struct TagAttributes {
    attributes: Vec<TagAttribute>,
    classes: Vec<Expr>,
    value: Option<Expr>,
    kind: Option<Expr>,
    checked: Option<Expr>,
    disabled: Option<Expr>,
    selected: Option<Expr>,
}

impl TagAttributes {
    fn drain_attr(attrs: &mut Vec<TagAttribute>, name: &str) -> Vec<TagAttribute> {
        let mut i = 0;
        let mut drained = Vec::new();
        while i < attrs.len() {
            if attrs[i].name.to_string() == name {
                drained.push(attrs.remove(i));
            } else {
                i += 1;
            }
        }
        drained
    }

    fn remove_attr(attrs: &mut Vec<TagAttribute>, name: &str) -> ParseResult<Option<Expr>> {
        let drained = TagAttributes::drain_attr(attrs, name);
        let attr_expr = if drained.len() == 1 {
            Some(drained[0].value.clone())
        } else if drained.len() > 1 {
            return Err(syn::Error::new_spanned(
                &drained[1].name,
                format!("only one {} allowed", name),
            ));
        } else {
            None
        };

        Ok(attr_expr)
    }
}

impl Parse for TagAttributes {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut attributes: Vec<TagAttribute> = Vec::new();
        while TagAttribute::peek(input.cursor()).is_some() {
            attributes.push(input.parse::<TagAttribute>()?);
        }

        let mut classes: Vec<Expr> = Vec::new();
        TagAttributes::drain_attr(&mut attributes, "class")
            .into_iter()
            .for_each(|TagAttribute { value, .. }| match value {
                Expr::Tuple(ExprTuple { elems, .. }) => {
                    elems.into_iter().for_each(|expr| classes.push(expr))
                }
                expr @ _ => classes.push(expr),
            });

        let value = TagAttributes::remove_attr(&mut attributes, "value")?;
        let kind = TagAttributes::remove_attr(&mut attributes, "type")?;
        let checked = TagAttributes::remove_attr(&mut attributes, "checked")?;
        let disabled = TagAttributes::remove_attr(&mut attributes, "disabled")?;
        let selected = TagAttributes::remove_attr(&mut attributes, "selected")?;

        attributes.sort_by(|a, b| a.name.to_string().partial_cmp(&b.name.to_string()).unwrap());
        let mut i = 0;
        while i + 1 < attributes.len() {
            if attributes[i].name.to_string() == attributes[i + 1].name.to_string() {
                let name = &attributes[i + 1].name;
                return Err(syn::Error::new_spanned(
                    name,
                    format!("only one {} allowed", name),
                ));
            }
            i += 1;
        }

        Ok(TagAttributes {
            attributes,
            classes,
            value,
            kind,
            checked,
            disabled,
            selected,
        })
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
