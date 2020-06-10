//! This module contains fragments implementation.
use super::{Key, VDiff, VNode, VText};
use crate::html::{AnyScope, NodeRef};
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
        VList { children, key }
    }

    /// Add `VNode` child.
    pub fn add_child(&mut self, child: VNode) {
        self.children.push(child);
    }

    /// Add multiple `VNode` children.
    pub fn add_children(&mut self, children: impl IntoIterator<Item = VNode>) {
        self.children.extend(children);
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
        // Here, we will try to diff the previous list elements with the new
        // ones we want to insert. For that, we will use two lists:
        //  - lefts: new elements to render in the DOM
        //  - rights: previously rendered elements.
        //
        // The left items are known since we want to insert them
        // (self.children). For the right ones, we will look at the ancestor,
        // i.e. the current DOM list element that we want to replace with self.

        if self.children.is_empty() {
            // Without a placeholder the next element becomes first
            // and corrupts the order of rendering
            // We use empty text element to stake out a place
            let placeholder = VText::new("".into());
            self.children.push(placeholder.into());
        }

        // Check for lefts to see if there are duplicates and show a warning
        // (no need to check in right, there cannot be duplicates).
        let n_keyed_lefts = {
            let mut hash_set = HashSet::with_capacity(self.children.len());
            for l in self.children.iter() {
                if let Some(k) = l.key() {
                    if !hash_set.insert(k) {
                        log::error!("Key '{}' is not unique in list but must be.", &k);
                    }
                }
            }
            hash_set.len()
        };
        let lefts_some_keyed = n_keyed_lefts > 0;
        let lefts_all_keyed = self.children.len() == n_keyed_lefts;
        if lefts_some_keyed && !lefts_all_keyed {
            log::error!(
                "Not all elements have keys in VList ({} keyed out of {}), this is currently not \
                supported. Ignoring keys.",
                n_keyed_lefts,
                self.children.len(),
            );
        }

        // Take rights from ancestor.
        let mut rights = match ancestor {
            // If the ancestor is also a VList, then the right list is the
            // previously rendered items.
            Some(VNode::VList(vlist)) => vlist.children,
            // Otherwise, there was a node before but it wasn't a list. Then,
            // use the current node as a single fragment list and let the
            // `apply` of `VNode` handle it.
            Some(vnode) if !lefts_some_keyed => vec![vnode],
            // Otherwise, we don't reuse it, as the chance that the element
            // is keyed and present in left is almost null.
            Some(mut vnode) => {
                vnode.detach(parent);
                vec![]
            }
            None => vec![],
        };

        // Collect the right/ancestor keyed elements in a hashmap.
        let mut rights_lookup = rights
            .iter()
            .enumerate()
            .filter_map(|(idx, vnode)| vnode.key().as_ref().map(|key| (key.clone(), idx)))
            .collect::<HashMap<_, _>>();

        // Determine which algorithm we use. If there are some keys, but not all
        // the elements are keyed, we consider it a degenerated case and use the
        // non-keyed algorithm. Importantly, don't use the keyed algorithm if
        // rights is empty (no need) or if there are no keys in rights (no need
        // neither).
        let use_keyed_algorithm =
            lefts_all_keyed && !rights.is_empty() && !rights_lookup.is_empty();

        // If there are keys in right but we are not using the keyed algorithm,
        // then we detach all the rights to be sure to not reuse a keyed
        // ancestor. We also detach rights if there are some keys but not all
        // have keys.
        if (!use_keyed_algorithm && !rights_lookup.is_empty())
            || (use_keyed_algorithm && rights_lookup.len() != rights.len())
        {
            rights.drain(..).for_each(|mut right| {
                right.detach(parent);
            });
        }

        // The algorithms are different when there are keys, because it is more
        // expensive and less frequent.
        if !use_keyed_algorithm {
            let mut rights = rights.into_iter().peekable();
            let mut last_next_sibling = NodeRef::default();
            let mut nodes: Vec<NodeRef> = self
                .children
                .iter_mut()
                .map(|left| {
                    let ancestor = rights.next();

                    // Create a new `next_sibling` reference which points to the next `right` or
                    // the outer list's `next_sibling` if there are no more `rights`.
                    let new_next_sibling = NodeRef::default();
                    if let Some(next_right) = rights.peek() {
                        new_next_sibling.set(Some(next_right.first_node()));
                    } else {
                        new_next_sibling.link(next_sibling.clone());
                    }

                    // Update the next list item and then link the previous left's `next_sibling` to the returned `node` reference
                    // so that the previous left has an up-to-date `next_sibling` (important for mounting a `Component`)
                    let node = left.apply(parent_scope, parent, new_next_sibling.clone(), ancestor);
                    last_next_sibling.link(node.clone());
                    last_next_sibling = new_next_sibling;
                    node
                })
                .collect();

            // If there are more `rights` than `lefts`, we need to make sure to link the last left's `next_sibling`
            // to the outer list's `next_sibling` so that it doesn't point at a `right` that is detached.
            last_next_sibling.link(next_sibling);

            // Detach all extra rights
            for mut right in rights {
                right.detach(parent);
            }

            assert!(!nodes.is_empty(), "VList should have at least one child");
            nodes.swap_remove(0)
        } else {
            // Here, we know that all the left and right elements have keys.

            // Copy the keys from rights, as we need them for moving the vnodes
            // at the end of the algorithm, but we will steal nodes from
            // `rights` so will not be able to use it.
            let right_keys: Vec<Key> = rights
                .iter()
                .map(|vnode| vnode.key().clone().expect("right must have a key"))
                .collect();

            // We will try to match the left vnodes with the right ones, and
            // store them in `matched_rights`: it is formed of vnodes from
            // right, in the same order than left. `reused_rights` is used to
            // know which right nodes have been matched and reused.
            let mut matched_rights: Vec<Option<VNode>> = vec![None; self.children.len()];
            let mut reused_rights = FixedBitSet::with_capacity(rights.len());

            // Generator of dummy vnodes, used when stealing vnodes from rights.
            let make_dummy = || {
                VNode::VText(VText {
                    text: String::default(),
                    reference: None,
                })
            };

            // Match all the elements that are not new, but may have moved.
            let mut new_lefts: HashSet<usize> = HashSet::with_capacity(self.children.len());
            for ((left_idx, left), matched_right) in self
                .children
                .iter()
                .enumerate()
                .zip(matched_rights.iter_mut())
            {
                let left_key = left.key().as_ref().expect("must have a key");
                if let Some(right_idx) = rights_lookup.remove(&left_key) {
                    let right = rights
                        .get_mut(right_idx)
                        .expect("the index from the map must be valid");
                    let right = core::mem::replace(right, make_dummy());
                    *matched_right = Some(right);
                    reused_rights.put(right_idx);
                } else {
                    // This left node is new.
                    new_lefts.insert(left_idx);
                }
            }
            let new_lefts = new_lefts; // remove mutability

            // Reconciliation loop, i.e. apply the least amount of
            // transformations to rights to make them identical to lefts.
            let mut matched_rights = matched_rights.into_iter().peekable();
            let mut last_next_sibling = NodeRef::default();
            let mut nodes: Vec<NodeRef> = self
                .children
                .iter_mut()
                .map(|left| {
                    let ancestor = matched_rights.next().unwrap();

                    // Create a new `next_sibling` reference which points to the next `right` or
                    // the outer list's `next_sibling` if there are no more `rights`.
                    let new_next_sibling = NodeRef::default();
                    if let Some(Some(next_right)) = matched_rights.peek() {
                        new_next_sibling.set(Some(next_right.first_node()));
                    } else {
                        new_next_sibling.link(next_sibling.clone());
                    }

                    // Update the next list item and then link the previous left's `next_sibling` to the returned `node` reference
                    // so that the previous left has an up-to-date `next_sibling` (important for mounting a `Component`)
                    let node = left.apply(parent_scope, parent, new_next_sibling.clone(), ancestor);
                    last_next_sibling.link(node.clone());
                    last_next_sibling = new_next_sibling;
                    node
                })
                .collect();

            // If there are more `rights` than `lefts`, we need to make sure to link the last left's `next_sibling`
            // to the outer list's `next_sibling` so that it doesn't point at a `right` that is detached.
            last_next_sibling.link(next_sibling);

            drop(matched_rights);

            // The remaining items in this map are the vnodes that have not been
            // reused, hence that have been deleted. We just rename the map for
            // clarity.
            let deleted_rights = rights_lookup;

            // Move in the DOM the nodes that have been reused.
            let mut moved: HashSet<Key> = HashSet::with_capacity(self.children.len());
            let mut lefts = self.children.iter().enumerate().peekable();
            let mut right_keys = right_keys.into_iter().peekable();
            let mut right_key = right_keys.next();
            let mut moves: Vec<(Node, Option<Node>)> = vec![];
            while let Some((idx, left)) = lefts.next() {
                // Ignore the new left vnodes, which are created at the correct
                // position.
                if new_lefts.contains(&idx) {
                    continue;
                }

                // Ignore the deleted right vnodes, and those corresponding to
                // already moved left vnodes.
                while let Some(key) = right_key.clone() {
                    if moved.contains(&key) || deleted_rights.contains_key(&key) {
                        right_key = right_keys.next();
                    } else {
                        break;
                    }
                }

                let left_key = left.key().as_ref().expect("left must have key");

                // Optimization: try to peek one after the current one, to
                // detect one-off moves. We know that if the current right is
                // keyed, it hasn't been deleted or already moved, so it must be
                // present in left.
                if let (a, Some(b)) = (left_key, right_keys.peek()) {
                    if a == b {
                        // Skip the right key. This will force a move of the
                        // matching left one later when we will process it.
                        right_key = right_keys.next();
                    }
                }

                match (left_key, &right_key) {
                    (left_key, Some(b)) if left_key == b => {
                        // Keys are matching, no move needed.
                        right_key = right_keys.next();
                    }
                    (left_key, _) => {
                        // Move the left vnode.
                        let left_node = left.first_node();
                        let next_sibling = match lefts.peek() {
                            Some((_, vnode)) => Some(vnode.first_node()),
                            _ => None,
                        };
                        moves.push((left_node, next_sibling));

                        // Remember that we moved it, to allow skipping the
                        // matching right if there is one.
                        moved.insert(left_key.clone());
                    }
                }
            }
            drop(moved);
            drop(lefts);
            drop(right_keys);
            drop(right_key);
            for (node, next_sibling) in moves.into_iter().rev() {
                super::insert_node(&node, parent, next_sibling);
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

            assert!(!nodes.is_empty(), "VList should have at least one child");
            nodes.swap_remove(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Element, Node};
    use crate::html::{AnyScope, Scope};
    use crate::prelude::*;
    use crate::virtual_dom::{Key, VChild, VComp, VDiff, VList, VNode, VTag, VText};
    use std::borrow::Cow;
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    struct Comp {
        id: usize,
        panic_if_changes: bool,
    }

    #[derive(Properties, Clone)]
    struct CountingCompProps {
        id: usize,
        panic_if_changes: bool,
    }

    impl Component for Comp {
        type Message = ();
        type Properties = CountingCompProps;

        fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
            Comp {
                id: props.id,
                panic_if_changes: props.panic_if_changes,
            }
        }

        fn change(&mut self, props: Self::Properties) -> ShouldRender {
            #[cfg(feature = "wasm_test")]
            wasm_bindgen_test::console_log!("Comp changed: {} -> {}", self.id, props.id);
            let changed = self.id != props.id;
            if self.panic_if_changes && changed {
                panic!(
                    "VComp changed but should not have: {} -> {}.",
                    self.id, props.id
                );
            }
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

    fn new_counting_vcomp(id: usize, key: Option<&'static str>, panic_if_changes: bool) -> VComp {
        VChild::<Comp>::new(
            CountingCompProps {
                id,
                panic_if_changes,
            },
            NodeRef::default(),
            key.map(|key| Key::from(key.to_string())),
        )
        .into()
    }

    #[cfg(feature = "web_sys")]
    fn inner_html(element: &Element) -> String {
        element.inner_html()
    }

    #[cfg(feature = "std_web")]
    fn inner_html(element: &Element) -> String {
        use stdweb::unstable::TryInto;
        use stdweb::web::{HtmlElement, IHtmlElement};
        let html_element: HtmlElement = element
            .clone()
            .try_into()
            .expect("Failed to convert element into HtmlElement");
        html_element.inner_text()
    }

    #[test]
    fn vlist_vdiff_apply_non_keyed_from_none_works_with_all_vnode_types_as_children() {
        let vref_element: Element = crate::utils::document().create_element("i").unwrap();
        let vref_node: Node = vref_element.clone().into();
        let mut vlist = VList::new_with_children(
            vec![
                VNode::VText(VText::new("a".into())),
                VNode::VTag(Box::new(VTag::new("span"))),
                VNode::VText(VText::new("c".into())),
                VNode::VText(VText::new("d".into())),
                VNode::VComp(new_counting_vcomp(0, None, false)),
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
        vlist.apply(&parent_scope, &parent_element, NodeRef::default(), None);

        assert_eq!(
            inner_html(&parent_element),
            "a<span></span>cd<p>0</p>foobar<i></i>",
            "The VList didn't render properly."
        );
    }

    #[test]
    fn vlist_vdiff_apply_keyed_from_none_works_with_all_vnode_types_as_children() {
        let vref_element: Element = crate::utils::document().create_element("i").unwrap();
        let vref_node: Node = vref_element.clone().into();
        let vtag = new_keyed_vtag("span", "vtag");
        let mut vlist = VList::new_with_children(
            vec![
                VNode::VText(VText::new("a".into())),
                VNode::VTag(Box::new(vtag)),
                VNode::VText(VText::new("c".into())),
                VNode::VText(VText::new("d".into())),
                VNode::VComp(new_counting_vcomp(0, Some("vchild"), true)),
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
        vlist.apply(&parent_scope, &parent_element, NodeRef::default(), None);

        assert_eq!(
            inner_html(&parent_element),
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
        ancestor_vlist.apply(&parent_scope, &parent_element, NodeRef::default(), None);
        assert_eq!(
            inner_html(&parent_element),
            ancestor_inner_html.as_ref(),
            "ancestor VList didn't render properly"
        );

        let mut vlist = VList::new_with_children(new_children, None);
        vlist.apply(
            &parent_scope,
            &parent_element,
            NodeRef::default(),
            Some(VNode::VList(ancestor_vlist)),
        );
        assert_eq!(
            inner_html(&parent_element),
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
    fn vlist_vdiff_non_keyed_from_ancestor_inserting_into_vlist_first_child() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VList::new_with_children(vec![VText::new("1".into()).into()], None).into(),
                VText::new("after".into()).into(),
            ],
            "1after",
            vec![
                VList::new_with_children(
                    vec![VText::new("1".into()).into(), VTag::new("p").into()],
                    None,
                )
                .into(),
                VText::new("after".into()).into(),
            ],
            "1<p></p>after",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_inserting_into_vlist_first_child() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VList::new_with_children(
                    vec![new_keyed_vtag("i", "i").into()],
                    Some(Key::from(String::from("VList"))),
                )
                .into(),
                new_keyed_vtag("p", "p").into(),
            ],
            "<i></i><p></p>",
            vec![
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("i", "i").into(),
                        new_keyed_vtag("e", "e").into(),
                    ],
                    Some(Key::from(String::from("VList"))),
                )
                .into(),
                new_keyed_vtag("p", "p").into(),
            ],
            "<i></i><e></e><p></p>",
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
    fn vlist_vdiff_keyed_list_of_list_from_ancestor_swap_1() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("i", "i").into(),
                        new_keyed_vtag("e", "e").into(),
                    ],
                    Some(Key::from(String::from("l1"))),
                )
                .into(),
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("a", "a").into(),
                        new_keyed_vtag("u", "u").into(),
                    ],
                    Some(Key::from(String::from("l2"))),
                )
                .into(),
            ],
            "<i></i><e></e><a></a><u></u>",
            vec![
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("a", "a").into(),
                        new_keyed_vtag("u", "u").into(),
                    ],
                    Some(Key::from(String::from("l2"))),
                )
                .into(),
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("i", "i").into(),
                        new_keyed_vtag("e", "e").into(),
                    ],
                    Some(Key::from(String::from("l1"))),
                )
                .into(),
            ],
            "<a></a><u></u><i></i><e></e>",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_list_of_list_from_ancestor_swap_no_collapse() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("i", "i").into(),
                        new_keyed_vtag("e", "e").into(),
                    ],
                    Some(Key::from(String::from("l1"))),
                )
                .into(),
                new_keyed_vtag("p", "p").into(),
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("a", "a").into(),
                        new_keyed_vtag("u", "u").into(),
                    ],
                    Some(Key::from(String::from("l2"))),
                )
                .into(),
            ],
            "<i></i><e></e><p></p><a></a><u></u>",
            vec![
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("a", "a").into(),
                        new_keyed_vtag("u", "u").into(),
                    ],
                    Some(Key::from(String::from("l2"))),
                )
                .into(),
                new_keyed_vtag("p", "p").into(),
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("i", "i").into(),
                        new_keyed_vtag("e", "e").into(),
                    ],
                    Some(Key::from(String::from("l1"))),
                )
                .into(),
            ],
            "<a></a><u></u><p></p><i></i><e></e>",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_with_multiple_children_keyed_types() {
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
                new_counting_vcomp(0, Some("VComp"), true).into(),
            ],
            "<u></u><a></a><i></i><p>0</p>",
            vec![
                new_counting_vcomp(0, Some("VComp"), true).into(),
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

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_insert_vcomp_front() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("u", "1").into(),
                new_keyed_vtag("a", "2").into(),
            ],
            "<u></u><a></a>",
            vec![
                new_counting_vcomp(0, Some("VComp"), true).into(),
                new_keyed_vtag("u", "1").into(),
                new_keyed_vtag("a", "2").into(),
            ],
            "<p>0</p><u></u><a></a>",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_insert_vcomp_middle() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("u", "1").into(),
                new_keyed_vtag("a", "2").into(),
            ],
            "<u></u><a></a>",
            vec![
                new_keyed_vtag("u", "1").into(),
                new_counting_vcomp(0, Some("VComp"), true).into(),
                new_keyed_vtag("a", "2").into(),
            ],
            "<u></u><p>0</p><a></a>",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_insert_vcomp_back() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("u", "1").into(),
                new_keyed_vtag("a", "2").into(),
            ],
            "<u></u><a></a>",
            vec![
                new_keyed_vtag("u", "1").into(),
                new_keyed_vtag("a", "2").into(),
                new_counting_vcomp(0, Some("VComp"), true).into(),
            ],
            "<u></u><a></a><p>0</p>",
        );
    }

    #[test]
    fn vlist_vdiff_keyed_from_ancestor_vcomp_children_reverse() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_counting_vcomp(1, Some("1"), true).into(),
                new_counting_vcomp(2, Some("2"), true).into(),
                new_counting_vcomp(3, Some("3"), true).into(),
            ],
            "<p>1</p><p>2</p><p>3</p>",
            vec![
                new_counting_vcomp(3, Some("3"), true).into(),
                new_counting_vcomp(2, Some("2"), true).into(),
                new_counting_vcomp(1, Some("1"), true).into(),
            ],
            "<p>3</p><p>2</p><p>1</p>",
        );
    }

    #[test]
    fn vlist_vdiff_do_not_reuse_non_vlist_ancestor_with_keyed_algorithm() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VTag::new("p").into(), // <- this node must not be reused ...
            ],
            "<p></p>",
            vec![VList::new_with_children(
                vec![
                    // v-- ... because the children here are keyed
                    new_keyed_vtag("a", "a").into(),
                    new_keyed_vtag("i", "i").into(),
                ],
                None,
            )
            .into()],
            "<a></a><i></i>",
        );
    }

    #[test]
    fn vlist_vdiff_mixed_keyed_and_non_keyed_from_ancestor_1() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("u", "u").into(),
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("a", "a").into(),
                        new_keyed_vtag("i", "i").into(),
                    ],
                    Some(Key::from("VList".to_string())),
                )
                .into(),
                VTag::new("p").into(),
            ],
            "<u></u><a></a><i></i><p></p>",
            vec![
                VTag::new("p").into(),
                new_keyed_vtag("u", "u").into(),
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("a", "a").into(),
                        new_keyed_vtag("i", "i").into(),
                    ],
                    Some(Key::from("VList".to_string())),
                )
                .into(),
            ],
            "<p></p><u></u><a></a><i></i>",
        );
    }

    #[test]
    fn vlist_vdiff_mixed_keyed_and_non_keyed_from_ancestor_2() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VTag::new("p").into(),
                new_keyed_vtag("u", "u").into(),
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("a", "a").into(),
                        new_keyed_vtag("i", "i").into(),
                    ],
                    Some(Key::from("VList".to_string())),
                )
                .into(),
            ],
            "<p></p><u></u><a></a><i></i>",
            vec![
                new_keyed_vtag("u", "u").into(),
                VTag::new("p").into(),
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("a", "a").into(),
                        new_keyed_vtag("i", "i").into(),
                    ],
                    Some(Key::from("VList".to_string())),
                )
                .into(),
            ],
            "<u></u><p></p><a></a><i></i>",
        );
    }

    #[test]
    fn vlist_vdiff_mixed_keyed_and_non_keyed_from_ancestor_3() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("a", "a").into(),
                        new_keyed_vtag("i", "i").into(),
                    ],
                    Some(Key::from("VList".to_string())),
                )
                .into(),
                VTag::new("p").into(),
                new_keyed_vtag("u", "u").into(),
            ],
            "<a></a><i></i><p></p><u></u>",
            vec![
                new_keyed_vtag("u", "u").into(),
                VTag::new("p").into(),
                VList::new_with_children(
                    vec![
                        new_keyed_vtag("a", "a").into(),
                        new_keyed_vtag("i", "i").into(),
                    ],
                    Some(Key::from("VList".to_string())),
                )
                .into(),
            ],
            "<u></u><p></p><a></a><i></i>",
        );
    }

    #[test]
    fn vlist_vdiff_mixed_keyed_and_non_keyed_from_ancestor_only_vtags_insert_1() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VTag::new("p").into(),
                VTag::new("a").into(),
                new_keyed_vtag("u", "u").into(),
            ],
            "<p></p><a></a><u></u>",
            vec![
                VTag::new("p").into(),
                VTag::new("i").into(),
                VTag::new("a").into(),
                new_keyed_vtag("u", "u").into(),
            ],
            "<p></p><i></i><a></a><u></u>",
        );
    }

    #[test]
    fn vlist_vdiff_mixed_keyed_and_non_keyed_from_ancestor_only_vtags_insert_2() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VTag::new("p").into(),
                VTag::new("a").into(),
                new_keyed_vtag("u", "u").into(),
            ],
            "<p></p><a></a><u></u>",
            vec![
                VTag::new("i").into(),
                VTag::new("p").into(),
                VTag::new("a").into(),
                new_keyed_vtag("u", "u").into(),
            ],
            "<i></i><p></p><a></a><u></u>",
        );
    }

    #[test]
    fn vlist_vdiff_mixed_keyed_and_non_keyed_from_ancestor_only_vtags_insert_3() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VTag::new("p").into(),
                VTag::new("a").into(),
                new_keyed_vtag("u", "u").into(),
            ],
            "<p></p><a></a><u></u>",
            vec![
                VTag::new("p").into(),
                VTag::new("a").into(),
                new_keyed_vtag("u", "u").into(),
                VTag::new("i").into(),
            ],
            "<p></p><a></a><u></u><i></i>",
        );
    }

    #[test]
    fn vlist_vdiff_mixed_keyed_and_non_keyed_from_ancestor_only_vtags_insert_4() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VTag::new("p").into(),
                VTag::new("a").into(),
                new_keyed_vtag("u", "u").into(),
            ],
            "<p></p><a></a><u></u>",
            vec![
                VTag::new("p").into(),
                VTag::new("a").into(),
                VTag::new("i").into(),
                new_keyed_vtag("u", "u").into(),
            ],
            "<p></p><a></a><i></i><u></u>",
        );
    }

    #[test]
    fn vlist_vdiff_mixed_keyed_and_non_keyed_from_ancestor_only_vtags_swap_1() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VTag::new("p").into(),
                VTag::new("a").into(),
                new_keyed_vtag("u", "u").into(),
            ],
            "<p></p><a></a><u></u>",
            vec![
                VTag::new("a").into(),
                VTag::new("p").into(),
                new_keyed_vtag("u", "u").into(),
            ],
            "<a></a><p></p><u></u>",
        );
    }

    #[test]
    fn vlist_vdiff_mixed_keyed_and_non_keyed_from_ancestor_only_vtags_swap_2() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                VTag::new("p").into(),
                VTag::new("a").into(),
                new_keyed_vtag("u", "u").into(),
            ],
            "<p></p><a></a><u></u>",
            vec![
                VTag::new("a").into(),
                new_keyed_vtag("u", "u").into(),
                VTag::new("p").into(),
            ],
            "<a></a><u></u><p></p>",
        );
    }

    #[test]
    fn vlist_vdiff_mixed_keyed_and_non_keyed_from_ancestor_only_vtags_swap_3() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("u", "u").into(),
                VTag::new("p").into(),
                VTag::new("a").into(),
            ],
            "<u></u><p></p><a></a>",
            vec![
                VTag::new("a").into(),
                VTag::new("p").into(),
                new_keyed_vtag("u", "u").into(),
            ],
            "<a></a><p></p><u></u>",
        );
    }

    #[test]
    fn vlist_vdiff_mixed_keyed_and_non_keyed_from_ancestor_only_vtags_replace_keyed_1() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("u", "u1").into(),
                VTag::new("p").into(),
                VTag::new("a").into(),
            ],
            "<u></u><p></p><a></a>",
            vec![
                new_keyed_vtag("u", "u2").into(),
                VTag::new("p").into(),
                VTag::new("a").into(),
            ],
            "<u></u><p></p><a></a>",
        );
    }

    #[test]
    fn vlist_vdiff_mixed_keyed_and_non_keyed_from_ancestor_only_vtags_replace_keyed_2() {
        test_vlist_vdiff_from_ancestor_works(
            vec![
                new_keyed_vtag("u", "u1").into(),
                VTag::new("p").into(),
                VTag::new("a").into(),
            ],
            "<u></u><p></p><a></a>",
            vec![
                VTag::new("a").into(),
                new_keyed_vtag("u", "u2").into(),
                VTag::new("p").into(),
            ],
            "<a></a><u></u><p></p>",
        );
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
