//! This module contains fragments bundles, a [BList]
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Deref;

use web_sys::Element;

use super::{BNode, BSubtree, DomSlot, test_log};
use crate::dom_bundle::{Reconcilable, ReconcileTarget};
use crate::html::AnyScope;
use crate::utils::RcExt;
use crate::virtual_dom::{Key, VList, VNode};

/// This struct represents a mounted [VList]
#[derive(Debug)]
pub(super) struct BList {
    /// Child [BNode]s in render order
    children: Vec<BNode>,
    /// All [BNode]s in the BList have keys
    fully_keyed: bool,
    key: Option<Key>,
}

impl VList {
    // Splits a VList for creating / reconciling to a BList.
    fn split_for_blist(self) -> (Option<Key>, bool, Vec<VNode>) {
        let fully_keyed = self.fully_keyed();

        let children = self
            .children
            .map(RcExt::unwrap_or_clone)
            .unwrap_or_default();

        (self.key, fully_keyed, children)
    }
}

impl Deref for BList {
    type Target = Vec<BNode>;

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

/// Helper struct, that keeps the position where the next element is to be placed at
#[derive(Clone)]
struct NodeWriter<'s> {
    root: &'s BSubtree,
    parent_scope: &'s AnyScope,
    parent: &'s Element,
    slot: DomSlot,
}

