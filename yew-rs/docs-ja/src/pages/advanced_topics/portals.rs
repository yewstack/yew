pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2![text("ポータルとは？")],
        p![
            text("ポータル (Portal) は、子要素を親コンポーネントのDOM階層外のDOMノードにレンダリングする方法を提供します。"),
            code("yew::create_portal(child, host)"),
            text(" は "),
            code("Html"),
            text(" 値を返し、"),
            code("child"),
            text(" を "),
            code("host"),
            text(" 要素の子要素としてレンダリングしますが、親コンポーネントの階層下ではありません。"),
        ],
        h2![text("使用方法")],
        p![
            text("ポータルの典型的な用途には、モーダルダイアログやホバーカード、さらに技術的な用途として、要素の "),
            link!["https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot", text("shadowRoot")],
            text(" の内容を制御すること、スタイルシートを周囲のドキュメントの "),
            code("<head>"),
            text(" に添付すること、"),
            code("<svg>"),
            text(" の中央の "),
            code("<defs>"),
            text(" 要素に参照される要素を収集することなどがあります。"),
        ],
        p![
            code("yew::create_portal"),
            text(" は低レベルの構成要素であることに注意してください。ライブラリはこれを使用してより高レベルのAPIを実装し、その後アプリケーションはこれらのAPIを使用できます。例えば、ここでは "),
            code("children"),
            text(" を "),
            code("yew"),
            text(" 以外の要素にレンダリングするシンプルなモーダルダイアログを示します。この要素は "),
            code(r#"id="modal_host""#),
            text(" で識別されます。"),
        ],
        code_block("rust", r##"use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Html,
}

#[component]
fn Modal(props: &ModalProps) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id("modal_host")
        .expect("Expected to find a #modal_host element");

    create_portal(
        props.children.clone(),
        modal_host.into(),
    )
}"##),
        h2![text("イベント処理")],
        p![text("ポータル内部の要素で発生するイベントは、仮想DOMのバブリングに従います。つまり、ポータルが要素の子要素としてレンダリングされる場合、その要素上のイベントリスナーは、ポータル内部から発生するイベントをキャプチャします。たとえポータルが実際のDOM内の無関係な位置にその内容をレンダリングしていてもです。")],
        p![text("これにより、開発者は使用しているコンポーネントがポータルを使用して実装されているかどうかを気にする必要がなくなります。いずれにせよ、その子要素上で発生するイベントはバブリングします。")],
        p![
            text("既知の問題として、ポータルから "),
            bold![text("閉じた")],
            text(" シャドウルートへのイベントは2回分配されます。1回はシャドウルート内部の要素に対して、もう1回はホスト要素自体に対してです。"),
            bold![text("開いた")],
            text(" シャドウルートは正常に動作しますので、これが影響する場合は、いつでもバグレポートを提出してください。"),
        ],
        h2![text("さらなる読み物")],
        ul![
            li![link![
                "https://github.com/yewstack/yew/tree/master/examples/portals",
                text("ポータルの例"),
            ]],
        ],
    ])
}

crate::doc_page!(
    "ポータル (Portals)",
    "/ja/docs/advanced-topics/portals",
    page_content()
);
