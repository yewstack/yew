pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "每個函數元件都是一個",
            link!(
                "https://zh.wikipedia.org/zh-hk/%E7%BA%AF%E5%87%BD%E6%95%B0",
                "純",
            ),
            "函數，它接受一個屬性物件並傳回一個",
            code("Html"),
            " 物件。純函數是指在給定相同輸入時，總是會傳回相同輸出的函數。",
        ],
        p![
            "這個例子是一個純組件。對於給定的屬性 ",
            code("is_loading"),
            "，它總是傳回相同的 ",
            code("Html"),
            "，沒有任何副作用。",
        ],
        code_block(
            "rust",
            r#"use yew::{Properties, component, Html, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[component]
fn HelloWorld(props: &Props) -> Html {
    if props.is_loading {
        html! { "Loading" }
    } else {
        html! { "Hello world" }
    }
}"#,
        ),
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "如果您有一個內部純元件，它不使用 hooks 和其他元件機制，您通常可以將其編寫為傳回 ",
                code("Html"),
                " 的普通函數，從而避免 Yew 運行元件生命週期相關的一些開銷。使用 ",
                doc_link!(
                    crate::pages::concepts::html::literals_and_expressions,
                    #"expressions",
                    "表達式語法",
                ),
                " 在 ",
                code("html!"),
                " 中渲染它們。",
            ],
        ),
        h2!["非純組件"],
        p![
            "您可能想知道，如果元件不使用任何全域變量，那麼它是否可以是不\"純\"的，\
             因為它只是在每次渲染時調用的固定函數。 這就是下一個主題 - ",
            doc_link!(
                crate::pages::concepts::function_components::hooks::introduction,
                "hooks",
            ),
            " 的用武之地。",
        ],
    ])
}

crate::doc_page!(
    "純組件",
    "/zh-Hant/docs/concepts/function-components/pure-components",
    page_content()
);
