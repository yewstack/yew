crate::doc_page!(
    "Debugging",
    "/ja/docs/more/debugging",
    Content::new(vec![
        h2(vec![text("パニック")]),
        p(vec![
            text("Rustシンボルで良いスタックトレースをするには"),
            link(
                "https://github.com/rustwasm/console_error_panic_hook",
                vec![code("console_error_panic")]
            ),
            text("クレートを使用してください。\
                 注意として、"),
            code("cargo-web"),
            text("でビルドされたものとは互換性がありません。"),
        ]),
        h2(vec![text("コンソールでのログ")]),
        p(vec![text(
            "一般的に、WasmのWebアプリはブラウザのAPIと連携することができ、\
             console.logのAPIも例外ではありません。いつくかの選択肢があります:"
        )]),
        h3(vec![link(
            "https://crates.io/crates/wasm-logger",
            vec![code("wasm-logger")]
        )]),
        p(vec![
            text("このクレートはRustの"),
            code("log"),
            text("クレートと親和性があります。"),
        ]),
        code_block(
            "rust",
            "// セットアップ\nfn main() {\n    wasm_logger::init(wasm_logger::Config::default());\n}\n\n// 使用方法\nlog::info!(\"Update: {:?}\", msg);"
        ),
        h3(vec![link(
            "https://docs.rs/yew/latest/yew/services/console/struct.ConsoleService.html",
            vec![code("ConsoleService")]
        )]),
        p(vec![
            text("このサービスはYewに含まれており、"),
            code("\"services\""),
            text("の機能が有効化されている場合は利用可能です。"),
        ]),
        code_block(
            "rust",
            "// 使用方法\nConsoleService::info(format!(\"Update: {:?}\", msg).as_ref());"
        ),
        h2(vec![text("ソースマップ")]),
        p(vec![text(
            "今のところはRust/WasmのWebアプリにはソースマップへの第一級のサポートがありません。\
             もちろん、これは変更される可能性があります。これが当てはまらない場合、または進捗が見られる場合は、変更を提案してください！"
        )]),
        h3(vec![text("最新情報")]),
        p(vec![
            text("[2019年12月] "),
            link(
                "https://developers.google.com/web/updates/2019/12/webassembly#the_future",
                vec![text("Chrome DevTools update")]
            ),
        ]),
        blockquote(vec![p(vec![text(
            "やらなければいけないことがまだたくさんあります。例えばツール側ではEmscripten（Binaryen）とwasm-pack（wasm-bindgen）がそれらが実行する変換に関するDWARF情報の更新をまだサポートしていません。"
        )])]),
        p(vec![
            text("[2020] "),
            link(
                "https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger",
                vec![text("Rust Wasmデバッグガイド")]
            ),
        ]),
        blockquote(vec![p(vec![
            text("残念なことに、WebAssemblyのデバッグの物語はまだ未成熟です。ほとんどのUnixのシステムでは"),
            link("http://dwarfstd.org/", vec![text("DWARF")]),
            text("は実行中のプログラムをソースレベルで検査するためにデバッガに必要な情報をエンコードするために使用されます。Windowsには同様の情報をエンコードする代替形式があります。現在、WebAssemblyに相当するものはありません。"),
        ])]),
        p(vec![
            text("[2019] "),
            link(
                "https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging",
                vec![text("Rust Wasmロードマップ")]
            ),
        ]),
        blockquote(vec![p(vec![text(
            "デバッグはトリッキーです。なぜなら、多くの話はこの活動チームの手の届かないところにあり、WebAssemblyの標準化団体とブラウザ開発者ツールを実装している人たちの両方に依存しているからです。"
        )])]),
    ])
);
