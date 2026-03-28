pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p!["シングルページアプリケーション (SPA) のルーターは、URL に基づいて異なるページを表示する処理を行います。リンクをクリックしたときに異なるリモートリソースを要求するデフォルトの動作とは異なり、ルーターはアプリケーション内の有効なルートを指すようにローカルで URL を設定します。その後、ルーターはこの変更を検出し、レンダリングする内容を決定します。"],
        p![
            "Yew は ",
            code("yew-router"),
            " クレートでルーターサポートを提供します。使用を開始するには、依存関係を ",
            code("Cargo.toml"),
            " ファイルに追加してください。",
        ],
        code_block("toml", "yew-router = { git = \"https://github.com/yewstack/yew.git\" }"),
        p![
            "必要なツールはすべて ",
            code("yew_router::prelude"),
            " モジュールで提供されています。",
        ],
        h2!["使用方法"],
        p![
            "まず、",
            code("Route"),
            " を定義する必要があります。",
        ],
        p![
            "ルートは ",
            code("Routable"),
            " を派生する ",
            code("enum"),
            " で定義されます。この列挙型は ",
            code("Clone + PartialEq"),
            " を実装する必要があります。",
        ],
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
        p![
            code("Route"),
            " と ",
            code("<Switch />"),
            " コンポーネントはペアで使用され、後者はブラウザの現在の URL に一致するパスのバリアントを見つけ、それを ",
            code("render"),
            " コールバックに渡します。その後、コールバックがレンダリングする内容を決定します。パスが一致しない場合、ルーターは ",
            code("not_found"),
            " 属性を持つパスにナビゲートします。指定されたルートがない場合、何もレンダリングされず、一致するルートがないことを示すメッセージがコンソールに記録されます。",
        ],
        p![
            "yew-router のほとんどのコンポーネント、特に ",
            code("<Link />"),
            " と ",
            code("<Switch />"),
            " は、ある Router コンポーネント（例： ",
            code("<BrowserRouter />"),
            "）の（深い）子要素である必要があります。通常、アプリケーションには 1 つの Router しか必要なく、通常は最上位の ",
            code("<App />"),
            " コンポーネントによって直ちにレンダリングされます。Router はコンテキストを登録し、これは Links と Switches の機能に必要です。以下に例を示します。",
        ],
        admonition![AdmonitionType::Caution, None,
            p![
                "ブラウザ環境で ",
                code("yew-router"),
                " を使用する場合、",
                code("<BrowserRouter />"),
                " を強く推奨します。他のルータータイプについては ",
                link!("https://docs.rs/yew-router/", "API リファレンス"),
                " を参照してください。",
            ],
        ],
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
        h3!["パスセグメント"],
        p![
            "ルーターは、動的および名前付きワイルドカードセグメントを使用してルートから情報を抽出することもできます。次に、",
            code("<Switch />"),
            " 内で投稿の ID にアクセスし、それを適切なコンポーネントにプロパティとして渡すことができます。",
        ],
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
        admonition![AdmonitionType::Note, None,
            p![
                code("Post {id: String}"),
                " の代わりに通常の ",
                code("Post"),
                " バリアントを使用することもできます。例えば、",
                code("Post"),
                " が別のルーターと一緒にレンダリングされる場合、そのフィールドは冗長になる可能性があります。詳細については、以下の",
                link!("#nested-router", "ネストされたルーター"),
                "セクションを参照してください。",
            ],
        ],
        p![
            "フィールドは ",
            code("Route"),
            " 列挙型の一部として ",
            code("Clone + PartialEq"),
            " を実装する必要があることに注意してください。また、シリアル化と逆シリアル化のために ",
            code("std::fmt::Display"),
            " と ",
            code("std::str::FromStr"),
            " を実装する必要があります。整数、浮動小数点数、および文字列などのプリミティブ型はこれらの要件を既に満たしています。",
        ],
        p![
            "パスの形式が一致しても、逆シリアル化が失敗した場合（",
            code("FromStr"),
            " に基づく）、ルーターはルートが一致しないと見なし、見つからないルートをレンダリングしようとします（または、見つからないルートが指定されていない場合は空白ページをレンダリングします）。",
        ],
        p!["以下の例を参照してください："],
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
        p![
            "セグメントが 255 を超えると、",
            code("u8::from_str()"),
            " は失敗し、",
            code("ParseIntError"),
            " を返します。この場合、ルーターはルートが一致しないと見なします。",
        ],
        img(
            "/img/router-deserialization-failure-behavior.gif",
            "ルーターの逆シリアル化失敗の動作",
        ),
        p![
            "ルーティング構文やパラメータのバインディング方法の詳細については、",
            link!(
                "https://docs.rs/route-recognizer/0.3.1/route_recognizer/#routing-params",
                "route-recognizer",
            ),
            " を参照してください。",
        ],
        h3!["位置 (Location)"],
        p![
            "ルーターはコンテキストを介して一般的な ",
            code("Location"),
            " 構造体を提供し、ルート情報にアクセスするために使用できます。これらはフックまたは ",
            code("ctx.link()"),
            " 上の便利な関数を介して取得できます。",
        ],
        h3!["ナビゲーション"],
        p![
            code("yew_router"),
            " はナビゲーションを処理するためのいくつかのツールを提供します。",
        ],
        h4!["リンク"],
        p![
            code("<Link />"),
            " は ",
            code("<a>"),
            " 要素としてレンダリングされ、",
            code("onclick"),
            " イベントハンドラは ",
            link!("https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault", "preventDefault"),
            " を呼び出し、ターゲットページを履歴にプッシュして必要なページをレンダリングします。これはシングルページアプリケーションに期待される動作です。通常のアンカー要素のデフォルトの ",
            code("onclick"),
            " はページをリロードします。",
        ],
        p![
            code("<Link />"),
            " コンポーネントはその子要素を ",
            code("<a>"),
            " 要素に渡します。これはアプリ内ルーティングのための ",
            code("<a/>"),
            " の代替として考えることができます。違いは、",
            code("href"),
            " の代わりに ",
            code("to"),
            " 属性を提供する必要があることです。使用例は以下の通りです：",
        ],
        code_block(
            "rust",
            r#"<Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>"#,
        ),
        p!["構造体変数も正常に動作します："],
        code_block(
            "rust",
            r#"<Link<Route> to={Route::Post { id: "new-yew-release".to_string() }}>{ "Yew!" }</Link<Route>>"#,
        ),
        h4!["ナビゲーションインターフェース"],
        p![
            "ナビゲーター API は、関数コンポーネントと構造体コンポーネントの両方で提供されます。これにより、コールバックがルートを変更できるようになります。どちらの場合でも、",
            code("Navigator"),
            " インスタンスを取得してルートを操作できます。",
        ],
        h5!["関数コンポーネント"],
        p![
            "関数コンポーネントの場合、基礎となるナビゲータープロバイダーが変更されると、",
            code("use_navigator"),
            " フックはコンポーネントを再レンダリングします。以下は、クリック時に ",
            code("Home"),
            " ルートにナビゲートするボタンを実装する例です。",
        ],
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
        admonition![AdmonitionType::Caution, None,
            p![
                "ここでの例では ",
                code("Callback::from"),
                " を使用しています。ターゲットルートがコンポーネントのルートと同じになる可能性がある場合、または安全のために、通常のコールバックを使用してください。例えば、各ページにロゴボタンがあり、そのボタンをクリックするとホームに戻るとします。ホームページでそのボタンを2回クリックすると、同じHomeルートがプッシュされ、",
                code("use_navigator"),
                " フックが再レンダリングをトリガーしないため、コードがクラッシュします。",
            ],
        ],
        p![
            "現在の位置をスタックに新しい位置としてプッシュするのではなく置き換えたい場合は、",
            code("navigator.push()"),
            " の代わりに ",
            code("navigator.replace()"),
            " を使用してください。",
        ],
        p![
            code("navigator"),
            " はコールバックに移動する必要があるため、他のコールバックで再利用できないことに気付くかもしれません。幸いなことに、",
            code("navigator"),
            " は ",
            code("Clone"),
            " を実装しているため、異なるルートに対して複数のボタンを設定する方法は次のとおりです：",
        ],
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
        h5!["構造体コンポーネント"],
        p![
            "構造体コンポーネントの場合、",
            code("ctx.link().navigator()"),
            " API を使用して ",
            code("Navigator"),
            " インスタンスを取得できます。残りの部分は関数コンポーネントの場合と同じです。以下は、単一のボタンをレンダリングするビュー関数の例です。",
        ],
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
        h4!["リダイレクト"],
        p![
            code("yew-router"),
            " は prelude に ",
            code("<Redirect />"),
            " コンポーネントも提供しています。これはナビゲーター API と同様の効果を実現するために使用できます。このコンポーネントは、ターゲットルートとして ",
            code("to"),
            " 属性を受け取ります。",
            code("<Redirect/>"),
            " がレンダリングされると、ユーザーは指定されたルートにリダイレクトされます。以下はその例です：",
        ],
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
        admonition![AdmonitionType::Tip, Some("`Redirect` と `Navigator` の選択方法"),
            p![
                "Navigator API はコールバック内でルートを操作する唯一の方法です。一方、",
                code("<Redirect />"),
                " はコンポーネント内の戻り値として使用できます。また、",
                link!("#nested-router", "ネストされたルーター"),
                "の switch 関数など、他の非コンポーネントコンテキストでも ",
                code("<Redirect />"),
                " を使用することができます。",
            ],
        ],
        h3!["変更のリスニング"],
        h4!["関数コンポーネント"],
        p![
            code("use_location"),
            " と ",
            code("use_route"),
            " フックを使用できます。提供された値が変更されると、コンポーネントが再レンダリングされます。",
        ],
        h4!["構造体コンポーネント"],
        p![
            "ルートの変更に応答するために、",
            code("ctx.link()"),
            " の ",
            code("add_location_listener()"),
            " メソッドにコールバッククロージャを渡すことができます。",
        ],
        admonition![AdmonitionType::Note, None,
            p!["位置リスナーが削除されると、それは登録解除されます。ハンドルをコンポーネントの状態に保存することを確認してください。"],
        ],
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
        p![
            code("ctx.link().location()"),
            " と ",
            code("ctx.link().route::<R>()"),
            " も、一度だけ位置とルートを取得するために使用できます。",
        ],
        h3!["クエリパラメータ"],
        h4!["ナビゲーション時にクエリパラメータを指定する"],
        p![
            "新しいルートにナビゲートする際にクエリパラメータを指定するには、",
            code("navigator.push_with_query"),
            " または ",
            code("navigator.replace_with_query"),
            " 関数を使用します。これは ",
            code("serde"),
            " を使用してパラメータを URL のクエリ文字列にシリアル化するため、",
            code("Serialize"),
            " を実装している任意の型を渡すことができます。最も簡単な形式は文字列ペアを含む ",
            code("HashMap"),
            " です。",
        ],
        h4!["現在のルートのクエリパラメータを取得する"],
        p![
            "クエリパラメータを取得するには、",
            code("location.query"),
            " を使用します。これは ",
            code("serde"),
            " を使用して URL のクエリ文字列からパラメータを逆シリアル化します。",
        ],
        h2_id!["nested-router", "ネストされたルーター"],
        p!["アプリケーションが大きくなると、ネストされたルーターが役立つ場合があります。次のルーター構造を考えてみましょう："],
        themed_img("/img/nested-router-light.svg", "/img/nested-router-dark.svg", "nested router structure"),
        p![
            "ネストされた ",
            code("SettingsRouter"),
            " は、すべての ",
            code("/settings"),
            " で始まる URL を処理します。また、一致しない URL をメインの ",
            code("NotFound"),
            " ルートにリダイレクトします。したがって、",
            code("/settings/gibberish"),
            " は ",
            code("/404"),
            " にリダイレクトされます。",
        ],
        admonition![AdmonitionType::Caution, None,
            p!["このインターフェースはまだ開発中であり、このように記述する方法は最終決定されていません。"],
        ],
        p!["以下のコードで実装できます："],
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
        h3!["ベースパス (Basename)"],
        p![
            code("yew-router"),
            " を使用してベースパス (Basename) を定義できます。ベースパスはすべてのルートの共通プレフィックスです。ナビゲーター API と ",
            code("<Switch />"),
            " コンポーネントはどちらもベースパスの設定をサポートしています。プッシュされるすべてのルートにはベースパスのプレフィックスが追加され、すべてのスイッチはパスを ",
            code("Routable"),
            " に解析する前にベースパスを削除します。",
        ],
        p![
            "Router コンポーネントにベースパス属性が提供されていない場合、HTML ファイルの ",
            code("<base />"),
            " 要素の href 属性を使用し、HTML ファイルに ",
            code("<base />"),
            " 要素がない場合は ",
            code("/"),
            " にフォールバックします。",
        ],
        h2!["関連例"],
        ul![
            li![link!("https://github.com/yewstack/yew/tree/master/examples/router", "ルーター")],
        ],
        h2!["インターフェースリファレンス"],
        ul![
            li![link!("https://docs.rs/yew-router/", "yew-router")],
        ],
    ])
    .with_description("Yew's official router")
}

crate::doc_page!(
    "ルーター (Router)",
    "/ja/docs/concepts/router",
    page_content()
);
