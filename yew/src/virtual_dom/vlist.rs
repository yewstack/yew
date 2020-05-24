//! This module contains fragments implementation.
use super::{Key, VDiff, VDiffNodePosition, VNode, VText};
use crate::html::AnyScope;
use cfg_if::cfg_if;
use std::ops::{Deref, DerefMut};
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Element, Node};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::{Element, Node};
    }
}
cfg_if! {
    if #[cfg(feature = "fast_hasher")] {
        type HashMap<K, V> = ahash::AHashMap<K, V, ahash::RandomState>;
        type HashSet<K> = ahash::AHashSet<K, ahash::RandomState>;
    } else {
        use std::collections::{HashMap, HashSet};
    }
}

/// This struct represents a fragment of the Virtual DOM tree.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct VList {
    /// The list of children nodes.
    pub children: Vec<VNode>,
    /// Never use a placeholder element if set to true.
    elide_placeholder: bool,

    pub key: Option<Key>,
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
    pub fn new_with_children(children: Vec<VNode>, key: Option<Key>) -> Self {
        VList {
            children,
            elide_placeholder: false,
            key,
        }
    }

    /// Creates a new empty `VList` instance which does not need a placeholder node.
    pub(crate) fn new_without_placeholder() -> Self {
        VList {
            children: Vec::new(),
            elide_placeholder: true,
            key: None,
        }
    }

    /// Add `VNode` child.
    pub fn add_child(&mut self, child: VNode) {
        self.children.push(child);
    }
}

impl VDiff for VList {
    fn detach(&mut self, parent: &Element) -> VDiffNodePosition {
        let mut next_sibling = VDiffNodePosition::FirstChild;
        for mut child in self.children.drain(..) {
            next_sibling = child.detach(parent);
        }
        next_sibling
    }

    fn apply(
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        mut node_position: VDiffNodePosition,
        ancestor: Option<VNode>,
    ) -> Option<Node> {
        // Here, we will try to diff the previous list elements with the new
        // ones we want to insert. For that, we will use two lists:
        //  - lefts: new elements to render in the DOM
        //  - rights: previously rendered elements.
        //
        // The left items are known since we want to insert them
        // (self.children). For the right ones, we will look at the ancestor,
        // i.e. the current DOM list element that we want to replace with self.

        let mut rights = match ancestor {
            // If the ancestor is of the same type that this node, then the
            // right list is the previously rendered items.
            Some(VNode::VList(vlist)) => vlist.children,
            // Otherwise, there was a node before but it wasn't a list. Then,
            // use the current node as a single fragment list and let the
            // `apply` of `VNode` handle it.
            Some(vnode) => vec![vnode],
            None => Vec::new(),
        };

        if self.children.is_empty() && !self.elide_placeholder {
            // When the list is empty, without a placeholder the next element
            // becomes first and corrupts the order of rendering. We use empty
            // text element to stake out a place.
            let placeholder = VText::new("".into());
            self.children.push(placeholder.into());
        }

        // Check for lefts to see if there are duplicates and show a warning.
        // (no need to check in right, there cannot be duplicates)
        {
            let mut hash_set = HashSet::with_capacity(self.children.len());
            for l in self.children.iter() {
                if let Some(k) = l.key() {
                    if hash_set.contains(&k) {
                        log::error!("Key '{}' is not unique in list but must be.", &k);
                    } else {
                        hash_set.insert(k);
                    }
                }
            }
        }

        // Collect the keyed elements in a hashmap.
        let rights_lookup = rights
            .iter()
            .enumerate()
            .filter_map(|(idx, vnode)| vnode.key().as_ref().map(|key| (key.clone(), idx)))
            .collect::<HashMap<_, _>>();

        // Actual diffing loop.
        let mut current_right_idx: usize = 0;
        let mut left_surplus = self.children.len().saturating_sub(rights.len());
        for left in self.children.iter_mut() {
            // Ignore the dummy nodes we may have inserted. We can recognize
            // them because the are VText nodes with empty text and no
            // reference.
            while let Some(VNode::VText(vtext)) = rights.get(current_right_idx) {
                if vtext.reference.is_none() && vtext.text.is_empty() {
                    current_right_idx += 1;
                } else {
                    break;
                }
            }

            let right_key = rights
                .get(current_right_idx)
                .and_then(|vnode| vnode.key().as_ref());
            let (right_idx, reused) = match (left.key(), right_key) {
                (Some(left_key), Some(right_key)) if left_key == right_key => {
                    current_right_idx += 1;
                    (Some(current_right_idx - 1), false)
                }
                (Some(left_key), _) => match rights_lookup.get(left_key) {
                    Some(id) if *id > current_right_idx => {
                        // We found the existing node with the same key, but its
                        // order has been changed, so we must move it to its new
                        // location if we want to reuse the node.
                        (Some(*id), true)
                    }
                    _ if left_surplus > 0 => {
                        left_surplus = left_surplus.saturating_sub(1);
                        (None, true)
                    }
                    _ => {
                        current_right_idx += 1;
                        (Some(current_right_idx - 1), false)
                    }
                },
                (None, _) => {
                    current_right_idx += 1;
                    (Some(current_right_idx - 1), false)
                }
            };

            let right = right_idx.and_then(|right_idx| {
                rights.get_mut(right_idx).map(|right| {
                    let dummy_vnode = VNode::VText(VText::new(String::default()));
                    core::mem::replace(right, dummy_vnode)
                })
            });

            if reused {
                if let Some(right) = &right {
                    // Move the ancestor node.
                    super::insert_node(
                        &right.reference().expect("there must be a reference"),
                        parent,
                        &node_position,
                    );
                } else {
                    // If there isn't an ancestor, then the new node will be
                    // inserted at the correct position so there is nothing to
                    // do here.
                }
            }

            node_position = match left.apply(parent_scope, parent, node_position, right) {
                Some(node) => VDiffNodePosition::After(node),
                None => VDiffNodePosition::LastChild,
            };
        }

        // Detach all previously rendered elements that have not been reused,
        // which can be seen because reused nodes are replaced by dummy ones
        // without reference to actual DOM node.
        for mut right in rights {
            if right.reference().is_some() {
                right.detach(parent);
            }
        }

        match node_position {
            VDiffNodePosition::Before(node) => Some(node),
            VDiffNodePosition::After(node) => Some(node),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{html, Component, ComponentLink, Html, ShouldRender};
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    struct Comp;

    impl Component for Comp {
        type Message = ();
        type Properties = ();

        fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
            Comp
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            unimplemented!();
        }

        fn change(&mut self, _: Self::Properties) -> ShouldRender {
            unimplemented!();
        }

        fn view(&self) -> Html {
            unimplemented!();
        }
    }

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
