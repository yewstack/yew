//! This module contains Yew's implementation of Sans-Mount Rendering (SMR), to support
//! future feature work such as Static Site Generation and Server-Side Rendering (SSR).
//! This functionality allows Yew Components to be rendered to a string without needing
//! to be mounted onto a DOM node first.
//!
//! *This module is only available if the `sans_mount_render` feature is enabled.*

use super::{VComp, VList, VNode, VTag, VText};
use yew_validation::{is_valid_html_attribute_name, is_valid_sgml_tag};
use htmlescape;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use thiserror::Error as ThisError;

/// Represents a block of HTML string content generated via Sans-Mount Rendering
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HtmlString(String);

impl HtmlString {
    fn new(html: String) -> Self {
        Self(html)
    }
}

impl Display for HtmlString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents errors associated with conversion of Yew structures to HTML.
#[derive(Debug, ThisError)]
pub enum HtmlRenderError {
    /// Malformed/unserializable attribute name
    #[error("cannot serialize invalid attribute name `{0}`")]
    InvalidAttributeName(String),

    /// Malformed/unserializable tag name
    #[error("cannot serialize invalid tag name `{0}`")]
    InvalidTagName(String),

    /// Unsupported VRef serialization
    #[error("cannot serialize VRef because that is unsupported")]
    UnserializableVRef,
}

impl TryFrom<VComp> for HtmlString {
    type Error = HtmlRenderError;

    fn try_from(value: VComp) -> Result<HtmlString, HtmlRenderError> {
        let html: String = match &value.scope {
            None => "".to_string(),
            Some(scope) => match scope.root_vnode() {
                None => "".to_string(),
                Some(root_vnode) => HtmlString::try_from(root_vnode.clone())?.to_string(),
            },
        };
        Ok(HtmlString::new(html))
    }
}

/// HTML output for a VTag is not necessarily deterministic due to the
/// serialization of props which do not have a particular ordering.
impl TryFrom<VTag> for HtmlString {
    type Error = HtmlRenderError;

    fn try_from(value: VTag) -> Result<HtmlString, HtmlRenderError> {
        let mut result = "".to_string();
        let tag_name = htmlescape::encode_minimal(&value.tag).to_lowercase();

        if !is_valid_sgml_tag(&tag_name) {
            return Err(HtmlRenderError::InvalidTagName(tag_name));
        }

        result.push_str(&format!("<{}", tag_name));

        for (key_unclean, value) in &value.attributes {
            let key = key_unclean.to_lowercase();
            // checked, value (special if textarea), disabled, href?, selected,
            // kind -> type if input, disallow ref, disallow LISTENER_SET, class

            if !is_valid_html_attribute_name(key.as_str()) {
                return Err(HtmlRenderError::InvalidAttributeName(key));
            }

            // textareas' innerHTML properties are specified via the `value` prop which doesn't
            // exist in HTML, so we defer this prop's serialization until later in the process.
            if tag_name == "textarea" && key == "value" {
                continue;
            }

            result.push_str(&format!(
                " {}=\"{}\"",
                htmlescape::encode_minimal(&key),
                htmlescape::encode_attribute(&value)
            ));
        }

        if value.checked {
            result.push_str(&" checked")
        }

        if tag_name == "input" {
            if let Some(kind) = &value.kind {
                result.push_str(&format!(
                    " type=\"{}\"",
                    htmlescape::encode_attribute(&kind)
                ));
            }
        }

        let children_html = match tag_name.as_ref() {
            "textarea" => {
                let vtext = VText::new(value.value.clone().unwrap_or_else(String::new));
                HtmlString::try_from(vtext)
            }
            _ => HtmlString::try_from(value.children),
        }?.to_string();
        
        if children_html == "" {
            result.push_str(&" />");
        } else {
            result.push_str(&">");
            result.push_str(&children_html);
            result.push_str(&format!("</{}>", tag_name));
        }

        result.shrink_to_fit();
        Ok(HtmlString::new(result))
    }
}

impl TryFrom<VText> for HtmlString {
    type Error = HtmlRenderError;

    fn try_from(value: VText) -> Result<HtmlString, HtmlRenderError> {
        Ok(HtmlString::new(htmlescape::encode_minimal(&value.text)))
    }
}

