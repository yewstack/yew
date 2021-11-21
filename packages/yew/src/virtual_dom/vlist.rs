//! This module contains fragments implementation.
use super::{Key, VDiff, VNode, VText};
use crate::html::{AnyScope, NodeRef};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use web_sys::Element;

/// This struct represents a fragment of the Virtual DOM tree.
#[derive(Clone, Debug, PartialEq)]
pub struct VList {
    /// The list of child [VNode]s
    children: Vec<VNode>,

    /// All [VNode]s in the VList have keys
    fully_keyed: bool,

    pub key: Option<Key>,
}

impl Default for VList {
    fn default() -> Self {
        Self {
            children: Default::default(),
            key: None,
            fully_keyed: true,
        }
    }
}

impl Deref for VList {
    type Target = Vec<VNode>;

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl DerefMut for VList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Caller might change the keys of the VList or add unkeyed children.
        // Defensively assume they will.
        self.fully_keyed = false;

        &mut self.children
    }
}

/// Log an operation during tests for debugging purposes
/// Set RUSTFLAGS="--cfg verbose_tests" environment variable to activate.
macro_rules! test_log {
    ($fmt:literal, $($arg:expr),* $(,)?) => {
        #[cfg(all(feature = "wasm_test", verbose_tests))]
        ::wasm_bindgen_test::console_log!(concat!("\t  ", $fmt), $($arg),*);
    };
}

/// Advance the next sibling reference (from right to left) and log it for testing purposes
/// Set RUSTFLAGS="--cfg verbose_tests" environment variable to activate.
macro_rules! advance_next_sibling {
    ($current:expr, $advance:expr) => {{
        #[cfg(all(feature = "wasm_test", verbose_tests))]
        let current = format!("{:?}", $current);
        let next = $advance();
        test_log!("advance next_sibling: {} -> {:?}", current, next);
        next
    }};
}

impl VList {
    /// Creates a new empty [VList] instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [VList] instance with children.
    pub fn with_children(children: Vec<VNode>, key: Option<Key>) -> Self {
        VList {
            fully_keyed: children.iter().all(|ch| ch.has_key()),
            children,
            key,
        }
    }

    /// Add [VNode] child.
    pub fn add_child(&mut self, child: VNode) {
        if self.fully_keyed && !child.has_key() {
            self.fully_keyed = false;
        }
        self.children.push(child);
    }

    /// Add multiple [VNode] children.
    pub fn add_children(&mut self, children: impl IntoIterator<Item = VNode>) {
        let it = children.into_iter();
        let bound = it.size_hint();
        self.children.reserve(bound.1.unwrap_or(bound.0));
        for ch in it {
            self.add_child(ch);
        }
    }

    /// Recheck, if the all the children have keys.
    ///
    /// Run this, after modifying the child list that contained only keyed children prior to the
    /// mutable dereference.
    pub fn recheck_fully_keyed(&mut self) {
        self.fully_keyed = self.children.iter().all(|ch| ch.has_key());
    }

    /// Diff and patch unkeyed child lists
    fn apply_unkeyed(
        parent_scope: &AnyScope,
        parent: &Element,
        mut next_sibling: NodeRef,
        lefts: &mut [VNode],
        rights: Vec<VNode>,
    ) -> NodeRef {
        let mut diff = lefts.len() as isize - rights.len() as isize;
        let mut lefts_it = lefts.iter_mut().rev();
        let mut rights_it = rights.into_iter().rev();

        macro_rules! apply {
            ($l:expr, $r:expr) => {
                test_log!("parent={:?}", parent.outer_html());
                next_sibling = advance_next_sibling!(next_sibling, || $l.apply(
                    parent_scope,
                    parent,
                    next_sibling,
                    $r
                ));
            };
        }

        // Add missing nodes
        while diff > 0 {
            let l = lefts_it.next().unwrap();
            test_log!("adding: {:?}", l);
            apply!(l, None);
            diff -= 1;
        }
        // Remove extra nodes
        while diff < 0 {
            let mut r = rights_it.next().unwrap();
            test_log!("removing: {:?}", r);
            r.detach(parent);
            diff += 1;
        }

        for (l, r) in lefts_it.zip(rights_it) {
            test_log!("patching: {:?} -> {:?}", r, l);
            apply!(l, r.into());
        }

        next_sibling
    }

