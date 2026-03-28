crate::doc_page!(
    "Refs",
    "/zh-Hant/docs/advanced-topics/struct-components/refs",
    Content::new(vec![
        h2!["Refs"],
        p![
            code("ref"),
            " 關鍵字可以被使用在任何 HTML 的元素或是元件，用來得到那個物件附加的 DOM ",
            code("Element"),
            "。這個可以在 view 生命周期方法之外，改變 DOM。",
        ],
        p!["對於要存取 canvas 元素，或滾動到頁面不同的區塊，很有幫助。"],
        p!["語法可以這樣使用："],
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
    ])
    .with_description("Out-of-band DOM access")
);
