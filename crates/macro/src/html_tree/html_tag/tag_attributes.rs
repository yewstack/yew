use crate::html_tree::HtmlProp as TagAttribute;
use crate::PeekValue;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Expr, ExprClosure, ExprTuple, Ident, Pat};

pub struct TagAttributes {
    pub attributes: Vec<TagAttribute>,
    pub listeners: Vec<TokenStream>,
    pub classes: Option<ClassesForm>,
    pub value: Option<Expr>,
    pub kind: Option<Expr>,
    pub checked: Option<Expr>,
    pub disabled: Option<Expr>,
    pub selected: Option<Expr>,
    pub href: Option<Expr>,
}

pub enum ClassesForm {
    Tuple(Vec<Expr>),
    Single(Expr),
}

pub struct TagListener {
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
        m.insert("ontouchcancel", "TouchCancel");
        m.insert("ontouchend", "TouchEnd");
        m.insert("ontouchenter", "TouchEnter");
        m.insert("ontouchmove", "TouchMove");
        m.insert("ontouchstart", "TouchStart");
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
            let name_str = attrs[i].label.to_string();
            if let Some(event_type) = LISTENER_MAP.get(&name_str.as_str()) {
                let TagAttribute { label, value } = attrs.remove(i);
                drained.push(TagListener {
                    name: label.name,
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
            if attrs[i].label.to_string() == name {
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
            expr => ClassesForm::Single(expr),
        }
    }

    fn map_listener(listener: TagListener) -> ParseResult<TokenStream> {
        let TagListener {
            name,
            event_name,
            handler,
        } = listener;

        match handler {
            Expr::Closure(closure) => {
                let ExprClosure {
                    inputs,
                    body,
                    or1_token,
                    or2_token,
                    ..
                } = closure;

                let or_span = quote! {#or1_token#or2_token};
                if inputs.len() != 1 {
                    return Err(syn::Error::new_spanned(
                        or_span,
                        "there must be one closure argument",
                    ));
                }

                let var = match inputs.first().unwrap() {
                    Pat::Ident(pat) => Ok(pat.into_token_stream()),
                    Pat::Wild(pat) => Ok(pat.into_token_stream()),
                    _ => Err(syn::Error::new_spanned(or_span, "invalid closure argument")),
                }?;
                let handler =
                    Ident::new(&format!("__yew_{}_handler", name.to_string()), name.span());
                let listener =
                    Ident::new(&format!("__yew_{}_listener", name.to_string()), name.span());
                let segment = syn::PathSegment {
                    ident: Ident::new(&event_name, name.span()),
                    arguments: syn::PathArguments::None,
                };
                let var_type = quote! { ::yew::events::#segment };
                let wrapper_type = quote! { ::yew::html::#name::Wrapper };
                let listener_stream = quote_spanned! {name.span()=> {
                    let #handler = move | #var: #var_type | #body;
                    let #listener = #wrapper_type::from(#handler);
                    #listener
                }};

                Ok(listener_stream)
            }
            _ => Err(syn::Error::new_spanned(
                &name,
                format!("`{}` attribute value should be a closure", name),
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
        attributes.sort_by(|a, b| {
            a.label
                .to_string()
                .partial_cmp(&b.label.to_string())
                .unwrap()
        });
        let mut i = 0;
        while i + 1 < attributes.len() {
            if attributes[i].label.to_string() == attributes[i + 1].label.to_string() {
                let label = &attributes[i + 1].label;
                return Err(syn::Error::new_spanned(
                    label,
                    format!("only one `{}` attribute allowed", label),
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
