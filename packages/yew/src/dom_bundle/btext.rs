//! This module contains the bundle implementation of text [BText].

use super::{insert_node, BNode, DomBundle, Reconcilable};
use crate::html::AnyScope;
use crate::virtual_dom::{AttrValue, VText};
use crate::NodeRef;
use gloo::console;
use gloo_utils::document;
use web_sys::{Element, Text as TextNode};

/// The bundle implementation to [VText]
pub(super) struct BText {
    text: AttrValue,
    text_node: TextNode,
}

impl DomBundle for BText {
    fn detach(self, parent: &Element, parent_to_detach: bool) {
        if !parent_to_detach {
            let result = parent.remove_child(&self.text_node);

            if result.is_err() {
                console::warn!("Node not found to remove VText");
            }
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
    fn reconcile_node(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        match bundle {
            BNode::Text(btext) => self.reconcile(parent_scope, parent, next_sibling, btext),
            _ => self.replace(parent_scope, parent, next_sibling, bundle),
        }
    }
    fn reconcile(
        self,
        _parent_scope: &AnyScope,
        _parent: &Element,
        _next_sibling: NodeRef,
        btext: &mut Self::Bundle,
    ) -> NodeRef {
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
        f.debug_struct("BText").field("text", &self.text).finish()
    }
}

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    use web_sys::Node;

    use crate::dom_bundle::{Fragment, Hydratable};
    use wasm_bindgen::JsCast;

    impl Hydratable for VText {
        fn hydrate(
            self,
            parent_scope: &AnyScope,
            parent: &Element,
            fragment: &mut Fragment,
        ) -> (NodeRef, Self::Bundle) {
            if let Some(m) = fragment.front().cloned() {
                // better safe than sorry.
                if m.node_type() == Node::TEXT_NODE {
                    if let Ok(m) = m.dyn_into::<TextNode>() {
                        // pop current node.
                        fragment.pop_front();

                        // TODO: It may make sense to assert the text content in the text node against
                        // the VText when #[cfg(debug_assertions)] is true, but this may be complicated.
                        // We always replace the text value for now.
                        //
                        // Please see the next comment for a detailed explanation.
                        m.set_node_value(Some(self.text.as_ref()));

                        return (
                            NodeRef::new(m.clone().into()),
                            BText {
                                text: self.text,
                                text_node: m,
                            },
                        );
                    }
                }
            }

            // If there are multiple text nodes placed back-to-back in SSR, it may be parsed as a single
            // text node by browser, hence we need to add extra text nodes here if the next node is not a text node.
            // Similarly, the value of the text node may be a combination of multiple VText vnodes.
            // So we always need to override their values.
            self.attach(
                parent_scope,
                parent,
                fragment
                    .front()
                    .cloned()
                    .map(|m| NodeRef::new(m.into()))
                    .unwrap_or_default(),
            )
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
