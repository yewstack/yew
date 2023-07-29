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

#[derive(Debug)]
pub struct TestLayout<'a> {
    pub name: &'a str,
    pub node: VNode,
    pub expected: &'a str,
}

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
