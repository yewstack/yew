//! This module contains fragments implementation.
use super::{Key, VDiff, VDiffNodePosition, VNode, VText};
use crate::html::AnyScope;
use cfg_if::cfg_if;
use fixedbitset::FixedBitSet;
use std::collections::{HashMap, HashSet};
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

        // Collect the keyed elements in a hashmap.
        let mut rights_lookup = rights
            .iter()
            .enumerate()
            .filter_map(|(idx, vnode)| vnode.key().as_ref().map(|key| (key.clone(), idx)))
            .collect::<HashMap<_, _>>();

        // The algorithms are different when there are keys, because it is more
        // expensive and less frequent.
        let keyed = !rights_lookup.is_empty();
        if !keyed {
            let mut rights = rights.into_iter();
            for left in self.children.iter_mut() {
                let right = rights.next();
                let node = left.apply(parent_scope, parent, node_position, right);
                node_position = match node {
                    Some(node) => VDiffNodePosition::After(node),
                    None => VDiffNodePosition::LastChild,
                };
            }
            for mut right in rights {
                right.detach(parent);
            }
            match node_position {
                VDiffNodePosition::Before(node) => Some(node),
                VDiffNodePosition::After(node) => Some(node),
                _ => None,
            }
        } else {
            // Here the only thing we know is that _some_ virtual nodes have
            // keys, but maybe not all of them.
            log::trace!("Using keyed algorithm");

            // Check for lefts to see if there are duplicates and show a warning
            // (no need to check in right, there cannot be duplicates).
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

            // Copy the keys from rights, as we need them for moving the vnodes
            // at the end of the algorithm, but we will steal nodes from
            // `rights` so will not be able to use it.
            let right_keys: Vec<Option<Key>> = rights.iter().map(VNode::key).cloned().collect();

            // We will try to match the left vnodes with the right ones, and
            // store them in `matched_rights`: it is formed of vnodes from
            // right, in the same order than left. `reused_rights` is used to
            // know which right nodes have been matched.
            let mut matched_rights: Vec<Option<VNode>> = vec![None; self.children.len()];
            let mut reused_rights = FixedBitSet::with_capacity(rights.len());

            // Generator of dummy vnodes, used to steal vnodes from rights.
            let make_dummy = || {
                VNode::VText(VText {
                    text: String::default(),
                    reference: None,
                })
            };

            // First, match only the keyed elements.
            let mut n_matched: usize = 0;
            for (left, matched_right) in self.children.iter().zip(matched_rights.iter_mut()) {
                if let Some(key) = left.key() {
                    if let Some(right_idx) = rights_lookup.remove(&key) {
                        let right = rights
                            .get_mut(right_idx)
                            .expect("the index from the map must be valid");
                        let right = core::mem::replace(right, make_dummy());
                        *matched_right = Some(right);
                        reused_rights.put(right_idx);
                        n_matched += 1;
                    } else {
                        // This left node is new.
                    }
                }
            }

            // Then, we find free vnodes for the unmatched left vnodes.
            if n_matched < self.children.len() {
                // Flip the bits to allow iterating over the free right vnodes.
                let free_rights = {
                    let mut toggled = reused_rights.clone();
                    toggled.toggle_range(..);
                    toggled
                };

                // Build an iterator that will yield all the remaining rights
                // that can be reused. Note that we do _not_ reuse keyed vnodes.
                let mut free_rights = free_rights
                    .ones()
                    .map(|idx| {
                        let right = rights
                            .get_mut(idx)
                            .expect("the index from the free_rights must be valid");
                        let right = if right.key().is_some() {
                            Some(core::mem::replace(right, make_dummy()))
                        } else {
                            None
                        };
                        (idx, right)
                    })
                    .filter(|(_, right)| right.is_some());

                for matched_right in matched_rights.iter_mut() {
                    if matched_right.is_none() {
                        if let Some((right_idx, Some(right))) = free_rights.next() {
                            *matched_right = Some(right);
                            reused_rights.put(right_idx);
                        }
                    }
                }
            }

            log::trace!("Before reconciliation, node matching:");
            for (left, right) in self.children.iter().zip(matched_rights.iter()) {
                log::trace!(
                    "{:?} - {:?}",
                    left.key(),
                    right.as_ref().map(|vnode| vnode.key().clone())
                );
            }

            // Reconciliation loop, i.e. apply the least amount of
            // transformations to rights to make them identical to lefts.
            let mut created_lefts: HashSet<Key> = HashSet::with_capacity(self.children.len());
            for (left, right) in self.children.iter_mut().zip(matched_rights.into_iter()) {
                // Collect the keyed left elements that don't have a matching
                // ancestor/right. They correspond to the newly created keyed
                // elements.
                if let (Some(key), None) = (left.key(), &right) {
                    created_lefts.insert(key.clone());
                }

                let node = left.apply(parent_scope, parent, node_position, right);
                node_position = match node {
                    Some(node) => VDiffNodePosition::After(node),
                    None => VDiffNodePosition::LastChild,
                };
            }

            // The remaining items in this map are the vnodes that have not been
            // reused, hence that have been deleted. We just rename the map for
            // clarity.
            let deleted_rights = rights_lookup;

            // Move in the DOM the nodes that have been reused.
            let mut moved: HashSet<Key> = HashSet::with_capacity(self.children.len());
            let mut lefts_peekable = self.children.iter().peekable();
            let mut right_keys = right_keys.into_iter().peekable();
            let mut right_key = right_keys.next();
            let mut moves: Vec<(Node, VDiffNodePosition)> = vec![];
            while let Some(left) = lefts_peekable.next() {
                // Ignore the new left vnodes, which are created at the correct
                // position.
                if let Some(key) = left.key() {
                    if created_lefts.contains(key) {
                        log::trace!("Skipping new left: {}", key);
                        continue;
                    }
                }

                // Ignore the deleted right vnodes, and those corresponding to
                // already moved left vnodes.
                while let Some(Some(key)) = right_key.clone() {
                    if moved.contains(&key) {
                        right_key = right_keys.next();
                        log::trace!("Skipping moved right: {}", key);
                    } else if deleted_rights.contains_key(&key) {
                        right_key = right_keys.next();
                        log::trace!("Skipping deleted right: {}", key);
                    } else {
                        break;
                    }
                }

                // Optimization: try to peek one after the current one, to
                // detect one-off moves. We know that if the current right is
                // keyed, it hasn't been deleted or already moved, so it must be
                // present in left.
                if let (Some(a), Some(Some(b))) = (left.key(), right_keys.peek()) {
                    if a == b {
                        log::trace!("Skipped right vnode {:?}, because next matches left: {}", right_key, b);
                        // Skip the right key. This will force a move of the
                        // matching left one later when we will process it.
                        right_key = right_keys.next();
                    }
                }

                match (left.key(), right_key.clone()) {
                    (a, Some(b)) if *a == b => {
                        log::trace!("Not moving matching keys: {:?}", a);
                        right_key = right_keys.next();
                    }
                    (None, Some(None)) => {
                        right_key = {
                            log::trace!("Not moving None keys");
                            right_keys.next()
                        }
                    }
                    (None, b) => log::trace!("Skipping None left. (right={:?})", b),
                    (Some(left_key), ref b) => {
                        log::trace!("Moving left: {} (right={:?})", left_key, b);

                        // Move the left vnode.
                        if let Some(left_node) = left.reference() {
                            let position = match lefts_peekable.peek() {
                                Some(next_sibling) => next_sibling
                                    .reference()
                                    .map(VDiffNodePosition::Before)
                                    .unwrap_or(VDiffNodePosition::LastChild),
                                _ => VDiffNodePosition::LastChild,
                            };
                            moves.push((left_node, position));

                            // Remember that we moved it, to allow skipping the
                            // matching right if there is one.
                            moved.insert(left_key.clone());
                        } else {
                            log::error!(
                                "Failed to move vnode {}: it doesn't have a reference!",
                                left_key
                            );
                        }
                    }
                }
            }
            log::debug!("finished collecting moves");
            for (node, position) in moves.into_iter().rev() {
                log::debug!("moving");
                super::insert_node(&node, parent, &position);
            }

            // Detach all previously rendered elements that have not been
            // reused, which can be seen because reused nodes are replaced by
            // dummy ones, that are VText nodes with empty text and without
            // references to actual DOM node.
            let not_reused_rights = {
                reused_rights.toggle_range(..);
                reused_rights
            };
            for not_reused_idx in not_reused_rights.ones() {
                let right = rights.get_mut(not_reused_idx).expect("id must exist");
                right.detach(parent);
            }

            log::trace!("keyed diffing complete");

            match node_position {
                VDiffNodePosition::Before(node) => Some(node),
                VDiffNodePosition::After(node) => Some(node),
                _ => None,
            }
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
