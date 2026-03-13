pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![text(
            "シングルページアプリケーション (SPA) のルーターは、URL に基づいて異なるページを表示する処理を行います。リンクをクリックしたときに異なるリモートリソースを要求するデフォルトの動作とは異なり、ルーターはアプリケーション内の有効なルートを指すようにローカルで URL を設定します。その後、ルーターはこの変更を検出し、レンダリングする内容を決定します。"
        )]),
        p(vec![
            text("Yew は "),
            code("yew-router"),
            text(" クレートでルーターサポートを提供します。使用を開始するには、依存関係を "),
            code("Cargo.toml"),
            text(" ファイルに追加してください。"),
        ]),
        code_block("toml", "yew-router = { git = \"https://github.com/yewstack/yew.git\" }"),
        p(vec![
            text("必要なツールはすべて "),
            code("yew_router::prelude"),
            text(" モジュールで提供されています。"),
        ]),
        h2(vec![text("使用方法")]),
        p(vec![
            text("まず、"),
            code("Route"),
            text(" を定義する必要があります。"),
        ]),
        p(vec![
            text("ルートは "),
            code("Routable"),
            text(" を派生する "),
            code("enum"),
            text(" で定義されます。この列挙型は "),
            code("Clone + PartialEq"),
            text(" を実装する必要があります。"),
        ]),
        code_block(
            "rust",
            r##"use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}"##
        ),
        p(vec![
            code("Route"),
            text(" と "),
            code("<Switch />"),
            text(" コンポーネントはペアで使用され、後者はブラウザの現在の URL に一致するパスのバリアントを見つけ、それを "),
            code("render"),
            text(" コールバックに渡します。その後、コールバックがレンダリングする内容を決定します。パスが一致しない場合、ルーターは "),
            code("not_found"),
            text(" 属性を持つパスにナビゲートします。指定されたルートがない場合、何もレンダリングされず、一致するルートがないことを示すメッセージがコンソールに記録されます。"),
        ]),
        p(vec![
            text("yew-router のほとんどのコンポーネント、特に "),
            code("<Link />"),
            text(" と "),
            code("<Switch />"),
            text(" は、ある Router コンポーネント（例： "),
            code("<BrowserRouter />"),
            text("）の（深い）子要素である必要があります。通常、アプリケーションには 1 つの Router しか必要なく、通常は最上位の "),
            code("<App />"),
            text(" コンポーネントによって直ちにレンダリングされます。Router はコンテキストを登録し、これは Links と Switches の機能に必要です。以下に例を示します。"),
        ]),
        admonition(AdmonitionType::Caution, None, vec![
            p(vec![
                text("ブラウザ環境で "),
                code("yew-router"),
                text(" を使用する場合、"),
                code("<BrowserRouter />"),
                text(" を強く推奨します。他のルータータイプについては "),
                link("https://docs.rs/yew-router/", vec![text("API リファレンス")]),
                text(" を参照してください。"),
            ]),
        ]),
        code_block(
            "rust",
            r##"use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}"##
        ),
        h3(vec![text("パスセグメント")]),
        p(vec![
            text("ルーターは、動的および名前付きワイルドカードセグメントを使用してルートから情報を抽出することもできます。次に、"),
            code("<Switch />"),
            text(" 内で投稿の ID にアクセスし、それを適切なコンポーネントにプロパティとして渡すことができます。"),
        ]),
        code_block(
            "rust",
            r##"use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/post/:id")]
    Post { id: String },
    #[at("/*path")]
    Misc { path: String },
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Post { id } => html! {<p>{format!("You are looking at Post {}", id)}</p>},
        Route::Misc { path } => html! {<p>{format!("Matched some other path: {}", path)}</p>},
    }
}"##
        ),
        admonition(AdmonitionType::Note, None, vec![
            p(vec![
                code("Post {id: String}"),
                text(" の代わりに通常の "),
                code("Post"),
                text(" バリアントを使用することもできます。例えば、"),
                code("Post"),
                text(" が別のルーターと一緒にレンダリングされる場合、そのフィールドは冗長になる可能性があります。詳細については、以下の"),
                link("#nested-router", vec![text("ネストされたルーター")]),
                text("セクションを参照してください。"),
            ]),
        ]),
        p(vec![
            text("フィールドは "),
            code("Route"),
            text(" 列挙型の一部として "),
            code("Clone + PartialEq"),
            text(" を実装する必要があることに注意してください。また、シリアル化と逆シリアル化のために "),
            code("std::fmt::Display"),
            text(" と "),
            code("std::str::FromStr"),
            text(" を実装する必要があります。整数、浮動小数点数、および文字列などのプリミティブ型はこれらの要件を既に満たしています。"),
        ]),
        p(vec![
            text("パスの形式が一致しても、逆シリアル化が失敗した場合（"),
            code("FromStr"),
            text(" に基づく）、ルーターはルートが一致しないと見なし、見つからないルートをレンダリングしようとします（または、見つからないルートが指定されていない場合は空白ページをレンダリングします）。"),
        ]),
        p(vec![text("以下の例を参照してください：")]),
        code_block(
            "rust",
            r##"#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/news/:id")]
    News { id: u8 },
    #[not_found]
    #[at("/404")]
    NotFound,
}
// switch 関数は News と id をレンダリングします。ここでは省略されています。"##
        ),
        p(vec![
            text("セグメントが 255 を超えると、"),
            code("u8::from_str()"),
            text(" は失敗し、"),
            code("ParseIntError"),
            text(" を返します。この場合、ルーターはルートが一致しないと見なします。"),
        ]),
        img(
            "/img/router-deserialization-failure-behavior.gif",
            "ルーターの逆シリアル化失敗の動作",
        ),
        p(vec![
            text("ルーティング構文やパラメータのバインディング方法の詳細については、"),
            link(
                "https://docs.rs/route-recognizer/0.3.1/route_recognizer/#routing-params",
                vec![text("route-recognizer")],
            ),
            text(" を参照してください。"),
        ]),
        h3(vec![text("位置 (Location)")]),
        p(vec![
            text("ルーターはコンテキストを介して一般的な "),
            code("Location"),
            text(" 構造体を提供し、ルート情報にアクセスするために使用できます。これらはフックまたは "),
            code("ctx.link()"),
            text(" 上の便利な関数を介して取得できます。"),
        ]),
        h3(vec![text("ナビゲーション")]),
        p(vec![
            code("yew_router"),
            text(" はナビゲーションを処理するためのいくつかのツールを提供します。"),
        ]),
        h4(vec![text("リンク")]),
        p(vec![
            code("<Link />"),
            text(" は "),
            code("<a>"),
            text(" 要素としてレンダリングされ、"),
            code("onclick"),
            text(" イベントハンドラは "),
            link("https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault", vec![text("preventDefault")]),
            text(" を呼び出し、ターゲットページを履歴にプッシュして必要なページをレンダリングします。これはシングルページアプリケーションに期待される動作です。通常のアンカー要素のデフォルトの "),
            code("onclick"),
            text(" はページをリロードします。"),
        ]),
        p(vec![
            code("<Link />"),
            text(" コンポーネントはその子要素を "),
            code("<a>"),
            text(" 要素に渡します。これはアプリ内ルーティングのための "),
            code("<a/>"),
            text(" の代替として考えることができます。違いは、"),
            code("href"),
            text(" の代わりに "),
            code("to"),
            text(" 属性を提供する必要があることです。使用例は以下の通りです："),
        ]),
        code_block(
            "rust",
            r#"<Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>"#,
        ),
        p(vec![text("構造体変数も正常に動作します：")]),
        code_block(
            "rust",
            r#"<Link<Route> to={Route::Post { id: "new-yew-release".to_string() }}>{ "Yew!" }</Link<Route>>"#,
        ),
        h4(vec![text("ナビゲーションインターフェース")]),
        p(vec![
            text("ナビゲーター API は、関数コンポーネントと構造体コンポーネントの両方で提供されます。これにより、コールバックがルートを変更できるようになります。どちらの場合でも、"),
            code("Navigator"),
            text(" インスタンスを取得してルートを操作できます。"),
        ]),
        h5(vec![text("関数コンポーネント")]),
        p(vec![
            text("関数コンポーネントの場合、基礎となるナビゲータープロバイダーが変更されると、"),
            code("use_navigator"),
            text(" フックはコンポーネントを再レンダリングします。以下は、クリック時に "),
            code("Home"),
            text(" ルートにナビゲートするボタンを実装する例です。"),
        ]),
        code_block(
            "rust",
            r##"#[component(MyComponent)]
pub fn my_component() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <>
            <button {onclick}>{"Click to go home"}</button>
        </>
    }
}"##
        ),
        admonition(AdmonitionType::Caution, None, vec![
            p(vec![
                text("ここでの例では "),
                code("Callback::from"),
                text(" を使用しています。ターゲットルートがコンポーネントのルートと同じになる可能性がある場合、または安全のために、通常のコールバックを使用してください。例えば、各ページにロゴボタンがあり、そのボタンをクリックするとホームに戻るとします。ホームページでそのボタンを2回クリックすると、同じHomeルートがプッシュされ、"),
                code("use_navigator"),
                text(" フックが再レンダリングをトリガーしないため、コードがクラッシュします。"),
            ]),
        ]),
        p(vec![
            text("現在の位置をスタックに新しい位置としてプッシュするのではなく置き換えたい場合は、"),
            code("navigator.push()"),
            text(" の代わりに "),
            code("navigator.replace()"),
            text(" を使用してください。"),
        ]),
        p(vec![
            code("navigator"),
            text(" はコールバックに移動する必要があるため、他のコールバックで再利用できないことに気付くかもしれません。幸いなことに、"),
            code("navigator"),
            text(" は "),
            code("Clone"),
            text(" を実装しているため、異なるルートに対して複数のボタンを設定する方法は次のとおりです："),
        ]),
        code_block(
            "rust",
            r##"use yew::prelude::*;
use yew_router::prelude::*;

#[component(NavItems)]
pub fn nav_items() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));
        html! {
            <button {onclick}>{"click to go home"}</button>
        }
    };

    let go_to_first_post_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Post { id: "first-post".to_string() }));
        html! {
            <button {onclick}>{"click to go the first post"}</button>
        }
    };

    let go_to_secure_button = {
        let onclick = Callback::from(move |_| navigator.push(&Route::Secure));
        html! {
            <button {onclick}>{"click to go to secure"}</button>
        }
    };

    html! {
        <>
            {go_home_button}
            {go_to_first_post_button}
            {go_to_secure_button}
        </>
    }
}"##
        ),
        h5(vec![text("構造体コンポーネント")]),
        p(vec![
            text("構造体コンポーネントの場合、"),
            code("ctx.link().navigator()"),
            text(" API を使用して "),
            code("Navigator"),
            text(" インスタンスを取得できます。残りの部分は関数コンポーネントの場合と同じです。以下は、単一のボタンをレンダリングするビュー関数の例です。"),
        ]),
        code_block(
            "rust",
            r#"fn view(&self, ctx: &Context<Self>) -> Html {
    let navigator = ctx.link().navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&MainRoute::Home));
    html!{
        <button {onclick}>{"Go Home"}</button>
    }
}"#
        ),
        h4(vec![text("リダイレクト")]),
        p(vec![
            code("yew-router"),
            text(" は prelude に "),
            code("<Redirect />"),
            text(" コンポーネントも提供しています。これはナビゲーター API と同様の効果を実現するために使用できます。このコンポーネントは、ターゲットルートとして "),
            code("to"),
            text(" 属性を受け取ります。"),
            code("<Redirect/>"),
            text(" がレンダリングされると、ユーザーは指定されたルートにリダイレクトされます。以下はその例です："),
        ]),
        code_block(
            "rust",
            r##"#[component(SomePage)]
fn some_page() -> Html {
    // `use_user` フックを使用してユーザーを取得
    let user = match use_user() {
        Some(user) => user,
        // ユーザーが `None` の場合、ログインページにリダイレクト
        None => return html! {
            <Redirect<Route> to={Route::Login}/>
        },
    };
    // ... 実際のページ内容
}"##
        ),
        admonition(AdmonitionType::Tip, Some("`Redirect` と `Navigator` の選択方法"), vec![
            p(vec![
                text("Navigator API はコールバック内でルートを操作する唯一の方法です。一方、"),
                code("<Redirect />"),
                text(" はコンポーネント内の戻り値として使用できます。また、"),
                link("#nested-router", vec![text("ネストされたルーター")]),
                text("の switch 関数など、他の非コンポーネントコンテキストでも "),
                code("<Redirect />"),
                text(" を使用することができます。"),
            ]),
        ]),
        h3(vec![text("変更のリスニング")]),
        h4(vec![text("関数コンポーネント")]),
        p(vec![
            code("use_location"),
            text(" と "),
            code("use_route"),
            text(" フックを使用できます。提供された値が変更されると、コンポーネントが再レンダリングされます。"),
        ]),
        h4(vec![text("構造体コンポーネント")]),
        p(vec![
            text("ルートの変更に応答するために、"),
            code("ctx.link()"),
            text(" の "),
            code("add_location_listener()"),
            text(" メソッドにコールバッククロージャを渡すことができます。"),
        ]),
        admonition(AdmonitionType::Note, None, vec![
            p(vec![text("位置リスナーが削除されると、それは登録解除されます。ハンドルをコンポーネントの状態に保存することを確認してください。")]),
        ]),
        code_block(
            "rust",
            r#"fn create(ctx: &Context<Self>) -> Self {
    let listener = ctx.link()
        .add_location_listener(ctx.link().callback(
            // イベントを処理する
        ))
        .unwrap();
    MyComponent {
        _listener: listener
    }
}"#
        ),
        p(vec![
            code("ctx.link().location()"),
            text(" と "),
            code("ctx.link().route::<R>()"),
            text(" も、一度だけ位置とルートを取得するために使用できます。"),
        ]),
        h3(vec![text("クエリパラメータ")]),
        h4(vec![text("ナビゲーション時にクエリパラメータを指定する")]),
        p(vec![
            text("新しいルートにナビゲートする際にクエリパラメータを指定するには、"),
            code("navigator.push_with_query"),
            text(" または "),
            code("navigator.replace_with_query"),
            text(" 関数を使用します。これは "),
            code("serde"),
            text(" を使用してパラメータを URL のクエリ文字列にシリアル化するため、"),
            code("Serialize"),
            text(" を実装している任意の型を渡すことができます。最も簡単な形式は文字列ペアを含む "),
            code("HashMap"),
            text(" です。"),
        ]),
        h4(vec![text("現在のルートのクエリパラメータを取得する")]),
        p(vec![
            text("クエリパラメータを取得するには、"),
            code("location.query"),
            text(" を使用します。これは "),
            code("serde"),
            text(" を使用して URL のクエリ文字列からパラメータを逆シリアル化します。"),
        ]),
        h2_id("nested-router", vec![text("ネストされたルーター")]),
        p(vec![text("アプリケーションが大きくなると、ネストされたルーターが役立つ場合があります。次のルーター構造を考えてみましょう：")]),
        themed_img("/img/nested-router-light.svg", "/img/nested-router-dark.svg", "nested router structure"),
        p(vec![
            text("ネストされた "),
            code("SettingsRouter"),
            text(" は、すべての "),
            code("/settings"),
            text(" で始まる URL を処理します。また、一致しない URL をメインの "),
            code("NotFound"),
            text(" ルートにリダイレクトします。したがって、"),
            code("/settings/gibberish"),
            text(" は "),
            code("/404"),
            text(" にリダイレクトされます。"),
        ]),
        admonition(AdmonitionType::Caution, None, vec![
            p(vec![text("このインターフェースはまだ開発中であり、このように記述する方法は最終決定されていません。")]),
        ]),
        p(vec![text("以下のコードで実装できます：")]),
        code_block(
            "rust",
            r##"use yew::prelude::*;
use yew_router::prelude::*;
use gloo::utils::window;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone, Routable, PartialEq)]
enum MainRoute {
    #[at("/")]
    Home,
    #[at("/news")]
    News,
    #[at("/contact")]
    Contact,
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/*")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
enum SettingsRoute {
    #[at("/settings")]
    Profile,
    #[at("/settings/friends")]
    Friends,
    #[at("/settings/theme")]
    Theme,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<h1>{"Home"}</h1>},
        MainRoute::News => html! {<h1>{"News"}</h1>},
        MainRoute::Contact => html! {<h1>{"Contact"}</h1>},
        MainRoute::SettingsRoot | MainRoute::Settings => html! { <Switch<SettingsRoute> render={switch_settings} /> },
        MainRoute::NotFound => html! {<h1>{"Not Found"}</h1>},
    }
}

fn switch_settings(route: SettingsRoute) -> Html {
    match route {
        SettingsRoute::Profile => html! {<h1>{"Profile"}</h1>},
        SettingsRoute::Friends => html! {<h1>{"Friends"}</h1>},
        SettingsRoute::Theme => html! {<h1>{"Theme"}</h1>},
        SettingsRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::NotFound}/>}
    }
}

#[component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>
    }
}"##
        ),
        h3(vec![text("ベースパス (Basename)")]),
        p(vec![
            code("yew-router"),
            text(" を使用してベースパス (Basename) を定義できます。ベースパスはすべてのルートの共通プレフィックスです。ナビゲーター API と "),
            code("<Switch />"),
            text(" コンポーネントはどちらもベースパスの設定をサポートしています。プッシュされるすべてのルートにはベースパスのプレフィックスが追加され、すべてのスイッチはパスを "),
            code("Routable"),
            text(" に解析する前にベースパスを削除します。"),
        ]),
        p(vec![
            text("Router コンポーネントにベースパス属性が提供されていない場合、HTML ファイルの "),
            code("<base />"),
            text(" 要素の href 属性を使用し、HTML ファイルに "),
            code("<base />"),
            text(" 要素がない場合は "),
            code("/"),
            text(" にフォールバックします。"),
        ]),
        h2(vec![text("関連例")]),
        ul(vec![
            li(vec![link("https://github.com/yewstack/yew/tree/master/examples/router", vec![text("ルーター")])]),
        ]),
        h2(vec![text("インターフェースリファレンス")]),
        ul(vec![
            li(vec![link("https://docs.rs/yew-router/", vec![text("yew-router")])]),
        ]),
    ])
}

crate::doc_page!(
    "ルーター (Router)",
    "/ja/docs/concepts/router",
    page_content()
);
