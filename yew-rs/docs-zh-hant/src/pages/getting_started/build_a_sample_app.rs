pub fn page_content_versioned(version: Option<&str>) -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    let yew_dep = match version {
        Some(v) => format!("yew = {{ version = \"{v}\", features = [\"csr\"] }}"),
        None => "# 開發版本的 Yew\nyew = { git = \"https://github.com/yewstack/yew/\", features = \
                 [\"csr\"] }"
            .to_string(),
    };
    Content::new(vec![
        p(vec![text(
            "當您的環境準備好後，您可以選擇使用一個包含基本 Yew \
             應用所需樣板的起始模板，或手動設定一個小項目。",
        )]),
        h2(vec![text("使用模板快速起步")]),
        p(vec![
            text("請依照 "),
            link(
                "https://github.com/cargo-generate/cargo-generate",
                vec![text("cargo-generate")],
            ),
            text(" 的安裝說明安裝工具，然後執行下列指令："),
        ]),
        code_block(
            "shell",
            r#"cargo generate yewstack/yew-trunk-minimal-template"#,
        ),
        h2(vec![text("手動設定應用")]),
        h3(vec![text("建立項目")]),
        p(vec![text("首先，請建立一個新的 cargo 專案。")]),
        code_block("bash", r#"cargo new yew-app"#),
        p(vec![text("開啟新建立的目錄。")]),
        code_block("bash", r#"cd yew-app"#),
        h3(vec![text("運行一個 hello world 範例")]),
        p(vec![
            text("為了驗證 Rust 環境是否設定正確，使用 "),
            code("cargo run"),
            text(" 執行初始專案。您應該會看到一個 \"Hello World!\" 訊息。"),
        ]),
        code_block(
            "bash",
            r#"cargo run
# output: Hello World!"#,
        ),
        h3(vec![text("將項目設定為 Yew web 應用")]),
        p(vec![text(
            "為了將這個簡單的命令列應用程式轉換為一個基本的 Yew web 應用程序，需要進行一些更改。",
        )]),
        h4(vec![text("更新 Cargo.toml")]),
        p(vec![text("將 "), code("yew"), text(" 加入到依賴清單中。")]),
        code_block_title(
            "toml",
            "Cargo.toml",
            format!(
                "[package]\nname = \"yew-app\"\nversion = \"0.1.0\"\nedition = \
                 \"2021\"\n\n[dependencies]\n{yew_dep}"
            ),
        ),
        admonition(
            AdmonitionType::Info,
            None,
            vec![
                p(vec![
                    text("如果你只是正在建立一個應用程序，你只需要 "),
                    code("csr"),
                    text(" 特性。它將啟用 "),
                    code("Renderer"),
                    text(" 和所有與客戶端渲染相關的程式碼。"),
                ]),
                p(vec![text(
                    "如果你正在製作一個函式庫，請不要啟用此特性，\
                     因為它會將客戶端渲染邏輯拉入伺服器端渲染包中。",
                )]),
                p(vec![
                    text("如果你需要 Renderer 進行測試或範例，你應該在 "),
                    code("dev-dependencies"),
                    text(" 中啟用它。"),
                ]),
            ],
        ),
        h4(vec![text("更新 main.rs")]),
        p(vec![
            text("我們需要產生一個模板，設定一個名為 "),
            code("App"),
            text(" 的根元件，該元件渲染一個按鈕，當點擊時更新其值。用以下程式碼取代 "),
            code("src/main.rs"),
            text(" 的內容。"),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                code("main"),
                text(" 函數中的 "),
                code("yew::Renderer::<App>::new().render()"),
                text(" 呼叫啟動您的應用程式並將其掛載到頁面的 "),
                code("<body>"),
                text(" 標籤上。如果您想要使用任何動態屬性啟動您的應用程序，您可以使用 "),
                code("yew::Renderer::<App>::with_props(..).render()"),
                text("。"),
            ])],
        ),
        code_block_title(
            "rust",
            "main.rs",
            r#"use yew::prelude::*;

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
}"#,
        ),
        h4(vec![text("建立 index.html")]),
        p(vec![
            text("最後，在應用程式的根目錄中新增一個 "),
            code("index.html"),
            text(" 檔案。"),
        ]),
        code_block_title(
            "html",
            "index.html",
            r#"<!doctype html>
<html>
    <head>
        <meta charset="utf-8" />
        <title>Yew App</title>
    </head>
    <body></body>
</html>"#,
        ),
        h2(vec![text("查看您的 Web 應用")]),
        p(vec![text("運行以下命令在本地建置和提供應用程式。")]),
        code_block("bash", r#"trunk serve"#),
        admonition(
            AdmonitionType::Info,
            None,
            vec![p(vec![
                text("新增選項 '--open' 來開啟您的預設瀏覽器 "),
                code("trunk serve --open"),
                text("。"),
            ])],
        ),
        p(vec![
            text(
                "Trunk 將在您修改任何原始程式碼檔案時即時重新建立您的應用程式。 \
                 預設情況下，伺服器將在位址 '127.0.0.1' 的連接埠 '8080' 上監聽 => ",
            ),
            link("http://127.0.0.1:8080", vec![text("http://localhost:8080")]),
            text("。 若要變更這部分配置，請建立以下檔案並根據需要進行編輯："),
        ]),
        code_block_title(
            "toml",
            "Trunk.toml",
            r#"[serve]
# 區域網路上的監聽位址
address = "127.0.0.1"
# 廣域網路上的監聽位址
# address = "0.0.0.0"
# 監聽的端口
port = 8000"#,
        ),
        h2(vec![text("恭喜")]),
        p(vec![text(
            "現在您已經成功設定了您的 Yew 開發環境，並建立了您的第一個 Web 應用程式。",
        )]),
        p(vec![
            text("嘗試這個應用程序，並查看"),
            link("/zh-Hant/docs/getting-started/examples", vec![text("範例")]),
            text("以進一步學習。"),
        ]),
    ])
}

pub fn page_content() -> yew_site_lib::Content {
    page_content_versioned(None)
}

crate::doc_page!(
    "建立一個範例應用",
    "/zh-Hant/docs/getting-started/build-a-sample-app",
    page_content()
);
