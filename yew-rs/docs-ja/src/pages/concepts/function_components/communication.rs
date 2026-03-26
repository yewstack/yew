pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["親コンポーネントから子コンポーネントへのメッセージ送信"],
        p![
            "データを ",
            link!["/ja/docs/concepts/function-components/properties", "props",],
            " として渡すと、再レンダリングが発生し、\
             これが子コンポーネントにメッセージを渡す方法です。",
        ],
        h2!["子コンポーネントから親コンポーネントへのメッセージ送信"],
        p![
            "props を介してコールバックを渡し、\
             子コンポーネントはイベントでそれを呼び出すことができます。",
            link![
                "/ja/docs/concepts/function-components/callbacks#passing-callbacks-as-props",
                "例",
            ],
        ],
    ])
}

crate::doc_page!(
    "コンポーネント間の通信",
    "/ja/docs/concepts/function-components/communication",
    page_content()
);
