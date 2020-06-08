//! This module contains fragments implementation.
use super::{VDiff, VNode, VText};
use crate::html::{AnyScope, NodeRef};
use cfg_if::cfg_if;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Element, Node};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::{Element, Node};
    }
}

/// This struct represents a fragment of the Virtual DOM tree.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct VList {
    /// The list of children nodes.
    pub children: Vec<VNode>,
    pub key: Option<String>,
}

impl Deref for VList {
    type Target = Vec<VNode>;

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl DerefMut for VList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

impl VList {
    /// Creates a new empty `VList` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `VList` instance with children.
    pub fn new_with_children(children: Vec<VNode>, key: Option<String>) -> Self {
        VList { children, key }
    }

    /// Add `VNode` child.
    pub fn add_child(&mut self, child: VNode) {
        self.children.push(child);
    }
}

impl VDiff for VList {
    fn detach(&mut self, parent: &Element) {
        for mut child in self.children.drain(..) {
            child.detach(parent);
        }
    }

    fn apply(
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: Option<VNode>,
    ) -> NodeRef {
        let ancestor_children = {
            match ancestor {
                // If element matched this type
                Some(VNode::VList(vlist)) => {
                    // Previously rendered items
                    vlist.children
                }
                Some(vnode) => {
                    // Use the current node as a single fragment list
                    // and let the `apply` of `VNode` to handle it.
                    vec![vnode]
                }
                None => Vec::new(),
            }
        };

        if self.children.is_empty() {
            // Without a placeholder the next element becomes first
            // and corrupts the order of rendering
            // We use empty text element to stake out a place
            let placeholder = VText::new("".into());
            self.children.push(placeholder.into());
        }

        // Check for lefts to see if there are duplicates and show a warning.
        {
            let mut hash_set = HashSet::with_capacity(self.children.len());
            for l in self.children.iter() {
                if let Some(k) = l.key() {
                    if !hash_set.insert(k) {
                        log::error!("Duplicate key of {}", &k);
                    }
                }
            }

            // This warning should be removed in https://github.com/yewstack/yew/pull/1231
            if !hash_set.is_empty() {
                log::warn!("Keys currently have no effect");
            }
        }

        let ancestor_len = ancestor_children.len();
        let mut rights = ancestor_children.into_iter().peekable();
        let lefts = self.children.iter_mut().enumerate();
        let mut last_next_sibling = NodeRef::default();
        let mut nodes: Vec<NodeRef> = lefts
            .map(|(index, left)| {
                let ancestor = rights.next();
                let new_next_sibling = NodeRef::default();
                if let Some(next_right) = rights.peek() {
                    new_next_sibling.set(Some(next_right.first_node()));
                } else {
                    new_next_sibling.link(next_sibling.clone());
                }

                let node = left.apply(parent_scope, parent, new_next_sibling.clone(), ancestor);
                last_next_sibling.link(node.clone());
                last_next_sibling = new_next_sibling;
                node
            })
            .collect();

        for mut right in rights {
            right.detach(parent);
        }

        assert!(!nodes.is_empty(), "VList should have at least one child");
        nodes.remove(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn check_fragments() {
        let fragment = html! {
            <>
            </>
        };
        html! {
            <div>
                { fragment }
            </div>
        };
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
            node: html! {
                <>
                    {"a"}
                    {"b"}
                    <>
                        {"c"}
                        {"d"}
                    </>
                    {"e"}
                </>
            },
            expected: "abcde",
        };

        let layout2 = TestLayout {
            node: html! {
                <>
                    {"a"}
                    {"b"}
                    <></>
                    {"e"}
                    {"f"}
                </>
            },
            expected: "abef",
        };

        let layout3 = TestLayout {
            node: html! {
                <>
                    {"a"}
                    <></>
                    {"b"}
                    {"e"}
                </>
            },
            expected: "abe",
        };

        let layout4 = TestLayout {
            node: html! {
                <>
                    {"a"}
                    <>
                        {"c"}
                        {"d"}
                    </>
                    {"b"}
                    {"e"}
                </>
            },
            expected: "acdbe",
        };

        diff_layouts(vec![layout1, layout2, layout3, layout4]);
    }
}
