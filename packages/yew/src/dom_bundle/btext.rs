//! This module contains the bundle implementation of text [BText].

use gloo::utils::document;
use web_sys::{Element, Text as TextNode};

use super::{BNode, BSubtree, DomSlot, Reconcilable, ReconcileTarget};
use crate::html::AnyScope;
use crate::virtual_dom::{AttrValue, VText};

/// The bundle implementation to [VText]
pub(super) struct BText {
    text: AttrValue,
    text_node: TextNode,
}

impl ReconcileTarget for BText {
    fn detach(self, _root: &BSubtree, parent: &Element, parent_to_detach: bool) {
        if !parent_to_detach {
            let result = parent.remove_child(&self.text_node);

            if result.is_err() {
                tracing::warn!("Node not found to remove VText");
            }
        }
    }

    fn shift(&self, next_parent: &Element, slot: DomSlot) -> DomSlot {
        slot.insert(next_parent, &self.text_node);

        DomSlot::at(self.text_node.clone().into())
    }
}

impl Reconcilable for VText {
    type Bundle = BText;

    fn attach(
        self,
        _root: &BSubtree,
        _parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
    ) -> (DomSlot, Self::Bundle) {
        let Self { text } = self;
        let text_node = document().create_text_node(&text);
        slot.insert(parent, &text_node);
        let node_ref = DomSlot::at(text_node.clone().into());
        (node_ref, BText { text, text_node })
    }

    /// Renders virtual node over existing `TextNode`, but only if value of text has changed.
    fn reconcile_node(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
        bundle: &mut BNode,
    ) -> DomSlot {
        match bundle {
            BNode::Text(btext) => self.reconcile(root, parent_scope, parent, slot, btext),
            _ => self.replace(root, parent_scope, parent, slot, bundle),
        }
    }

    fn reconcile(
        self,
        _root: &BSubtree,
        _parent_scope: &AnyScope,
        _parent: &Element,
        _slot: DomSlot,
        btext: &mut Self::Bundle,
    ) -> DomSlot {
        let Self { text } = self;
        let ancestor_text = std::mem::replace(&mut btext.text, text);
        if btext.text != ancestor_text {
            btext.text_node.set_node_value(Some(&btext.text));
        }
        DomSlot::at(btext.text_node.clone().into())
    }
}

impl std::fmt::Debug for BText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BText").field("text", &self.text).finish()
    }
}

#[cfg(feature = "hydration")]
mod feat_hydration {
    use wasm_bindgen::JsCast;
    use web_sys::Node;

    use super::*;
    use crate::dom_bundle::{Fragment, Hydratable};

    impl Hydratable for VText {
        fn hydrate(
            self,
            _root: &BSubtree,
            _parent_scope: &AnyScope,
            parent: &Element,
            fragment: &mut Fragment,
        ) -> Self::Bundle {
            let next_sibling = if let Some(m) = fragment.front().cloned() {
                // better safe than sorry.
                if m.node_type() == Node::TEXT_NODE {
                    let m = m.unchecked_into::<TextNode>();
                    // pop current node.
                    fragment.pop_front();

                    // TODO: It may make sense to assert the text content in the text node
                    // against the VText when #[cfg(debug_assertions)]
                    // is true, but this may be complicated.
                    // We always replace the text value for now.
                    //
                    // Please see the next comment for a detailed explanation.
                    m.set_node_value(Some(self.text.as_ref()));

                    return BText {
                        text: self.text,
                        text_node: m,
                    };
                }
                Some(m)
            } else {
                fragment.sibling_at_end().cloned()
            };

            // If there are multiple text nodes placed back-to-back in SSR, it may be parsed as a
            // single text node by browser, hence we need to add extra text nodes here
            // if the next node is not a text node. Similarly, the value of the text
            // node may be a combination of multiple VText vnodes. So we always need to
            // override their values.
            let text_node = document().create_text_node("");
            DomSlot::create(next_sibling).insert(parent, &text_node);
            BText {
                text: "".into(),
                text_node,
            }
        }
    }
}

#[cfg(test)]
mod test {
    extern crate self as yew;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use crate::html;

    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn text_as_root() {
        let _ = html! {
            "Text Node As Root"
        };

        let _ = html! {
            { "Text Node As Root" }
        };
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod layout_tests {
    extern crate self as yew;

    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use crate::html;
    use crate::tests::layout_tests::{diff_layouts, TestLayout};

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
