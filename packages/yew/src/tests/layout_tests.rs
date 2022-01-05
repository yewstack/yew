use crate::dom_bundle::{BNode, VDiff};
use crate::html::AnyScope;
use crate::virtual_dom::{VNode, VText};
use crate::{Component, Context, Html};
use gloo::console::log;
use web_sys::Node;
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
    let parent_scope: AnyScope = AnyScope::test();
    let parent_element = document.create_element("div").unwrap();
    let parent_node: Node = parent_element.clone().into();
    let end_node = document.create_text_node("END");
    parent_node.append_child(&end_node).unwrap();
    let empty_node: VNode = VText::new("").into();

    // Tests each layout independently
    let next_sibling = NodeRef::new(end_node.into());
    for layout in layouts.iter() {
        // Apply the layout
        let vnode = layout.node.clone();
        log!("Independently apply layout '{}'", layout.name);

        let (_, mut node) = vnode.attach(&parent_scope, &parent_element, next_sibling.clone());
        assert_eq!(
            parent_element.inner_html(),
            format!("{}END", layout.expected),
            "Independent apply failed for layout '{}'",
            layout.name,
        );

        // Diff with no changes
        let vnode = layout.node.clone();

        log!("Independently reapply layout '{}'", layout.name);

        vnode.apply(
            &parent_scope,
            &parent_element,
            next_sibling.clone(),
            &mut node,
        );
        assert_eq!(
            parent_element.inner_html(),
            format!("{}END", layout.expected),
            "Independent reapply failed for layout '{}'",
            layout.name,
        );

        // Detach
        empty_node.clone().apply(
            &parent_scope,
            &parent_element,
            next_sibling.clone(),
            &mut node,
        );
        assert_eq!(
            parent_element.inner_html(),
            "END",
            "Independent detach failed for layout '{}'",
            layout.name,
        );
    }

    // Sequentially apply each layout
    let mut ancestor: Option<BNode> = None;
    for layout in layouts.iter() {
        let next_vnode = layout.node.clone();

        log!("Sequentially apply layout '{}'", layout.name);
        next_vnode.apply_sequentially(
            &parent_scope,
            &parent_element,
            next_sibling.clone(),
            &mut ancestor,
        );
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
        next_vnode.apply_sequentially(
            &parent_scope,
            &parent_element,
            next_sibling.clone(),
            &mut ancestor,
        );
        assert_eq!(
            parent_element.inner_html(),
            format!("{}END", layout.expected),
            "Sequential detach failed for layout '{}'",
            layout.name,
        );
    }

    // Detach last layout
    empty_node.apply_sequentially(&parent_scope, &parent_element, next_sibling, &mut ancestor);
    assert_eq!(
        parent_element.inner_html(),
        "END",
        "Failed to detach last layout"
    );
}
