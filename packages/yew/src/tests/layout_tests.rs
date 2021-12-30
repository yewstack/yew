use crate::html::{AnyScope, Scope};
use crate::virtual_dom::{VDiff, VNode, VText};
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
    let parent_scope: AnyScope = Scope::<Comp>::new(None).into();
    let parent_element = document.create_element("div").unwrap();
    let parent_node: Node = parent_element.clone().into();
    let end_node = document.create_text_node("END");
    parent_node.append_child(&end_node).unwrap();
    let mut empty_node: VNode = VText::new("").into();

    // Tests each layout independently
    let next_sibling = NodeRef::new(end_node.into());
    for layout in layouts.iter() {
        // Apply the layout
        let mut node = layout.node.clone();
        log!("Independently apply layout '{}'", layout.name);

        node.apply(&parent_scope, &parent_element, next_sibling.clone(), None);
        assert_eq!(
            parent_element.inner_html(),
            format!("{}END", layout.expected),
            "Independent apply failed for layout '{}'",
            layout.name,
        );

        // Diff with no changes
        let mut node_clone = layout.node.clone();

        log!("Independently reapply layout '{}'", layout.name);

        node_clone.apply(
            &parent_scope,
            &parent_element,
            next_sibling.clone(),
            Some(node),
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
            Some(node_clone),
        );
        assert_eq!(
            parent_element.inner_html(),
            "END",
            "Independent detach failed for layout '{}'",
            layout.name,
        );
    }

    // Sequentially apply each layout
    let mut ancestor: Option<VNode> = None;
    for layout in layouts.iter() {
        let mut next_node = layout.node.clone();

        log!("Sequentially apply layout '{}'", layout.name);
        next_node.apply(
            &parent_scope,
            &parent_element,
            next_sibling.clone(),
            ancestor,
        );
        assert_eq!(
            parent_element.inner_html(),
            format!("{}END", layout.expected),
            "Sequential apply failed for layout '{}'",
            layout.name,
        );
        ancestor = Some(next_node);
    }

    // Sequentially detach each layout
    for layout in layouts.into_iter().rev() {
        let mut next_node = layout.node.clone();

        log!("Sequentially detach layout '{}'", layout.name);
        next_node.apply(
            &parent_scope,
            &parent_element,
            next_sibling.clone(),
            ancestor,
        );
        assert_eq!(
            parent_element.inner_html(),
            format!("{}END", layout.expected),
            "Sequential detach failed for layout '{}'",
            layout.name,
        );
        ancestor = Some(next_node);
    }

    // Detach last layout
    empty_node.apply(&parent_scope, &parent_element, next_sibling, ancestor);
    assert_eq!(
        parent_element.inner_html(),
        "END",
        "Failed to detach last layout"
    );
}