    /// Diff and patch fully keyed child lists.
    ///
    /// Optimized for node addition or removal from either end of the list and small changes in the
    /// middle.
    fn apply_keyed(
        parent_scope: &AnyScope,
        parent: &Element,
        mut next_sibling: NodeRef,
        lefts: &mut [VNode],
        rights: Vec<VNode>,
    ) -> NodeRef {
        use std::mem::{transmute, MaybeUninit};

        macro_rules! map_keys {
            ($src:expr) => {
                $src.iter()
                    .map(|v| v.key().expect("unkeyed child in fully keyed list"))
                    .collect::<Vec<Key>>()
            };
        }
        let lefts_keys = map_keys!(lefts);
        let rights_keys = map_keys!(rights);

        /// Find the first differing key in 2 iterators
        fn diff_i<'a, 'b>(
            a: impl Iterator<Item = &'a Key>,
            b: impl Iterator<Item = &'b Key>,
        ) -> usize {
            a.zip(b).take_while(|(a, b)| a == b).count()
        }

        // Find first key mismatch from the front
        let from_start = diff_i(lefts_keys.iter(), rights_keys.iter());

        if from_start == std::cmp::min(lefts.len(), rights.len()) {
            // No key changes
            return Self::apply_unkeyed(parent_scope, parent, next_sibling, lefts, rights);
        }

        // Find first key mismatch from the back
        let from_end = diff_i(
            lefts_keys[from_start..].iter().rev(),
            rights_keys[from_start..].iter().rev(),
        );

        macro_rules! apply {
            ($l:expr, $r:expr) => {
                test_log!("patching: {:?} -> {:?}", $r, $l);
                apply!(Some($r) => $l);
            };
            ($l:expr) => {
                test_log!("adding: {:?}", $l);
                apply!(None => $l);
            };
            ($ancestor:expr => $l:expr) => {
                test_log!("parent={:?}", parent.outer_html());
                next_sibling = advance_next_sibling!(
                    next_sibling,
                    || $l.apply(parent_scope, parent, next_sibling, $ancestor)
                );
            };
        }

        // We partially deconstruct the rights vector in several steps.
        // This can not be done without default swapping or 2 big vector reallocation overhead in
        // safe Rust.
        let mut rights: Vec<MaybeUninit<VNode>> = unsafe { transmute(rights) };

        // Takes a value from the rights vector.
        //
        // We guarantee this is only done once because the consumer routine ranges don't overlap.
        // We guarantee this is done for all nodes because all ranges are processed. There are no
        // early returns (except panics) possible before full vector depletion.
        macro_rules! take {
            ($src:expr) => {
                unsafe {
                    let mut dst = MaybeUninit::<VNode>::uninit();
                    std::ptr::copy_nonoverlapping($src.as_mut_ptr(), dst.as_mut_ptr(), 1);
                    dst.assume_init()
                }
            };
        }

        // Diff matching children at the end
        let lefts_to = lefts_keys.len() - from_end;
        let rights_to = rights_keys.len() - from_end;
        for (l, r) in lefts[lefts_to..]
            .iter_mut()
            .zip(rights[rights_to..].iter_mut())
            .rev()
        {
            apply!(l, take!(r));
        }

        // Diff mismatched children in the middle
        let mut next: Option<&Key> = None;
        let mut rights_diff: HashMap<&Key, (VNode, Option<&Key>)> =
            HashMap::with_capacity(rights_to - from_start);
        for (k, v) in rights_keys[from_start..rights_to]
            .iter()
            .zip(rights[from_start..rights_to].iter_mut())
            .rev()
        {
            rights_diff.insert(k, (take!(v), next.take()));
            next = Some(k);
        }
        next = None;
        for (l_k, l) in lefts_keys[from_start..lefts_to]
            .iter()
            .zip(lefts[from_start..lefts_to].iter_mut())
            .rev()
        {
            match rights_diff.remove(l_k) {
                // Reorder and diff any existing children
                Some((r, r_next)) => {
                    match (r_next, next) {
                        // If the next sibling was already the same, we don't need to move the node
                        (Some(r_next), Some(l_next)) if r_next == l_next => (),
                        _ => {
                            test_log!("moving as next: {:?}", r);
                            r.move_before(parent, &next_sibling.get());
                        }
                    }
                    apply!(l, r);
                }
                // Add new children
                None => {
                    apply!(l);
                }
            }
            next = Some(l_k);
        }

        // Remove any extra rights
        for (_, (mut r, _)) in rights_diff.drain() {
            test_log!("removing: {:?}", r);
            r.detach(parent);
        }

        // Diff matching children at the start
        for (l, r) in lefts[..from_start]
            .iter_mut()
            .zip(rights[..from_start].iter_mut())
            .rev()
        {
            apply!(l, take!(r));
        }

        next_sibling
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
            self.add_child(VText::new("").into());
        }

        let lefts = &mut self.children;
        let (rights, rights_fully_keyed) = match ancestor {
            // If the ancestor is also a VList, then the "right" list is the previously
            // rendered items.
            Some(VNode::VList(v)) => (v.children, v.fully_keyed),

            // If the ancestor was not a VList, then the "right" list is a single node
            Some(v) => {
                let has_key = v.has_key();
                (vec![v], has_key)
            }

            // No unkeyed nodes in an empty VList
            _ => (vec![], true),
        };
        test_log!("lefts: {:?}", lefts);
        test_log!("rights: {:?}", rights);

        #[allow(clippy::let_and_return)]
        let first = if self.fully_keyed && rights_fully_keyed {
            Self::apply_keyed(parent_scope, parent, next_sibling, lefts, rights)
        } else {
            Self::apply_unkeyed(parent_scope, parent, next_sibling, lefts, rights)
        };
        test_log!("result: {:?}", lefts);
        first
    }
}

#[cfg(test)]
mod layout_tests {
    extern crate self as yew;

    use crate::html;
    use crate::virtual_dom::layout_tests::{diff_layouts, TestLayout};

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
    use crate::virtual_dom::layout_tests::{diff_layouts, TestLayout};
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