impl TryFrom<VList> for HtmlString {
    type Error = HtmlRenderError;

    fn try_from(value: VList) -> Result<HtmlString, HtmlRenderError> {
        let mut result = "".to_string();
        for child in value.children {
            let html = HtmlString::try_from(child)?.to_string();
            result.push_str(&html);
        }

        result.shrink_to_fit();
        Ok(HtmlString::new(result))
    }
}

impl TryFrom<VNode> for HtmlString {
    type Error = HtmlRenderError;

    fn try_from(value: VNode) -> Result<HtmlString, HtmlRenderError> {
        Ok(match value {
            VNode::VTag(vtag) => HtmlString::try_from(*vtag)?,
            VNode::VText(vtext) => HtmlString::try_from(vtext)?,
            VNode::VComp(vcomp) => HtmlString::try_from(vcomp)?,
            VNode::VList(vlist) => HtmlString::try_from(vlist)?,
            VNode::VRef(_) => Err(HtmlRenderError::UnserializableVRef)?,
        })
    }
}

#[cfg(test)]
mod tests_vtext {
    use super::HtmlString;
    use crate::html;
    use std::convert::TryFrom;

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn text_as_root_smr() {
        let a = html! {
            "Text Node As Root"
        };

        let b = html! {
            { "Text Node As Root" }
        };

        assert_eq!(
            HtmlString::try_from(a.clone()).expect("HTML stringify error"),
            HtmlString::try_from(b.clone()).expect("HTML stringify error")
        );
        assert!(
            HtmlString::try_from(b)
                .expect("HTML stringify error")
                .to_string()
                == "Text Node As Root"
        );
    }

    #[test]
    fn special_chars_smr() {
        let a = html! {
            "some special-chars\"> here!"
        };

        let b = html! {
            { "some special-chars\"> here!" }
        };

        assert_eq!(
            HtmlString::try_from(a.clone()).expect("HTML stringify error"),
            HtmlString::try_from(b.clone()).expect("HTML stringify error")
        );
        assert_eq!(
            HtmlString::try_from(b.clone())
                .expect("HTML stringify error")
                .to_string(),
            "some special-chars&quot;&gt; here!"
        );
    }
}

#[cfg(test)]
mod tests_vtag {
    use super::*;
    use crate::html::NodeRef;
    use std::convert::TryFrom;

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn it_stringifies_simple() {
        let p = html! {
            <p></p>
        };

        if let VNode::VTag(p) = p {
            let p_html = HtmlString::try_from(*p)
                .expect("HTML stringify error")
                .to_string();

            assert_eq!(p_html, "<p />");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn it_stringifies_complex() {
        let other_sym = "bar";
        let div = html! {
            <div class=("foo", other_sym)>
                { "quux" }
            </div>
        };
        let p = html! {
            <p aria-controls="it-works">
                { "test" }
                {div}
            </p>
        };

        if let VNode::VTag(p) = p {
            let p_html = HtmlString::try_from(*p)
                .expect("HTML stringify error")
                .to_string();

            assert_eq!(
                p_html,
                "<p aria-controls=\"it&#x2D;works\">test<div class=\"foo&#x20;bar\">quux</div></p>"
            );
        } else {
            assert!(false);
        }
    }

    #[test]
    fn it_stringifies_attrs() {
        let div = html! {
            <div a="b" b="a" />
        };

        if let VNode::VTag(div) = div {
            let div_html = HtmlString::try_from(*div)
                .expect("HTML stringify error")
                .to_string();
            let order_1 = "<div a=\"b\" b=\"a\" />";
            let order_2 = "<div b=\"a\" a=\"b\" />";
            assert!(div_html == order_1 || div_html == order_2);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn it_does_not_stringify_special_attrs() {
        let node_ref = NodeRef::default();

        let div = html! {
            <div ref=node_ref />
        };

        if let VNode::VTag(div) = div {
            let div_html = HtmlString::try_from(*div)
                .expect("HTML stringify error")
                .to_string();
            assert_eq!(div_html, "<div />");
        } else {
            assert!(false);
        }
    }
}
