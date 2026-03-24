pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![text(
            "Yew アプリケーションをサーバーにデプロイする準備ができたら、\
             いくつかのデプロイオプションがあります。",
        )],
        p![
            code("trunk build --release"),
            text(
                " は、リリースモードでアプリケーションをビルドします。HTTP \
                 サーバーを設定して、サイトにアクセスしたときに ",
            ),
            code("index.html"),
            text(" を提供し、静的パス（例："),
            code("index_<hash>.js"),
            text(" および "),
            code("index_bg_<hash>.wasm"),
            text(
                "）のリクエストに対して trunk が生成した dist \
                 ディレクトリから適切なコンテンツを提供する必要があります。",
            ),
        ],
        admonition![
            AdmonitionType::Important,
            Some("trunk serve --release について"),
            p![
                code("trunk serve --release"),
                text(
                    " を使用してアプリケーションを提供しないでください。\
                     これは開発中にリリースビルドをテストするためだけに使用されるべきです。",
                ),
            ],
        ],
        h2![text("サーバー設定")],
        h3![text("index.html をフォールバックとして提供する")],
        p![
            text("アプリケーションが "),
            link!("/ja/docs/concepts/router", text("Yew ルーター")),
            text(" を使用している場合、存在しないファイルへのリクエスト時にサーバーが "),
            code("index.html"),
            text(" を返すように設定する必要があります。"),
        ],
        p![
            text("Yew ルーターを使用するアプリケーションは "),
            link!(
                "https://developer.mozilla.org/en-US/docs/Glossary/SPA",
                text("シングルページアプリケーション (SPA)"),
            ),
            text(
                " として構築されています。ユーザーが実行中のクライアントから URL \
                 にナビゲートすると、ルーターが URL を解釈してそのページにルーティングします。",
            ),
        ],
        p![text(
            "しかし、ページをリフレッシュしたり、アドレスバーに URL \
             を入力したりすると、これらの操作は実行中のアプリケーションではなく、\
             ブラウザー自体によって処理されます。ブラウザーはその URL \
             を直接サーバーにリクエストし、ルーターをバイパスします。誤って設定されたサーバーは \
             404 - 見つかりません 状態を返します。",
        )],
        p![
            code("index.html"),
            text(" を返すことで、アプリケーションは通常通りにロードされ、ルーターがルート "),
            code("/show/42"),
            text(" を認識して適切なコンテンツを表示するまで、リクエストが "),
            code("/"),
            text(" であるかのように動作します。"),
        ],
        h3![text("Web Assembly リソースに正しい MIME タイプを設定する",)],
        p![
            text("WASM ファイルは "),
            code("application/wasm"),
            text(" MIME タイプで "),
            link!(
                "https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type",
                text("Content-Type ヘッダー"),
            ),
            text(" を設定する必要があります。"),
        ],
        p![text(
            "ほとんどのサーバーとホスティングサービスはデフォルトでこれを行います。\
             サーバーがこれを行わない場合は、そのドキュメントを参照してください。ほとんどの Web \
             ブラウザーでは、誤った MIME タイプは次のようなエラーを引き起こします：",
        )],
        code_block(
            "ignore",
            r#"`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:
 TypeError: WebAssembly: Response has unsupported MIME type 'text/plain' expected 'application/wasm'"#,
        ),
        h2![text("相対パスのビルド")],
        p![
            text("デフォルトでは、trunk はサイトが "),
            code("/"),
            text(" で提供されると仮定し、それに応じてサイトをビルドします。この動作は、"),
            code("index.html"),
            text(" ファイルに "),
            code("<base data-trunk-public-url />"),
            text(" を追加することで上書きできます。Trunk はこのタグを書き換えて、"),
            code("--public-url"),
            text(" に渡された値を含めます。Yew ルーターは "),
            code("<base />"),
            text(" の存在を自動的に検出し、適切に処理します。"),
        ],
        h2![text("環境変数を使用して動作をカスタマイズする")],
        p![
            text(
                "通常、環境変数を使用してビルド環境をカスタマイズします。\
                 アプリケーションがブラウザで実行されるため、\
                 実行時に環境変数を読み取ることはできません。 ",
            ),
            link!(
                "https://doc.rust-lang.org/std/macro.env.html",
                code("std::env!"),
            ),
            text(" マクロは、コンパイル時に環境変数の値を取得できます。"),
        ],
    ])
}

crate::doc_page!("デプロイ", "/ja/docs/more/deployment", page_content());
