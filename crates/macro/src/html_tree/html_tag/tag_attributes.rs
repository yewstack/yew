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
    pub value: Option<Expr>,
    pub kind: Option<Expr>,
    pub checked: Option<Expr>,
    pub disabled: Option<Expr>,
    pub selected: Option<Expr>,
    pub node_ref: Option<Expr>,
    pub href: Option<Expr>,
}

pub enum ClassesForm {
    Tuple(Vec<Expr>),
    Single(Expr),
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

        let classes =
            TagAttributes::remove_attr(&mut attributes, "class").map(TagAttributes::map_classes);
        let value = TagAttributes::remove_attr(&mut attributes, "value");
        let kind = TagAttributes::remove_attr(&mut attributes, "type");
        let checked = TagAttributes::remove_attr(&mut attributes, "checked");
        let disabled = TagAttributes::remove_attr(&mut attributes, "disabled");
        let selected = TagAttributes::remove_attr(&mut attributes, "selected");
        let node_ref = TagAttributes::remove_attr(&mut attributes, "ref");
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
            node_ref,
            href,
        })
    }
}
