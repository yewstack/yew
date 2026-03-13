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
        h2(vec![text("紹介")]),
        p(vec![
            text("この実践チュートリアルでは、Yew を使用して Web アプリケーションを構築する方法を学びます。 "),
            bold(vec![text("Yew")]),
            text(" は、"),
            link("https://webassembly.org/", vec![text("WebAssembly")]),
            text(" を使用してフロントエンド Web アプリケーションを構築するためのモダンな "),
            link("https://www.rust-lang.org/", vec![text("Rust")]),
            text(" フレームワークです。 Yew は Rust の強力な型システムを活用し、再利用可能で保守しやすく、良好に構造化されたアーキテクチャを奨励します。 Rust の "),
            link("https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html", vec![text("crates")]),
            text(" と呼ばれるライブラリのエコシステムは、状態管理などの一般的なパターンのためのコンポーネントを提供します。 Rust のパッケージマネージャー "),
            link("https://doc.rust-lang.org/cargo/", vec![text("Cargo")]),
            text(" を使用すると、Yew などの多くの crate を "),
            link("https://crates.io", vec![text("crates.io")]),
            text(" から利用できます。"),
        ]),
        h3(vec![text("構築する内容")]),
        p(vec![text("Rustconf は、Rust コミュニティが毎年開催する星間集会です。 Rustconf 2020 には多くの講演があり、大量の情報が提供されました。 この実践チュートリアルでは、他の Rustaceans がこれらの講演を理解し、1つのページから視聴できるようにする Web アプリケーションを構築します。")]),
        h2(vec![text("セットアップ")]),
        h3(vec![text("前提条件")]),
        p(vec![
            text("このチュートリアルは、Rust に精通していることを前提としています。Rust の初心者である場合、無料の "),
            link("https://doc.rust-lang.org/book/ch00-00-introduction.html", vec![text("Rust 本")]),
            text(" は初心者にとって素晴らしい出発点であり、経験豊富な Rust 開発者にとっても優れたリソースです。"),
        ]),
        p(vec![
            text("最新バージョンの Rust がインストールされていることを確認するには、"),
            code("rustup update"),
            text(" を実行するか、"),
            link("https://www.rust-lang.org/tools/install", vec![text("Rust をインストール")]),
            text(" します。"),
        ]),
        p(vec![
            text("Rust をインストールした後、Cargo を使用して以下のコマンドを実行し、"),
            code("trunk"),
            text(" をインストールします："),
        ]),
        code_block("bash", "cargo install trunk"),
        p(vec![text("WASM のビルドターゲットも追加する必要があります。次のコマンドを実行します：")]),
        code_block("bash", "rustup target add wasm32-unknown-unknown"),
        h3(vec![text("プロジェクトの設定")]),
        p(vec![text("まず、新しい cargo プロジェクトを作成します：")]),
        code_block("bash", r#"cargo new yew-app
cd yew-app"#),
        p(vec![text("Rust 環境が正しく設定されていることを確認するために、cargo ビルドツールを使用して初期プロジェクトを実行します。 ビルドプロセスの出力に続いて、期待される \"Hello, world!\" メッセージが表示されるはずです。")]),
        code_block("bash", "cargo run"),
        h2(vec![text("最初の静的ページ")]),
        p(vec![text("このシンプルなコマンドラインアプリケーションを基本的な Yew Web アプリケーションに変換するために、いくつかの変更が必要です。")]),
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
            p(vec![
                text("アプリケーションを構築するだけの場合は、"),
                code("csr"),
                text(" 特性のみが必要です。これにより、"),
                code("Renderer"),
                text(" とクライアントサイドレンダリングに関連するすべてのコードが有効になります。"),
            ]),
            p(vec![text("ライブラリを作成している場合は、この特性を有効にしないでください。クライアントサイドレンダリングロジックがサーバーサイドレンダリングパッケージに含まれてしまいます。")]),
            p(vec![
                text("テストやサンプルのために Renderer が必要な場合は、"),
                code("dev-dependencies"),
                text(" で有効にするべきです。"),
            ]),
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
        p(vec![
            text("それでは、プロジェクトのルートディレクトリに "),
            code("index.html"),
            text(" を作成しましょう。"),
        ]),
        code_block_title("html", "index.html", r#"<!doctype html>
<html lang="en">
    <head></head>
    <body></body>
</html>"#),
        h3(vec![text("開発サーバーの起動")]),
        p(vec![text("以下のコマンドを実行して、アプリケーションをビルドし、ローカルで提供します。")]),
        code_block("bash", "trunk serve --open"),
        admonition(AdmonitionType::Info, None, vec![
            p(vec![
                code("--open"),
                text(" オプションを削除して、"),
                code("trunk serve"),
                text(" を実行した後にデフォルトのブラウザを開かないようにします。"),
            ]),
        ]),
        p(vec![
            text("Trunk は、ソースコードファイルを変更するたびにアプリケーションをリアルタイムで再構築します。 デフォルトでは、サーバーはアドレス '127.0.0.1' のポート '8080' でリッスンします => "),
            link("http://127.0.0.1:8080", vec![text("http://localhost:8080")]),
            text("。 この設定を変更するには、次のファイルを作成して必要に応じて編集します："),
        ]),
        code_block_title("toml", "Trunk.toml", r##"[serve]
# ローカルネットワーク上のリッスンアドレス
address = "127.0.0.1"
# 広域ネットワーク上のリッスンアドレス
# address = "0.0.0.0"
# リッスンするポート
port = 8000"##),
        p(vec![
            text("もし興味があれば、"),
            code("trunk help"),
            text(" および "),
            code("trunk help <subcommand>"),
            text(" を実行して、進行中のプロセスの詳細についてさらに学ぶことができます。"),
        ]),
        h3(vec![text("おめでとうございます")]),
        p(vec![text("これで、Yew 開発環境を正常にセットアップし、最初の Yew Web アプリケーションを構築しました。")]),
        h2(vec![text("HTML の構築")]),
        p(vec![text("Yew は Rust のプロシージャルマクロを利用しており、JSX（JavaScript の拡張で、JavaScript 内で HTML に似たコードを書くことができる）に似た構文を提供して、マークアップを作成します。")]),
        h3(vec![text("クラシック HTML への変換")]),
        p(vec![
            text("私たちのウェブサイトがどのように見えるかについての良いアイデアが既にあるので、単純にドラフトを "),
            code("html!"),
            text(" と互換性のある表現に変換することができます。シンプルな HTML を書くことに慣れているなら、"),
            code("html!"),
            text(" でマークアップを書くのに問題はないはずです。このマクロは HTML といくつかの違いがあることに注意してください："),
        ]),
        ol(vec![
            li(vec![
                text("式は中括弧（"),
                code("{ }"),
                text("）で囲む必要があります。"),
            ]),
            li(vec![
                text("ルートノードは1つだけでなければなりません。コンテナにラップせずに複数の要素を持ちたい場合は、空のタグ/フラグメント（"),
                code("<> ... </>"),
                text("）を使用できます。"),
            ]),
            li(vec![text("要素は正しく閉じる必要があります。")]),
        ]),
        p(vec![text("レイアウトを構築したいので、元の HTML は次のようになります：")]),
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
        p(vec![
            text("それでは、この HTML を "),
            code("html!"),
            text(" に変換しましょう。次のコードスニペットを "),
            code("app"),
            text(" 関数の本体に入力（またはコピー/ペースト）して、関数が "),
            code("html!"),
            text(" の値を返すようにします。"),
        ]),
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
        p(vec![text("ブラウザページをリフレッシュすると、次の出力が表示されるはずです：")]),
        img("/img/tutorial_application_screenshot.png", "Running WASM application screenshot"),
        h3(vec![text("マークアップ内でRustの構造を使用する")]),
        p(vec![
            text("Rustでマークアップを書く大きな利点の1つは、マークアップ内でRustのすべての利点を享受できることです。 今では、HTML内にビデオリストをハードコーディングするのではなく、それらを "),
            code("Vec"),
            text(" の "),
            code("Video"),
            text(" 構造体として定義します。 データを保持するために、"),
            code("main.rs"),
            text(" または選択した任意のファイルにシンプルな "),
            code("struct"),
            text(" を作成します。"),
        ]),
        code_block("rust", r##"#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: AttrValue,
    speaker: AttrValue,
    url: AttrValue,
}"##),
        p(vec![
            text("次に、この構造体のインスタンスを "),
            code("app"),
            text(" 関数内で作成し、ハードコーディングされたデータの代わりにそれらを使用します："),
        ]),
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
        p(vec![
            text("それらを表示するために、"),
            code("for"),
            text(" ループをマクロ内のハードコーディングされた HTML の代わりに使用できます："),
        ]),
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
            p(vec![
                text("リスト項目にキーを使用することで、Yew はリスト内のどの項目が変更されたかを追跡し、より高速な再レンダリングを実現できます。"),
                link("/ja/docs/concepts/html/lists", vec![text("リストには常にキーを使用することをお勧めします")]),
                text("。"),
            ]),
        ]),
        h2(vec![text("コンポーネント")]),
        p(vec![text("コンポーネントは Yew アプリケーションの構成要素です。コンポーネントを組み合わせることで（他のコンポーネントで構成されることもあります）、アプリケーションを構築します。再利用可能性を考慮してコンポーネントを構築し、それらを汎用的に保つことで、コードやロジックを繰り返すことなく、アプリケーションの複数の部分でそれらを使用できるようになります。")]),
        p(vec![
            text("これまで使用してきた "),
            code("app"),
            text(" 関数は "),
            code("App"),
            text(" と呼ばれるコンポーネントであり、\"関数コンポーネント\"と呼ばれます。"),
        ]),
        ol(vec![
            li(vec![text("構造体コンポーネント")]),
            li(vec![text("関数コンポーネント")]),
        ]),
        p(vec![text("このチュートリアルでは、関数コンポーネントを使用します。")]),
        p(vec![
            text("では、"),
            code("App"),
            text(" コンポーネントをより小さなコンポーネントに分割しましょう。まず、ビデオリストを独自のコンポーネントに抽出します。"),
        ]),
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
        p(vec![
            code("VideosList"),
            text(" 関数コンポーネントのパラメータに注意してください。関数コンポーネントは1つの引数しか受け取らず、その引数は \"props\"（\"properties\" の略）を定義します。Props は親コンポーネントから子コンポーネントにデータを渡すために使用されます。この場合、"),
            code("VideosListProps"),
            text(" は props を定義する構造体です。"),
        ]),
        admonition(AdmonitionType::Important, Some("重要"), vec![
            p(vec![
                text("props に使用される構造体は "),
                code("Properties"),
                text(" を派生実装する必要があります。"),
            ]),
        ]),
        p(vec![
            text("次に、"),
            code("VideosList"),
            text(" コンポーネントを使用するように "),
            code("App"),
            text(" コンポーネントを更新できます。"),
        ]),
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
        p(vec![
            text("ブラウザウィンドウを確認することで、リストが期待通りにレンダリングされているかどうかを検証できます。リストのレンダリングロジックをそのコンポーネントに移動しました。これにより、"),
            code("App"),
            text(" コンポーネントのソースコードが短くなり、読みやすく理解しやすくなりました。"),
        ]),
        h3(vec![text("アプリケーションをインタラクティブにする")]),
        p(vec![
            text("ここでの最終目標は、選択したビデオを表示することです。そのためには、"),
            code("VideosList"),
            text(" コンポーネントがビデオを選択したときに親コンポーネントに\"通知\"する必要があります。これは "),
            code("Callback"),
            text(" を使用して行います。この概念は\"ハンドラの伝播\"と呼ばれます。props を変更して "),
            code("on_click"),
            text(" コールバックを受け取るようにします："),
        ]),
        code_block("rust", r##"#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
// highlight-next-line
    on_click: Callback<Video>,
}"##),
        p(vec![
            text("次に、選択したビデオをコールバックに\"emit\"するように "),
            code("VideosList"),
            text(" コンポーネントを変更します。"),
        ]),
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
        p(vec![
            text("次に、"),
            code("VideosList"),
            text(" の使用を変更してそのコールバックを渡す必要があります。しかし、その前に、新しいコンポーネント "),
            code("VideoDetails"),
            text(" を作成し、ビデオがクリックされたときに表示されるようにします。"),
        ]),
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
        p(vec![
            text("次に、"),
            code("App"),
            text(" コンポーネントを変更して、ビデオが選択されたときに "),
            code("VideoDetails"),
            text(" コンポーネントを表示するようにします。"),
        ]),
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
        p(vec![
            text("今は "),
            code("use_state"),
            text(" について心配する必要はありません。後でこの問題に戻ります。"),
            code("if let Some(video) = &*selected_video"),
            text(" のパターンに注目してください。"),
            code("selected_video"),
            text(" の状態が "),
            code("Some"),
            text(" の場合にのみ "),
            code("VideoDetails"),
            text(" コンポーネントを表示します。この条件付きレンダリングは "),
            code("html!"),
            text(" マクロ内で直接サポートされています。"),
        ]),
        h3(vec![text("状態の処理")]),
        p(vec![
            text("以前に使用した "),
            code("use_state"),
            text(" を覚えていますか？それは \"フック\" と呼ばれる特殊な関数です。フックは関数コンポーネントのライフサイクルに \"フック\" して操作を実行するために使用されます。このフックや他のフックについては"),
            link("/ja/docs/concepts/function-components/hooks", vec![text("こちら")]),
            text("で詳しく学ぶことができます。"),
        ]),
        admonition(AdmonitionType::Note, None, vec![
            p(vec![
                text("構造体コンポーネントは異なる動作をします。これらについては"),
                link("/ja/docs/advanced-topics/struct-components", vec![text("ドキュメント")]),
                text("を参照してください。"),
            ]),
        ]),
        h2(vec![text("データの取得（外部 REST API の使用）")]),
        p(vec![text("実際のアプリケーションでは、データは通常ハードコーディングされているのではなく、API から取得されます。外部ソースからビデオリストを取得してみましょう。そのためには、以下のクレートを追加する必要があります：")]),
        ul(vec![
            li(vec![
                link("https://crates.io/crates/gloo-net", vec![code("gloo-net")]),
                text(" - fetch 呼び出しを行うために使用します。"),
            ]),
            li(vec![
                link("https://serde.rs", vec![code("serde")]),
                text(" とその派生特性 - JSON 応答をデシリアライズするために使用します。"),
            ]),
            li(vec![
                link("https://crates.io/crates/wasm-bindgen-futures", vec![code("wasm-bindgen-futures")]),
                text(" - Rust の Future を Promise として実行するために使用します。"),
            ]),
        ]),
        p(vec![
            code("Cargo.toml"),
            text(" ファイルの依存関係を更新しましょう："),
        ]),
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
            p(vec![
                text("依存関係を選択する際には、それらが "),
                code("wasm32"),
                text(" と互換性があることを確認してください！そうでない場合、アプリケーションを実行することはできません。"),
            ]),
        ]),
        p(vec![
            code("Deserialize"),
            text(" 特性を派生するように "),
            code("Video"),
            text(" 構造体を更新します："),
        ]),
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
        p(vec![
            text("最後のステップとして、ハードコーディングされたデータを使用するのではなく、fetch リクエストを行うように "),
            code("App"),
            text(" コンポーネントを更新する必要があります。"),
        ]),
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
            p(vec![
                text("ここでは "),
                code("unwrap"),
                text(" を使用していますが、これはデモアプリケーションのためです。実際のアプリケーションでは、"),
                link("https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html", vec![text("適切なエラーハンドリング")]),
                text("を行うことをお勧めします。"),
            ]),
        ]),
        p(vec![text("さて、ブラウザを確認して、すべてが期待通りに動作しているかを確認しましょう......CORS の問題がなければ。これを解決するために、プロキシサーバーが必要です。幸いなことに、trunk はこの機能を提供しています。")]),
        p(vec![text("以下の行を更新します：")]),
        code_block("rust", r##"// highlight-next-line
let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")"##),
        p(vec![text("次に、以下のコマンドを使用してサーバーを再起動します：")]),
        code_block("bash", "trunk serve --proxy-backend=https://yew.rs/tutorial"),
        p(vec![text("ページをリフレッシュすると、すべてが期待通りに動作するはずです。")]),
        h2(vec![text("まとめ")]),
        p(vec![text("おめでとうございます！外部 API からデータを取得し、ビデオリストを表示する Web アプリケーションを作成しました。")]),
        h2(vec![text("次に")]),
        p(vec![text("このアプリケーションは、完璧または有用になるまでにはまだ長い道のりがあります。このチュートリアルを完了した後、より高度なトピックを探求するための出発点として使用できます。")]),
        h3(vec![text("スタイル")]),
        p(vec![
            text("私たちのアプリケーションは非常に見栄えが悪いです。CSS やその他のスタイルがありません。残念ながら、Yew は組み込みのスタイルコンポーネントを提供していません。スタイルシートを追加する方法については、"),
            link("https://trunkrs.dev/assets/", vec![text("Trunk のアセット")]),
            text("を参照してください。"),
        ]),
        h3(vec![text("さらなる依存ライブラリ")]),
        p(vec![
            text("私たちのアプリケーションは、非常に少ない外部依存関係を使用しています。使用できる多くのクレートがあります。詳細については、"),
            link("/community/external-libs", vec![text("外部ライブラリ")]),
            text("を参照してください。"),
        ]),
        h3(vec![text("Yew についてもっと知る")]),
        p(vec![
            text("私たちの"),
            link("/ja/docs/getting-started", vec![text("公式ドキュメント")]),
            text("を読んでください。多くの概念についてより詳細に説明しています。Yew API についてもっと知りたい場合は、"),
            link("https://docs.rs/yew", vec![text("API ドキュメント")]),
            text("を参照してください。"),
        ]),
    ])
}

pub fn page_content() -> yew_site_lib::Content {
    page_content_versioned(None)
}

crate::doc_page!("チュートリアル", "/ja/docs/tutorial", page_content());
