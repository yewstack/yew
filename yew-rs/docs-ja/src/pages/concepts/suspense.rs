pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p!["サスペンス (Suspense) は、タスクが完了するまでコンポーネントのレンダリングを一時停止し、その間にフォールバック（プレースホルダー）UI を表示する方法です。"],
        p!["これは、サーバーからデータを取得したり、プロキシがタスクを完了するのを待ったり、他のバックグラウンド非同期タスクを実行したりするために使用できます。"],
        p!["サスペンスが表示される前に、データ取得は通常、コンポーネントのレンダリング後（レンダリング時取得）またはレンダリング前（取得後レンダリング）に発生します。"],
        h3!["レンダリングしながらダウンロード"],
        p!["サスペンス (Suspense) は、新しい方法を提供し、コンポーネントがレンダリング中にデータリクエストを発行できるようにします。コンポーネントがデータリクエストを発行すると、レンダリングプロセスが一時停止され、リクエストが完了するまでフォールバック UI が表示されます。"],
        p!["サスペンスを使用するには、フック (Hook) を使用することをお勧めします。"],
        code_block("rust", r##"use yew::prelude::*;

#[component(Content)]
fn content() -> HtmlResult {
    let user = use_user()?;

    Ok(html! {<div>{"Hello, "}{&user.name}</div>})
}

#[component(App)]
fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}"##),
        p![
            "上記の例では、",
            code("use_user"),
            " フックはユーザー情報の読み込み中にコンポーネントのレンダリングを一時停止し、",
            code("user"),
            " が読み込まれる前に ",
            code("Loading..."),
            " プレースホルダーを表示します。",
        ],
        p![
            "コンポーネントのレンダリングを一時停止するフックを定義するには、",
            code("SuspensionResult<T>"),
            " を返す必要があります。コンポーネントが一時停止する必要がある場合、フックは ",
            code("Err(Suspension)"),
            " を返すべきであり、ユーザーはそれを ",
            code("?"),
            " でアンパックする必要があります。これにより、それが ",
            code("Html"),
            " に変換されます。",
        ],
        code_block("rust", r##"use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

struct User {
    name: String,
}

#[hook]
fn use_user() -> SuspensionResult<User> {
    match load_user() {
        // ユーザーが読み込まれたら、それを Ok(user) として返します。
        Some(user) => Ok(user),
        None => {
            // ユーザーがまだ読み込まれていない場合、`Suspension` を作成し、
            // データの読み込みが完了したときに `SuspensionHandle::resume` を呼び出します。
            // これにより、コンポーネントは自動的に再レンダリングされます。
            let (s, handle) = Suspension::new();
            on_load_user_complete(move || {handle.resume();});
            Err(s)
        },
    }
}"##),
        h4!["サスペンスフック (Hook) の実装に関する注意事項"],
        p![
            link!("https://docs.rs/yew/latest/yew/suspense/struct.Suspension.html#method.new", code("Suspension::new")),
            " は 2 つの値を返します：サスペンスコンテキスト自体とサスペンスハンドル。後者はサスペンスされたコンポーネントを再レンダリングするタイミングを管理し、2 つの方法で操作できます：",
        ],
        ol![
            li![
                "その ",
                link!("https://docs.rs/yew/latest/yew/suspense/struct.SuspensionHandle.html#method.resume", code("resume")),
                " メソッドを呼び出す。",
            ],
            li!["ハンドルを破棄する。"],
        ],
        admonition![AdmonitionType::Danger, None,
            p![
                "サスペンスハンドルは、新しいデータを受け取ってコンポーネントを更新するまで保存する必要があります。そうしないと、サスペンスされたコンポーネントが無限再レンダリングループに入り、パフォーマンスに影響を与えます。上記の例では、サスペンスハンドルはクロージャに移動し、",
                code("on_load_user_complete"),
                " に渡されることで保存されます。仮想ユーザーが読み込まれると、クロージャが呼び出され、",
                code("handle.resume()"),
                " が呼び出され、サスペンスコンテキストに関連するコンポーネントが再レンダリングされます。",
            ],
        ],
        h1!["完全な例"],
        code_block("rust", r##"use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[derive(Debug)]
struct User {
    name: String,
}

fn load_user() -> Option<User> {
    todo!()  // 省略
}

fn on_load_user_complete<F: FnOnce()>(_fn: F) {
    todo!()  // 省略
}

#[hook]
fn use_user() -> SuspensionResult<User> {
    match load_user() {
        // ユーザーが読み込まれたら、それを Ok(user) として返します。
        Some(user) => Ok(user),
        None => {
            // ユーザーがまだ読み込まれていない場合、`Suspension` を作成し、
            // データの読み込みが完了したときに `SuspensionHandle::resume` を呼び出します。
            // これにより、コンポーネントは自動的に再レンダリングされます。
            let (s, handle) = Suspension::new();
            on_load_user_complete(move || {handle.resume();});
            Err(s)
        },
    }
}

#[component(Content)]
fn content() -> HtmlResult {
    let user = use_user()?;

    Ok(html! {<div>{"Hello, "}{&user.name}</div>})
}

#[component(App)]
fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}"##),
        h3!["構造体コンポーネントでプレースホルダーを使用する"],
        p![
            "構造体コンポーネントを直接サスペンドすることはできません。しかし、関数コンポーネントを",
            doc_link!(crate::pages::advanced_topics::struct_components::hoc, "高階コンポーネント"),
            "として使用し、プレースホルダーに基づいたデータ取得を実現することができます。",
        ],
        p![
            "Yew リポジトリの",
            link!("https://github.com/yewstack/yew/tree/master/examples/suspense/src/struct_consumer.rs", "プレースホルダーの例"),
            "は、このコンポーネントの使用方法を示しています。",
        ],
        h2!["関連例"],
        ul![
            li![link!("https://github.com/yewstack/yew/tree/master/examples/suspense", "プレースホルダー")],
        ],
    ])
}

crate::doc_page!(
    "サスペンス (Suspense)",
    "/ja/docs/concepts/suspense",
    page_content()
);
