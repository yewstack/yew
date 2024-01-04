//! Snapshot testing of Yew components
//!
//! This tests must be run in browser and thus require the `csr` feature to be enabled
use gloo::console::log;

use crate::dom_bundle::{BSubtree, Bundle, DomSlot};
use crate::html::AnyScope;
use crate::virtual_dom::VNode;
use crate::{scheduler, Component, Context, Html};

struct Comp;
impl Component for Comp {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        unimplemented!()
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        unimplemented!();
    }

    fn changed(&mut self, _ctx: &Context<Self>, _: &Self::Properties) -> bool {
        unimplemented!()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        unimplemented!()
    }
}

/// Renders a [`Html`] and returns the [`web_sys::Element`] which allows
/// for querying the DOM to verify specific parts of the component.
///
/// Use this if the node is very large and/or has many parts which are irrelevant
/// to the test (e.g. styling).
///
/// Note that this only renders the node once and does not allow interaction
/// with the node (i.e. does not trigger rerenders).
pub fn render_node(vnode: Html) -> web_sys::Element {
    let document = gloo::utils::document();
    let scope: AnyScope = AnyScope::test();
    let parent_element = document.create_element("div").unwrap();
    let root = BSubtree::create_root(&parent_element);

    let slot = DomSlot::at_end();
    let mut bundle = Bundle::new();
    bundle.reconcile(&root, &scope, &parent_element, slot, vnode);
    scheduler::start_now();

    parent_element
}

/// A struct which defines the string of HTML that is the expected output of a [`VNode`].
#[derive(Debug)]
pub struct TestLayout<'a> {
    /// Name of the test layout for inspecability.
    pub name: &'a str,
    /// The node which should be rendered.
    pub node: VNode,
    /// The expected HTML string output of rendering `node`.
    pub expected: &'a str,
}

/// Iterate over multiple layouts and check them for correctness.
pub fn diff_layouts(layouts: Vec<TestLayout<'_>>) {
    let document = gloo::utils::document();
    let scope: AnyScope = AnyScope::test();
    let parent_element = document.create_element("div").unwrap();
    let root = BSubtree::create_root(&parent_element);

    let end_node = document.create_text_node("END");
    parent_element.append_child(&end_node).unwrap();

    // Tests each layout independently
    let slot = DomSlot::at(end_node.into());
    for layout in layouts.iter() {
        // Apply the layout
        let vnode = layout.node.clone();
        log!("Independently apply layout '{}'", layout.name);

        let mut bundle = Bundle::new();
        bundle.reconcile(&root, &scope, &parent_element, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(
            parent_element.inner_html(),
            format!("{}END", layout.expected),
            "Independent apply failed for layout '{}'",
            layout.name,
        );

        // Diff with no changes
        let vnode = layout.node.clone();

        log!("Independently reapply layout '{}'", layout.name);

        bundle.reconcile(&root, &scope, &parent_element, slot.clone(), vnode);
        scheduler::start_now();
        assert_eq!(
            parent_element.inner_html(),
            format!("{}END", layout.expected),
            "Independent reapply failed for layout '{}'",
            layout.name,
        );

        // Detach
        bundle.detach(&root, &parent_element, false);
        scheduler::start_now();
        assert_eq!(
            parent_element.inner_html(),
            "END",
            "Independent detach failed for layout '{}'",
            layout.name,
        );
    }

    // Sequentially apply each layout
    let mut bundle = Bundle::new();
    for layout in layouts.iter() {
        let next_vnode = layout.node.clone();

        log!("Sequentially apply layout '{}'", layout.name);
        bundle.reconcile(&root, &scope, &parent_element, slot.clone(), next_vnode);

        scheduler::start_now();
        assert_eq!(
            parent_element.inner_html(),
            format!("{}END", layout.expected),
            "Sequential apply failed for layout '{}'",
            layout.name,
        );
    }

    // Sequentially detach each layout
    for layout in layouts.into_iter().rev() {
        let next_vnode = layout.node.clone();

        log!("Sequentially detach layout '{}'", layout.name);
        bundle.reconcile(&root, &scope, &parent_element, slot.clone(), next_vnode);

        scheduler::start_now();
        assert_eq!(
            parent_element.inner_html(),
            format!("{}END", layout.expected),
            "Sequential detach failed for layout '{}'",
            layout.name,
        );
    }

    // Detach last layout
    bundle.detach(&root, &parent_element, false);
    scheduler::start_now();
    assert_eq!(
        parent_element.inner_html(),
        "END",
        "Failed to detach last layout"
    );
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;
    use crate::prelude::*;

    #[wasm_bindgen_test]
    fn render_node_returns_element_containing_expected_outer_html() {
        let elem = render_node(html!("test string"));
        assert_eq!(elem.outer_html(), "<div>test string</div>")
    }
}
