use std::collections::HashSet;

use once_cell::sync::Lazy;
use syn::parse::{Parse, ParseStream};

use super::{Prop, Props, SpecialProps};

pub struct ElementProps {
    pub attributes: Vec<Prop>,
    pub listeners: Vec<Prop>,
    pub classes: Option<Prop>,
    pub booleans: Vec<Prop>,
    pub value: Option<Prop>,
    pub defaultvalue: Option<Prop>,
    pub checked: Option<Prop>,
    pub special: SpecialProps,
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

        let classes = props.pop("class");
        let value = props.pop("value");
        let checked = props.pop("checked");
        let defaultvalue = props.pop("defaultvalue");

        check_case_collisions(&props.prop_list)?;

        let special = props.special;

        Ok(Self {
            attributes: props.prop_list.into_vec(),
            classes,
            listeners: listeners.into_vec(),
            checked,
            booleans: booleans.into_vec(),
            value,
            special,
            defaultvalue,
        })
    }
}

static BOOLEAN_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        // Living Standard
        // From: https://html.spec.whatwg.org/#attributes-3
        // where `Value` = Boolean attribute
        // Note: `checked` is uniquely handled in the html! macro.
        "allowfullscreen",
        "async",
        "autofocus",
        "autoplay",
        "controls",
        "default",
        "defer",
        "disabled",
        "formnovalidate",
        "hidden",
        "inert",
        "ismap",
        "itemscope",
        "loop",
        "multiple",
        "muted",
        "nomodule",
        "novalidate",
        "open",
        "playsinline",
        "readonly",
        "required",
        "reversed",
        "selected",
        "truespeed",
        // Not-yet-standardized
        "webkitdirectory",
    ]
    .into()
});

const SPECIAL_ELEMENT_PROPS: &[&str] = &["class", "value", "checked", "defaultvalue"];

fn check_case_collisions(remaining: &super::PropList) -> syn::Result<()> {
    crate::join_errors(remaining.iter().filter_map(|prop| {
        let lower = prop.label.to_ascii_lowercase_string();
        if LISTENER_SET.contains(lower.as_str())
            || BOOLEAN_SET.contains(lower.as_str())
            || SPECIAL_ELEMENT_PROPS.contains(&lower.as_str())
        {
            Some(syn::Error::new_spanned(
                &prop.label,
                format!("HTML attribute names are case-insensitive. Did you mean `{lower}`?"),
            ))
        } else {
            None
        }
    }))
}

static LISTENER_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
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
        // onfocusin + onfocusout not in standard but added due to browser support
        // see issue 1896: https://github.com/yewstack/yew/issues/1896
        "onfocusin",
        "onfocusout",
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
    .into()
});
