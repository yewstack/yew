pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            text("通常、データは props を介して親コンポーネントから子コンポーネントに渡されます。しかし、多くの中間コンポーネントを介してデータを渡す必要がある場合や、アプリケーション内の多くのコンポーネントが同じ情報を必要とする場合、props を介してデータを渡すことは冗長で煩わしいものになります。コンテキストはこの問題を解決し、親コンポーネントがデータをその下のツリー内の任意のコンポーネントに渡すことを可能にし、props を介してデータを渡す必要がなくなります。"),
        ],
        h2![text("Props を使用する際の問題：\"Prop Drilling\"")],
        p![
            link!("/ja/docs/concepts/function-components/properties", text("props")),
            text(" を介してデータを親コンポーネントから直接子コンポーネントに渡すことは良い方法です。しかし、深くネストされたコンポーネントツリーを介してデータを渡す必要がある場合や、複数のコンポーネントが同じデータを共有する必要がある場合、props を渡すことは煩雑になります。一般的なデータ共有の解決策は、データを共通の祖先に持ち上げ、子コンポーネントがそれを props として受け取るようにすることです。しかし、これにより props が複数のコンポーネントを介して渡される必要がある場合があります。この状況は \"Prop Drilling\" と呼ばれます。"),
        ],
        p![text("以下の例を考えてみましょう。これは props を介してテーマを渡しています：")],
        code_block("rust", r##"use yew::{html, Component, Context, Html, Properties, component};

#[derive(Clone, PartialEq)]
pub struct Theme {
    foreground: String,
    background: String,
}

#[derive(PartialEq, Properties)]
pub struct NavbarProps {
    theme: Theme,
}

#[component]
fn Navbar(props: &NavbarProps) -> Html {
    html! {
        <div>
            <Title theme={props.theme.clone()}>
                { "App title" }
            </Title>
            <NavButton theme={props.theme.clone()}>
                { "Somewhere" }
            </NavButton>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ThemeProps {
    theme: Theme,
    children: Html,
}

#[component]
fn Title(_props: &ThemeProps) -> Html {
    html! {
        // impl
    }
}

#[component]
fn NavButton(_props: &ThemeProps) -> Html {
    html! {
        // impl
    }
}

/// アプリのルート
#[component]
fn App() -> Html {
    let theme = Theme {
        foreground: "yellow".to_owned(),
        background: "pink".to_owned(),
    };

    html! {
        <Navbar {theme} />
    }
}"##),
        p![
            text("私たちはテーマ設定を "),
            code("Navbar"),
            text(" に渡して、それが "),
            code("Title"),
            text(" と "),
            code("NavButton"),
            text(" に到達するようにしています。もし "),
            code("Title"),
            text(" と "),
            code("NavButton"),
            text(" のようなテーマにアクセスする必要があるコンポーネントが、prop を介さずに直接テーマにアクセスできるとしたら、もっと良いでしょう。コンテキストはこの問題を解決し、親コンポーネントがデータ（この場合はテーマ）をその子コンポーネントに渡すことを可能にします。"),
        ],
        h2![text("コンテキストの使用")],
        h3![text("ステップ 1：コンテキストの提供")],
        p![
            text("コンテキストを消費するには、コンテキストプロバイダーが必要です。"),
            code("ContextProvider<T>"),
            text(" は、"),
            code("T"),
            text(" がコンテキスト構造体として使用されるプロバイダーです。"),
            code("T"),
            text(" は "),
            code("Clone"),
            text(" と "),
            code("PartialEq"),
            text(" を実装する必要があります。"),
            code("ContextProvider"),
            text(" は、その子コンポーネントがコンテキストを持つコンポーネントです。コンテキストが変更されると、子コンポーネントは再レンダリングされます。データを渡すための構造体が定義されます。"),
            code("ContextProvider"),
            text(" は次のように使用できます："),
        ],
        code_block("rust", r##"use yew::prelude::*;
/// アプリのテーマ
#[derive(Clone, Debug, PartialEq)]
struct Theme {
    foreground: String,
    background: String,
}

/// メインコンポーネント
#[component]
pub fn App() -> Html {
    let ctx = use_state(|| Theme {
        foreground: "#000000".to_owned(),
        background: "#eeeeee".to_owned(),
    });

    html! {
        // `ctx` は `Rc<UseStateHandle<Theme>>` 型であり、`Theme` が必要です
        // したがって、デリファレンスします。
        <ContextProvider<Theme> context={(*ctx).clone()}>
            // ここにあるすべての子コンポーネントとその子コンポーネントは、このコンテキストにアクセスします。
            <Toolbar />
        </ContextProvider<Theme>>
    }
}

/// ツールバー
/// このコンポーネントはコンテキストにアクセスできます。
#[component]
pub fn Toolbar() -> Html {
    html! {
        <div>
            <ThemedButton />
        </div>
    }
}

/// `Toolbar` 内に配置されたボタン
/// このコンポーネントは、コンポーネントツリー内の `ThemeContextProvider` の子コンポーネントであるため、
/// コンテキストにアクセスできます。
#[component]
pub fn ThemedButton() -> Html {
    let theme = use_context::<Theme>().expect("no ctx found");

    html! {
        <button style={format!("background: {}; color: {};", theme.background, theme.foreground)}>
            { "Click me!" }
        </button>
    }
}"##),
        h3![text("ステップ 2：コンテキストの使用")],
        h4![text("関数コンポーネント")],
        p![
            code("use_context"),
            text(" フックは、関数コンポーネント内でコンテキストを使用するために使用されます。詳細については、"),
            link!("https://yew-rs-api.web.app/next/yew/functional/fn.use_context.html", text("use_context ドキュメント")),
            text(" を参照してください。"),
        ],
        h4![text("構造体コンポーネント")],
        p![text("構造体コンポーネント内でコンテキストを使用するには、2つの方法があります：")],
        ul![
            li![
                link!("/ja/docs/advanced-topics/struct-components/hoc", text("高階コンポーネント")),
                text("：高階関数コンポーネントがコンテキストを使用し、必要なデータを構造体コンポーネントに渡します。"),
            ],
            li![
                text("構造体コンポーネント内で直接コンテキストを使用します。詳細については、"),
                link!("https://github.com/yewstack/yew/tree/master/examples/contexts/src/struct_component_subscriber.rs", text("構造体コンポーネントのコンシューマーとしての例")),
                text(" を参照してください。"),
            ],
        ],
        h2![text("使用シナリオ")],
        p![text("通常、ツリーの異なる部分のリモートコンポーネントでデータを使用する必要がある場合、コンテキストが役立ちます。以下はいくつかの例です：")],
        ul![
            li![
                bold![text("テーマ")],
                text("：アプリケーションのトップにコンテキストを配置し、アプリケーションのテーマを保持し、視覚的な外観を調整するために使用できます（上記の例を参照）。"),
            ],
            li![
                bold![text("現在のユーザーアカウント")],
                text("：多くの場合、コンポーネントは現在ログインしているユーザーを知る必要があります。コンテキストを使用して、現在のユーザーオブジェクトをコンポーネントに提供できます。"),
            ],
        ],
        h3![text("コンテキストを使用する前の考慮事項")],
        p![text("コンテキストは非常に使いやすいですが、それが誤用/過度に使用される可能性もあります。複数のレベル深いコンポーネントに props を共有するためにコンテキストを使用できるからといって、必ずしもそうすべきではありません。")],
        p![
            text("例えば、コンポーネントを抽出して、そのコンポーネントを別のコンポーネントの子コンポーネントとして渡すことができます。例えば、"),
            code("Layout"),
            text(" コンポーネントが "),
            code("articles"),
            text(" を prop として受け取り、それを "),
            code("ArticleList"),
            text(" コンポーネントに渡す場合、"),
            code("Layout"),
            text(" コンポーネントをリファクタリングして、子コンポーネントを props として受け取り、"),
            code("<Layout> <ArticleList {articles} /> </Layout>"),
            text(" と表示するようにするべきです。"),
        ],
        h2![text("子コンポーネントのコンテキスト値を変更する")],
        p![
            text("Rust の所有権ルールにより、コンテキストには子コンポーネントが呼び出せる "),
            code("&mut self"),
            text(" メソッドを持つことができません。コンテキストの値を変更するには、リデューサーと組み合わせて使用する必要があります。これは、"),
            link!("https://yew-rs-api.web.app/next/yew/functional/fn.use_reducer.html", code("use_reducer")),
            text(" フックを使用して行うことができます。"),
        ],
        p![
            link!("https://github.com/yewstack/yew/tree/master/examples/contexts", text("コンテキストの例")),
            text(" は、可変コンテキストの使用を示しています。"),
        ],
        h2![text("さらなる読み物")],
        ul![
            li![
                link!("https://github.com/yewstack/yew/tree/master/examples/contexts", text("コンテキストの例")),
            ],
        ],
    ])
}

crate::doc_page!(
    "コンテキスト (Contexts)",
    "/ja/docs/concepts/contexts",
    page_content()
);
