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
    pub value: Option<Expr>,
    pub kind: Option<Expr>,
    pub checked: Option<Expr>,
    pub node_ref: Option<Expr>,
    pub key: Option<Expr>,
    pub href: Option<Expr>,
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
                "onclick",
                "ondoubleclick",
                "onkeypress",
                "onkeydown",
                "onkeyup",
                "onmousedown",
                "onmousemove",
                "onmouseout",
                "onmouseenter",
                "onmouseleave",
                "onmousewheel",
                "onmouseover",
                "onmouseup",
                "ontouchcancel",
                "ontouchend",
                "ontouchenter",
                "ontouchmove",
                "ontouchstart",
                "ongotpointercapture",
                "onlostpointercapture",
                "onpointercancel",
                "onpointerdown",
                "onpointerenter",
                "onpointerleave",
                "onpointermove",
                "onpointerout",
                "onpointerover",
                "onpointerup",
                "onscroll",
                "onblur",
                "onfocus",
                "onsubmit",
                "oninput",
                "onchange",
                "ondrag",
                "ondragstart",
                "ondragend",
                "ondragenter",
                "ondragleave",
                "ondragover",
                "ondragexit",
                "ondrop",
                "oncontextmenu",
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
                    format!("only one `{}` attribute allowed", label),
                ));
            }
            i += 1;
        }
        let booleans = TagAttributes::drain_boolean(&mut attributes);

        let classes =
            TagAttributes::remove_attr(&mut attributes, "class").map(TagAttributes::map_classes);
        let value = TagAttributes::remove_attr(&mut attributes, "value");
        let kind = TagAttributes::remove_attr(&mut attributes, "type");
        let checked = TagAttributes::remove_attr(&mut attributes, "checked");
        let node_ref = TagAttributes::remove_attr(&mut attributes, "ref");
        let key = TagAttributes::remove_attr(&mut attributes, "key");

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
