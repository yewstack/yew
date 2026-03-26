crate::doc_page!(
    "Refs",
    "/zh-Hans/docs/advanced-topics/struct-components/refs",
    Content::new(vec![
        p![
            code("ref"),
            " 关键词可被用在任何 HTML 元素或组件内部以获得该项所附加到的 DOM 元素。这可被用于在 ",
            code("view"),
            " 生命周期方法之外来对 DOM 进行更改。"
        ],
        p![
            "这对于获取 canvas 元素或者滚动到页面的不同部分是有用的。 For example, using a ",
            code("NodeRef"),
            " in a component's ",
            code("rendered"),
            " method allows you to make draw calls to a canvas element after it has been rendered \
             from ",
            code("view"),
            "."
        ],
        p!["语法如下："],
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
}"#
        ),
        h2!["Relevant examples"],
        ul![li![link!(
            "https://github.com/yewstack/yew/tree/master/examples/node_refs",
            "Node Refs"
        )]]
    ])
);
