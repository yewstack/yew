//! This module contains the bundle implementation of text [BText].

use super::{insert_node, BNode, DomBundle, Reconcilable};
use crate::html::AnyScope;
use crate::virtual_dom::{AttrValue, VText};
use crate::NodeRef;
use gloo::console;
use gloo_utils::document;
use web_sys::{Element, Text as TextNode};

/// Bind text to a dom element.
/// Reuses the virtual dom structure of text.
pub struct BText {
    text: AttrValue,
    text_node: TextNode,
}

impl DomBundle for BText {
    /// Remove VText from parent.
    fn detach(self, parent: &Element) {
        let node = &self.text_node;
        if parent.remove_child(node).is_err() {
            console::warn!("Node not found to remove VText");
        }
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) {
        let node = &self.text_node;

        next_parent
            .insert_before(node, next_sibling.get().as_ref())
            .unwrap();
    }
}

impl Reconcilable for VText {
    type Bundle = BText;

    fn attach(
        self,
        _parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let Self { text } = self;
        let text_node = document().create_text_node(&text);
        insert_node(&text_node, parent, next_sibling.get().as_ref());
        let node_ref = NodeRef::new(text_node.clone().into());
        (node_ref, BText { text, text_node })
    }

    /// Renders virtual node over existing `TextNode`, but only if value of text has changed.
    fn reconcile(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: &mut BNode,
    ) -> NodeRef {
        let btext = match ancestor {
            BNode::BText(btext) => btext,
            _ => {
                let (node_ref, self_) = self.attach(parent_scope, parent, next_sibling);
                ancestor.replace(parent, self_.into());
                return node_ref;
            }
        };
        let Self { text } = self;
        let ancestor_text = std::mem::replace(&mut btext.text, text);
        if btext.text != ancestor_text {
            btext.text_node.set_node_value(Some(&btext.text));
        }
        NodeRef::new(btext.text_node.clone().into())
    }
}

impl std::fmt::Debug for BText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BText {{ text: \"{}\" }}", self.text)
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
