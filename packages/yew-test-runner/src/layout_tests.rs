//! Snapshot testing of Yew components

use yew::virtual_dom::VNode;
use yew::{Html, Renderer};

use crate::scaffold::{TestScaffold, TestScaffoldProps};

#[derive(Debug)]
pub struct TestLayout<'a> {
    pub name: &'a str,
    pub node: VNode,
    pub expected: &'a str,
}

pub fn diff_layouts(layouts: Vec<TestLayout<'_>>) {
    let document = gloo::utils::document();
    let parent_element = document.create_element("div").unwrap();

    // start with empty children
    let mut test_host = Renderer::<TestScaffold>::with_root(parent_element.clone()).render();
    yew::scheduler::__unstable_start_now();

    // Tests each layout independently
    for layout in layouts.iter() {
        // Apply the layout
        tracing::debug!(name = layout.name, "Independently apply layout");

        let vnode = layout.node.clone();
        test_host.update(TestScaffoldProps { test_case: vnode });
        yew::scheduler::__unstable_start_now();
        assert_eq!(
            parent_element.inner_html(),
            format!("{}", layout.expected),
            "Independent apply failed for layout '{}'",
            layout.name,
        );

        // Diff with no changes
        tracing::debug!(name = layout.name, "Independently reapply layout");

        let vnode = layout.node.clone();
        test_host.update(TestScaffoldProps { test_case: vnode });
        yew::scheduler::__unstable_start_now();
        assert_eq!(
            parent_element.inner_html(),
            format!("{}", layout.expected),
            "Independent reapply failed for layout '{}'",
            layout.name,
        );

        // Detach
        test_host.update(TestScaffoldProps {
            test_case: Html::default(),
        });
        yew::scheduler::__unstable_start_now();
        assert_eq!(
            parent_element.inner_html(),
            "",
            "Independent detach failed for layout '{}'",
            layout.name,
        );
    }

    // Sequentially apply each layout
    for layout in layouts.iter() {
        tracing::debug!(name = layout.name, "Sequentially apply layout");

        let vnode = layout.node.clone();
        test_host.update(TestScaffoldProps { test_case: vnode });
        yew::scheduler::__unstable_start_now();
        assert_eq!(
            parent_element.inner_html(),
            format!("{}", layout.expected),
            "Sequential apply failed for layout '{}'",
            layout.name,
        );
    }

    // Sequentially detach each layout
    for layout in layouts.into_iter().rev() {
        let vnode = layout.node.clone();

        tracing::debug!(name = layout.name, "Sequentially detach layout");
        test_host.update(TestScaffoldProps { test_case: vnode });
        yew::scheduler::__unstable_start_now();
        assert_eq!(
            parent_element.inner_html(),
            format!("{}", layout.expected),
            "Sequential detach failed for layout '{}'",
            layout.name,
        );
    }

    // Detach last layout
    test_host.destroy();
    assert_eq!(
        parent_element.inner_html(),
        "",
        "Failed to detach last layout"
    );
}
