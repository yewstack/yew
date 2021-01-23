use super::{Prop, Props, SpecialProps};
use lazy_static::lazy_static;
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, ExprTuple};

pub enum ClassesForm {
    Tuple(ExprTuple),
    Single(Box<Expr>),
}
impl ClassesForm {
    fn from_expr(expr: Expr) -> Self {
        match expr {
            Expr::Tuple(expr_tuple) => ClassesForm::Tuple(expr_tuple),
            expr => ClassesForm::Single(Box::new(expr)),
        }
    }
}

pub struct ElementProps {
    pub attributes: Vec<Prop>,
    pub listeners: Vec<Prop>,
    pub classes: Option<ClassesForm>,
    pub booleans: Vec<Prop>,
    pub value: Option<Prop>,
    pub kind: Option<Prop>,
    pub checked: Option<Prop>,
    pub node_ref: Option<Prop>,
    pub key: Option<Prop>,
}

impl Parse for ElementProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut props = input.parse::<Props>()?;

        let listeners =
            props.drain_filter(|prop| LISTENER_SET.contains(prop.label.to_string().as_str()));

        // Multiple listener attributes are allowed, but no others
        props.check_no_duplicates()?;

        let booleans =
            props.drain_filter(|prop| BOOLEAN_SET.contains(prop.label.to_string().as_str()));
        booleans.check_all(|prop| {
            if prop.question_mark.is_some() {
                Err(syn::Error::new_spanned(
                    &prop.label,
                    "boolean attributes don't support being used as an optional attribute (hint: a value of false results in the attribute not being set)"
                ))
            } else {
                Ok(())
            }
        })?;

        let classes = props
            .pop_nonoptional("class")?
            .map(|prop| ClassesForm::from_expr(prop.value));
        let value = props.pop("value");
        let kind = props.pop("type");
        let checked = props.pop_nonoptional("checked")?;

        let SpecialProps { node_ref, key } = props.special;

        Ok(Self {
            attributes: props.prop_list.into_vec(),
            classes,
            listeners: listeners.into_vec(),
            checked,
            booleans: booleans.into_vec(),
            value,
            kind,
            node_ref,
            key,
        })
    }
}

lazy_static! {
    static ref BOOLEAN_SET: HashSet<&'static str> = {
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
        .into_iter()
        .collect()
    };
}

lazy_static! {
    static ref LISTENER_SET: HashSet<&'static str> = {
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
        .into_iter()
        .collect()
    };
}
