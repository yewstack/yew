//! This module contains the implementation of a virtual text node `VText`.

use super::{VDiff, VNode};

use crate::html::{AnyScope, NodeRef};
use crate::utils::document;

use cfg_if::cfg_if;
use log::warn;
use std::cmp::PartialEq;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Element, INode, TextNode};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::{Element, Text as TextNode};
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::{ToHtmlString};
        use htmlescape;
    }
}

/// A type for a virtual
/// [`TextNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
/// representation.
#[derive(Clone, Debug)]
pub struct VText {
    /// Contains a text of the node.
    pub text: String,
    /// A reference to the `TextNode`.
    pub reference: Option<TextNode>,
}

impl VText {
    /// Creates new virtual text node with a content.
    pub fn new(text: String) -> Self {
        VText {
            text,
            reference: None,
        }
    }
}

#[cfg(feature = "ssr")]
impl ToHtmlString for VText {
    fn to_html_string(&self) -> String {
        htmlescape::encode_minimal(&self.text)
    }
}

impl VDiff for VText {
    /// Remove VText from parent.
    fn detach(&mut self, parent: &Element) {
        let node = self
            .reference
            .take()
            .expect("tried to remove not rendered VText from DOM");
        if parent.remove_child(&node).is_err() {
            warn!("Node not found to remove VText");
        }
    }

    /// Renders virtual node over existing `TextNode`, but only if value of text had changed.
    fn apply(
        &mut self,
        _parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: Option<VNode>,
    ) -> NodeRef {
        if let Some(mut ancestor) = ancestor {
            if let VNode::VText(mut vtext) = ancestor {
                self.reference = vtext.reference.take();
                let text_node = self
                    .reference
                    .clone()
                    .expect("Rendered VText nodes should have a ref");
                if self.text != vtext.text {
                    text_node.set_node_value(Some(&self.text));
                }

                return NodeRef::new(text_node.into());
            }

            ancestor.detach(parent);
        }

        let text_node = document().create_text_node(&self.text);
        super::insert_node(&text_node, parent, next_sibling.get());
        self.reference = Some(text_node.clone());
        NodeRef::new(text_node.into())
    }
}

impl PartialEq for VText {
    fn eq(&self, other: &VText) -> bool {
        self.text == other.text
    }
}

#[cfg(test)]
mod test {
    use crate::html;

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "ssr")]
    use super::ToHtmlString;

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn text_as_root() {
        html! {
            "Text Node As Root"
        };

        html! {
            { "Text Node As Root" }
        };
    }

    #[test]
    #[cfg(feature = "ssr")]
    fn text_as_root_ssr() {
        let a = html! {
            "Text Node As Root"
        };

        let b = html! {
            { "Text Node As Root" }
        };

        assert_eq!(&a.to_html_string(), &b.to_html_string());
        assert!(b.clone().to_html_string() == "Text Node As Root");
    }

    #[test]
    #[cfg(feature = "ssr")]
    fn special_chars_ssr() {
        let a = html! {
            "some special-chars\"> here!"
        };

        let b = html! {
            { "some special-chars\"> here!" }
        };

        assert_eq!(&a.to_html_string(), &b.to_html_string());
        assert_eq!(
            b.clone().to_html_string(),
            "some special-chars&quot;&gt; here!"
        );
    }
}

#[cfg(all(test, feature = "web_sys"))]
mod layout_tests {
    use crate::virtual_dom::layout_tests::{diff_layouts, TestLayout};

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn diff() {
        let layout1 = TestLayout {
            node: html! { "a" },
            expected: "a",
        };

        let layout2 = TestLayout {
            node: html! { "b" },
            expected: "b",
        };

        let layout3 = TestLayout {
            node: html! {
                <>
                    {"a"}
                    {"b"}
                </>
            },
            expected: "ab",
        };

        let layout4 = TestLayout {
            node: html! {
                <>
                    {"b"}
                    {"a"}
                </>
            },
            expected: "ba",
        };

        diff_layouts(vec![layout1, layout2, layout3, layout4]);
    }
}
