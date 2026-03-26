pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "Yew アプリケーションをコンパイル、ビルド、パッケージ、およびデバッグするためにいくつかのツールが必要です。最初に、",
            link!("https://trunkrs.dev/", "Trunk"),
            " を使用することをお勧めします。Trunk は Rust 用の WASM Web アプリケーションパッケージツールです。",
        ],
        h2!["Rust のインストール"],
        p![
            "Rust をインストールするには、",
            link!("https://www.rust-lang.org/tools/install", "公式の手順"),
            " に従ってください。",
        ],
        admonition!(AdmonitionType::Important, Some("Important"),
            p![
                "Yew がサポートする最低 Rust バージョン（MSRV）は ",
                code("1.84.0"),
                " です。古いバージョンではコンパイルできません。",
                code("rustup show"),
                "（「active toolchain」の下）または ",
                code("rustc --version"),
                " を使用してツールチェーンのバージョンを確認できます。ツールチェーンを更新するには、",
                code("rustup update"),
                " を実行してください。",
            ],
        ),
        h2!["WebAssembly ターゲットのインストール"],
        p![
            "Rust は異なる「ターゲット」（例えば異なるプロセッサ）に対してソースコードをコンパイルできます。ブラウザベースの WebAssembly 用のコンパイルターゲットは ",
            code("wasm32-unknown-unknown"),
            " と呼ばれます。以下のコマンドは、開発環境に WebAssembly ターゲットを追加します。",
        ],
        code_block("shell", "rustup target add wasm32-unknown-unknown"),
        h2!["Trunk のインストール"],
        p![
            "Trunk は、デプロイとパッケージ管理に推奨されるツールであり、ドキュメントやサンプル全体で使用されています。",
        ],
        code_block("shell",
"# 注意：これはすべての内容をゼロからコンパイルするため、インストールに時間がかかる場合があります
# Trunk は多くの主要なパッケージマネージャーに対して事前構築されたバイナリを提供しています
# 詳細については、https://trunkrs.dev/#install を参照してください
cargo install --locked trunk"
        ),
        h3!["他のオプション"],
        p![
            "Trunk の他にも、Yew アプリケーションをパッケージ化するための他のオプションがあります。以下のオプションのいずれかを試してみることをお勧めします：",
        ],
        ul![
            li![link!("https://github.com/drager/wasm-pack/", "wasm-pack")],
            li![link!("https://github.com/IMI-eRnD-Be/wasm-run", "wasm-run")],
            li![link!("https://github.com/rustminded/xtask-wasm/", "xtask-wasm"), "（まだ初期開発段階です）"],
        ],
        h2!["次のステップ"],
        p![
            "開発環境の設定が完了したら、ドキュメントの読み進めを続けることができます。実践を通じて学ぶのが好きな方は、",
            link!("/ja/docs/tutorial", "チュートリアル"),
            "をチェックすることをお勧めします。",
        ],
    ])
}

crate::doc_page!("始めに", "/ja/docs/getting-started", page_content());
