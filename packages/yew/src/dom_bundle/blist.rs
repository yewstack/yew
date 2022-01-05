//! This module contains fragments bundles.
use super::test_log;
use super::BNode;
use crate::dom_bundle::{DomBundle, VDiff};
use crate::html::{AnyScope, NodeRef};
use crate::virtual_dom::{Key, VList, VNode, VText};
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Deref;
use web_sys::Element;

/// This struct represents a fragment of the Virtual DOM tree.
#[derive(Debug)]
pub struct BList {
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

struct ElementWriter<'s> {
    parent_scope: &'s AnyScope,
    parent: &'s Element,
    next_sibling: NodeRef,
}

impl<'s> ElementWriter<'s> {
    fn add(self, node: VNode) -> (Self, BNode) {
        test_log!("adding: {:?}", node);
        test_log!("  parent={:?}, next_sibling={:?}", self.parent.outer_html(), self.next_sibling);
        let (next, bundle) = node.attach(self.parent_scope, self.parent, self.next_sibling);
        test_log!("  next_position: {:?}", next);
        (
            Self {
                next_sibling: next,
                ..self
            },
            bundle,
        )
    }

    fn patch(self, node: VNode, ancestor: &mut BNode) -> Self {
        test_log!("patching: {:?} -> {:?}", ancestor, node);
        test_log!("  parent={:?}, next_sibling={:?}", self.parent.outer_html(), self.next_sibling);
        // Advance the next sibling reference (from right to left)
        ancestor.move_before(self.parent, &self.next_sibling.get());
        let next = node.apply(self.parent_scope, self.parent, self.next_sibling, ancestor);
        test_log!("  next_position: {:?}", next);
        Self {
            next_sibling: next,
            ..self
        }
    }
}

struct NodeEntry(BNode);
impl Borrow<Key> for NodeEntry {
    fn borrow(&self) -> &Key {
        self.0.key().expect("unkeyed child in fully keyed list")
    }
}
impl Hash for NodeEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        <Self as Borrow<Key>>::borrow(self).hash(state)
    }
}
impl PartialEq for NodeEntry {
    fn eq(&self, other: &Self) -> bool {
        <Self as Borrow<Key>>::borrow(self) == <Self as Borrow<Key>>::borrow(other)
    }
}
impl Eq for NodeEntry {}

impl BNode {
    fn make_list(&mut self) -> &mut BList {
        match self {
            Self::BList(blist) => blist,
            self_ => {
                let b = std::mem::replace(self_, BNode::BList(BList::new()));
                let self_list = match self_ {
                    BNode::BList(blist) => blist,
                    _ => unreachable!("just been set to the variant"),
                };
                self_list.fully_keyed = b.has_key();
                self_list.key = b.key().cloned();
                self_list.rev_children.push(b);
                self_list
            }
        }
    }
}

impl BList {
    pub(crate) const fn new() -> BList {
        BList {
            rev_children: vec![],
            fully_keyed: true,
            key: None,
        }
    }

