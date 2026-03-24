crate::doc_page!(
    "組件之間的通訊",
    "/zh-Hant/docs/concepts/function-components/communication",
    Content::new(vec![
        h2![text("父元件向子元件發送訊息")],
        p![
            text("將資料作為 "),
            link!("", text("props")),
            text(" 傳遞，這會導致重新渲染，這是向子元件傳遞訊息的方法。"),
        ],
        h2![text("子元件向父元件發送訊息")],
        p![
            text("透過 props 傳遞一個回調，子元件在事件上可以呼叫。 "),
            link!("callbacks#passing-callbacks-as-props", text("範例")),
        ],
    ])
);
