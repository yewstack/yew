pub fn page_content_versioned(version: Option<&str>) -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    let yew_dep = match version {
        Some(v) => format!("yew = {{ version = \"{v}\", features = [\"csr\"] }}"),
        None => {
            "yew = { git = \"https://github.com/yewstack/yew/\", features = [\"csr\"] }".to_string()
        }
    };
    Content::new(vec![
        p!["環境が整ったら、基本的な Yew アプリケーションに必要なテンプレートを使用するか、小さなプロジェクトを手動で設定することができます。"],
        h2!["テンプレートを使用して迅速に開始"],
        p![
            link!("https://github.com/cargo-generate/cargo-generate", "cargo-generate"),
            " のインストール手順に従ってツールをインストールし、次のコマンドを実行します：",
        ],
        code_block("shell", r#"cargo generate yewstack/yew-trunk-minimal-template"#),
        h2!["手動でアプリケーションを設定する"],
        h3!["プロジェクトの作成"],
        p!["まず、新しい cargo プロジェクトを作成してください。"],
        code_block("bash", r#"cargo new yew-app"#),
        p!["新しく作成したディレクトリを開きます。"],
        code_block("bash", r#"cd yew-app"#),
        h3!["Hello World サンプルを実行する"],
        p![
            "Rust 環境が正しく設定されているかを確認するために、",
            code("cargo run"),
            " を使用して初期プロジェクトを実行します。\"Hello World!\" メッセージが表示されるはずです。",
        ],
        code_block("bash", r#"cargo run
# output: Hello World!"#),
        h3!["プロジェクトを Yew Web アプリケーションに設定する"],
        p!["このシンプルなコマンドラインアプリケーションを基本的な Yew Web アプリケーションに変換するために、いくつかの変更が必要です。"],
        h4!["Cargo.toml の更新"],
        p![
            "依存関係リストに ",
            code("yew"),
            " を追加します。",
        ],
        code_block_title("toml", "Cargo.toml", format!(
            "[package]\n\
             name = \"yew-app\"\n\
             version = \"0.1.0\"\n\
             edition = \"2021\"\n\
             \n\
             [dependencies]\n\
             # 開発バージョンの Yew\n\
             {yew_dep}"
        )),
        admonition!(AdmonitionType::Info, None,
            p![
                "アプリケーションを構築するだけの場合は、",
                code("csr"),
                " 特性のみが必要です。これにより、",
                code("Renderer"),
                " とクライアントサイドレンダリングに関連するすべてのコードが有効になります。",
            ],
            p!["ライブラリを作成している場合は、この特性を有効にしないでください。クライアントサイドレンダリングロジックがサーバーサイドレンダリングパッケージに含まれることになります。"],
            p![
                "テストやサンプルのために Renderer が必要な場合は、",
                code("dev-dependencies"),
                " で有効にするべきです。",
            ],
        ),
        h4!["main.rs の更新"],
        p![
            "テンプレートを生成し、クリック時に値を更新するボタンをレンダリングする ",
            code("App"),
            " という名前のルートコンポーネントを設定する必要があります。以下のコードで ",
            code("src/main.rs"),
            " の内容を置き換えます。",
        ],
        admonition!(AdmonitionType::Note, None,
            p![
                code("main"),
                " 関数内の ",
                code("yew::Renderer::<App>::new().render()"),
                " 呼び出しは、アプリケーションを起動し、ページの ",
                code("<body>"),
                " タグにマウントします。動的なプロパティを使用してアプリケーションを起動したい場合は、",
                code("yew::Renderer::<App>::with_props(..).render()"),
                " を使用できます。",
            ],
        ),
        code_block_title("rust", "main.rs", r#"use yew::prelude::*;

#[component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}"#),
        h4!["index.html の作成"],
        p![
            "最後に、アプリケーションのルートディレクトリに ",
            code("index.html"),
            " ファイルを追加します。",
        ],
        code_block_title("html", "index.html", r#"<!doctype html>
<html>
    <head>
        <meta charset="utf-8" />
        <title>Yew App</title>
    </head>
    <body></body>
</html>"#),
        h2!["アプリケーションの表示"],
        p!["以下のコマンドを実行して、ローカルでアプリケーションをビルドおよび提供します。"],
        code_block("bash", r#"trunk serve"#),
        admonition!(AdmonitionType::Info, None,
            p![
                code("--open"),
                " オプションを追加して、デフォルトのブラウザを開くことができます：",
                code("trunk serve --open"),
                "。",
            ],
        ),
        p![
            "Trunk は、ソースコードファイルを変更するたびにアプリケーションをリアルタイムで再構築します。 デフォルトでは、サーバーはアドレス '127.0.0.1' のポート '8080' でリッスンします => ",
            link!("http://127.0.0.1:8080", "http://localhost:8080"),
            "。 この設定を変更するには、次のファイルを作成して必要に応じて編集してください：",
        ],
        code_block_title("toml", "Trunk.toml", "[serve]\n\
# ローカルネットワーク上のリッスンアドレス\n\
address = \"127.0.0.1\"\n\
# 広域ネットワーク上のリッスンアドレス\n\
# address = \"0.0.0.0\"\n\
# リッスンするポート\n\
port = 8000"),
        h2!["おめでとうございます"],
        p!["これで、Yew 開発環境の設定が完了し、最初の Web アプリケーションを構築できました。"],
        p![
            "このアプリケーションを試してみて、さらに学習するために",
            doc_link!(crate::pages::getting_started::examples, "サンプル"),
            "を参照してください。",
        ],
    ])
}

pub fn page_content() -> yew_site_lib::Content {
    page_content_versioned(None)
}

crate::doc_page!(
    "サンプルアプリケーションの構築",
    "/ja/docs/getting-started/build-a-sample-app",
    page_content()
);
