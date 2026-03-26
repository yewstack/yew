pub fn page_content_versioned(version: Option<&str>) -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    let yew_dep_csr = match version {
        Some(v) => format!("yew = {{ version = \"{v}\", features = [\"csr\"] }}"),
        None => {
            "yew = { git = \"https://github.com/yewstack/yew/\", features = [\"csr\"] }".to_string()
        }
    };
    let yew_dep_csr_serde = match version {
        Some(v) => format!("yew = {{ version = \"{v}\", features = [\"csr\", \"serde\"] }}"),
        None => "yew = { git = \"https://github.com/yewstack/yew/\", features = [\"csr\", \
                 \"serde\"] }"
            .to_string(),
    };
    Content::new(vec![
        h2!["紹介"],
        p![
            "この実践チュートリアルでは、Yew を使用して Web アプリケーションを構築する方法を学びます。 ",
            bold!["Yew"],
            " は、",
            link!["https://webassembly.org/", "WebAssembly"],
            " を使用してフロントエンド Web アプリケーションを構築するためのモダンな ",
            link!["https://www.rust-lang.org/", "Rust"],
            " フレームワークです。 Yew は Rust の強力な型システムを活用し、再利用可能で保守しやすく、良好に構造化されたアーキテクチャを奨励します。 Rust の ",
            link!["https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html", "crates"],
            " と呼ばれるライブラリのエコシステムは、状態管理などの一般的なパターンのためのコンポーネントを提供します。 Rust のパッケージマネージャー ",
            link!["https://doc.rust-lang.org/cargo/", "Cargo"],
            " を使用すると、Yew などの多くの crate を ",
            link!["https://crates.io", "crates.io"],
            " から利用できます。",
        ],
        h3!["構築する内容"],
        p!["Rustconf は、Rust コミュニティが毎年開催する星間集会です。 Rustconf 2020 には多くの講演があり、大量の情報が提供されました。 この実践チュートリアルでは、他の Rustaceans がこれらの講演を理解し、1つのページから視聴できるようにする Web アプリケーションを構築します。"],
        h2!["セットアップ"],
        h3!["前提条件"],
        p![
            "このチュートリアルは、Rust に精通していることを前提としています。Rust の初心者である場合、無料の ",
            link!["https://doc.rust-lang.org/book/ch00-00-introduction.html", "Rust 本"],
            " は初心者にとって素晴らしい出発点であり、経験豊富な Rust 開発者にとっても優れたリソースです。",
        ],
        p![
            "最新バージョンの Rust がインストールされていることを確認するには、",
            code("rustup update"),
            " を実行するか、",
            link!["https://www.rust-lang.org/tools/install", "Rust をインストール"],
            " します。",
        ],
        p![
            "Rust をインストールした後、Cargo を使用して以下のコマンドを実行し、",
            code("trunk"),
            " をインストールします：",
        ],
        code_block("bash", "cargo install trunk"),
        p!["WASM のビルドターゲットも追加する必要があります。次のコマンドを実行します："],
        code_block("bash", "rustup target add wasm32-unknown-unknown"),
        h3!["プロジェクトの設定"],
        p!["まず、新しい cargo プロジェクトを作成します："],
        code_block("bash", r#"cargo new yew-app
cd yew-app"#),
        p!["Rust 環境が正しく設定されていることを確認するために、cargo ビルドツールを使用して初期プロジェクトを実行します。 ビルドプロセスの出力に続いて、期待される \"Hello, world!\" メッセージが表示されるはずです。"],
        code_block("bash", "cargo run"),
        h2!["最初の静的ページ"],
        p!["このシンプルなコマンドラインアプリケーションを基本的な Yew Web アプリケーションに変換するために、いくつかの変更が必要です。"],
        code_block_title("toml", "Cargo.toml", format!(
            "[package]\n\
             name = \"yew-app\"\n\
             version = \"0.1.0\"\n\
             edition = \"2021\"\n\
             \n\
             [dependencies]\n\
             // highlight-next-line\n\
             {yew_dep_csr}"
        )),
        admonition(AdmonitionType::Info, None, vec![
            p![
                "アプリケーションを構築するだけの場合は、",
                code("csr"),
                " 特性のみが必要です。これにより、",
                code("Renderer"),
                " とクライアントサイドレンダリングに関連するすべてのコードが有効になります。",
            ],
            p!["ライブラリを作成している場合は、この特性を有効にしないでください。クライアントサイドレンダリングロジックがサーバーサイドレンダリングパッケージに含まれてしまいます。"],
            p![
                "テストやサンプルのために Renderer が必要な場合は、",
                code("dev-dependencies"),
                " で有効にするべきです。",
            ],
        ]),
        code_block_title("rust", "src/main.rs", r##"use yew::prelude::*;

#[component]
fn App() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}"##),
        p![
            "それでは、プロジェクトのルートディレクトリに ",
            code("index.html"),
            " を作成しましょう。",
        ],
        code_block_title("html", "index.html", r#"<!doctype html>
<html lang="en">
    <head></head>
    <body></body>
</html>"#),
        h3!["開発サーバーの起動"],
        p!["以下のコマンドを実行して、アプリケーションをビルドし、ローカルで提供します。"],
        code_block("bash", "trunk serve --open"),
        admonition(AdmonitionType::Info, None, vec![
            p![
                code("--open"),
                " オプションを削除して、",
                code("trunk serve"),
                " を実行した後にデフォルトのブラウザを開かないようにします。",
            ],
        ]),
        p![
            "Trunk は、ソースコードファイルを変更するたびにアプリケーションをリアルタイムで再構築します。 デフォルトでは、サーバーはアドレス '127.0.0.1' のポート '8080' でリッスンします => ",
            link!["http://127.0.0.1:8080", "http://localhost:8080"],
            "。 この設定を変更するには、次のファイルを作成して必要に応じて編集します：",
        ],
        code_block_title("toml", "Trunk.toml", r##"[serve]
# ローカルネットワーク上のリッスンアドレス
address = "127.0.0.1"
# 広域ネットワーク上のリッスンアドレス
# address = "0.0.0.0"
# リッスンするポート
port = 8000"##),
        p![
            "もし興味があれば、",
            code("trunk help"),
            " および ",
            code("trunk help <subcommand>"),
            " を実行して、進行中のプロセスの詳細についてさらに学ぶことができます。",
        ],
        h3!["おめでとうございます"],
        p!["これで、Yew 開発環境を正常にセットアップし、最初の Yew Web アプリケーションを構築しました。"],
        h2!["HTML の構築"],
        p!["Yew は Rust のプロシージャルマクロを利用しており、JSX（JavaScript の拡張で、JavaScript 内で HTML に似たコードを書くことができる）に似た構文を提供して、マークアップを作成します。"],
        h3!["クラシック HTML への変換"],
        p![
            "私たちのウェブサイトがどのように見えるかについての良いアイデアが既にあるので、単純にドラフトを ",
            code("html!"),
            " と互換性のある表現に変換することができます。シンプルな HTML を書くことに慣れているなら、",
            code("html!"),
            " でマークアップを書くのに問題はないはずです。このマクロは HTML といくつかの違いがあることに注意してください：",
        ],
        ol![
            li![
                "式は中括弧（",
                code("{ }"),
                "）で囲む必要があります。",
            ],
            li![
                "ルートノードは1つだけでなければなりません。コンテナにラップせずに複数の要素を持ちたい場合は、空のタグ/フラグメント（",
                code("<> ... </>"),
                "）を使用できます。",
            ],
            li!["要素は正しく閉じる必要があります。"],
        ],
        p!["レイアウトを構築したいので、元の HTML は次のようになります："],
        code_block("html", r##"<h1>RustConf Explorer</h1>
<div>
    <h3>Videos to watch</h3>
    <p>John Doe: Building and breaking things</p>
    <p>Jane Smith: The development process</p>
    <p>Matt Miller: The Web 7.0</p>
    <p>Tom Jerry: Mouseless development</p>
</div>
<div>
    <h3>John Doe: Building and breaking things</h3>
    <img
        src="https://placehold.co/640x360.png?text=Video+Player+Placeholder"
        alt="video thumbnail"
    />
</div>"##),
        p![
            "それでは、この HTML を ",
            code("html!"),
            " に変換しましょう。次のコードスニペットを ",
            code("app"),
            " 関数の本体に入力（またはコピー/ペースト）して、関数が ",
            code("html!"),
            " の値を返すようにします。",
        ],
        code_block("rust", r##"#[component]
fn App() -> Html {
// highlight-start
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
                <p>{ "John Doe: Building and breaking things" }</p>
                <p>{ "Jane Smith: The development process" }</p>
                <p>{ "Matt Miller: The Web 7.0" }</p>
                <p>{ "Tom Jerry: Mouseless development" }</p>
            </div>
            <div>
                <h3>{ "John Doe: Building and breaking things" }</h3>
                <img src="https://placehold.co/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
            </div>
        </>
    }
// highlight-end
}"##),
        p!["ブラウザページをリフレッシュすると、次の出力が表示されるはずです："],
        img("/img/tutorial_application_screenshot.png", "Running WASM application screenshot"),
        h3!["マークアップ内でRustの構造を使用する"],
        p![
            "Rustでマークアップを書く大きな利点の1つは、マークアップ内でRustのすべての利点を享受できることです。 今では、HTML内にビデオリストをハードコーディングするのではなく、それらを ",
            code("Vec"),
            " の ",
            code("Video"),
            " 構造体として定義します。 データを保持するために、",
            code("main.rs"),
            " または選択した任意のファイルにシンプルな ",
            code("struct"),
            " を作成します。",
        ],
        code_block("rust", r##"#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: AttrValue,
    speaker: AttrValue,
    url: AttrValue,
}"##),
        p![
            "次に、この構造体のインスタンスを ",
            code("app"),
            " 関数内で作成し、ハードコーディングされたデータの代わりにそれらを使用します：",
        ],
        code_block("rust", r##"#[component]
fn App() -> Html {
// highlight-start
    let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".into(),
            speaker: "John Doe".into(),
            url: "https://youtu.be/PsaFVLr8t4E".into(),
        },
        Video {
            id: 2,
            title: "The development process".into(),
            speaker: "Jane Smith".into(),
            url: "https://youtu.be/PsaFVLr8t4E".into(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".into(),
            speaker: "Matt Miller".into(),
            url: "https://youtu.be/PsaFVLr8t4E".into(),
        },
        Video {
            id: 4,
            title: "Mouseless development".into(),
            speaker: "Tom Jerry".into(),
            url: "https://youtu.be/PsaFVLr8t4E".into(),
        },
    ];
// highlight-end"##),
        p![
            "それらを表示するために、",
            code("for"),
            " ループをマクロ内のハードコーディングされた HTML の代わりに使用できます：",
        ],
        code_block("rust", r#"    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
// highlight-start
                for video in &videos {
                    <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
                }
// highlight-end
            </div>
            // ...
        </>
    }"#),
        admonition(AdmonitionType::Tip, None, vec![
            p![
                "リスト項目にキーを使用することで、Yew はリスト内のどの項目が変更されたかを追跡し、より高速な再レンダリングを実現できます。",
                doc_link![crate::pages::concepts::html::lists, "リストには常にキーを使用することをお勧めします"],
                "。",
            ],
        ]),
        h2!["コンポーネント"],
        p!["コンポーネントは Yew アプリケーションの構成要素です。コンポーネントを組み合わせることで（他のコンポーネントで構成されることもあります）、アプリケーションを構築します。再利用可能性を考慮してコンポーネントを構築し、それらを汎用的に保つことで、コードやロジックを繰り返すことなく、アプリケーションの複数の部分でそれらを使用できるようになります。"],
        p![
            "これまで使用してきた ",
            code("app"),
            " 関数は ",
            code("App"),
            " と呼ばれるコンポーネントであり、\"関数コンポーネント\"と呼ばれます。",
        ],
        ol![
            li!["構造体コンポーネント"],
            li!["関数コンポーネント"],
        ],
        p!["このチュートリアルでは、関数コンポーネントを使用します。"],
        p![
            "では、",
            code("App"),
            " コンポーネントをより小さなコンポーネントに分割しましょう。まず、ビデオリストを独自のコンポーネントに抽出します。",
        ],
        code_block("rust", r##"#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
}

#[component]
fn VideosList(VideosListProps { videos }: &VideosListProps) -> Html {
    html! {
        for video in videos {
            <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }
}"##),
        p![
            code("VideosList"),
            " 関数コンポーネントのパラメータに注意してください。関数コンポーネントは1つの引数しか受け取らず、その引数は \"props\"（\"properties\" の略）を定義します。Props は親コンポーネントから子コンポーネントにデータを渡すために使用されます。この場合、",
            code("VideosListProps"),
            " は props を定義する構造体です。",
        ],
        admonition(AdmonitionType::Important, Some("重要"), vec![
            p![
                "props に使用される構造体は ",
                code("Properties"),
                " を派生実装する必要があります。",
            ],
        ]),
        p![
            "次に、",
            code("VideosList"),
            " コンポーネントを使用するように ",
            code("App"),
            " コンポーネントを更新できます。",
        ],
        code_block("rust", r#"#[component]
fn App() -> Html {
    // ...
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
// highlight-next-line
                <VideosList {videos} />
            </div>
            // ...
        </>
    }
}"#),
        p![
            "ブラウザウィンドウを確認することで、リストが期待通りにレンダリングされているかどうかを検証できます。リストのレンダリングロジックをそのコンポーネントに移動しました。これにより、",
            code("App"),
            " コンポーネントのソースコードが短くなり、読みやすく理解しやすくなりました。",
        ],
        h3!["アプリケーションをインタラクティブにする"],
        p![
            "ここでの最終目標は、選択したビデオを表示することです。そのためには、",
            code("VideosList"),
            " コンポーネントがビデオを選択したときに親コンポーネントに\"通知\"する必要があります。これは ",
            code("Callback"),
            " を使用して行います。この概念は\"ハンドラの伝播\"と呼ばれます。props を変更して ",
            code("on_click"),
            " コールバックを受け取るようにします：",
        ],
        code_block("rust", r##"#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
// highlight-next-line
    on_click: Callback<Video>,
}"##),
        p![
            "次に、選択したビデオをコールバックに\"emit\"するように ",
            code("VideosList"),
            " コンポーネントを変更します。",
        ],
        code_block("rust", r##"#[component]
// highlight-start
fn VideosList(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_select = |video: &Video| {
        let on_click = on_click.clone();
        let video = video.clone();
        Callback::from(move |_| {
            on_click.emit(video.clone())
        })
    };

    html! {
        for video in videos {
            <p key={video.id} onclick={on_select(video)}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }
// highlight-end
}"##),
        p![
            "次に、",
            code("VideosList"),
            " の使用を変更してそのコールバックを渡す必要があります。しかし、その前に、新しいコンポーネント ",
            code("VideoDetails"),
            " を作成し、ビデオがクリックされたときに表示されるようにします。",
        ],
        code_block("rust", r##"#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[component]
fn VideoDetails(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ &*video.title }</h3>
            <img src="https://placehold.co/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}"##),
        p![
            "次に、",
            code("App"),
            " コンポーネントを変更して、ビデオが選択されたときに ",
            code("VideoDetails"),
            " コンポーネントを表示するようにします。",
        ],
        code_block("rust", r#"// highlight-start
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };
// highlight-end

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
// highlight-next-line
                <VideosList {videos} on_click={on_video_select} />
            </div>
// highlight-start
            if let Some(video) = &*selected_video {
                <VideoDetails video={video.clone()} />
            }
// highlight-end
        </>
    }"#),
        p![
            "今は ",
            code("use_state"),
            " について心配する必要はありません。後でこの問題に戻ります。",
            code("if let Some(video) = &*selected_video"),
            " のパターンに注目してください。",
            code("selected_video"),
            " の状態が ",
            code("Some"),
            " の場合にのみ ",
            code("VideoDetails"),
            " コンポーネントを表示します。この条件付きレンダリングは ",
            code("html!"),
            " マクロ内で直接サポートされています。",
        ],
        h3!["状態の処理"],
        p![
            "以前に使用した ",
            code("use_state"),
            " を覚えていますか？それは \"フック\" と呼ばれる特殊な関数です。フックは関数コンポーネントのライフサイクルに \"フック\" して操作を実行するために使用されます。このフックや他のフックについては",
            doc_link![crate::pages::concepts::function_components::hooks::introduction, "こちら"],
            "で詳しく学ぶことができます。",
        ],
        admonition(AdmonitionType::Note, None, vec![
            p![
                "構造体コンポーネントは異なる動作をします。これらについては",
                doc_link![crate::pages::advanced_topics::struct_components::introduction, "ドキュメント"],
                "を参照してください。",
            ],
        ]),
        h2!["データの取得（外部 REST API の使用）"],
        p!["実際のアプリケーションでは、データは通常ハードコーディングされているのではなく、API から取得されます。外部ソースからビデオリストを取得してみましょう。そのためには、以下のクレートを追加する必要があります："],
        ul![
            li![
                link!["https://crates.io/crates/gloo-net", code("gloo-net")],
                " - fetch 呼び出しを行うために使用します。",
            ],
            li![
                link!["https://serde.rs", code("serde")],
                " とその派生特性 - JSON 応答をデシリアライズするために使用します。",
            ],
            li![
                link!["https://crates.io/crates/wasm-bindgen-futures", code("wasm-bindgen-futures")],
                " - Rust の Future を Promise として実行するために使用します。",
            ],
        ],
        p![
            code("Cargo.toml"),
            " ファイルの依存関係を更新しましょう：",
        ],
        code_block_title("toml", "Cargo.toml", format!(
            "[dependencies]\n\
             // highlight-start\n\
             {yew_dep_csr_serde}\n\
             gloo-net = \"0.6\"\n\
             serde = {{ version = \"1.0\", features = [\"derive\"] }}\n\
             wasm-bindgen-futures = \"0.4\"\n\
             // highlight-end"
        )),
        admonition(AdmonitionType::Note, None, vec![
            p![
                "依存関係を選択する際には、それらが ",
                code("wasm32"),
                " と互換性があることを確認してください！そうでない場合、アプリケーションを実行することはできません。",
            ],
        ]),
        p![
            code("Deserialize"),
            " 特性を派生するように ",
            code("Video"),
            " 構造体を更新します：",
        ],
        code_block("rust", r##"use yew::prelude::*;
// highlight-next-line
use serde::Deserialize;

// highlight-next-line
#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: AttrValue,
    speaker: AttrValue,
    url: AttrValue,
}"##),
        p![
            "最後のステップとして、ハードコーディングされたデータを使用するのではなく、fetch リクエストを行うように ",
            code("App"),
            " コンポーネントを更新する必要があります。",
        ],
        code_block("rust", r##"use yew::prelude::*;
// highlight-next-line
use gloo_net::http::Request;

#[component]
fn App() -> Html {
// highlight-start
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
        });
    }
// highlight-end

    // ...

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
// highlight-next-line
                <VideosList videos={(*videos).clone()} on_click={on_video_select} />
            </div>
            // ...
        </>
    }
}"##),
        admonition(AdmonitionType::Note, None, vec![
            p![
                "ここでは ",
                code("unwrap"),
                " を使用していますが、これはデモアプリケーションのためです。実際のアプリケーションでは、",
                link!["https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html", "適切なエラーハンドリング"],
                "を行うことをお勧めします。",
            ],
        ]),
        p!["さて、ブラウザを確認して、すべてが期待通りに動作しているかを確認しましょう......CORS の問題がなければ。これを解決するために、プロキシサーバーが必要です。幸いなことに、trunk はこの機能を提供しています。"],
        p!["以下の行を更新します："],
        code_block("rust", r##"// highlight-next-line
let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")"##),
        p!["次に、以下のコマンドを使用してサーバーを再起動します："],
        code_block("bash", "trunk serve --proxy-backend=https://yew.rs/tutorial"),
        p!["ページをリフレッシュすると、すべてが期待通りに動作するはずです。"],
        h2!["まとめ"],
        p!["おめでとうございます！外部 API からデータを取得し、ビデオリストを表示する Web アプリケーションを作成しました。"],
        h2!["次に"],
        p!["このアプリケーションは、完璧または有用になるまでにはまだ長い道のりがあります。このチュートリアルを完了した後、より高度なトピックを探求するための出発点として使用できます。"],
        h3!["スタイル"],
        p![
            "私たちのアプリケーションは非常に見栄えが悪いです。CSS やその他のスタイルがありません。残念ながら、Yew は組み込みのスタイルコンポーネントを提供していません。スタイルシートを追加する方法については、",
            link!["https://trunkrs.dev/assets/", "Trunk のアセット"],
            "を参照してください。",
        ],
        h3!["さらなる依存ライブラリ"],
        p![
            "私たちのアプリケーションは、非常に少ない外部依存関係を使用しています。使用できる多くのクレートがあります。詳細については、",
            link!["/community/external-libs", "外部ライブラリ"],
            "を参照してください。",
        ],
        h3!["Yew についてもっと知る"],
        p![
            "私たちの",
            doc_link![crate::pages::getting_started::introduction, "公式ドキュメント"],
            "を読んでください。多くの概念についてより詳細に説明しています。Yew API についてもっと知りたい場合は、",
            link!["https://docs.rs/yew", "API ドキュメント"],
            "を参照してください。",
        ],
    ])
}

pub fn page_content() -> yew_site_lib::Content {
    page_content_versioned(None)
}

crate::doc_page!("チュートリアル", "/ja/docs/tutorial", page_content());