impl NodeWriter<'_> {
    /// Write a new node that has no ancestor
    fn add(self, node: VNode) -> (Self, BNode) {
        test_log!("adding: {:?}", node);
        test_log!(
            "  parent={:?}, slot={:?}",
            self.parent.outer_html(),
            self.slot
        );
        let (next, bundle) = node.attach(self.root, self.parent_scope, self.parent, self.slot);
        test_log!("  next_slot: {:?}", next);
        (Self { slot: next, ..self }, bundle)
    }

    /// Shift a bundle into place without patching it
    fn shift(&self, bundle: &BNode) {
        bundle.shift(self.parent, self.slot.clone());
    }

    /// Patch a bundle with a new node
    fn patch(self, node: VNode, bundle: &mut BNode) -> Self {
        test_log!("patching: {:?} -> {:?}", bundle, node);
        test_log!(
            "  parent={:?}, slot={:?}",
            self.parent.outer_html(),
            self.slot
        );
        // Advance the next sibling reference (from right to left)
        let next =
            node.reconcile_node(self.root, self.parent_scope, self.parent, self.slot, bundle);
        test_log!("  next_position: {:?}", next);
        Self { slot: next, ..self }
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
                self_list.children.push(b);
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
            children: vec![],
            fully_keyed: true,
            key: None,
        }
    }

    /// Get the key of the underlying fragment
    pub fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }

    /// Diff and patch unkeyed child lists
    ///
    /// Pairs children front-to-front by render-order position so that leading
    /// nodes are always reconciled with themselves when the list grows or
    /// shrinks at the tail.
    fn apply_unkeyed(
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
        lefts: Vec<VNode>,
        rights: &mut Vec<BNode>,
    ) -> DomSlot {
        let mut writer = NodeWriter {
            root,
            parent_scope,
            parent,
            slot,
        };

        // Remove excess old nodes from the end of render order.
        if lefts.len() < rights.len() {
            for r in rights.drain(lefts.len()..) {
                test_log!("removing: {:?}", r);
                r.detach(root, parent, false);
            }
        }

        let paired_count = rights.len(); // min(left_len, old_len)
        let left_len = lefts.len();
        let mut lefts_rev = lefts.into_iter().rev();

        // Add excess new nodes at the tail of render order (rightmost first
        // for the NodeWriter).
        let excess_start = rights.len();
        for l in lefts_rev
            .by_ref()
            .take(left_len.saturating_sub(paired_count))
        {
            let (next_writer, el) = writer.add(l);
            rights.push(el);
            writer = next_writer;
        }
        // Items were pushed right-to-left; flip to render order.
        rights[excess_start..].reverse();

        // Patch paired nodes right-to-left.
        for (l, r) in lefts_rev.zip(rights[..paired_count].iter_mut().rev()) {
            writer = writer.patch(l, r);
        }

        writer.slot
    }

    /// Diff and patch fully keyed child lists.
    ///
    /// Optimized for node addition or removal from either end of the list and small changes in the
    /// middle.
    fn apply_keyed(
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
        left_vdoms: Vec<VNode>,
        bundles: &mut Vec<BNode>,
    ) -> DomSlot {
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

        if cfg!(debug_assertions) {
            let mut keys = HashSet::with_capacity(left_vdoms.len());
            for (idx, n) in left_vdoms.iter().enumerate() {
                let key = key!(n);
                debug_assert!(
                    keys.insert(key!(n)),
                    "duplicate key detected: {key} at index {idx}. Keys in keyed lists must be \
                     unique!",
                );
            }
        }

        // If keys match from the front for the entire shorter list, items were
        // only added or removed at the back. apply_unkeyed handles this since
        // it pairs front-to-front.
        let matching_len_start = matching_len(
            left_vdoms.iter().map(|v| key!(v)),
            bundles.iter().map(|v| key!(v)),
        );
        if matching_len_start == std::cmp::min(left_vdoms.len(), bundles.len()) {
            return Self::apply_unkeyed(root, parent_scope, parent, slot, left_vdoms, bundles);
        }

        // Find first key mismatch from the back of render order
        let matching_len_end = matching_len(
            left_vdoms.iter().rev().map(|v| key!(v)),
            bundles.iter().rev().map(|v| key!(v)),
        );

        // We partially drain the new vnodes in several steps.
        let mut lefts = left_vdoms;
        let mut writer = NodeWriter {
            root,
            parent_scope,
            parent,
            slot,
        };
        // Step 1. Diff matching children at the end of render order
        let lefts_to = lefts.len() - matching_len_end;
        let bundles_from = bundles.len() - matching_len_end;
        for (l, r) in lefts
            .drain(lefts_to..)
            .rev()
            .zip(bundles[bundles_from..].iter_mut().rev())
        {
            writer = writer.patch(l, r);
        }

        // Step 2. Diff matching children in the middle, that is between the first and last key
        // mismatch. Find first key mismatch from the front.
        let bundle_middle = if bundles_from >= matching_len_start {
            matching_len_start..bundles_from
        } else {
            // If this range in the other branch is "inverted", this implies that the incoming nodes
            // in lefts contain a duplicate key!
            // Pictogram:
            //                              v lefts_to
            // lefts:   | SSSSSSSS | ------ | EEEEEEEE |
            //                     ↕ matching_len_start
            // bundles: | SSS | ?? | EEE |
            //                ^ bundles_from
            // Both a key from the (S)tarting portion and (E)nding portion of lefts has matched a
            // key in the ? portion of bundles. Since the former can't overlap, a key
            // must be duplicate. Duplicates might lead to us forgetting about some
            // bundles entirely. It is NOT straight forward to adjust the below code to
            // consistently check and handle this. The duplicate keys might
            // be in the start or end portion.
            // With debug_assertions we can never reach this. For production code, hope for the best
            // by pretending. We still need to adjust some things so splicing doesn't
            // panic:
            bundles_from..bundles_from
        };
        let matching_len_start = bundle_middle.start;
        // Step 2.1. Splice out the existing middle part and build a lookup by key

        // BNode contains js objects that look suspicious to clippy but are harmless
        #[expect(clippy::mutable_key_type)]
        let mut spare_bundles: HashSet<KeyedEntry> = HashSet::with_capacity(bundle_middle.len());
        let mut spliced_middle = bundles.splice(bundle_middle, std::iter::empty());
        for (idx, r) in (&mut spliced_middle).enumerate() {
            #[cold]
            fn duplicate_in_bundle(root: &BSubtree, parent: &Element, r: BNode) {
                test_log!("removing: {:?}", r);
                r.detach(root, parent, false);
            }
            if let Some(KeyedEntry(_, dup)) = spare_bundles.replace(KeyedEntry(idx, r)) {
                duplicate_in_bundle(root, parent, dup);
            }
        }
        let middle_count = spare_bundles.len();

        // Step 2.2. Put the middle part back together in the new key order
        let mut replacements: Vec<BNode> = Vec::with_capacity((matching_len_start..lefts_to).len());
        // The goal is to shift as few nodes as possible.

        // We handle runs of in-order nodes. When we encounter one out-of-order, we decide whether:
        // - to shift all nodes in the current run to the position after the node before of the run,
        //   or to
        // - "commit" to the current run, shift all nodes after the end of the run that we might
        //   encounter in the future, and then start a new run.
        //
        // Indices are in render order (idx 0 = leftmost). Processing goes right-to-left, so we
        // expect decreasing indices for nodes already in the right relative order.

        // The node with `idx + 1 == barrier_idx` is already correctly placed if there is no run
        // active.
        // Nodes with `idx >= barrier_idx` are shifted unconditionally.
        // Also serves as the next expected node index if the order has not changed, and of the
        // position of ``
        let mut barrier_idx = middle_count;
        struct RunInformation<'a> {
            start_writer: NodeWriter<'a>,
            // Index in `replacements` where this run started
            start_idx: usize,
            // Index of the left-most (in render-order) bundle that is part of the run.
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
                    if idx > run.end_idx {
                        // Have to decide whether to shift or commit the current run. A few
                        // calculations: A perfect estimate of the amount of
                        // nodes we have to shift if we move this run:
                        let run_length = replacements.len() - run.start_idx;
                        // A very crude estimate of the amount of nodes we will have to shift if we
                        // commit the run. Note: nodes of the current run are counted here too.
                        let estimated_skipped_nodes = idx.min(barrier_idx) - run.end_idx;
                        // double run_length to counteract that the run is part of the
                        // estimated_skipped_nodes
                        if 2 * run_length > estimated_skipped_nodes {
                            // less work to commit to this run
                            barrier_idx = run.end_idx;
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
            let bundle = match ancestor {
                Some(KeyedEntry(idx, mut r_bundle)) => {
                    match current_run.as_mut() {
                        // hot path
                        // We know that idx <= run.end_idx, so this node doesn't need to shift
                        Some(run) => run.end_idx = idx,
                        None => match () {
                            // peep hole optimization, don't start a run as the element is
                            // already where it should be
                            _ if idx + 1 == barrier_idx => barrier_idx = idx,
                            // shift the node unconditionally, don't start a run
                            _ if idx >= barrier_idx => writer.shift(&r_bundle),
                            // start a run
                            _ => {
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
                }
                _ => {
                    // Even if there is an active run, we don't have to modify it
                    let (next_writer, bundle) = writer.add(l);
                    writer = next_writer;
                    bundle
                }
            };
            replacements.push(bundle);
        }
        // drop the splice iterator and immediately replace the range with the reordered elements
        drop(spliced_middle);
        // replacements was built right-to-left; reverse to render order
        replacements.reverse();
        bundles.splice(matching_len_start..matching_len_start, replacements);

        // Step 2.3. Remove any extra rights
        for KeyedEntry(_, r) in spare_bundles.drain() {
            test_log!("removing: {:?}", r);
            r.detach(root, parent, false);
        }

        // Step 3. Diff matching children at the start
        for (l, r) in lefts
            .drain(..) // matching_len_start.. has been drained already
            .rev()
            .zip(bundles[..matching_len_start].iter_mut().rev())
        {
            writer = writer.patch(l, r);
        }

        writer.slot
    }
}

impl ReconcileTarget for BList {
    fn detach(self, root: &BSubtree, parent: &Element, parent_to_detach: bool) {
        for child in self.children.into_iter() {
            child.detach(root, parent, parent_to_detach);
        }
    }

    fn shift(&self, next_parent: &Element, mut slot: DomSlot) -> DomSlot {
        for node in self.children.iter().rev() {
            slot = node.shift(next_parent, slot);
        }

        slot
    }
}

impl Reconcilable for VList {
    type Bundle = BList;

    fn attach(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
    ) -> (DomSlot, Self::Bundle) {
        let mut self_ = BList::new();
        let node_ref = self.reconcile(root, parent_scope, parent, slot, &mut self_);
        (node_ref, self_)
    }

    fn reconcile_node(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
        bundle: &mut BNode,
    ) -> DomSlot {
        // 'Forcefully' pretend the existing node is a list. Creates a
        // singleton list if it isn't already.
        let blist = bundle.make_list();
        self.reconcile(root, parent_scope, parent, slot, blist)
    }

    fn reconcile(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
        blist: &mut BList,
    ) -> DomSlot {
        // Here, we will try to diff the previous list elements with the new
        // ones we want to insert. For that, we will use two lists:
        //  - lefts: new elements to render in the DOM
        //  - rights: previously rendered elements.
        //
        // The left items are known since we want to insert them
        // (self.children). For the right ones, we will look at the bundle,
        // i.e. the current DOM list element that we want to replace with self.
        let (key, fully_keyed, lefts) = self.split_for_blist();

        let rights = &mut blist.children;
        test_log!("lefts: {:?}", lefts);
        test_log!("rights: {:?}", rights);

        if let Some(additional) = lefts.len().checked_sub(rights.len()) {
            rights.reserve_exact(additional);
        }
        let first = if fully_keyed && blist.fully_keyed {
            BList::apply_keyed(root, parent_scope, parent, slot, lefts, rights)
        } else {
            BList::apply_unkeyed(root, parent_scope, parent, slot, lefts, rights)
        };
        blist.fully_keyed = fully_keyed;
        blist.key = key;
        test_log!("result: {:?}", rights);
        first
    }
}

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;
    use crate::dom_bundle::{DynamicDomSlot, Fragment, Hydratable};

    impl Hydratable for VList {
        fn hydrate(
            self,
            root: &BSubtree,
            parent_scope: &AnyScope,
            parent: &Element,
            fragment: &mut Fragment,
            prev_next_sibling: &mut Option<DynamicDomSlot>,
        ) -> Self::Bundle {
            let (key, fully_keyed, vchildren) = self.split_for_blist();

            let mut children = Vec::with_capacity(vchildren.len());

            for child in vchildren.into_iter() {
                let child = child.hydrate(root, parent_scope, parent, fragment, prev_next_sibling);

                children.push(child);
            }

            BList {
                children,
                fully_keyed,
                key,
            }
        }
    }
}

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
#[cfg(test)]
mod layout_tests {
    extern crate self as yew;

    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use crate::html;
    use crate::tests::layout_tests::{TestLayout, diff_layouts};

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn diff() {
        let layout1 = TestLayout {
            name: "1",
            node: html! {
                {"a"}
                {"b"}
                <>
                    {"c"}
                    {"d"}
                </>
                {"e"}
            },
            expected: "abcde",
        };

        let layout2 = TestLayout {
            name: "2",
            node: html! {
                {"a"}
                {"b"}
                <></>
                {"e"}
                {"f"}
            },
            expected: "abef",
        };

        let layout3 = TestLayout {
            name: "3",
            node: html! {
                {"a"}
                <></>
                {"b"}
                {"e"}
            },
            expected: "abe",
        };

        let layout4 = TestLayout {
            name: "4",
            node: html! {
                {"a"}
                <>
                    {"c"}
                    {"d"}
                </>
                {"b"}
                {"e"}
            },
            expected: "acdbe",
        };

        diff_layouts(vec![layout1, layout2, layout3, layout4]);
    }
}

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
#[cfg(test)]
mod layout_tests_keys {
    extern crate self as yew;

    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    use web_sys::Node;

    use crate::tests::layout_tests::{TestLayout, diff_layouts};
    use crate::virtual_dom::VNode;
    use crate::{Children, Component, Context, Html, Properties, html};

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
            html! { { for ctx.props().children.iter() } }
        }
    }

    #[test]
    fn diff() {
        let mut layouts = vec![];

        let vref_node: Node = gloo::utils::document().create_element("i").unwrap().into();
        layouts.push(TestLayout {
            name: "All VNode types as children",
            node: html! {
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
            },
            expected: "a<span></span>cd<p>0</p>foobar<i></i>",
        });

        layouts.extend(vec![
            TestLayout {
                name: "Inserting into VList first child - before",
                node: html! {
                    <key="VList">
                        <i key="i"></i>
                    </>
                    <p key="p"></p>
                },
                expected: "<i></i><p></p>",
            },
            TestLayout {
                name: "Inserting into VList first child - after",
                node: html! {
                    <key="VList">
                        <i key="i"></i>
                        <e key="e"></e>
                    </>
                    <p key="p"></p>
                },
                expected: "<i></i><e></e><p></p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "No matches - before",
                node: html! {
                    <i key="i"></i>
                    <e key="e"></e>
                },
                expected: "<i></i><e></e>",
            },
            TestLayout {
                name: "No matches - after",
                node: html! {
                    <a key="a"></a>
                    <p key="p"></p>
                },
                expected: "<a></a><p></p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Append - before",
                node: html! {
                    <i key="i"></i>
                    <e key="e"></e>
                },
                expected: "<i></i><e></e>",
            },
            TestLayout {
                name: "Append - after",
                node: html! {
                    <i key="i"></i>
                    <e key="e"></e>
                    <p key="p"></p>
                },
                expected: "<i></i><e></e><p></p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Prepend - before",
                node: html! {
                    <i key="i"></i>
                    <e key="e"></e>
                },
                expected: "<i></i><e></e>",
            },
            TestLayout {
                name: "Prepend - after",
                node: html! {
                    <p key="p"></p>
                    <i key="i"></i>
                    <e key="e"></e>
                },
                expected: "<p></p><i></i><e></e>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Delete first - before",
                node: html! {
                    <i key="i"></i>
                    <e key="e"></e>
                    <p key="p"></p>
                },
                expected: "<i></i><e></e><p></p>",
            },
            TestLayout {
                name: "Delete first - after",
                node: html! {
                    <e key="e"></e>
                    <p key="p"></p>
                },
                expected: "<e></e><p></p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Delete last - before",
                node: html! {
                    <i key="i"></i>
                    <e key="e"></e>
                    <p key="p"></p>
                },
                expected: "<i></i><e></e><p></p>",
            },
            TestLayout {
                name: "Delete last - after",
                node: html! {
                    <i key="i"></i>
                    <e key="e"></e>
                },
                expected: "<i></i><e></e>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Delete last and change node type - before",
                node: html! {
                    <i key="i"></i>
                    <e key="e"></e>
                    <p key="p"></p>
                },
                expected: "<i></i><e></e><p></p>",
            },
            TestLayout {
                name: "Delete last - after",
                node: html! {
                    <List key="i"><i/></List>
                    <List key="e"><e/></List>
                    <List key="a"><a/></List>
                },
                expected: "<i></i><e></e><a></a>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Delete middle - before",
                node: html! {
                    <i key="i"></i>
                    <e key="e"></e>
                    <p key="p"></p>
                    <a key="a"></a>
                },
                expected: "<i></i><e></e><p></p><a></a>",
            },
            TestLayout {
                name: "Delete middle - after",
                node: html! {
                    <i key="i"></i>
                    <e key="e2"></e>
                    <p key="p2"></p>
                    <a key="a"></a>
                },
                expected: "<i></i><e></e><p></p><a></a>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Delete middle and change node type - before",
                node: html! {
                    <i key="i"></i>
                    <e key="e"></e>
                    <p key="p"></p>
                    <a key="a"></a>
                },
                expected: "<i></i><e></e><p></p><a></a>",
            },
            TestLayout {
                name: "Delete middle and change node type- after",
                node: html! {
                    <List key="i2"><i/></List>
                    <e key="e"></e>
                    <List key="p"><p/></List>
                    <List key="a2"><a/></List>
                },
                expected: "<i></i><e></e><p></p><a></a>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Reverse - before",
                node: html! {
                    <i key="i"></i>
                    <e key="e"></e>
                    <p key="p"></p>
                    <u key="u"></u>
                },
                expected: "<i></i><e></e><p></p><u></u>",
            },
            TestLayout {
                name: "Reverse - after",
                node: html! {
                    <u key="u"></u>
                    <p key="p"></p>
                    <e key="e"></e>
                    <i key="i"></i>
                },
                expected: "<u></u><p></p><e></e><i></i>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Reverse and change node type - before",
                node: html! {
                    <i key="i"></i>
                    <key="i1"></>
                    <key="i2"></>
                    <key="i3"></>
                    <e key="e"></e>
                    <key="yo">
                        <p key="p"></p>
                    </>
                    <u key="u"></u>
                },
                expected: "<i></i><e></e><p></p><u></u>",
            },
            TestLayout {
                name: "Reverse and change node type - after",
                node: html! {
                    <List key="u"><u/></List>
                    <List key="p"><p/></List>
                    <List key="e"><e/></List>
                    <List key="i"><i/></List>
                },
                expected: "<u></u><p></p><e></e><i></i>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap 1&2 - before",
                node: html! {
                    <i key="1"></i>
                    <e key="2"></e>
                    <p key="3"></p>
                    <a key="4"></a>
                    <u key="5"></u>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap 1&2 - after",
                node: html! {
                    <e key="2"></e>
                    <i key="1"></i>
                    <p key="3"></p>
                    <a key="4"></a>
                    <u key="5"></u>
                },
                expected: "<e></e><i></i><p></p><a></a><u></u>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap 1&2 and change node type - before",
                node: html! {
                    <i key="1"></i>
                    <e key="2"></e>
                    <p key="3"></p>
                    <a key="4"></a>
                    <u key="5"></u>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap 1&2 and change node type - after",
                node: html! {
                    <List key="2"><e/></List>
                    <List key="1"><i/></List>
                    <List key="3"><p/></List>
                    <List key="4"><a/></List>
                    <List key="5"><u/></List>
                },
                expected: "<e></e><i></i><p></p><a></a><u></u>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "test - before",
                node: html! {
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
                },
                expected: "<e></e><p></p><a></a><u></u><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap 4&5 - after",
                node: html! {
                    <e key="1"></e>
                    <key="2">
                        <p key="p"></p>
                        <i key="i"></i>
                    </>
                },
                expected: "<e></e><p></p><i></i>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap 4&5 - before",
                node: html! {
                    <i key="1"></i>
                    <e key="2"></e>
                    <p key="3"></p>
                    <a key="4"></a>
                    <u key="5"></u>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap 4&5 - after",
                node: html! {
                    <i key="1"></i>
                    <e key="2"></e>
                    <p key="3"></p>
                    <u key="5"></u>
                    <a key="4"></a>
                },
                expected: "<i></i><e></e><p></p><u></u><a></a>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap 1&5 - before",
                node: html! {
                    <i key="1"></i>
                    <e key="2"></e>
                    <p key="3"></p>
                    <a key="4"></a>
                    <u key="5"></u>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap 1&5 - after",
                node: html! {
                    <u key="5"></u>
                    <e key="2"></e>
                    <p key="3"></p>
                    <a key="4"></a>
                    <i key="1"></i>
                },
                expected: "<u></u><e></e><p></p><a></a><i></i>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Move 2 after 4 - before",
                node: html! {
                    <i key="1"></i>
                    <e key="2"></e>
                    <p key="3"></p>
                    <a key="4"></a>
                    <u key="5"></u>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Move 2 after 4 - after",
                node: html! {
                    <i key="1"></i>
                    <p key="3"></p>
                    <a key="4"></a>
                    <e key="2"></e>
                    <u key="5"></u>
                },
                expected: "<i></i><p></p><a></a><e></e><u></u>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap 1,2 <-> 3,4 - before",
                node: html! {
                    <i key="1"></i>
                    <e key="2"></e>
                    <p key="3"></p>
                    <a key="4"></a>
                    <u key="5"></u>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap 1,2 <-> 3,4 - after",
                node: html! {
                    <p key="3"></p>
                    <a key="4"></a>
                    <i key="1"></i>
                    <e key="2"></e>
                    <u key="5"></u>
                },
                expected: "<p></p><a></a><i></i><e></e><u></u>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap lists - before",
                node: html! {
                    <key="1">
                        <i></i>
                        <e></e>
                    </>
                    <key="2">
                        <a></a>
                        <u></u>
                    </>
                },
                expected: "<i></i><e></e><a></a><u></u>",
            },
            TestLayout {
                name: "Swap lists - after",
                node: html! {
                    <key="2">
                        <a></a>
                        <u></u>
                    </>
                    <key="1">
                        <i></i>
                        <e></e>
                    </>
                },
                expected: "<a></a><u></u><i></i><e></e>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Swap lists with in-between - before",
                node: html! {
                    <key="1">
                        <i></i>
                        <e></e>
                    </>
                    <p key="between"></p>
                    <key="2">
                        <a></a>
                        <u></u>
                    </>
                },
                expected: "<i></i><e></e><p></p><a></a><u></u>",
            },
            TestLayout {
                name: "Swap lists with in-between - after",
                node: html! {
                    <key="2">
                        <a></a>
                        <u></u>
                    </>
                    <p key="between"></p>
                    <key="1">
                        <i></i>
                        <e></e>
                    </>
                },
                expected: "<a></a><u></u><p></p><i></i><e></e>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Insert VComp front - before",
                node: html! {
                    <u key=1></u>
                    <a key=2></a>
                },
                expected: "<u></u><a></a>",
            },
            TestLayout {
                name: "Insert VComp front - after",
                node: html! {
                    <Comp id=0 key="comp"/>
                    <u key=1></u>
                    <a key=2></a>
                },
                expected: "<p>0</p><u></u><a></a>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Insert VComp middle - before",
                node: html! {
                    <u key=1></u>
                    <a key=2></a>
                },
                expected: "<u></u><a></a>",
            },
            TestLayout {
                name: "Insert VComp middle - after",
                node: html! {
                    <u key=1></u>
                    <Comp id=0 key="comp"/>
                    <a key=2></a>
                },
                expected: "<u></u><p>0</p><a></a>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Insert VComp back - before",
                node: html! {
                    <u key=1></u>
                    <a key=2></a>
                },
                expected: "<u></u><a></a>",
            },
            TestLayout {
                name: "Insert VComp back - after",
                node: html! {
                    <u key=1></u>
                    <a key=2></a>
                    <Comp id=0 key="comp"/>
                },
                expected: "<u></u><a></a><p>0</p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Reverse VComp children - before",
                node: html! {
                    <Comp id=1 key="comp-1"/>
                    <Comp id=2 key="comp-2"/>
                    <Comp id=3 key="comp-3"/>
                },
                expected: "<p>1</p><p>2</p><p>3</p>",
            },
            TestLayout {
                name: "Reverse VComp children - after",
                node: html! {
                    <Comp id=3 key="comp-3"/>
                    <Comp id=2 key="comp-2"/>
                    <Comp id=1 key="comp-1"/>
                },
                expected: "<p>3</p><p>2</p><p>1</p>",
            },
        ]);

        layouts.extend(vec![
            TestLayout {
                name: "Reverse VComp children with children - before",
                node: html! {
                    <List key="comp-1"><p>{"11"}</p><p>{"12"}</p></List>
                    <List key="comp-2"><p>{"21"}</p><p>{"22"}</p></List>
                    <List key="comp-3"><p>{"31"}</p><p>{"32"}</p></List>
                },
                expected: "<p>11</p><p>12</p><p>21</p><p>22</p><p>31</p><p>32</p>",
            },
            TestLayout {
                name: "Reverse VComp children with children - after",
                node: html! {
                    <List key="comp-3"><p>{"31"}</p><p>{"32"}</p></List>
                    <List key="comp-2"><p>{"21"}</p><p>{"22"}</p></List>
                    <List key="comp-1"><p>{"11"}</p><p>{"12"}</p></List>
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
                    <List key="comp-1"><p>{"1"}</p></List>
                    <List key="comp-3"><p>{"3"}</p></List>
                    <List key="comp-5"><p>{"5"}</p></List>
                    <List key="comp-2"><p>{"2"}</p></List>
                    <List key="comp-4"><p>{"4"}</p></List>
                    <List key="comp-6"><p>{"6"}</p></List>
                },
                expected: "<p>1</p><p>3</p><p>5</p><p>2</p><p>4</p><p>6</p>",
            },
            TestLayout {
                name: "Reorder VComp children with children - after",
                node: html! {
                    <Comp id=6 key="comp-6"/>
                    <Comp id=5 key="comp-5"/>
                    <Comp id=4 key="comp-4"/>
                    <Comp id=3 key="comp-3"/>
                    <Comp id=2 key="comp-2"/>
                    <Comp id=1 key="comp-1"/>
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

    #[test]
    //#[should_panic(expected = "duplicate key detected: vtag at index 1")]
    // can't inspect panic message in wasm :/
    #[should_panic]
    fn duplicate_keys() {
        let mut layouts = vec![];

        layouts.push(TestLayout {
            name: "A list with duplicate keys",
            node: html! {
                <i key="vtag" />
                <i key="vtag" />
            },
            expected: "<i></i><i></i>",
        });

        diff_layouts(layouts);
    }
}

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
#[cfg(test)]
mod node_identity_tests {
    extern crate self as yew;

    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use crate::dom_bundle::{BSubtree, Bundle, DomSlot};
    use crate::html::AnyScope;
    use crate::{Html, html, scheduler};

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn for_iterable_preserves_sibling_identity() {
        let document = gloo::utils::document();
        let scope: AnyScope = AnyScope::test();
        let parent = document.create_element("div").unwrap();
        let root = BSubtree::create_root(&parent);
        let end = document.create_text_node("END");
        parent.append_child(&end).unwrap();
        let slot = DomSlot::at(end.into());

        let items: Vec<Html> = vec![];
        let vnode = html! { <div id="stable"/>{for items} };
        let mut bundle = Bundle::new();
        bundle.reconcile(&root, &scope, &parent, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(parent.inner_html(), r#"<div id="stable"></div>END"#);

        let div_node = parent.first_child().expect("should have a child");
        assert_eq!(div_node.node_name(), "DIV");

        let items: Vec<Html> = vec![html! { <span/> }];
        let vnode = html! { <div id="stable"/>{for items} };
        bundle.reconcile(&root, &scope, &parent, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(
            parent.inner_html(),
            r#"<div id="stable"></div><span></span>END"#
        );

        let first = parent.first_child().expect("should have children");
        assert!(
            first.is_same_node(Some(&div_node)),
            "the <div> DOM node should be reused, not recreated (got <{}>, the old <div> was \
             destroyed by {{for}} flattening)",
            first.node_name(),
        );
    }

    #[test]
    fn vec_expression_preserves_sibling_identity() {
        let document = gloo::utils::document();
        let scope: AnyScope = AnyScope::test();
        let parent = document.create_element("div").unwrap();
        let root = BSubtree::create_root(&parent);
        let end = document.create_text_node("END");
        parent.append_child(&end).unwrap();
        let slot = DomSlot::at(end.into());

        let items: Vec<Html> = vec![];
        let vnode = html! { <div id="stable"/>{items} };
        let mut bundle = Bundle::new();
        bundle.reconcile(&root, &scope, &parent, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(parent.inner_html(), r#"<div id="stable"></div>END"#);

        let div_node = parent.first_child().expect("should have a child");
        assert_eq!(div_node.node_name(), "DIV");

        let items: Vec<Html> = vec![html! { <span/> }];
        let vnode = html! { <div id="stable"/>{items} };
        bundle.reconcile(&root, &scope, &parent, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(
            parent.inner_html(),
            r#"<div id="stable"></div><span></span>END"#
        );

        let first = parent.first_child().expect("should have children");
        assert!(
            first.is_same_node(Some(&div_node)),
            "the <div> DOM node should be reused, not recreated (got <{}>, the old <div> was \
             destroyed by Vec flattening)",
            first.node_name(),
        );
    }

    #[test]
    fn option_expression_preserves_sibling_identity() {
        let document = gloo::utils::document();
        let scope: AnyScope = AnyScope::test();
        let parent = document.create_element("div").unwrap();
        let root = BSubtree::create_root(&parent);
        let end = document.create_text_node("END");
        parent.append_child(&end).unwrap();
        let slot = DomSlot::at(end.into());

        let maybe: Option<Html> = None;
        let vnode = html! { <div id="stable"/>{maybe} };
        let mut bundle = Bundle::new();
        bundle.reconcile(&root, &scope, &parent, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(parent.inner_html(), r#"<div id="stable"></div>END"#);

        let div_node = parent.first_child().expect("should have a child");
        assert_eq!(div_node.node_name(), "DIV");

        let maybe: Option<Html> = Some(html! { <span/> });
        let vnode = html! { <div id="stable"/>{maybe} };
        bundle.reconcile(&root, &scope, &parent, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(
            parent.inner_html(),
            r#"<div id="stable"></div><span></span>END"#
        );

        let first = parent.first_child().expect("should have children");
        assert!(
            first.is_same_node(Some(&div_node)),
            "the <div> DOM node should be reused, not recreated (got <{}>, the old <div> was \
             destroyed by Option flattening)",
            first.node_name(),
        );
    }

    #[test]
    fn unkeyed_grow_preserves_leading_nodes() {
        let document = gloo::utils::document();
        let scope: AnyScope = AnyScope::test();
        let parent = document.create_element("div").unwrap();
        let root = BSubtree::create_root(&parent);
        let end = document.create_text_node("END");
        parent.append_child(&end).unwrap();
        let slot = DomSlot::at(end.into());

        let vnode = html! { <div/><span/> };
        let mut bundle = Bundle::new();
        bundle.reconcile(&root, &scope, &parent, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(parent.inner_html(), "<div></div><span></span>END");

        let div_node = parent.first_child().unwrap();
        assert_eq!(div_node.node_name(), "DIV");
        let span_node = div_node.next_sibling().unwrap();
        assert_eq!(span_node.node_name(), "SPAN");

        let vnode = html! { <div/><span/><p/> };
        bundle.reconcile(&root, &scope, &parent, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(parent.inner_html(), "<div></div><span></span><p></p>END");

        let first = parent.first_child().unwrap();
        assert!(
            first.is_same_node(Some(&div_node)),
            "growing a list should not recreate leading <div>",
        );
        let second = first.next_sibling().unwrap();
        assert!(
            second.is_same_node(Some(&span_node)),
            "growing a list should not recreate leading <span>",
        );
    }

    #[test]
    fn unkeyed_shrink_preserves_leading_nodes() {
        let document = gloo::utils::document();
        let scope: AnyScope = AnyScope::test();
        let parent = document.create_element("div").unwrap();
        let root = BSubtree::create_root(&parent);
        let end = document.create_text_node("END");
        parent.append_child(&end).unwrap();
        let slot = DomSlot::at(end.into());

        let vnode = html! { <div/><span/><p/> };
        let mut bundle = Bundle::new();
        bundle.reconcile(&root, &scope, &parent, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(parent.inner_html(), "<div></div><span></span><p></p>END");

        let div_node = parent.first_child().unwrap();
        let span_node = div_node.next_sibling().unwrap();

        let vnode = html! { <div/><span/> };
        bundle.reconcile(&root, &scope, &parent, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(parent.inner_html(), "<div></div><span></span>END");

        let first = parent.first_child().unwrap();
        assert!(
            first.is_same_node(Some(&div_node)),
            "shrinking a list should not recreate leading <div>",
        );
        let second = first.next_sibling().unwrap();
        assert!(
            second.is_same_node(Some(&span_node)),
            "shrinking a list should not recreate leading <span>",
        );
    }

    #[test]
    fn keyed_prepend_preserves_trailing_nodes() {
        let document = gloo::utils::document();
        let scope: AnyScope = AnyScope::test();
        let parent = document.create_element("div").unwrap();
        let root = BSubtree::create_root(&parent);
        let end = document.create_text_node("END");
        parent.append_child(&end).unwrap();
        let slot = DomSlot::at(end.into());

        let vnode = html! { <i key="i"/><e key="e"/> };
        let mut bundle = Bundle::new();
        bundle.reconcile(&root, &scope, &parent, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(parent.inner_html(), "<i></i><e></e>END");

        let i_node = parent.first_child().unwrap();
        let e_node = i_node.next_sibling().unwrap();

        let vnode = html! { <p key="p"/><i key="i"/><e key="e"/> };
        bundle.reconcile(&root, &scope, &parent, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(parent.inner_html(), "<p></p><i></i><e></e>END");

        let children = parent.child_nodes();
        let second = children.get(1).unwrap();
        let third = children.get(2).unwrap();
        assert!(
            second.is_same_node(Some(&i_node)),
            "prepending to a keyed list should preserve trailing <i>",
        );
        assert!(
            third.is_same_node(Some(&e_node)),
            "prepending to a keyed list should preserve trailing <e>",
        );
    }
}
