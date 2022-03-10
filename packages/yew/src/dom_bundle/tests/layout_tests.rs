use crate::dom_bundle::{BNode, BundleRoot, DomBundle, Reconcilable};
use crate::html::AnyScope;
use crate::scheduler;
use crate::virtual_dom::VNode;
use crate::{Component, Context, Html};
use gloo::console::log;
use yew::NodeRef;

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

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
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
    let document = gloo_utils::document();
    let scope: AnyScope = AnyScope::test();
    let parent_element = document.create_element("div").unwrap();
    let root = BundleRoot::create_root(&parent_element);

    let end_node = document.create_text_node("END");
    parent_element.append_child(&end_node).unwrap();

    // Tests each layout independently
    let next_sibling = NodeRef::new(end_node.into());
    for layout in layouts.iter() {
        // Apply the layout
        let vnode = layout.node.clone();
        log!("Independently apply layout '{}'", layout.name);

        let (_, mut bundle) = vnode.attach(&root, &scope, &parent_element, next_sibling.clone());
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

        vnode.reconcile_node(
            &root,
            &scope,
            &parent_element,
            next_sibling.clone(),
            &mut bundle,
        );
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
    let mut bundle: Option<BNode> = None;
    for layout in layouts.iter() {
        let next_vnode = layout.node.clone();

        log!("Sequentially apply layout '{}'", layout.name);
        next_vnode.reconcile_sequentially(
            &root,
            &scope,
            &parent_element,
            next_sibling.clone(),
            &mut bundle,
        );
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
        next_vnode.reconcile_sequentially(
            &root,
            &scope,
            &parent_element,
            next_sibling.clone(),
            &mut bundle,
        );
        scheduler::start_now();
        assert_eq!(
            parent_element.inner_html(),
            format!("{}END", layout.expected),
            "Sequential detach failed for layout '{}'",
            layout.name,
        );
    }

    // Detach last layout
    if let Some(bundle) = bundle {
        bundle.detach(&root, &parent_element, false);
    }
    scheduler::start_now();
    assert_eq!(
        parent_element.inner_html(),
        "END",
        "Failed to detach last layout"
    );
}
