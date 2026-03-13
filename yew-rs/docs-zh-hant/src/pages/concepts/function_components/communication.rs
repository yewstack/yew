pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("父元件向子元件發送訊息")]),
        p(vec![
            text("將資料作為 "),
            link(
                "/zh-Hant/docs/concepts/function-components/properties",
                vec![text("props")],
            ),
            text(" 傳遞，這會導致重新渲染，這是向子元件傳遞訊息的方法。"),
        ]),
        h2(vec![text("子元件向父元件發送訊息")]),
        p(vec![
            text("透過 props 傳遞一個回調，子元件在事件上可以呼叫。 "),
            link("callbacks#passing-callbacks-as-props", vec![text("範例")]),
        ]),
    ])
}

crate::doc_page!(
    "組件之間的通訊",
    "/zh-Hant/docs/concepts/function-components/communication",
    page_content()
);
