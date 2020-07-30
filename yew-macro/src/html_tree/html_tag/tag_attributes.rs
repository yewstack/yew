use crate::html_tree::HtmlProp as TagAttribute;
use crate::PeekValue;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::iter::FromIterator;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{Expr, ExprTuple};

pub struct TagAttributes {
    pub attributes: Vec<TagAttribute>,
    pub listeners: Vec<TagAttribute>,
    pub classes: Option<ClassesForm>,
    pub booleans: Vec<TagAttribute>,
    pub value: Option<TagAttribute>,
    pub kind: Option<TagAttribute>,
    pub checked: Option<TagAttribute>,
    pub node_ref: Option<Expr>,
    pub key: Option<Expr>,
    pub href: Option<TagAttribute>,
}

pub enum ClassesForm {
    Tuple(Vec<Expr>),
    Single(Box<Expr>),
}

lazy_static! {
    static ref BOOLEAN_SET: HashSet<&'static str> = {
        HashSet::from_iter(
            vec![
                "async",
                "autofocus",
                "controls",
                "default",
                "defer",
                "disabled",
                "hidden",
                "ismap",
                "loop",
                "multiple",
                "muted",
                "novalidate",
                "open",
                "readonly",
                "required",
                "selected",
            ]
            .into_iter(),
        )
    };
}

lazy_static! {
    static ref LISTENER_SET: HashSet<&'static str> = {
        HashSet::from_iter(
            vec![
                // Living Standard
                // From: https://html.spec.whatwg.org/multipage/webappapis.html#globaleventhandlers
                "onabort",
                "onauxclick",
                "onblur",
                "oncancel",
                "oncanplay",
                "oncanplaythrough",
                "onchange",
                "onclick",
                "onclose",
                "oncontextmenu",
                "oncuechange",
                "ondblclick",
                "ondrag",
                "ondragend",
                "ondragenter",
                "ondragexit",
                "ondragleave",
                "ondragover",
                "ondragstart",
                "ondrop",
                "ondurationchange",
                "onemptied",
                "onended",
                "onerror",
                "onfocus",
                "onformdata",
                "oninput",
                "oninvalid",
                "onkeydown",
                "onkeypress",
                "onkeyup",
                "onload",
                "onloadeddata",
                "onloadedmetadata",
                "onloadstart",
                "onmousedown",
                "onmouseenter",
                "onmouseleave",
                "onmousemove",
                "onmouseout",
                "onmouseover",
                "onmouseup",
                "onpause",
                "onplay",
                "onplaying",
                "onprogress",
                "onratechange",
                "onreset",
                "onresize",
                "onscroll",
                "onsecuritypolicyviolation",
                "onseeked",
                "onseeking",
                "onselect",
                "onslotchange",
                "onstalled",
                "onsubmit",
                "onsuspend",
                "ontimeupdate",
                "ontoggle",
                "onvolumechange",
                "onwaiting",
                "onwheel",

                // Standard HTML Document and Element
                // From: https://html.spec.whatwg.org/multipage/webappapis.html#documentandelementeventhandlers
                "oncopy",
                "oncut",
                "onpaste",

                // Others
                // From: https://developer.mozilla.org/en-US/docs/Web/API/GlobalEventHandlers
                "onanimationcancel",
                "onanimationend",
                "onanimationiteration",
                "onanimationstart",
                "ongotpointercapture",
                "onloadend",
                "onlostpointercapture",
                "onpointercancel",
                "onpointerdown",
                "onpointerenter",
                "onpointerleave",
                "onpointerlockchange",
                "onpointerlockerror",
                "onpointermove",
                "onpointerout",
                "onpointerover",
                "onpointerup",
                "onselectionchange",
                "onselectstart",
                "onshow",
                "ontouchcancel",
                "ontouchend",
                "ontouchmove",
                "ontouchstart",
                "ontransitioncancel",
                "ontransitionend",
                "ontransitionrun",
                "ontransitionstart",
            ]
            .into_iter(),
        )
    };
}

#[cfg(feature = "std_web")]
lazy_static! {
    static ref UNSUPPORTED_LISTENER_SET: HashSet<&'static str> = {
        HashSet::from_iter(
            vec![
                "oncancel",
                "oncanplay",
                "oncanplaythrough",
                "onclose",
                "oncuechange",
                "ondurationchange",
                "onemptied",
                "onended",
                "onformdata",
                "oninvalid",
                "onloadeddata",
                "onloadedmetadata",
                "onpause",
                "onplay",
                "onplaying",
                "onratechange",
                "onreset",
                "onsecuritypolicyviolation",
                "onseeked",
                "onseeking",
                "onselect",
                "onstalled",
                "onsuspend",
                "ontimeupdate",
                "ontoggle",
                "onvolumechange",
                "onwaiting",
                "oncopy",
                "oncut",
                "onpaste",
                "onanimationcancel",
                "onanimationend",
                "onanimationiteration",
                "onanimationstart",
                "onselectstart",
                "onshow",
                "ontransitioncancel",
                "ontransitionend",
                "ontransitionrun",
                "ontransitionstart",
            ]
            .into_iter(),
        )
    };
}

