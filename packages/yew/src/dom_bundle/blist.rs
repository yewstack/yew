//! This module contains fragments bundles, a [BList]
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Deref;

use web_sys::Element;

use super::{test_log, BNode, BSubtree};
use crate::dom_bundle::{Reconcilable, ReconcileTarget};
use crate::html::{AnyScope, NodeRef};
use crate::virtual_dom::{Key, VList, VNode, VText};

/// This struct represents a mounted [VList]
#[derive(Debug)]
pub(super) struct BList {
    /// The reverse (render order) list of child [BNode]s
    rev_children: Vec<BNode>,
    /// All [BNode]s in the BList have keys
    fully_keyed: bool,
    key: Option<Key>,
}

impl Deref for BList {
    type Target = Vec<BNode>;

    fn deref(&self) -> &Self::Target {
        &self.rev_children
    }
}

/// Helper struct, that keeps the position where the next element is to be placed at
#[derive(Clone)]
struct NodeWriter<'s> {
    root: &'s BSubtree,
    parent_scope: &'s AnyScope,
    parent: &'s Element,
    next_sibling: NodeRef,
}

impl<'s> NodeWriter<'s> {
    /// Write a new node that has no ancestor
    fn add(self, node: VNode) -> (Self, BNode) {
        test_log!("adding: {:?}", node);
        test_log!(
            "  parent={:?}, next_sibling={:?}",
            self.parent.outer_html(),
            self.next_sibling
        );
        let (next, bundle) =
            node.attach(self.root, self.parent_scope, self.parent, self.next_sibling);
        test_log!("  next_position: {:?}", next);
        (
            Self {
                next_sibling: next,
                ..self
            },
            bundle,
        )
    }

    /// Shift a bundle into place without patching it
    fn shift(&self, bundle: &mut BNode) {
        bundle.shift(self.parent, self.next_sibling.clone());
    }

    /// Patch a bundle with a new node
    fn patch(self, node: VNode, bundle: &mut BNode) -> Self {
        test_log!("patching: {:?} -> {:?}", bundle, node);
        test_log!(
            "  parent={:?}, next_sibling={:?}",
            self.parent.outer_html(),
            self.next_sibling
        );
        // Advance the next sibling reference (from right to left)
        let next = node.reconcile_node(
            self.root,
            self.parent_scope,
            self.parent,
            self.next_sibling,
            bundle,
        );
        test_log!("  next_position: {:?}", next);
        Self {
            next_sibling: next,
            ..self
        }
    }
}
/// Helper struct implementing [Eq] and [Hash] by only looking at a node's key
struct KeyedEntry(usize, BNode);
impl Borrow<Key> for KeyedEntry {
    fn borrow(&self) -> &Key {
        self.1.key().expect("unkeyed child in fully keyed list")
    }
}
impl Hash for KeyedEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        <Self as Borrow<Key>>::borrow(self).hash(state)
    }
}
impl PartialEq for KeyedEntry {
    fn eq(&self, other: &Self) -> bool {
        <Self as Borrow<Key>>::borrow(self) == <Self as Borrow<Key>>::borrow(other)
    }
}
impl Eq for KeyedEntry {}

impl BNode {
    /// Assert that a bundle node is a list, or convert it to a list with a single child
    fn make_list(&mut self) -> &mut BList {
        match self {
            Self::List(blist) => blist,
            self_ => {
                let b = std::mem::replace(self_, BNode::List(BList::new()));
                let self_list = match self_ {
                    BNode::List(blist) => blist,
                    _ => unreachable!("just been set to the variant"),
                };
                let key = b.key().cloned();
                self_list.rev_children.push(b);
                self_list.fully_keyed = key.is_some();
                self_list.key = key;
                self_list
            }
        }
    }
}

impl BList {
    /// Create a new empty [BList]
    pub const fn new() -> BList {
        BList {
            rev_children: vec![],
            fully_keyed: true,
            key: None,
        }
    }

