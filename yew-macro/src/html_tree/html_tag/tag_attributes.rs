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
    pub checked: Option<Expr>,
    pub node_ref: Option<Expr>,
    pub key: Option<Expr>,
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

    fn remove_attr_nonoptional(
        attrs: &mut Vec<TagAttribute>,
        name: &str,
    ) -> syn::Result<Option<TagAttribute>> {
        match Self::remove_attr(attrs, name) {
            Some(attr) => attr.ensure_not_optional().map(|_| Some(attr)),
            None => Ok(None),
        }
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
        for listener in Self::drain_listeners(&mut attributes) {
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
        let booleans = Self::drain_boolean(&mut attributes);
        for attr in &booleans {
            if attr.question_mark.is_some() {
                return Err(syn::Error::new_spanned(
                    &attr.label,
                        "boolean attributes don't support being used as an option attribute (hint: a value of false results in the attribute not being set)"
                ));
            }
        }

        let classes = Self::remove_attr_nonoptional(&mut attributes, "class")?
            .map(|a| Self::map_classes(a.value));
        let value = Self::remove_attr(&mut attributes, "value");
        let kind = Self::remove_attr(&mut attributes, "type");
        let checked = Self::remove_attr_nonoptional(&mut attributes, "checked")?.map(|v| v.value);
        let node_ref = Self::remove_attr_nonoptional(&mut attributes, "ref")?.map(|v| v.value);
        let key = Self::remove_attr_nonoptional(&mut attributes, "key")?.map(|v| v.value);

        Ok(Self {
            attributes,
            classes,
            listeners,
            checked,
            booleans,
            value,
            kind,
            node_ref,
            key,
        })
    }
}
