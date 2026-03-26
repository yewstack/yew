pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "コールバック関数は、コンポーネントツリー内で情報を上向きに伝達したり、イベント処理中に他のコンポーネント（例えばエージェントやDOM）と通信したりするために使用されます。内部的には、コールバック関数の型は単なる ",
            code("Fn"),
            " であり、安価にクローンできるように ",
            code("Rc"),
            " に包まれています。",
        ],
        p![
            "コールバック関数を手動で呼び出したい場合は、",
            code("emit"),
            " 関数を使用できます。",
        ],
        code_block("rust", r#"use yew::{html, Component, Context, Html, Callback};

let cb: Callback<String, String> = Callback::from(move |name: String| {
    format!("Bye {}", name)
});

let result = cb.emit(String::from("Bob"));  // コールバック関数を呼び出す
// web_sys::console::log_1(&result.into()); // コメントを解除すると、「Bye Bob」 が出力されます"#),
        h2!["コールバック関数をプロパティとして渡す"],
        p!["yew で一般的なパターンは、コールバック関数を作成し、それをプロパティとして子コンポーネントに渡すことです。"],
        code_block("rust", r#"use yew::{component, html, Html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_name_entry: Callback<String>,
}

#[component]
fn HelloWorld(props: &Props) -> Html {

    props.on_name_entry.emit(String::from("Bob"));

    html! { "Hello" }
}

// 次にプロパティ (Props) を提供します
#[component]
fn App() -> Html {
    let on_name_entry: Callback<String> = Callback::from(move |name: String| {
        let greeting = format!("Hey, {}!", name);
        // web_sys::console::log_1(&greeting.into()); // コメントを解除すると、ここにテキストが出力されます
    });

    html! { <HelloWorld {on_name_entry} /> }
}"#),
        h2!["DOM イベントとコールバック関数"],
        p!["コールバック関数は、DOM イベントに接続するためにも使用されます。"],
        p!["例えば、ここではユーザーがボタンをクリックしたときに呼び出されるコールバック関数を定義します："],
        code_block("rust", r#"use yew::{component, html, Html, Properties, Callback};

#[component]
fn App() -> Html {
    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
        // web_sys::console::log_1(&greeting.into()); // コメントを解除すると、ここにテキストが出力されます
    });

    html! {
        <button {onclick}>{ "Click" }</button>
    }
}"#),
    ])
}

crate::doc_page!(
    "コールバック (Callbacks)",
    "/ja/docs/concepts/function-components/callbacks",
    page_content()
);