    /// Get the key of the underlying fragment
    pub fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }

    /// Diff and patch unkeyed child lists
    fn apply_unkeyed(
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        lefts: Vec<VNode>,
        rights: &mut Vec<BNode>,
    ) -> NodeRef {
        let mut writer = NodeWriter {
            root,
            parent_scope,
            parent,
            next_sibling,
        };

        // Remove extra nodes
        if lefts.len() < rights.len() {
            for r in rights.drain(lefts.len()..) {
                test_log!("removing: {:?}", r);
                r.detach(root, parent, false);
            }
        }

        let mut lefts_it = lefts.into_iter().rev();
        for (r, l) in rights.iter_mut().zip(&mut lefts_it) {
            writer = writer.patch(l, r);
        }

        // Add missing nodes
        for l in lefts_it {
            let (next_writer, el) = writer.add(l);
            rights.push(el);
            writer = next_writer;
        }
        writer.next_sibling
    }

    /// Diff and patch fully keyed child lists.
    ///
    /// Optimized for node addition or removal from either end of the list and small changes in the
    /// middle.
    fn apply_keyed(
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        left_vdoms: Vec<VNode>,
        rev_bundles: &mut Vec<BNode>,
    ) -> NodeRef {
        macro_rules! key {
            ($v:expr) => {
                $v.key().expect("unkeyed child in fully keyed list")
            };
        }
        /// Find the first differing key in 2 iterators
        fn matching_len<'a, 'b>(
            a: impl Iterator<Item = &'a Key>,
            b: impl Iterator<Item = &'b Key>,
        ) -> usize {
            a.zip(b).take_while(|(a, b)| a == b).count()
        }

        // Find first key mismatch from the back
        let matching_len_end = matching_len(
            left_vdoms.iter().map(|v| key!(v)).rev(),
            rev_bundles.iter().map(|v| key!(v)),
        );

        // If there is no key mismatch, apply the unkeyed approach
        // Corresponds to adding or removing items from the back of the list
        if matching_len_end == std::cmp::min(left_vdoms.len(), rev_bundles.len()) {
            // No key changes
            return Self::apply_unkeyed(
                root,
                parent_scope,
                parent,
                next_sibling,
                left_vdoms,
                rev_bundles,
            );
        }

        // We partially drain the new vnodes in several steps.
        let mut lefts = left_vdoms;
        let mut writer = NodeWriter {
            root,
            parent_scope,
            parent,
            next_sibling,
        };
        // Step 1. Diff matching children at the end
        let lefts_to = lefts.len() - matching_len_end;
        for (l, r) in lefts
            .drain(lefts_to..)
            .rev()
            .zip(rev_bundles[..matching_len_end].iter_mut())
        {
            writer = writer.patch(l, r);
        }

        // Step 2. Diff matching children in the middle, that is between the first and last key
        // mismatch Find first key mismatch from the front
        let matching_len_start = matching_len(
            lefts.iter().map(|v| key!(v)),
            rev_bundles.iter().map(|v| key!(v)).rev(),
        );

        // Step 2.1. Splice out the existing middle part and build a lookup by key
        let rights_to = rev_bundles.len() - matching_len_start;
        let mut spliced_middle =
            rev_bundles.splice(matching_len_end..rights_to, std::iter::empty());
        let mut spare_bundles: HashSet<KeyedEntry> =
            HashSet::with_capacity((matching_len_end..rights_to).len());
        for (idx, r) in (&mut spliced_middle).enumerate() {
            spare_bundles.insert(KeyedEntry(idx, r));
        }

        // Step 2.2. Put the middle part back together in the new key order
        let mut replacements: Vec<BNode> = Vec::with_capacity((matching_len_start..lefts_to).len());
        // The goal is to shift as few nodes as possible.

        // We handle runs of in-order nodes. When we encounter one out-of-order, we decide whether:
        // - to shift all nodes in the current run to the position after the node before of the run,
        //   or to
        // - "commit" to the current run, shift all nodes before the end of the run that we might
        //   encounter in the future, and then start a new run.
        // Example of a run:
        //               barrier_idx --v                   v-- end_idx
        // spliced_middle  [ ... , M , N , C , D , E , F , G , ... ] (original element order)
        //                                 ^---^-----------^ the nodes that are part of the current
        // run                           v start_writer
        // replacements    [ ... , M , C , D , G ]                   (new element order)
        //                             ^-- start_idx
        let mut barrier_idx = 0; // nodes from spliced_middle[..barrier_idx] are shifted unconditionally
        struct RunInformation<'a> {
            start_writer: NodeWriter<'a>,
            start_idx: usize,
            end_idx: usize,
        }
        let mut current_run: Option<RunInformation<'_>> = None;

        for l in lefts
            .drain(matching_len_start..) // lefts_to.. has been drained
            .rev()
        {
            let ancestor = spare_bundles.take(key!(l));
            // Check if we need to shift or commit a run
            if let Some(run) = current_run.as_mut() {
                if let Some(KeyedEntry(idx, _)) = ancestor {
                    // If there are only few runs, this is a cold path
                    if idx < run.end_idx {
                        // Have to decide whether to shift or commit the current run. A few
                        // calculations: A perfect estimate of the amount of
                        // nodes we have to shift if we move this run:
                        let run_length = replacements.len() - run.start_idx;
                        // A very crude estimate of the amount of nodes we will have to shift if we
                        // commit the run: Note nodes of the current run
                        // should not be counted here!
                        let estimated_skipped_nodes = run.end_idx - idx.max(barrier_idx);
                        // double run_length to counteract that the run is part of the
                        // estimated_skipped_nodes
                        if 2 * run_length > estimated_skipped_nodes {
                            // less work to commit to this run
                            barrier_idx = 1 + run.end_idx;
                        } else {
                            // Less work to shift this run
                            for r in replacements[run.start_idx..].iter_mut().rev() {
                                run.start_writer.shift(r);
                            }
                        }
                        current_run = None;
                    }
                }
            }
            let bundle = if let Some(KeyedEntry(idx, mut r_bundle)) = ancestor {
                match current_run.as_mut() {
                    // hot path
                    // We know that idx >= run.end_idx, so this node doesn't need to shift
                    Some(run) => run.end_idx = idx,
                    None => match idx.cmp(&barrier_idx) {
                        // peep hole optimization, don't start a run as the element is already where
                        // it should be
                        Ordering::Equal => barrier_idx += 1,
                        // shift the node unconditionally, don't start a run
                        Ordering::Less => writer.shift(&mut r_bundle),
                        // start a run
                        Ordering::Greater => {
                            current_run = Some(RunInformation {
                                start_writer: writer.clone(),
                                start_idx: replacements.len(),
                                end_idx: idx,
                            })
                        }
                    },
                }
                writer = writer.patch(l, &mut r_bundle);
                r_bundle
            } else {
                // Even if there is an active run, we don't have to modify it
                let (next_writer, bundle) = writer.add(l);
                writer = next_writer;
                bundle
            };
            replacements.push(bundle);
        }
        // drop the splice iterator and immediately replace the range with the reordered elements
        drop(spliced_middle);
        rev_bundles.splice(matching_len_end..matching_len_end, replacements);

        // Step 2.3. Remove any extra rights
        for KeyedEntry(_, r) in spare_bundles.drain() {
            test_log!("removing: {:?}", r);
            r.detach(root, parent, false);
        }

        // Step 3. Diff matching children at the start
        let rights_to = rev_bundles.len() - matching_len_start;
        for (l, r) in lefts
            .drain(..) // matching_len_start.. has been drained already
            .rev()
            .zip(rev_bundles[rights_to..].iter_mut())
        {
            writer = writer.patch(l, r);
        }

        writer.next_sibling
    }
}

