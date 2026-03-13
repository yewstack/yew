pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            text("The "),
            code("ref"),
            text(" keyword can be used inside of any HTML element or component to get the DOM "),
            code("Element"),
            text(
                " that the item is attached to. This can be used to make changes to the DOM \
                 outside of the ",
            ),
            code("view"),
            text(" lifecycle method."),
        ]),
        p(vec![
            text(
                "This is useful for getting ahold of canvas elements, or scrolling to different \
                 sections of a page. For example, using a ",
            ),
            code("NodeRef"),
            text(" in a component's "),
            code("rendered"),
            text(
                " method allows you to make draw calls to a canvas element after it has been \
                 rendered from ",
            ),
            code("view"),
            text("."),
        ]),
        p(vec![text("The syntax is:")]),
        code_block(
            "rust",
            r#"use web_sys::Element;
use yew::{html, Component, Context, Html, NodeRef};

struct Comp {
    node_ref: NodeRef,
}

impl Component for Comp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            // highlight-next-line
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            // highlight-next-line
            <div ref={self.node_ref.clone()}></div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        // highlight-start
        let has_attributes = self.node_ref
            .cast::<Element>()
            .unwrap()
            .has_attributes();
        // highlight-end
    }
}"#,
        ),
        h2(vec![text("Relevant examples")]),
        ul(vec![li(vec![link(
            "https://github.com/yewstack/yew/tree/master/examples/node_refs",
            vec![text("Node Refs")],
        )])]),
    ])
}

crate::doc_page!(
    "Refs",
    "/docs/advanced-topics/struct-components/refs",
    page_content()
);