impl TagAttributes {
    fn drain_listeners(attrs: &mut Vec<TagAttribute>) -> Vec<TagAttribute> {
        let mut i = 0;
        let mut drained = Vec::new();
        while i < attrs.len() {
            let name_str = attrs[i].label.to_string();
            if LISTENER_SET.contains(&name_str.as_str()) {
                drained.push(attrs.remove(i));
            } else {
                i += 1;
            }
        }
        drained
    }

    fn drain_boolean(attrs: &mut Vec<TagAttribute>) -> Vec<TagAttribute> {
        let mut i = 0;
        let mut drained = Vec::new();
        while i < attrs.len() {
            let name_str = attrs[i].label.to_string();
            if BOOLEAN_SET.contains(&name_str.as_str()) {
                drained.push(attrs.remove(i));
            } else {
                i += 1;
            }
        }
        drained
    }

    fn remove_attr(attrs: &mut Vec<TagAttribute>, name: &str) -> Option<TagAttribute> {
        let mut i = 0;
        while i < attrs.len() {
            if attrs[i].label.to_string() == name {
                return Some(attrs.remove(i));
            } else {
                i += 1;
            }
        }
        None
    }

    fn map_classes(class_expr: Expr) -> ClassesForm {
        match class_expr {
            Expr::Tuple(ExprTuple { elems, .. }) => ClassesForm::Tuple(elems.into_iter().collect()),
            expr => ClassesForm::Single(Box::new(expr)),
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
            #[cfg(feature = "std_web")]
            {
                let label = &listener.label;
                if UNSUPPORTED_LISTENER_SET.contains(&label.to_string().as_str()) {
                    return Err(syn::Error::new_spanned(
                        &label,
                        format!(
                            "the listener `{}` is only available when using web-sys",
                            &label
                        ),
                    ));
                }
            }

            listeners.push(listener);
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
                    format!("the attribute `{}` can only be specified once", label),
                ));
            }
            i += 1;
        }
        let booleans = TagAttributes::drain_boolean(&mut attributes);
        for boolean in &booleans {
            if boolean.question_mark.is_some() {
                return Err(syn::Error::new_spanned(
                    &boolean.label,
                    format!(
                        "The '{}' attribute does not support being used as an optional attribute",
                        boolean.label
                    ),
                ));
            }
        }

        let classes = TagAttributes::remove_attr(&mut attributes, "class");
        if let Some(classes) = classes.as_ref() {
            if classes.question_mark.is_some() {
                return Err(syn::Error::new_spanned(
                    &classes.label,
                    "the 'class' attribute does not support being used as an optional attribute",
                ));
            }
        }
        let classes = classes.map(|a| TagAttributes::map_classes(a.value));
        let value = TagAttributes::remove_attr(&mut attributes, "value");
        let kind = TagAttributes::remove_attr(&mut attributes, "type");
        let checked = TagAttributes::remove_attr(&mut attributes, "checked");
        if let Some(checked) = checked.as_ref() {
            if checked.question_mark.is_some() {
                return Err(syn::Error::new_spanned(
                    &checked.label,
                    "the 'checked' attribute does not support being used as an optional attribute",
                ));
            }
        }
        let node_ref = TagAttributes::remove_attr(&mut attributes, "ref");
        if let Some(node_ref) = node_ref.as_ref() {
            if node_ref.question_mark.is_some() {
                return Err(syn::Error::new_spanned(
                    &node_ref.label,
                    "the 'ref' attribute does not support being used as an optional attribute",
                ));
            }
        }
        let node_ref = node_ref.map(|n| n.value);
        let key = TagAttributes::remove_attr(&mut attributes, "key");
        if let Some(key) = key.as_ref() {
            if key.question_mark.is_some() {
                return Err(syn::Error::new_spanned(
                    &key.label,
                    "the 'key' attribute does not support being used as an optional attribute",
                ));
            }
        }
        let key = key.map(|k| k.value);

        let href = TagAttributes::remove_attr(&mut attributes, "href");

        Ok(TagAttributes {
            attributes,
            classes,
            listeners,
            checked,
            booleans,
            value,
            kind,
            node_ref,
            href,
            key,
        })
    }
}