impl ReconcileTarget for BList {
    fn detach(self, root: &BSubtree, parent: &Element, parent_to_detach: bool) {
        for child in self.rev_children.into_iter() {
            child.detach(root, parent, parent_to_detach);
        }
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) -> NodeRef {
        let mut next_sibling = next_sibling;

        for node in self.rev_children.iter() {
            next_sibling = node.shift(next_parent, next_sibling.clone());
        }

        next_sibling
    }
}

impl Reconcilable for VList {
    type Bundle = BList;

    fn attach(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let mut self_ = BList::new();
        let node_ref = self.reconcile(root, parent_scope, parent, next_sibling, &mut self_);
        (node_ref, self_)
    }

    fn reconcile_node(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        // 'Forcefully' pretend the existing node is a list. Creates a
        // singleton list if it isn't already.
        let blist = bundle.make_list();
        self.reconcile(root, parent_scope, parent, next_sibling, blist)
    }

    fn reconcile(
        mut self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        blist: &mut BList,
    ) -> NodeRef {
        // Here, we will try to diff the previous list elements with the new
        // ones we want to insert. For that, we will use two lists:
        //  - lefts: new elements to render in the DOM
        //  - rights: previously rendered elements.
        //
        // The left items are known since we want to insert them
        // (self.children). For the right ones, we will look at the bundle,
        // i.e. the current DOM list element that we want to replace with self.

        if self.children.is_empty() {
            // Without a placeholder the next element becomes first
            // and corrupts the order of rendering
            // We use empty text element to stake out a place
            self.add_child(VText::new("").into());
        }

        let fully_keyed = self.fully_keyed();
        let lefts = self.children;
        let rights = &mut blist.rev_children;
        test_log!("lefts: {:?}", lefts);
        test_log!("rights: {:?}", rights);

        if let Some(additional) = lefts.len().checked_sub(rights.len()) {
            rights.reserve_exact(additional);
        }
        let first = if fully_keyed && blist.fully_keyed {
            BList::apply_keyed(root, parent_scope, parent, next_sibling, lefts, rights)
        } else {
            BList::apply_unkeyed(root, parent_scope, parent, next_sibling, lefts, rights)
        };
        blist.fully_keyed = fully_keyed;
        blist.key = self.key;
        test_log!("result: {:?}", rights);
        first
    }
}

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;
    use crate::dom_bundle::{Fragment, Hydratable};

    impl Hydratable for VList {
        fn hydrate(
            self,
            root: &BSubtree,
            parent_scope: &AnyScope,
            parent: &Element,
            fragment: &mut Fragment,
        ) -> (NodeRef, Self::Bundle) {
            let node_ref = NodeRef::default();
            let fully_keyed = self.fully_keyed();
            let vchildren = self.children;
            let mut children = Vec::with_capacity(vchildren.len());

            for (index, child) in vchildren.into_iter().enumerate() {
                let (child_node_ref, child) = child.hydrate(root, parent_scope, parent, fragment);

                if index == 0 {
                    node_ref.link(child_node_ref);
                }

                children.push(child);
            }

            children.reverse();

            (
                node_ref,
                BList {
                    rev_children: children,
                    fully_keyed,
                    key: self.key,
                },
            )
        }
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
            name: "2",
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
            name: "3",
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
            name: "4",
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

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod layout_tests_keys {
    extern crate self as yew;

    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    use web_sys::Node;

    use crate::tests::layout_tests::{diff_layouts, TestLayout};
    use crate::virtual_dom::VNode;
    use crate::{html, Children, Component, Context, Html, Properties};

    wasm_bindgen_test_configure!(run_in_browser);

    struct Comp {}

    #[derive(Properties, Clone, PartialEq)]
    struct CountingCompProps {
        id: usize,
        #[prop_or(false)]
        can_change: bool,
    }

    impl Component for Comp {
        type Message = ();
        type Properties = CountingCompProps;

        fn create(_: &Context<Self>) -> Self {
            Comp {}
        }

        fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
            unimplemented!();
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! { <p>{ ctx.props().id }</p> }
        }
    }

    #[derive(Clone, Properties, PartialEq)]
    pub struct ListProps {
        pub children: Children,
    }

    pub struct List();

    impl Component for List {
        type Message = ();
        type Properties = ListProps;

        fn create(_: &Context<Self>) -> Self {
            Self()
        }

        fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
            unimplemented!();
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! { <>{ for ctx.props().children.iter() }</> }
        }
    }

    #[test]
    fn diff() {
        let mut layouts = vec![];

        let vref_node: Node = gloo::utils::document().create_element("i").unwrap().into();
        layouts.push(TestLayout {
            name: "All VNode types as children",
            node: html! {
                <>
                    {"a"}
                    <span key="vtag"></span>
                    {"c"}
                    {"d"}
                    <Comp id=0 key="vchild" />
                    <key="vlist">
                        {"foo"}
                        {"bar"}
                    </>
                    {VNode::VRef(vref_node)}
                </>
            },
            expected: "a<span></span>cd<p>0</p>foobar<i></i>",
        });

        layouts.extend(vec![
            TestLayout {
                name: "Inserting into VList first child - before",
                node: html! {
                    <>
                        <key="VList">
                            <i key="i"></i>
                        </>
                        <p key="p"></p>
                    </>
                },
                expected: "<i></i><p></p>",
            },
            TestLayout {
                name: "Inserting into VList first child - after",
                node: html! {
                    <>
                        <key="VList">
                            <i key="i"></i>
                            <e key="e"></e>
                        </>
                        <p key="p"></p>
                    </>
                },
                expected: "<i></i><e></e><p></p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "No matches - before",
                node: html! {
                    <>
                        <i key="i"></i>
                        <e key="e"></e>
                    </>
                },
                expected: "<i></i><e></e>",
            },
            TestLayout {
                name: "No matches - after",
                node: html! {
                    <>
                        <a key="a"></a>
                        <p key="p"></p>
                    </>
                },
                expected: "<a></a><p></p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Append - before",
                node: html! {
                    <>
                        <i key="i"></i>
                        <e key="e"></e>
                    </>
                },
                expected: "<i></i><e></e>",
            },
            TestLayout {
                name: "Append - after",
                node: html! {
                    <>
                        <i key="i"></i>
                        <e key="e"></e>
                        <p key="p"></p>
                    </>
                },
                expected: "<i></i><e></e><p></p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Prepend - before",
                node: html! {
                    <>
                        <i key="i"></i>
                        <e key="e"></e>
                    </>
                },
                expected: "<i></i><e></e>",
            },
            TestLayout {
                name: "Prepend - after",
                node: html! {
                    <>
                        <p key="p"></p>
                        <i key="i"></i>
                        <e key="e"></e>
                    </>
                },
                expected: "<p></p><i></i><e></e>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Delete first - before",
                node: html! {
                    <>
                        <i key="i"></i>
                        <e key="e"></e>
                        <p key="p"></p>
                    </>
                },
                expected: "<i></i><e></e><p></p>",
            },
            TestLayout {
                name: "Delete first - after",
                node: html! {
                    <>
                        <e key="e"></e>
                        <p key="p"></p>
                    </>
                },
                expected: "<e></e><p></p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Delete last - before",
                node: html! {
                    <>
                        <i key="i"></i>
                        <e key="e"></e>
                        <p key="p"></p>
                    </>
                },
                expected: "<i></i><e></e><p></p>",
            },
            TestLayout {
                name: "Delete last - after",
                node: html! {
                    <>
                        <i key="i"></i>
                        <e key="e"></e>
                    </>
                },
                expected: "<i></i><e></e>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Delete last and change node type - before",
                node: html! {
                    <>
                        <i key="i"></i>
                        <e key="e"></e>
                        <p key="p"></p>
                    </>
                },
                expected: "<i></i><e></e><p></p>",
            },
            TestLayout {
                name: "Delete last - after",
                node: html! {
                    <>
                        <List key="i"><i/></List>
                        <List key="e"><e/></List>
                        <List key="a"><a/></List>
                    </>
                },
                expected: "<i></i><e></e><a></a>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Delete middle - before",
                node: html! {
                    <>
                        <i key="i"></i>
                        <e key="e"></e>
                        <p key="p"></p>
                        <a key="a"></a>
                    </>
                },
                expected: "<i></i><e></e><p></p><a></a>",
            },
            TestLayout {
                name: "Delete middle - after",
                node: html! {
                    <>
                        <i key="i"></i>
                        <e key="e2"></e>
                        <p key="p2"></p>
                        <a key="a"></a>
                    </>
                },
                expected: "<i></i><e></e><p></p><a></a>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Delete middle and change node type - before",
                node: html! {
                    <>
                        <i key="i"></i>
                        <e key="e"></e>
                        <p key="p"></p>
                        <a key="a"></a>
                    </>
                },
                expected: "<i></i><e></e><p></p><a></a>",
            },
            TestLayout {
                name: "Delete middle and change node type- after",
                node: html! {
                    <>
                        <List key="i2"><i/></List>
                        <e key="e"></e>
                        <List key="p"><p/></List>
                        <List key="a2"><a/></List>
                    </>
                },
                expected: "<i></i><e></e><p></p><a></a>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Reverse - before",
                node: html! {
                    <>
                        <i key="i"></i>
                        <e key="e"></e>
                        <p key="p"></p>
                        <u key="u"></u>
                    </>
                },
                expected: "<i></i><e></e><p></p><u></u>",
            },
            TestLayout {
                name: "Reverse - after",
                node: html! {
                    <>
                        <u key="u"></u>
                        <p key="p"></p>
                        <e key="e"></e>
                        <i key="i"></i>
                    </>
                },
                expected: "<u></u><p></p><e></e><i></i>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Reverse and change node type - before",
                node: html! {
                    <>
                        <i key="i"></i>
                        <key="i1"></>
                        <key="i2"></>
                        <key="i3"></>
                        <e key="e"></e>
                        <key="yo">
                            <p key="p"></p>
                        </>
                        <u key="u"></u>
                    </>
                },
                expected: "<i></i><e></e><p></p><u></u>",
            },
            TestLayout {
                name: "Reverse and change node type - after",
                node: html! {
                    <>
                        <List key="u"><u/></List>
                        <List key="p"><p/></List>
                        <List key="e"><e/></List>
                        <List key="i"><i/></List>
                    </>
                },
                expected: "<u></u><p></p><e></e><i></i>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap 1&2 - before",
                node: html! {
                    <>
                        <i key="1"></i>
                        <e key="2"></e>
                        <p key="3"></p>
                        <a key="4"></a>
                        <u key="5"></u>
                    </>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap 1&2 - after",
                node: html! {
                    <>
                        <e key="2"></e>
                        <i key="1"></i>
                        <p key="3"></p>
                        <a key="4"></a>
                        <u key="5"></u>
                    </>
                },
                expected: "<e></e><i></i><p></p><a></a><u></u>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap 1&2 and change node type - before",
                node: html! {
                    <>
                        <i key="1"></i>
                        <e key="2"></e>
                        <p key="3"></p>
                        <a key="4"></a>
                        <u key="5"></u>
                    </>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap 1&2 and change node type - after",
                node: html! {
                    <>
                        <List key="2"><e/></List>
                        <List key="1"><i/></List>
                        <List key="3"><p/></List>
                        <List key="4"><a/></List>
                        <List key="5"><u/></List>
                    </>
                },
                expected: "<e></e><i></i><p></p><a></a><u></u>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "test - before",
                node: html! {
                    <>
                        <key="1">
                            <e key="e"></e>
                            <p key="p"></p>
                            <a key="a"></a>
                            <u key="u"></u>
                        </>
                        <key="2">
                            <e key="e"></e>
                            <p key="p"></p>
                            <a key="a"></a>
                            <u key="u"></u>
                        </>
                    </>
                },
                expected: "<e></e><p></p><a></a><u></u><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap 4&5 - after",
                node: html! {
                    <>
                        <e key="1"></e>
                        <key="2">
                            <p key="p"></p>
                            <i key="i"></i>
                        </>
                    </>
                },
                expected: "<e></e><p></p><i></i>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap 4&5 - before",
                node: html! {
                    <>
                        <i key="1"></i>
                        <e key="2"></e>
                        <p key="3"></p>
                        <a key="4"></a>
                        <u key="5"></u>
                    </>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap 4&5 - after",
                node: html! {
                    <>
                        <i key="1"></i>
                        <e key="2"></e>
                        <p key="3"></p>
                        <u key="5"></u>
                        <a key="4"></a>
                    </>
                },
                expected: "<i></i><e></e><p></p><u></u><a></a>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap 1&5 - before",
                node: html! {
                    <>
                        <i key="1"></i>
                        <e key="2"></e>
                        <p key="3"></p>
                        <a key="4"></a>
                        <u key="5"></u>
                    </>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap 1&5 - after",
                node: html! {
                    <>
                        <u key="5"></u>
                        <e key="2"></e>
                        <p key="3"></p>
                        <a key="4"></a>
                        <i key="1"></i>
                    </>
                },
                expected: "<u></u><e></e><p></p><a></a><i></i>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Move 2 after 4 - before",
                node: html! {
                    <>
                        <i key="1"></i>
                        <e key="2"></e>
                        <p key="3"></p>
                        <a key="4"></a>
                        <u key="5"></u>
                    </>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Move 2 after 4 - after",
                node: html! {
                    <>
                        <i key="1"></i>
                        <p key="3"></p>
                        <a key="4"></a>
                        <e key="2"></e>
                        <u key="5"></u>
                    </>
                },
                expected: "<i></i><p></p><a></a><e></e><u></u>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap 1,2 <-> 3,4 - before",
                node: html! {
                    <>
                        <i key="1"></i>
                        <e key="2"></e>
                        <p key="3"></p>
                        <a key="4"></a>
                        <u key="5"></u>
                    </>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap 1,2 <-> 3,4 - after",
                node: html! {
                    <>
                        <p key="3"></p>
                        <a key="4"></a>
                        <i key="1"></i>
                        <e key="2"></e>
                        <u key="5"></u>
                    </>
                },
                expected: "<p></p><a></a><i></i><e></e><u></u>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap lists - before",
                node: html! {
                    <>
                        <key="1">
                            <i></i>
                            <e></e>
                        </>
                        <key="2">
                            <a></a>
                            <u></u>
                        </>
                    </>
                },
                expected: "<i></i><e></e><a></a><u></u>",
            },
            TestLayout {
                name: "Swap lists - after",
                node: html! {
                    <>
                        <key="2">
                            <a></a>
                            <u></u>
                        </>
                        <key="1">
                            <i></i>
                            <e></e>
                        </>
                    </>
                },
                expected: "<a></a><u></u><i></i><e></e>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap lists with in-between - before",
                node: html! {
                    <>
                        <key="1">
                            <i></i>
                            <e></e>
                        </>
                        <p key="between"></p>
                        <key="2">
                            <a></a>
                            <u></u>
                        </>
                    </>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap lists with in-between - after",
                node: html! {
                    <>
                        <key="2">
                            <a></a>
                            <u></u>
                        </>
                        <p key="between"></p>
                        <key="1">
                            <i></i>
                            <e></e>
                        </>
                    </>
                },
                expected: "<a></a><u></u><p></p><i></i><e></e>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Insert VComp front - before",
                node: html! {
                    <>
                        <u key=1></u>
                        <a key=2></a>
                    </>
                },
                expected: "<u></u><a></a>",
            },
            TestLayout {
                name: "Insert VComp front - after",
                node: html! {
                    <>
                        <Comp id=0 key="comp"/>
                        <u key=1></u>
                        <a key=2></a>
                    </>
                },
                expected: "<p>0</p><u></u><a></a>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Insert VComp middle - before",
                node: html! {
                    <>
                        <u key=1></u>
                        <a key=2></a>
                    </>
                },
                expected: "<u></u><a></a>",
            },
            TestLayout {
                name: "Insert VComp middle - after",
                node: html! {
                    <>
                        <u key=1></u>
                        <Comp id=0 key="comp"/>
                        <a key=2></a>
                    </>
                },
                expected: "<u></u><p>0</p><a></a>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Insert VComp back - before",
                node: html! {
                    <>
                        <u key=1></u>
                        <a key=2></a>
                    </>
                },
                expected: "<u></u><a></a>",
            },
            TestLayout {
                name: "Insert VComp back - after",
                node: html! {
                    <>
                        <u key=1></u>
                        <a key=2></a>
                        <Comp id=0 key="comp"/>
                    </>
                },
                expected: "<u></u><a></a><p>0</p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Reverse VComp children - before",
                node: html! {
                    <>
                        <Comp id=1 key="comp-1"/>
                        <Comp id=2 key="comp-2"/>
                        <Comp id=3 key="comp-3"/>
                    </>
                },
                expected: "<p>1</p><p>2</p><p>3</p>",
            },
            TestLayout {
                name: "Reverse VComp children - after",
                node: html! {
                    <>
                        <Comp id=3 key="comp-3"/>
                        <Comp id=2 key="comp-2"/>
                        <Comp id=1 key="comp-1"/>
                    </>
                },
                expected: "<p>3</p><p>2</p><p>1</p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Reverse VComp children with children - before",
                node: html! {
                    <>
                        <List key="comp-1"><p>{"11"}</p><p>{"12"}</p></List>
                        <List key="comp-2"><p>{"21"}</p><p>{"22"}</p></List>
                        <List key="comp-3"><p>{"31"}</p><p>{"32"}</p></List>
                    </>
                },
                expected: "<p>11</p><p>12</p><p>21</p><p>22</p><p>31</p><p>32</p>",
            },
            TestLayout {
                name: "Reverse VComp children with children - after",
                node: html! {
                    <>
                        <List key="comp-3"><p>{"31"}</p><p>{"32"}</p></List>
                        <List key="comp-2"><p>{"21"}</p><p>{"22"}</p></List>
                        <List key="comp-1"><p>{"11"}</p><p>{"12"}</p></List>
                    </>
                },
                expected: "<p>31</p><p>32</p><p>21</p><p>22</p><p>11</p><p>12</p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Complex component update - before",
                node: html! {
                    <List>
                        <Comp id=1 key="comp-1"/>
                        <Comp id=2 key="comp-2"/>
                    </List>
                },
                expected: "<p>1</p><p>2</p>",
            },
            TestLayout {
                name: "Complex component update - after",
                node: html! {
                    <List>
                        <List key="comp-1">
                            <Comp id=1 />
                        </List>
                        <List key="comp-2">
                            <p>{"2"}</p>
                        </List>
                    </List>
                },
                expected: "<p>1</p><p>2</p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Reorder VComp children with children - before",
                node: html! {
                    <>
                        <List key="comp-1"><p>{"1"}</p></List>
                        <List key="comp-3"><p>{"3"}</p></List>
                        <List key="comp-5"><p>{"5"}</p></List>
                        <List key="comp-2"><p>{"2"}</p></List>
                        <List key="comp-4"><p>{"4"}</p></List>
                        <List key="comp-6"><p>{"6"}</p></List>
                    </>
                },
                expected: "<p>1</p><p>3</p><p>5</p><p>2</p><p>4</p><p>6</p>",
            },
            TestLayout {
                name: "Reorder VComp children with children - after",
                node: html! {
                    <>
                        <Comp id=6 key="comp-6"/>
                        <Comp id=5 key="comp-5"/>
                        <Comp id=4 key="comp-4"/>
                        <Comp id=3 key="comp-3"/>
                        <Comp id=2 key="comp-2"/>
                        <Comp id=1 key="comp-1"/>
                    </>
                },
                expected: "<p>6</p><p>5</p><p>4</p><p>3</p><p>2</p><p>1</p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Replace and reorder components - before",
                node: html! {
                    <List>
                        <List key="comp-1"><p>{"1"}</p></List>
                        <List key="comp-2"><p>{"2"}</p></List>
                        <List key="comp-3"><p>{"3"}</p></List>
                    </List>
                },
                expected: "<p>1</p><p>2</p><p>3</p>",
            },
            TestLayout {
                name: "Replace and reorder components - after",
                node: html! {
                    <List>
                        <Comp id=3 key="comp-3" />
                        <Comp id=2 key="comp-2" />
                        <Comp id=1 key="comp-1" />
                    </List>
                },
                expected: "<p>3</p><p>2</p><p>1</p>",
            },
        ]);

        diff_layouts(layouts);
    }
}
