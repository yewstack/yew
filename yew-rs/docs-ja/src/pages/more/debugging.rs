pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("パニック (Panics)")]),
        p(vec![text("Yew はブラウザのコンソールにパニックログを自動的に出力します。")]),
        h2(vec![text("コンソールログ")]),
        p(vec![
            text("JavaScript では、"),
            code("console.log()"),
            text(" を使用してブラウザのコンソールに出力します。以下は Yew のいくつかのオプションです。"),
        ]),
        h3(vec![link("https://crates.io/crates/wasm-logger", vec![code("wasm-logger")])]),
        p(vec![
            code("wasm-logger"),
            text(" クレートは "),
            link("https://crates.io/crates/log", vec![text("log")]),
            text(" クレートと統合されており、ログレベル、ソース行、ファイル名をブラウザのコンソールに送信します。"),
        ]),
        code_block("rust", r#"use log::info;
use wasm_bindgen::JsValue;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let object = JsValue::from("world");
    info!("Hello {}", object.as_string().unwrap());
}"#),
        h3(vec![link("https://crates.io/crates/gloo-console", vec![code("gloo-console")])]),
        p(vec![
            text("このクレートは Gloo の一部で、ブラウザ API の Rust ラッパーを提供します。"),
            code("log!"),
            text(" マクロは "),
            code("JsValue"),
            text(" を直接受け入れることができ、"),
            code("wasm_logger"),
            text(" よりも使いやすいです。"),
        ]),
        code_block("rust", r#"use gloo_console::log;
use wasm_bindgen::JsValue;

fn main() {
    let object = JsValue::from("world");
    log!("Hello", object)
}"#),
        h3(vec![link("https://crates.io/crates/tracing-web", vec![code("tracing-web")])]),
        p(vec![
            code("tracing-web"),
            text(" は "),
            link("https://crates.io/crates/tracing-subscriber", vec![text("tracing-subscriber")]),
            text(" と一緒に使用でき、メッセージをブラウザのコンソールに出力します。"),
        ]),
        code_block("rust", r#"use tracing_subscriber::{
    fmt::{
        format::{FmtSpan, Pretty},
        time::UtcTime,
    },
    prelude::*,
};
use wasm_bindgen::JsValue;

fn main() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(tracing_web::MakeConsoleWriter)
        .with_span_events(FmtSpan::ACTIVE);
    let perf_layer = tracing_web::performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
    let object = JsValue::from("world");
    tracing::info!("Hello {}", object.as_string().unwrap());
}"#),
        h2(vec![text("コンポーネントライフサイクルのデバッグ")]),
        p(vec![
            link("https://crates.io/crates/tracing", vec![text("tracing")]),
            text(" は、コンポーネントのライフサイクルに関連するイベント情報を収集するために使用できます。"),
            code("tracing"),
            text(" には "),
            code("log"),
            text(" サポートの機能フラグもあり、"),
            code("wasm-logger"),
            text(" とうまく統合できます。"),
        ]),
        p(vec![
            link("https://docs.rs/tracing/latest/tracing/level_filters/index.html#compile-time-filters", vec![text("コンパイル時フィルタ")]),
            text(" は、詳細度を調整したりログ記録を無効にしたりするために使用できます。これにより、より小さな Wasm ファイルが生成されるはずです。"),
        ]),
        h2(vec![text("ソースマップ (Source Maps)")]),
        p(vec![
            link("https://developer.chrome.com/blog/wasm-debugging-2019/#enter-dwarf", vec![text("ソースマップ")]),
            text(" をサポートするいくつかの方法がありますが、いくつかの設定が必要です。"),
        ]),
        h2(vec![text("過去の記事")]),
        p(vec![text("以下は、Rust における WebAssembly デバッグの現状に関する過去の記事です。興味深い読み物かもしれません。")]),
        p(vec![
            text("[2019 年 12 月] "),
            link("https://developers.google.com/web/updates/2019/12/webassembly#the_future", vec![text("Chrome DevTools 更新")]),
        ]),
        blockquote(vec![p(vec![text(
            "これらの作業にはまだ多くのことが残されています。例えば、ツールの面では、\
             Emscripten（Binaryen）と wasm-pack（wasm-bindgen）は、\
             それらが実行する変換に対して DWARF 情報を更新することをまだサポートしていません。",
        )])]),
        p(vec![
            text("[2020 年] "),
            link("https://rustwasm.github.io/book/reference/debugging.html#using-a-debugger", vec![text("Rust Wasm デバッグガイド")]),
        ]),
        blockquote(vec![p(vec![text(
            "残念ながら、WebAssembly のデバッグ機能はまだ未成熟です。ほとんどの Unix システムでは、\
             DWARF が実行中のプログラムのソースレベルの検査に必要な情報をエンコードするために使用されますが、\
             Windows では同様の情報をエンコードする代替フォーマットがあります。しかし、現在のところ、\
             WebAssembly には対応するフォーマットがありません。",
        )])]),
        p(vec![
            text("[2019 年] "),
            link("https://rustwasm.github.io/rfcs/007-2019-roadmap.html#debugging", vec![text("Rust Wasm ロードマップ")]),
        ]),
        blockquote(vec![p(vec![text(
            "デバッグは難しいです。なぜなら、多くの状況がこの作業グループの管理下にないからです。\
             これは、WebAssembly の標準化機関やブラウザ開発者ツールを実装する人々に依存しています。",
        )])]),
    ])
}

crate::doc_page!("デバッグ", "/ja/docs/more/debugging", page_content());
