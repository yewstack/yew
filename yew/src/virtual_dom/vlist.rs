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
                        continue;
                    }
                }

                // Ignore the deleted right vnodes, and those corresponding to
                // already moved left vnodes.
                while let Some(Some(key)) = right_key.clone() {
                    if moved.contains(&key) {
                        right_key = right_keys.next();
                    } else if deleted_rights.contains_key(&key) {
                        right_key = right_keys.next();
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
                        // Skip the right key. This will force a move of the
                        // matching left one later when we will process it.
                        right_key = right_keys.next();
                    }
                }

                match (left.key(), right_key.clone()) {
                    (a, Some(b)) if *a == b => {
                        right_key = right_keys.next();
                    }
                    (None, Some(None)) => right_key = right_keys.next(),
                    (None, _) => {}
                    (Some(left_key), _) => {
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
            for (node, position) in moves.into_iter().rev() {
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
    use super::{Element, Node};
    use crate::html::{AnyScope, Scope};
    use crate::prelude::*;
    use crate::virtual_dom::{
        Key, VChild, VComp, VDiff, VDiffNodePosition, VList, VNode, VTag, VText,
    };
    use std::borrow::Cow;
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    struct Comp {
        id: usize,
    }

    #[derive(Properties, Clone)]
    struct CountingCompProps {
        id: usize,
    }

    impl Component for Comp {
        type Message = ();
        type Properties = CountingCompProps;

        fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
            Comp { id: props.id }
        }

        fn change(&mut self, props: Self::Properties) -> ShouldRender {
            wasm_bindgen_test::console_log!("Comp changed: {} -> {}", self.id, props.id);
            let changed = self.id != props.id;
            self.id = props.id;
            changed
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            unimplemented!();
        }

        fn view(&self) -> Html {
            html! { <p>{ self.id }</p> }
        }
    }

    #[test]
    fn check_fragment_has_child_of_tag_works() {
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

    fn new_keyed_vtag<T: Into<Cow<'static, str>>, K: ToString>(tag: T, key: K) -> VTag {
        let mut vtag = VTag::new(tag.into());
        vtag.key = Some(Key::from(key.to_string()));
        vtag
    }

    #[test]
    fn vlist_vdiff_apply_non_keyed_from_none_works_with_all_vnode_types_as_children() {
        let vchild: VChild<Comp> =
            VChild::new(CountingCompProps { id: 0 }, NodeRef::default(), None);
        let vref_element: Element = crate::utils::document().create_element("i").unwrap();
        let vref_node: Node = vref_element.clone().into();
        let mut vlist = VList::new_with_children(
            vec![
                VNode::VText(VText::new("a".into())),
                VNode::VTag(Box::new(VTag::new("span"))),
                VNode::VText(VText::new("c".into())),
                VNode::VText(VText::new("d".into())),
                VNode::VComp(vchild.into()),
                VNode::VList(VList::new_with_children(
                    vec![
                        VNode::VText(VText::new("foo".into())),
                        VNode::VText(VText::new("bar".into())),
                    ],
                    None,
                )),
                VNode::VRef(vref_node),
            ],
            None,
        );

        let parent_scope: AnyScope = Scope::<Comp>::new(None).into();
        let parent_element = crate::utils::document().create_element("div").unwrap();
        vlist.apply(
            &parent_scope,
            &parent_element,
            VDiffNodePosition::LastChild,
            None,
        );

        assert_eq!(
            parent_element.inner_html(),
            "a<span></span>cd<p>0</p>foobar<i></i>",
            "The VList didn't render properly."
        );
    }

    #[test]
    fn vlist_vdiff_apply_keyed_from_none_works_with_all_vnode_types_as_children() {
        let vchild: VChild<Comp> = VChild::new(
            CountingCompProps { id: 0 },
            NodeRef::default(),
            Some(Key::from(String::from("vchild"))),
        );
        let vref_element: Element = crate::utils::document().create_element("i").unwrap();
        let vref_node: Node = vref_element.clone().into();
        let vtag = new_keyed_vtag("span", "vtag");
        let mut vlist = VList::new_with_children(
            vec![
                VNode::VText(VText::new("a".into())),
                VNode::VTag(Box::new(vtag)),
                VNode::VText(VText::new("c".into())),
                VNode::VText(VText::new("d".into())),
                VNode::VComp(vchild.into()),
                VNode::VList(VList::new_with_children(
                    vec![
                        VNode::VText(VText::new("foo".into())),
                        VNode::VText(VText::new("bar".into())),
                    ],
                    Some(Key::from(String::from("vlist"))),
                )),
                VNode::VRef(vref_node),
            ],
            None,
        );

        let parent_scope: AnyScope = Scope::<Comp>::new(None).into();
        let parent_element = crate::utils::document().create_element("div").unwrap();
        vlist.apply(
            &parent_scope,
            &parent_element,
            VDiffNodePosition::LastChild,
            None,
        );

        assert_eq!(
            parent_element.inner_html(),
            "a<span></span>cd<p>0</p>foobar<i></i>",
            "The VList didn't render properly."
        );
    }

    fn test_vlist_vdiff_from_ancestor_works(
        ancestor_children: Vec<VNode>,
        ancestor_inner_html: impl AsRef<str>,
        new_children: Vec<VNode>,
        new_inner_html: impl AsRef<str>,
    ) {
        let parent_scope: AnyScope = Scope::<Comp>::new(None).into();
        let parent_element = crate::utils::document().create_element("div").unwrap();

        let mut ancestor_vlist = VList::new_with_children(ancestor_children, None);
        ancestor_vlist.apply(
            &parent_scope,
            &parent_element,
            VDiffNodePosition::LastChild,
            None,
        );
        assert_eq!(
            parent_element.inner_html(),
            ancestor_inner_html.as_ref(),
            "ancestor VList didn't render properly"
        );

        let mut vlist = VList::new_with_children(new_children, None);
        vlist.apply(
            &parent_scope,
            &parent_element,
            VDiffNodePosition::FirstChild,
            Some(VNode::VList(ancestor_vlist)),
        );
        assert_eq!(
            parent_element.inner_html(),
            new_inner_html.as_ref(),
            "new VList didn't render properly"
        );
    }

    #[test]
    fn vlist_vdiff_non_keyed_from_ancestor_works_append() {
        test_vlist_vdiff_from_ancestor_works(
            vec![VText::new("a".into()).into(), VText::new("b".into()).into()],
            "ab",
            vec![
                VText::new("a".into()).into(),
                VText::new("b".into()).into(),
                VText::new("c".into()).into(),
            ],
            "abc",
        );
    }

    #[test]
    fn vlist_vdiff_non_keyed_from_ancestor_works_prepend() {
        test_vlist_vdiff_from_ancestor_works(
            vec![VText::new("a".into()).into(), VText::new("b".into()).into()],
            "ab",
            vec![
                VText::new("c".into()).into(),
                VText::new("a".into()).into(),
                VText::new("b".into()).into(),
            ],
            "cab",
        );
    }

    #[test]
    fn vlist_vdiff_non_keyed_from_ancestor_works_delete() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VText::new("a".into()).into(),
                VText::new("b".into()).into(),
                VText::new("c".into()).into(),
            ],
            "abc",
            vec![VText::new("a".into()).into(), VText::new("b".into()).into()],
            "ab",
        );
    }

    #[test]
    fn vlist_vdiff_non_keyed_from_ancestor_works_swap() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VText::new("a".into()).into(),
                VText::new("b".into()).into(),
                VText::new("c".into()).into(),
            ],
            "abc",
            vec![
                VText::new("a".into()).into(),
                VText::new("c".into()).into(),
                VText::new("b".into()).into(),
            ],
            "acb",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_works_append() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("e", "2").into(),
            ],
            "<i></i><e></e>",
            vec![
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("e", "2").into(),
                new_keyed_vtag("p", "3").into(),
            ],
            "<i></i><e></e><p></p>",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_works_prepend() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("e", "2").into(),
            ],
            "<i></i><e></e>",
            vec![
                new_keyed_vtag("p", "3").into(),
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("e", "2").into(),
            ],
            "<p></p><i></i><e></e>",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_works_delete_first() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("e", "2").into(),
                new_keyed_vtag("p", "3").into(),
            ],
            "<i></i><e></e><p></p>",
            vec![
                new_keyed_vtag("e", "2").into(),
                new_keyed_vtag("p", "3").into(),
            ],
            "<e></e><p></p>",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_works_delete_last() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("e", "2").into(),
                new_keyed_vtag("p", "3").into(),
            ],
            "<i></i><e></e><p></p>",
            vec![
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("e", "2").into(),
            ],
            "<i></i><e></e>",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_works_reverse() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("i", "r").into(),
                new_keyed_vtag("e", "u").into(),
                new_keyed_vtag("p", "s").into(),
                new_keyed_vtag("u", "t").into(),
            ],
            "<i></i><e></e><p></p><u></u>",
            vec![
                new_keyed_vtag("u", "t").into(),
                new_keyed_vtag("p", "s").into(),
                new_keyed_vtag("e", "u").into(),
                new_keyed_vtag("i", "r").into(),
            ],
            "<u></u><p></p><e></e><i></i>",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_works_swap() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("e", "2").into(),
                new_keyed_vtag("p", "3").into(),
                new_keyed_vtag("a", "4").into(),
                new_keyed_vtag("u", "5").into(),
            ],
            "<i></i><e></e><p></p><a></a><u></u>",
            vec![
                new_keyed_vtag("e", "2").into(),
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("p", "3").into(),
                new_keyed_vtag("a", "4").into(),
                new_keyed_vtag("u", "5").into(),
            ],
            "<e></e><i></i><p></p><a></a><u></u>",
        );
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("e", "2").into(),
                new_keyed_vtag("p", "3").into(),
                new_keyed_vtag("a", "4").into(),
                new_keyed_vtag("u", "5").into(),
            ],
            "<i></i><e></e><p></p><a></a><u></u>",
            vec![
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("p", "3").into(),
                new_keyed_vtag("a", "4").into(),
                new_keyed_vtag("e", "2").into(),
                new_keyed_vtag("u", "5").into(),
            ],
            "<i></i><p></p><a></a><e></e><u></u>",
        );
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("e", "2").into(),
                new_keyed_vtag("p", "3").into(),
                new_keyed_vtag("a", "4").into(),
                new_keyed_vtag("u", "5").into(),
            ],
            "<i></i><e></e><p></p><a></a><u></u>",
            vec![
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("e", "2").into(),
                new_keyed_vtag("p", "3").into(),
                new_keyed_vtag("u", "5").into(),
                new_keyed_vtag("a", "4").into(),
            ],
            "<i></i><e></e><p></p><u></u><a></a>",
        );
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("i", "1").into(),
                new_keyed_vtag("e", "2").into(),
                new_keyed_vtag("p", "3").into(),
                new_keyed_vtag("a", "4").into(),
                new_keyed_vtag("u", "5").into(),
            ],
            "<i></i><e></e><p></p><a></a><u></u>",
            vec![
                new_keyed_vtag("u", "5").into(),
                new_keyed_vtag("e", "2").into(),
                new_keyed_vtag("p", "3").into(),
                new_keyed_vtag("a", "4").into(),
                new_keyed_vtag("i", "1").into(),
            ],
            "<u></u><e></e><p></p><a></a><i></i>",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_with_multiple_children_keyed_types() {
        let vcomp: VComp = VChild::<Comp>::new(
            CountingCompProps { id: 0 },
            NodeRef::default(),
            Some(Key::from("VComp".to_string())),
        )
        .into();
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("u", "1").into(),
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("a", "a").into(),
                        new_keyed_vtag("i", "i").into(),
                    ],
                    Some(Key::from("VList".to_string())),
                )
                .into(),
                vcomp.clone().into(),
            ],
            "<u></u><a></a><i></i><p>0</p>",
            vec![
                vcomp.into(),
                new_keyed_vtag("u", "1").into(),
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("a", "a").into(),
                        new_keyed_vtag("i", "i").into(),
                    ],
                    Some(Key::from("VList".to_string())),
                )
                .into(),
            ],
            "<p>0</p><u></u><a></a><i></i>",
        );
    }
}
