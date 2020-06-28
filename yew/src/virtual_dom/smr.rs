#![cfg(feature = "sans_mount_render")]

use htmlescape;
use std::convert::TryFrom;
use thiserror::Error as ThisError;
use crate::sgml_tags::{is_valid_html_attribute_name, is_valid_sgml_tag};
use super::{VText, VTag, VList, VNode, VRef, VComp};

/// Represents a block of HTML string content.
#[derive(Debug, PartialEq, Eq)]
pub struct Html {
    html: String,
}

impl Html {
    fn new(html: String) -> Self {
        Html {
            html: html
        }
    }
}

impl Display for Html {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.html)
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

impl TryFrom<VComp> for Html {
    type Error = HtmlRenderError;

    fn try_from(value: VComp) -> Result<Html, HtmlRenderError> {
        let html: String = match &value.scope {
            None => "".to_string(),
            Some(scope) => match scope.root_vnode() {
                None => "".to_string(),
                Some(root_vnode) => Html::try_from(root_vnode.clone())?.to_string(),
            },
        };
        Ok(Html::new(html))
    }
}

/// HTML output for a VTag is not necessarily deterministic due to the
/// serialization of props which do not have a particular ordering.
impl TryFrom<VTag> for Html {
    type Error = HtmlRenderError;

    fn try_from(value: VTag) -> Result<Html, HtmlRenderError> {
        let mut result: String = "".to_string();
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

            result.push_str(
                format!(
                    " {}=\"{}\"",
                    htmlescape::encode_minimal(&key),
                    htmlescape::encode_attribute(&value)
                )
                .as_ref(),
            );
        }

        if value.checked {
            result.push_str(" checked")
        }

        if tag_name == "input" {
            if let Some(kind) = &value.kind {
                result.push_str(
                    format!(" type=\"{}\"", htmlescape::encode_attribute(&kind)).as_ref(),
                );
            }
        }

        let children_html: Html = match tag_name.as_ref() {
            "textarea" => {
                let vtext = VText::new(value.value.clone().unwrap_or_else(String::new));
                Html::try_from(vtext)
            }
            _ => Html::try_from(value.children),
        }?;
        let children_html = children_html.to_string();
        if children_html == "" {
            result.push_str(" />");
        } else {
            result.push_str(">");
            result.push_str(children_html.as_ref());
            result.push_str(format!("</{}>", tag_name).as_ref());
        }

        result.shrink_to_fit();
        Ok(Html::new(result))
    }
}

impl TryFrom<VText> for Html {
    type Error = HtmlRenderError;

    fn try_from(value: VText) -> Result<Html, HtmlRenderError> {
        Ok(Html::new(htmlescape::encode_minimal(&value.text)))
    }
}

impl TryFrom<VList> for Html {
    type Error = HtmlRenderError;

    fn try_from(value: VList) -> Result<Html, HtmlRenderError> {
        let mut result: String = "".to_string();
        for child in value.children {
            let html = Html::try_from(child)?.to_string();
            result.push_str(&html);
        }

        result.shrink_to_fit();
        Ok(Html::new(result))
    }
}

impl TryFrom<VNode> for Html {
    type Error = HtmlRenderError;

    fn try_from(value: VNode) -> Result<Html, HtmlRenderError> {
        Ok(match value {
            VNode::VTag(vtag) => Html::try_from(*vtag)?,
            VNode::VText(vtext) => Html::try_from(vtext)?,
            VNode::VComp(vcomp) => Html::try_from(vcomp)?,
            VNode::VList(vlist) => Html::try_from(vlist)?,
            VNode::VRef(_) => Err(HtmlRenderError::UnserializableVRef)?,
        })
    }
}


#[cfg(test)]
mod test_vtext {
    use crate::html;
    use super::Html;
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
            Html::try_from(a.clone()).expect("HTML stringify error"),
            Html::try_from(b.clone()).expect("HTML stringify error")
        );
        assert!(
            Html::try_from(b).expect("HTML stringify error").to_string() == "Text Node As Root"
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
            Html::try_from(a.clone()).expect("HTML stringify error"),
            Html::try_from(b.clone()).expect("HTML stringify error")
        );
        assert_eq!(
            Html::try_from(b.clone())
                .expect("HTML stringify error")
                .to_string(),
            "some special-chars&quot;&gt; here!"
        );
    }
}

#[cfg(test)]
mod tests_vtag {
    use super::*;
    use crate::html;
    use std::any::TypeId;
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
            let p_html = Html::try_from(*p)
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
            let p_html = Html::try_from(*p)
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
            let div_html = Html::try_from(*div)
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
            let div_html = Html::try_from(*div)
                .expect("HTML stringify error")
                .to_string();
            assert_eq!(div_html, "<div />");
        } else {
            assert!(false);
        }
    }
}
