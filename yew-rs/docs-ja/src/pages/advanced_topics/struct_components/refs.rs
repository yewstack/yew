pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("ref"),
            " キーワードは、任意の HTML 要素やコンポーネントに使用して、その要素に付随する DOM ",
            code("Element"),
            " を取得できます。これにより、",
            code("view"),
            " ライフサイクルメソッドの外で DOM を変更することができます。",
        ],
        p![
            "これは、canvas \
             要素を取得したり、ページの異なる部分にスクロールしたりするのに便利です。例えば、\
             コンポーネントの ",
            code("rendered"),
            " メソッドで ",
            code("NodeRef"),
            " を使用すると、",
            code("view"),
            " からレンダリングされた後に canvas 要素に描画呼び出しを行うことができます。",
        ],
        p!["構文は次のとおりです："],
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
        h2!["関連例"],
        ul![li![link!(
            "https://github.com/yewstack/yew/tree/master/examples/node_refs",
            "ノード参照",
        )]],
    ])
}

crate::doc_page!(
    "参照 (Refs)",
    "/ja/docs/advanced-topics/struct-components/refs",
    page_content()
);
