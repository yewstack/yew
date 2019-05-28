use super::HtmlProp as TagAttribute;
use super::HtmlPropSuffix as TagSuffix;
use super::HtmlTree;
use crate::Peek;
use boolinator::Boolinator;
use lazy_static::lazy_static;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use std::collections::HashMap;
use syn::buffer::Cursor;
use syn::parse;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Expr, ExprClosure, ExprTuple, Ident, Token};

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
            href,
            listeners,
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
        let add_href = href.iter().map(|href| {
            quote! {
                let __yew_href: $crate::html::Href = #href.into();
                __yew_vtag.add_attribute("href", &__yew_href);
            }
        });
        let set_classes = classes.iter().map(|classes_form| match classes_form {
            ClassesForm::Tuple(classes) => quote! { #(__yew_vtag.add_class(&(#classes));)* },
            ClassesForm::Single(classes) => quote! {
                __yew_vtag.set_classes(&(#classes));
            },
        });

        tokens.extend(quote! {{
            let mut __yew_vtag = $crate::virtual_dom::vtag::VTag::new(#tag_name);
            #(#set_classes)*
            #(__yew_vtag.add_attribute(#attr_names, &(#attr_values));)*
            #(__yew_vtag.add_listener(::std::boxed::Box::new(#listeners));)*
            #(#set_kind)*
            #(#set_value)*
            #(#set_checked)*
            #(#add_disabled)*
            #(#add_selected)*
            #(#add_href)*
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
    listeners: Vec<TokenStream>,
    classes: Option<ClassesForm>,
    value: Option<Expr>,
    kind: Option<Expr>,
    checked: Option<Expr>,
    disabled: Option<Expr>,
    selected: Option<Expr>,
    href: Option<Expr>,
}

enum ClassesForm {
    Tuple(Vec<Expr>),
    Single(Expr),
}

struct TagListener {
    name: Ident,
    handler: Expr,
    event_name: String,
}

lazy_static! {
    static ref LISTENER_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("onclick", "ClickEvent");
        m.insert("ondoubleclick", "DoubleClickEvent");
        m.insert("onkeypress", "KeyPressEvent");
        m.insert("onkeydown", "KeyDownEvent");
        m.insert("onkeyup", "KeyUpEvent");
        m.insert("onmousedown", "MouseDownEvent");
        m.insert("onmousemove", "MouseMoveEvent");
        m.insert("onmouseout", "MouseOutEvent");
        m.insert("onmouseenter", "MouseEnterEvent");
        m.insert("onmouseleave", "MouseLeaveEvent");
        m.insert("onmousewheel", "MouseWheelEvent");
        m.insert("onmouseover", "MouseOverEvent");
        m.insert("onmouseup", "MouseUpEvent");
        m.insert("ongotpointercapture", "GotPointerCaptureEvent");
        m.insert("onlostpointercapture", "LostPointerCaptureEvent");
        m.insert("onpointercancel", "PointerCancelEvent");
        m.insert("onpointerdown", "PointerDownEvent");
        m.insert("onpointerenter", "PointerEnterEvent");
        m.insert("onpointerleave", "PointerLeaveEvent");
        m.insert("onpointermove", "PointerMoveEvent");
        m.insert("onpointerout", "PointerOutEvent");
        m.insert("onpointerover", "PointerOverEvent");
        m.insert("onpointerup", "PointerUpEvent");
        m.insert("onscroll", "ScrollEvent");
        m.insert("onblur", "BlurEvent");
        m.insert("onfocus", "FocusEvent");
        m.insert("onsubmit", "SubmitEvent");
        m.insert("oninput", "InputData");
        m.insert("onchange", "ChangeData");
        m.insert("ondrag", "DragEvent");
        m.insert("ondragstart", "DragStartEvent");
        m.insert("ondragend", "DragEndEvent");
        m.insert("ondragenter", "DragEnterEvent");
        m.insert("ondragleave", "DragLeaveEvent");
        m.insert("ondragover", "DragOverEvent");
        m.insert("ondragexit", "DragExitEvent");
        m.insert("ondrop", "DragDropEvent");
        m.insert("oncontextmenu", "ContextMenuEvent");
        m
    };
}

impl TagAttributes {
    fn drain_listeners(attrs: &mut Vec<TagAttribute>) -> Vec<TagListener> {
        let mut i = 0;
        let mut drained = Vec::new();
        while i < attrs.len() {
            let name_str = attrs[i].name.to_string();
            if let Some(event_type) = LISTENER_MAP.get(&name_str.as_str()) {
                let TagAttribute { name, value } = attrs.remove(i);
                drained.push(TagListener {
                    name,
                    handler: value,
                    event_name: event_type.to_owned().to_string(),
                });
            } else {
                i += 1;
            }
        }
        drained
    }

    fn remove_attr(attrs: &mut Vec<TagAttribute>, name: &str) -> Option<Expr> {
        let mut i = 0;
        while i < attrs.len() {
            if attrs[i].name.to_string() == name {
                return Some(attrs.remove(i).value);
            } else {
                i += 1;
            }
        }
        None
    }

    fn map_classes(class_expr: Expr) -> ClassesForm {
        match class_expr {
            Expr::Tuple(ExprTuple { elems, .. }) => ClassesForm::Tuple(elems.into_iter().collect()),
            expr @ _ => ClassesForm::Single(expr),
        }
    }

    fn map_listener(listener: TagListener) -> ParseResult<TokenStream> {
        let TagListener {
            name,
            event_name,
            handler,
        } = listener;

        match handler {
            Expr::Closure(ExprClosure { inputs, body, .. }) => {
                if inputs.len() != 1 {
                    return Err(syn::Error::new_spanned(
                        inputs,
                        "there must be one closure argument",
                    ));
                }
                let var = match inputs.first().unwrap().into_value() {
                    syn::FnArg::Inferred(pat) => pat,
                    _ => return Err(syn::Error::new_spanned(inputs, "invalid closure argument")),
                };
                let handler =
                    Ident::new(&format!("__yew_{}_handler", name.to_string()), name.span());
                let listener =
                    Ident::new(&format!("__yew_{}_listener", name.to_string()), name.span());
                let segment = syn::PathSegment {
                    ident: Ident::new(&event_name, name.span()),
                    arguments: syn::PathArguments::None,
                };
                let var_type = quote! { $crate::events::#segment };
                let wrapper_type = quote! { $crate::html::#name::Wrapper };
                let listener_stream = quote_spanned! {name.span()=> {
                    let #handler = move | #var: #var_type | #body;
                    let #listener = #wrapper_type::from(#handler);
                    #listener
                }};

                Ok(listener_stream)
            }
            _ => Err(syn::Error::new_spanned(
                &name,
                format!("{} attribute value should be a closure", name),
            )),
        }
    }
}

impl Parse for TagAttributes {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut attributes: Vec<TagAttribute> = Vec::new();
        while TagAttribute::peek(input.cursor()).is_some() {
            attributes.push(input.parse::<TagAttribute>()?);
        }

        let mut listeners = Vec::new();
        for listener in TagAttributes::drain_listeners(&mut attributes) {
            listeners.push(TagAttributes::map_listener(listener)?);
        }

        // Multiple listener attributes are allowed, but no others
        attributes.sort_by(|a, b| a.name.to_string().partial_cmp(&b.name.to_string()).unwrap());
        let mut i = 0;
        while i + 1 < attributes.len() {
            if attributes[i].name.to_string() == attributes[i + 1].name.to_string() {
                let name = &attributes[i + 1].name;
                return Err(syn::Error::new_spanned(
                    name,
                    format!("only one `{}` attribute allowed", name),
                ));
            }
            i += 1;
        }

        let classes =
            TagAttributes::remove_attr(&mut attributes, "class").map(TagAttributes::map_classes);
        let value = TagAttributes::remove_attr(&mut attributes, "value");
        let kind = TagAttributes::remove_attr(&mut attributes, "type");
        let checked = TagAttributes::remove_attr(&mut attributes, "checked");
        let disabled = TagAttributes::remove_attr(&mut attributes, "disabled");
        let selected = TagAttributes::remove_attr(&mut attributes, "selected");
        let href = TagAttributes::remove_attr(&mut attributes, "href");

        Ok(TagAttributes {
            attributes,
            classes,
            listeners,
            value,
            kind,
            checked,
            disabled,
            selected,
            href,
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
