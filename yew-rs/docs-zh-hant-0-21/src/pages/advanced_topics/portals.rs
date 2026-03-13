crate::doc_page!(
    "傳送門 (Portals)",
    "/zh-Hant/docs/advanced-topics/portals",
    Content::new(vec![
        h2(vec![text("什麼是 Portal？")]),
        p(vec![
            text(
                "傳送門 (Portal) 提供了一種將子元素渲染到父元件的 DOM 層次結構以外的 DOM \
                 節點的方法。 "
            ),
            code("yew::create_portal(child, host)"),
            text(" 傳回一個 "),
            code("Html"),
            text(" 值，它將 "),
            code("child"),
            text(" 渲染為 "),
            code("host"),
            text(" 元素的子元素，而不是在其父元件的層次結構下。"),
        ]),
        h2(vec![text("用法")]),
        p(vec![
            text("傳送門的典型用途包括模態對話框和懸停卡片，以及更多技術應用，例如控制元素的"),
            link(
                "https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot",
                vec![text("shadowRoot")],
            ),
            text(" 的內容，將樣式表附加到周圍文檔的"),
            code("<head>"),
            text(" 中，以及在"),
            code("<svg>"),
            text(" 的中央"),
            code("<defs>"),
            text(" 元素中收集引用的元素。"),
        ]),
        p(vec![
            text("請注意，"),
            code("yew::create_portal"),
            text(
                " 是一個低階建置區塊。庫應該使用它來實現更高級的 API，然後應用程式可以使用這些 \
                 API。例如，這裡是一個簡單的模態對話框，它將其 "
            ),
            code("children"),
            text(" 渲染到 "),
            code("yew"),
            text(" 以外的元素中，該元素由 "),
            code("id=\"modal_host\""),
            text(" 標識。"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn Modal(props: &ModalProps) -> Html {
    let modal_host = gloo::utils::document()
        .get_element_by_id("modal_host")
        .expect("Expected to find a #modal_host element");

    create_portal(
        props.children.clone(),
        modal_host.into(),
    )
}"#,
        ),
        h2(vec![text("事件處理")]),
        p(vec![text(
            "傳送門內部元素上發出的事件遵循虛擬 DOM \
             冒泡。也就是說，如果傳送門被渲染為元素的子元素，\
             那麼該元素上的事件監聽器將捕捉從傳送門內部分發出的事件，即使傳送門將其內容渲染在實際 \
             DOM 中的不相關位置。"
        ),]),
        p(vec![text(
            "這使開發人員無需關心他們使用的組件是使用傳送門實現的還是沒有使用傳送門實現的。\
             無論如何，其子元素上觸發的事件都會冒泡。"
        ),]),
        p(vec![
            text("已知問題是，從傳送門到 "),
            bold(vec![text("關閉")]),
            text(
                " 的 shadow root 的事件將被分發兩次，一次針對 shadow root \
                 內部的元素，一次針對宿主元素本身。請記住，"
            ),
            bold(vec![text("打開")]),
            text(" 的 shadow root 可以正常工作。如果這影響到您，請隨時提交錯誤報告。"),
        ]),
        h2(vec![text("進一步閱讀")]),
        ul(vec![li(vec![link(
            "https://github.com/yewstack/yew/tree/master/examples/portals",
            vec![text("傳送門範例")],
        )])]),
    ])
);