    pub(crate) fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }

    /// Diff and patch unkeyed child lists
    fn apply_unkeyed(
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        lefts: Vec<VNode>,
        rights: &mut Vec<BNode>,
    ) -> NodeRef {
        let mut writer = ElementWriter {
            parent_scope,
            parent,
            next_sibling,
        };

        // Remove extra nodes
        if lefts.len() < rights.len() {
            for r in rights.drain(lefts.len()..) {
                test_log!("removing: {:?}", r);
                r.detach(parent);
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
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        lefts: Vec<VNode>,
        rights: &mut Vec<BNode>,
    ) -> NodeRef {
        /// Find the first differing key in 2 iterators
        fn matching_len<'a, 'b>(
            a: impl Iterator<Item = &'a Key>,
            b: impl Iterator<Item = &'b Key>,
        ) -> usize {
            a.zip(b).take_while(|(a, b)| a == b).count()
        }

        // Find first key mismatch from the back
        let matching_len_end = matching_len(
            lefts
                .iter()
                .map(|v| v.key().expect("unkeyed child in fully keyed list"))
                .rev(),
            rights
                .iter()
                .map(|v| v.key().expect("unkeyed child in fully keyed list")),
        );

        if matching_len_end == std::cmp::min(lefts.len(), rights.len()) {
            // No key changes
            return Self::apply_unkeyed(parent_scope, parent, next_sibling, lefts, rights);
        }
        // We partially deconstruct the new vector in several steps.
        let mut lefts = lefts;
        let mut writer = ElementWriter {
            parent_scope,
            parent,
            next_sibling,
        };
        // Diff matching children at the end
        let lefts_to = lefts.len() - matching_len_end;
        for (l, r) in lefts
            .drain(lefts_to..)
            .rev()
            .zip(rights[..matching_len_end].iter_mut())
        {
            writer = writer.patch(l, r);
        }
        // Find first key mismatch from the front
        let matching_len_start = matching_len(
            lefts
                .iter()
                .map(|v| v.key().expect("unkeyed child in fully keyed list")),
            rights
                .iter()
                .map(|v| v.key().expect("unkeyed child in fully keyed list"))
                .rev(),
        );

        // Diff mismatched children in the middle
        let rights_to = rights.len() - matching_len_start;
        let mut spliced_middle = rights.splice(matching_len_end..rights_to, std::iter::empty());
        let mut rights_diff: HashSet<NodeEntry> =
            HashSet::with_capacity((matching_len_end..rights_to).len());
        for r in &mut spliced_middle {
            rights_diff.insert(NodeEntry(r));
        }
        let mut replacements: Vec<BNode> = Vec::with_capacity((matching_len_start..lefts_to).len());
        for l in lefts
            .drain(matching_len_start..) // lefts_to.. has been drained
            .rev()
        {
            let l_key = l.key().expect("unkeyed child in fully keyed list");
            let bundle = match rights_diff.take(l_key) {
                Some(NodeEntry(mut r_bundle)) => {
                    writer = writer.patch(l, &mut r_bundle);
                    r_bundle
                }
                None => {
                    let (next_writer, bundle) = writer.add(l);
                    writer = next_writer;
                    bundle
                }
            };
            replacements.push(bundle);
        }
        // now drop the splice iterator
        std::mem::drop(spliced_middle);
        rights.splice(matching_len_end..matching_len_end, replacements);

        // Remove any extra rights
        for NodeEntry(r) in rights_diff.drain() {
            test_log!("removing: {:?}", r);
            r.detach(parent);
        }

        // Diff matching children at the start
        let rights_to = rights.len() - matching_len_start;
        for (l, r) in lefts
            .drain(..) // matching_len_start.. has been drained already
            .rev()
            .zip(rights[rights_to..].iter_mut())
        {
            writer = writer.patch(l, r);
        }

        writer.next_sibling
    }
}

impl DomBundle for BList {
    fn detach(self, parent: &Element) {
        for child in self.rev_children.into_iter() {
            child.detach(parent);
        }
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) {
        for node in self.rev_children.iter().rev() {
            node.shift(next_parent, next_sibling.clone());
        }
    }
}

impl VDiff for VList {
    type Bundle = BList;

    fn attach(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let mut self_ = BNode::BList(BList::new());
        let node_ref = self.apply(parent_scope, parent, next_sibling, &mut self_);
        let self_ = match self_ {
            BNode::BList(self_) => self_,
            _ => unreachable!("applying list should leave a VList in bundle ref"),
        };
        (node_ref, self_)
    }

    fn apply(
        mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: &mut BNode,
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
            self.add_child(VText::new("").into());
        }

        let lefts = self.children;
        let blist = ancestor.make_list();
        let rights = &mut blist.rev_children;
        test_log!("lefts: {:?}", lefts);
        test_log!("rights: {:?}", rights);

        if let Some(additional) = rights.len().checked_sub(lefts.len()) {
            rights.reserve_exact(additional);
        }
        #[allow(clippy::let_and_return)]
        let first = if self.fully_keyed && blist.fully_keyed {
            BList::apply_keyed(parent_scope, parent, next_sibling, lefts, rights)
        } else {
            BList::apply_unkeyed(parent_scope, parent, next_sibling, lefts, rights)
        };
        blist.fully_keyed = self.fully_keyed;
        blist.key = self.key;
        test_log!("result: {:?}", rights);
        first
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

#[cfg(test)]
mod layout_tests_keys {
    extern crate self as yew;

    use crate::html;
    use crate::tests::layout_tests::{diff_layouts, TestLayout};
    use crate::virtual_dom::VNode;
    use crate::{Children, Component, Context, Html, Properties};
    use web_sys::Node;

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
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

        let vref_node: Node = gloo_utils::document().create_element("i").unwrap().into();
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
