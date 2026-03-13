crate::doc_page!(
    "第一個簡單的 App",
    "/zh-Hant/docs/getting-started/build-a-sample-app",
    Content::new(vec![
        p(vec![text("首先，先建立一個新的 binary 專案：")]),
        code_block("bash", r#"cargo new --bin yew-app && cd yew-app"#),
        p(vec![
            text("在依賴庫裡加入 "),
            code("yew"),
            text(" 與 "),
            code("wasm-bindgen"),
            text("（最新的版號，請參考"),
            link("https://docs.rs/yew", vec![text("這裡")]),
            text("）"),
        ]),
        code_block(
            "text",
            r#"[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[dependencies]
yew = "0.16"
wasm-bindgen = "0.2""#,
        ),
        p(vec![
            text("將下面的模板複製進你的 "),
            code("src/lib.rs"),
            text(" 檔案："),
        ]),
        code_block(
            "rust",
            r#"use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick={self.link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}"#,
        ),
        p(vec![text(
            "模板會建置名叫 Model 的根元件 Component，Model 會顯示一個按鈕，當你按下按鈕時，Model \
             會更新自己的狀態。需要特別注意的是，在 main() 裡的 \
             App::<Model>::new().mount_to_body()，他會啟動你的 app 並且掛載 Model 裡的 HTML 到 \
             <body> 標籤中。如果你想要在啟動應用程式時，帶入動態的屬性，你可以改用 \
             App::<Model>::new().mount_to_body_with_props(..)。",
        )]),
        p(vec![
            text("最後，在你的專案，新增 "),
            code("static"),
            text(" 資料夾，並新增 "),
            code("index.html"),
            text(" 檔案到 static 裡。"),
        ]),
        code_block("bash", r#"mkdir static"#),
        code_block(
            "html",
            r#"<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Yew Sample App</title>
        <script type="module">
            import init from "./wasm.js"
            init()
        </script>
    </head>
    <body></body>
</html>"#,
        ),
        h2(vec![text("執行你的 App！")]),
        p(vec![
            text("使用 "),
            link(
                "https://drager.github.io/wasm-pack/book/",
                vec![code("wasm-pack")],
            ),
            text(" 來執行專案是比較好的選擇。如果你還沒有做任何準備，先用 "),
            code("cargo install wasm-pack"),
            text(" 安裝 "),
            code("wasm-pack"),
            text("，然後用下面的指令，建置與開啟開發用伺服器："),
        ]),
        code_block(
            "bash",
            r#"wasm-pack build --target web --out-name wasm --out-dir ./static"#,
        ),
        p(vec![text(
            "wasm-pack 會在 ./static 裡產生一個 bundle，裡面包含專案編成的 WebAssembly，以及 \
             JavaScript 的包裹器，這些東西都會在你的專案執行時被載入。",
        )]),
        p(vec![text(
            "最後，用你最喜歡的網頁伺服器，去啟動在 ./static 底下的檔案。範例：",
        )]),
        code_block(
            "bash",
            r#"cargo +nightly install miniserve
miniserve ./static --index index.html"#,
        ),
    ])
);
