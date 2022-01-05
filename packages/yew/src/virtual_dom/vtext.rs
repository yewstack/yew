//! This module contains the implementation of a virtual text node `VText`.

use super::AttrValue;
use crate::dom_bundle::{insert_node, BNode, DomBundle, VDiff};
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

impl DomBundle for VText {
    /// Remove VText from parent.
    fn detach(mut self, parent: &Element) {
        let node = self
            .reference
            .take()
            .expect("tried to remove not rendered VText from DOM");
        if parent.remove_child(&node).is_err() {
            console::warn!("Node not found to remove VText");
        }
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) {
        let node = self
            .reference
            .as_ref()
            .expect("tried to shift not rendered VTag from DOM");

        next_parent
            .insert_before(node, next_sibling.get().as_ref())
            .unwrap();
    }
}

impl VDiff for VText {
    type Bundle = VText;

    fn attach(
        mut self,
        _parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let text_node = document().create_text_node(&self.text);
        insert_node(&text_node, parent, next_sibling.get().as_ref());
        self.reference = Some(text_node.clone());
        let node_ref = NodeRef::new(text_node.into());
        (node_ref, self)
    }

    /// Renders virtual node over existing `TextNode`, but only if value of text has changed.
    fn apply(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: &mut BNode,
    ) -> NodeRef {
        if let BNode::BText(ref mut vtext) = ancestor {
            let ancestor = std::mem::replace(vtext, self);
            vtext.reference = ancestor.reference;
            let text_node = vtext
                .reference
                .clone()
                .expect("Rendered VText nodes should have a ref");
            if vtext.text != ancestor.text {
                text_node.set_node_value(Some(&vtext.text));
            }

            return NodeRef::new(text_node.into());
        }
        let (node_ref, self_) = self.attach(parent_scope, parent, next_sibling);
        ancestor.replace(parent, self_.into());
        node_ref
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
