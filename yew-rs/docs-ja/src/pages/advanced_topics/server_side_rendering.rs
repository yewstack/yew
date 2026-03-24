pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h1![text("サーバーサイドレンダリング (Server-Side Rendering)")],
        p![text("デフォルトでは、Yewコンポーネントはクライアントサイドでレンダリングされます。ユーザーがウェブサイトにアクセスすると、サーバーは実際のコンテンツを含まない骨組みのHTMLファイルとWebAssemblyパッケージをブラウザに送信します。すべてのコンテンツはクライアントサイドでWebAssemblyパッケージによってレンダリングされます。これをクライアントサイドレンダリングと呼びます。")],
        p![text("この方法はほとんどのウェブサイトにとって有効ですが、いくつかの注意点があります：")],
        ol![
            li![text("ユーザーはWebAssemblyパッケージがダウンロードされ、初期レンダリングが完了するまで何も表示されません。これにより、ネットワークが遅い場合にユーザーエクスペリエンスが悪化する可能性があります。")],
            li![text("一部の検索エンジンは動的にレンダリングされたウェブページのコンテンツをサポートしておらず、サポートしている検索エンジンでも通常は動的なウェブサイトのランキングが低くなります。")],
        ],
        p![text("これらの問題を解決するために、ウェブサイトをサーバーサイドでレンダリングすることができます。")],
        h2![text("動作原理")],
        p![
            text("Yewはページをサーバーサイドでレンダリングするための "),
            code("ServerRenderer"),
            text(" を提供しています。"),
        ],
        p![
            text("Yewコンポーネントをサーバーサイドでレンダリングするには、"),
            code("ServerRenderer::<App>::new()"),
            text(" を使用してレンダラーを作成し、"),
            code("renderer.render().await"),
            text(" を呼び出して "),
            code("<App />"),
            text(" を "),
            code("String"),
            text(" としてレンダリングします。"),
        ],
        code_block("rust", r#"use yew::prelude::*;
use yew::ServerRenderer;

#[component]
fn App() -> Html {
    html! {<div>{"Hello, World!"}</div>}
}

// この例が CI の WASM 環境で動作することを保証するために `flavor = "current_thread"` を使用しています。
// マルチスレッドを使用したい場合は、デフォルトの `#[tokio::main]` マクロを使用できます。
#[tokio::main(flavor = "current_thread")]
async fn no_main() {
    let renderer = ServerRenderer::<App>::new();

    let rendered = renderer.render().await;

    // プリント: <div>Hello, World!</div>
    println!("{}", rendered);
}"#),
        h2![text("コンポーネントのライフサイクル")],
        p![text("クライアントサイドレンダリングとは異なり、サーバーサイドレンダリング時のコンポーネントのライフサイクルは異なります。")],
        p![
            text("コンポーネントが最初に "),
            code("Html"),
            text(" として正常にレンダリングされるまで、"),
            code("use_effect"),
            text("（および "),
            code("use_effect_with"),
            text("）以外のすべてのフックは正常に動作します。"),
        ],
        admonition![AdmonitionType::Caution, Some("ブラウザインターフェースは利用できません！"),
            p![
                code("web_sys"),
                text(" などのブラウザ関連のインターフェースは、サーバーサイドレンダリング時には利用できません。これらを使用しようとすると、アプリケーションがクラッシュします。このロジックは "),
                code("use_effect"),
                text(" または "),
                code("use_effect_with"),
                text(" に隔離する必要があります。これらはサーバーサイドレンダリング時には実行されないためです。"),
            ],
        ],
        admonition![AdmonitionType::Danger, Some("構造化コンポーネント"),
            p![
                text("サーバーサイドレンダリング時に構造化コンポーネントを使用することは可能ですが、クライアントサイドの安全なロジック（関数コンポーネントの "),
                code("use_effect"),
                text(" フックなど）とライフサイクルイベントの間には明確な境界がなく、ライフサイクルイベントの呼び出し順序もクライアントとは異なります。"),
            ],
            p![
                text("さらに、構造化コンポーネントは、すべての子コンポーネントがレンダリングされ "),
                code("destroy"),
                text(" メソッドが呼び出されるまでメッセージを受け取り続けます。開発者は、コンポーネントに渡される可能性のあるメッセージがブラウザインターフェースを呼び出すロジックにリンクされないようにする必要があります。"),
            ],
            p![text("サーバーサイドレンダリングをサポートするアプリケーションを設計する際は、特別な理由がない限り、関数コンポーネントを使用することをお勧めします。")],
        ],
        h2![text("サーバーサイドレンダリング中のデータ取得")],
        p![text("データ取得はサーバーサイドレンダリングとハイドレーション（hydration）中の難点の一つです。")],
        p![text("従来の方法では、コンポーネントがレンダリングされるとすぐに利用可能になります（仮想DOMを出力してレンダリングします）。コンポーネントがデータを取得する必要がない場合、この方法は有効です。しかし、コンポーネントがレンダリング時にデータを取得しようとするとどうなるでしょうか？")],
        p![text("以前は、Yewにはコンポーネントがまだデータを取得しているかどうかを検出するメカニズムがありませんでした。データ取得クライアントは、初期レンダリング中に何が要求されたかを検出し、要求が完了した後に再レンダリングをトリガーするソリューションを実装する責任がありました。サーバーはこのプロセスを繰り返し、応答を返す前にレンダリング中に追加の保留中の要求がないことを確認します。")],
        p![text("これは、コンポーネントを繰り返しレンダリングするため、CPUリソースを浪費するだけでなく、データクライアントは、サーバー側で取得したデータをハイドレーション中に利用可能にする方法を提供する必要があり、初期レンダリングで返される仮想DOMがサーバーサイドレンダリングのDOMツリーと一致することを保証する必要があります。これは実現が難しい場合があります。")],
        p![
            text("Yewは、"),
            code("<Suspense />"),
            text(" を使用してこの問題を解決する異なるアプローチを採用しています。"),
        ],
        p![
            code("<Suspense />"),
            text(" は特別なコンポーネントで、クライアント側で使用する場合、コンポーネントがデータを取得（保留）している間にフォールバックUIを表示し、データ取得が完了した後に通常のUIに戻る方法を提供します。"),
        ],
        p![text("アプリケーションがサーバーサイドレンダリングされると、Yewはコンポーネントが保留状態でなくなるまで待機し、それを文字列バッファにシリアル化します。")],
        p![
            text("ハイドレーション中、"),
            code("<Suspense />"),
            text(" コンポーネント内の要素は、すべての子コンポーネントが保留状態でなくなるまでハイドレーションされません。"),
        ],
        p![text("この方法により、開発者はサーバーサイドレンダリングに対応したクライアント非依存のアプリケーションを簡単に構築し、データ取得を行うことができます。")],
        h2![text("`<head>` タグのレンダリング")],
        p![
            text("SSR でよく必要とされるのは、クローラーやソーシャルプレビューが最初のロード時に正しいメタデータを参照できるよう、動的な "),
            code("<head>"),
            text(" コンテンツ（"),
            code("<title>"),
            text("、"),
            code("<meta>"),
            text(" など）をレンダリングすることです。"),
        ],
        p![
            code("ServerRenderer"),
            text(" はコンポーネントツリー（通常はドキュメントの body 部分）のみをレンダリングし、"),
            code("<head>"),
            text(" にはアクセスできません。そのため、head タグは "),
            bold![text("Yew の外部でサーバー側に")],
            text("生成し、クライアントに送信する前に HTML テンプレートに埋め込む必要があります。"),
        ],
        p![
            link!["https://github.com/yewstack/yew/blob/master/examples/ssr_router/src/bin/ssr_router_server.rs", text("ssr_router サンプル")],
            text(" はこのパターンを示しています：サーバーはリクエスト URL からルートを判別し、適切な "),
            code("<title>"),
            text(" および "),
            code("<meta>"),
            text(" タグを生成して、Trunk が生成した "),
            code("index.html"),
            text(" の "),
            code("</head>"),
            text(" の前に挿入します。"),
        ],
        admonition![AdmonitionType::Info, None,
            p![
                text("完全に SSR 互換のサードパーティソリューションとして、"),
                link!["https://docs.rs/bounce/latest/bounce/helmet/index.html", text("Bounce の <Helmet/> コンポーネント")],
                text(" が利用できます。"),
            ],
        ],
        h2![text("サーバーサイドレンダリングハイドレーション（SSR Hydration）")],
        p![
            text("ハイドレーションは、Yewアプリケーションをサーバー側で生成されたHTMLファイルに接続するプロセスです。デフォルトでは、"),
            code("ServerRenderer"),
            text("はハイドレーション可能なHTML文字列を出力し、追加情報を含んでハイドレーションを容易にします。"),
            code("Renderer::hydrate"),
            text(" メソッドを呼び出すと、Yewは最初からレンダリングするのではなく、アプリケーションが生成した仮想DOMとサーバーレンダラーが生成したHTML文字列を調整します。"),
        ],
        admonition![AdmonitionType::Caution, None,
            p![
                code("ServerRenderer"),
                text(" が作成したHTMLマークアップを正常にハイドレーションするためには、クライアントはSSRに使用されたレイアウトと完全に一致する仮想DOMレイアウトを生成する必要があります。要素を含まないコンポーネントも含めてです。特定の実装でのみ使用されるコンポーネントがある場合は、"),
                code("PhantomComponent"),
                text(" を使用して追加のコンポーネントの位置を埋めることを検討してください。"),
            ],
        ],
        admonition![AdmonitionType::Warning, None,
            p![
                text("SSR出力（静的HTML）をブラウザが初期レンダリングした後、実際のDOMが期待されるDOMと一致する場合にのみ、ハイドレーションは成功します。HTMLが規格に準拠していない場合、ハイドレーションは失敗する可能性があります。ブラウザは不正なHTMLのDOM構造を変更する可能性があり、実際のDOMが期待されるDOMと異なることがあります。例えば、"),
                link!["https://github.com/yewstack/yew/issues/2684", text("<tbody> のない <table> がある場合、ブラウザはDOMに <tbody> を追加する可能性があります")],
                text("。"),
            ],
        ],
        h2![text("ハイドレーション中のコンポーネントライフサイクル")],
        p![
            text("ハイドレーション中、コンポーネントは作成後に2回連続してレンダリングされます。すべてのエフェクトは2回目のレンダリングが完了した後に呼び出されます。コンポーネントのレンダリング関数に副作用がないことを確認することが重要です。状態を変更したり、追加のレンダリングをトリガーしたりしないようにしてください。現在、状態を変更したり追加のレンダリングをトリガーしたりするコンポーネントがある場合は、それらを "),
            code("use_effect"),
            text(" フックに移動してください。"),
        ],
        p![
            text("ハイドレーション中、構造化コンポーネントを使用してサーバーサイドレンダリングを行うことができます。ビュー関数はレンダリング関数の前に複数回呼び出されます。レンダリング関数が呼び出されるまで、DOMは未接続と見なされ、"),
            code("rendered()"),
            text(" メソッドが呼び出される前にレンダリングノードにアクセスすることを防ぐ必要があります。"),
        ],
        h2![text("例")],
        code_block("rust", r#"use yew::prelude::*;
use yew::Renderer;

#[component]
fn App() -> Html {
    html! {<div>{"Hello, World!"}</div>}
}

fn main() {
    let renderer = Renderer::<App>::new();

    // body 要素の下のすべてのコンテンツをハイドレーションし、末尾の要素を削除します（存在する場合）。
    renderer.hydrate();
}"#),
        p![
            text("例: "),
            link!["https://github.com/yewstack/yew/tree/master/examples/simple_ssr", text("simple_ssr")],
        ],
        p![
            text("例: "),
            link!["https://github.com/yewstack/yew/tree/master/examples/ssr_router", text("ssr_router")],
        ],
        h2![text("シングルスレッドモード")],
        p![
            text("Yewは "),
            code("yew::LocalServerRenderer"),
            text(" を使用してシングルスレッドでのサーバーサイドレンダリングをサポートしています。このモードはWASIのようなシングルスレッド環境に適しています。"),
        ],
        code_block("rust", r#"// `wasm32-wasip1` または `wasm32-wasip2` ターゲットを使用してビルドしてください。

use yew::prelude::*;
use yew::LocalServerRenderer;

#[component]
fn App() -> Html {
    use yew_router::prelude::*;

    html! {
        <>
            <h1>{"Yew WASI SSR demo"}</h1>
        </>
    }
}

pub async fn render() -> String {
    let renderer = LocalServerRenderer::<App>::new();
    let html_raw = renderer.render().await;

    let mut body = String::new();
    body.push_str("<body>");
    body.push_str("<div id='app'>");
    body.push_str(&html_raw);
    body.push_str("</div>");
    body.push_str("</body>");

    body
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("{}", render().await);
}"#),
        p![
            text("例: "),
            link!["https://github.com/yewstack/yew/tree/master/examples/wasi_ssr_module", text("wasi_ssr_module")],
        ],
        admonition![AdmonitionType::Note, None,
            p![
                code("wasm32-unknown-unknown"),
                text(" ターゲットを使用してSSRアプリケーションをビルドする場合、"),
                code("not_browser_env"),
                text(" 機能フラグを使用して、Yew内部のブラウザ固有のAPIへのアクセスを無効にすることができます。これは、Cloudflare Workerのようなサーバーレスプラットフォームで非常に便利です。"),
            ],
        ],
        admonition![AdmonitionType::Caution, None,
            p![
                text("サーバーサイドレンダリングは現在実験的な機能です。バグを見つけた場合は、"),
                link!["https://github.com/yewstack/yew/issues/new?assignees=&labels=bug&template=bug_report.md&title=", text("GitHubで報告してください")],
                text("。"),
            ],
        ],
    ])
}

crate::doc_page!(
    "サーバーサイドレンダリング",
    "/ja/docs/advanced-topics/server-side-rendering",
    page_content()
);
