//! This module contains the implementation of a virtual text node `VText`.

use super::{AttrValue, VDiff, VNode};
use crate::html::{AnyScope, NodeRef};
use gloo::console;
use gloo_utils::document;
use std::cmp::PartialEq;
use web_sys::{Element, Text as TextNode};

/// A type for a virtual
/// [`TextNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
/// representation.
#[derive(Clone)]
pub struct VText {
    /// Contains a text of the node.
    pub text: AttrValue,
    /// A reference to the `TextNode`.
    pub reference: Option<TextNode>,
}

impl VText {
    /// Creates new virtual text node with a content.
    pub fn new(text: impl Into<AttrValue>) -> Self {
        VText {
            text: text.into(),
            reference: None,
        }
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use super::*;

    impl VText {
        pub(crate) async fn render_to_string(&self, w: &mut String) {
            html_escape::encode_text_to_string(&self.text, w);
        }
    }
}

impl std::fmt::Debug for VText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VText {{ text: \"{}\", reference: {} }}",
            self.text,
            match &self.reference {
                Some(_) => "Some(...)",
                None => "None",
            }
        )
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
            console::warn!("Node not found to remove VText");
        }
    }

    fn shift(&self, previous_parent: &Element, next_parent: &Element, next_sibling: NodeRef) {
        let node = self
            .reference
            .as_ref()
            .expect("tried to shift not rendered VTag from DOM");

        previous_parent.remove_child(node).unwrap();
        next_parent
            .insert_before(node, next_sibling.get().as_ref())
            .unwrap();
    }

    /// Renders virtual node over existing `TextNode`, but only if value of text has changed.
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
        super::insert_node(&text_node, parent, next_sibling.get().as_ref());
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
    extern crate self as yew;

    use crate::html;

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

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
}

#[cfg(test)]
mod layout_tests {
    extern crate self as yew;

    use crate::html;
    use crate::tests::layout_tests::{diff_layouts, TestLayout};

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn diff() {
        let layout1 = TestLayout {
            name: "1",
            node: html! { "a" },
            expected: "a",
        };

        let layout2 = TestLayout {
            name: "2",
            node: html! { "b" },
            expected: "b",
        };

        let layout3 = TestLayout {
            name: "3",
            node: html! {
                <>
                    {"a"}
                    {"b"}
                </>
            },
            expected: "ab",
        };

        let layout4 = TestLayout {
            name: "4",
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

#[cfg(all(test, not(target_arch = "wasm32"), feature = "ssr"))]
mod ssr_tests {
    use tokio::test;

    use super::*;

    #[test]
    async fn test_simple_str() {
        let vtext = VText::new("abc");

        let mut s = String::new();

        vtext.render_to_string(&mut s).await;

        assert_eq!("abc", s.as_str());
    }
}
