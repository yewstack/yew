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
        pub(crate) async fn render_to_string(
            &self,
            w: &mut String,
            _parent_scope: &AnyScope,
            _hydratable: bool,
        ) {
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
    fn detach(&mut self, parent: &Element, parent_to_detach: bool) {
        let node = self
            .reference
            .take()
            .expect("tried to remove not rendered VText from DOM");
        if !parent_to_detach {
            let result = parent.remove_child(&node);

            if result.is_err() {
                console::warn!("Node not found to remove VText");
            }
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

            ancestor.detach(parent, false);
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

#[cfg_attr(documenting, doc(cfg(feature = "hydration")))]
#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    use std::collections::VecDeque;

    use web_sys::Node;

    use crate::virtual_dom::VHydrate;
    use wasm_bindgen::JsCast;

    use crate::virtual_dom::insert_node;

    impl VHydrate for VText {
        fn hydrate(
            &mut self,
            _parent_scope: &AnyScope,
            parent: &Element,
            fragment: &mut VecDeque<Node>,
        ) -> NodeRef {
            assert!(
                self.reference.is_none(),
                "trying to hydrate a mounted VText."
            );

            if let Some(m) = fragment.front().cloned() {
                if m.node_type() == Node::TEXT_NODE {
                    if let Ok(m) = m.dyn_into::<TextNode>() {
                        // pop current node.
                        fragment.pop_front();

                        // always update node value, see reason below.
                        m.set_node_value(Some(self.text.as_ref()));
                        self.reference = Some(m.clone());

                        return NodeRef::new(m.into());
                    }
                }
            }

            // If there are multiple text nodes placed back-to-back, it may be parsed as a single
            // text node, hence we need to add extra text nodes here if the next node is not a text node.
            let text_node = document().create_text_node(&self.text);
            insert_node(&text_node, parent, fragment.front());
            self.reference = Some(text_node.clone());
            NodeRef::new(text_node.into())
        }
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
