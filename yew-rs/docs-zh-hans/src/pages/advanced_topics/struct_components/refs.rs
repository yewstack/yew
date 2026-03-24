pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("ref"),
            text(" 关键字可以在任何 HTML 元素或组件中使用，以获取附加到该元素的 DOM "),
            code("Element"),
            text("。这可以用于在 "),
            code("view"),
            text(" 生命周期方法之外对 DOM 进行更改。"),
        ],
        p![
            text("这对于获取 canvas 元素或滚动到页面的不同部分很有用。例如，在组件的 "),
            code("rendered"),
            text(" 方法中使用 "),
            code("NodeRef"),
            text(" 允许您在从 "),
            code("view"),
            text(" 渲染后对 canvas 元素进行绘制调用。"),
        ],
        p![text("语法如下：")],
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
        h2![text("相关示例")],
        ul![li![link!(
            "https://github.com/yewstack/yew/tree/master/examples/node_refs",
            text("节点引用"),
        )]],
    ])
}

crate::doc_page!(
    "引用 (Refs)",
    "/zh-Hans/docs/advanced-topics/struct-components/refs",
    page_content()
);
