crate::doc_page!(
    "引用 (Refs)",
    "/zh-Hant/docs/advanced-topics/struct-components/refs",
    Content::new(vec![
        p![
            code("ref"),
            text(" 關鍵字可以在任何 HTML 元素或元件中使用，以取得附加到該元素的 DOM "),
            code("Element"),
            text("。這可以用於在 "),
            code("view"),
            text(" 生命週期方法之外對 DOM 進行更改。"),
        ],
        p![
            text("這對於獲取 canvas 元素或滾動到頁面的不同部分很有用。例如，在元件的 "),
            code("rendered"),
            text(" 方法中使用 "),
            code("NodeRef"),
            text(" 允許您在從 "),
            code("view"),
            text(" 渲染後對 canvas 元素進行繪製呼叫。"),
        ],
        p![text("文法如下：")],
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
        h2![text("相關範例")],
        ul![li![link!(
            "https://github.com/yewstack/yew/tree/master/examples/node_refs",
            text("節點引用"),
        )]],
    ])
);
