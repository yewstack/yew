pub fn page_content_versioned(version: Option<&str>) -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    let yew_dep_csr = match version {
        Some(v) => format!("yew = {{ version = \"{v}\", features = [\"csr\"] }}"),
        None => {
            "yew = { git = \"https://github.com/yewstack/yew/\", features = [\"csr\"] }".to_string()
        }
    };
    Content::new(vec![
        h2!["介紹"],
        p![
            "在這個實作教程中，我們將學習如何使用 Yew 建立 Web 應用程式。 ",
            bold!["Yew"],
            " 是一個現代的 ",
            link!["https://www.rust-lang.org/", "Rust"],
            " 框架，用於使用 ",
            link!["https://webassembly.org/", "WebAssembly"],
            " 建立前端 Web 應用程式。 Yew 透過利用 Rust \
             強大的類型系統，鼓勵可重複使用、可維護和良好結構化的架構。 \
             一個龐大的社群所創造的函式庫生態系統，稱為Rust 中的",
            link![
                "https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html",
                "crates"
            ],
            "，為常用模式（如狀態管理）提供了元件。 Rust 的套件管理器 ",
            link!["https://doc.rust-lang.org/cargo/", "Cargo"],
            " 允許我們利用 ",
            link!["https://crates.io", "crates.io"],
            " 上提供的大量 crate，例如 Yew。",
        ],
        h3!["我們將要建構的內容"],
        p!["Rustconf 是 Rust 社群每年舉辦的星際派對。 Rustconf 2020 \
            有大量的演講，提供了大量的資訊。 在這個實作教程中，我們將建立一個 Web \
            應用程序，幫助其他 Rustaceans 了解這些演講並從一個頁面觀看它們。"],
        h2!["設定"],
        h3!["先決條件"],
        p![
            "這個教程假設您已經熟悉 Rust。如果您是Rust 的新手，免費的",
            link![
                "https://doc.rust-lang.org/book/ch00-00-introduction.html",
                "Rust 書"
            ],
            " 為初學者提供了一個很好的起點，並且即使對於有經驗的Rust \
             開發人員來說，它仍然是一個很好的資源。",
        ],
        p![
            "確保安裝了最新版本的 Rust，方法是執行 ",
            code("rustup update"),
            " 或",
            link!["https://www.rust-lang.org/tools/install", "安裝 Rust"],
            "。",
        ],
        p![
            "安裝 Rust 後，您可以使用 Cargo 執行以下命令安裝 ",
            code("trunk"),
            "：",
        ],
        code_block("bash", r#"cargo install trunk"#),
        p!["我們還需要新增 WASM 建置目標，執行以下命令："],
        code_block("bash", r#"rustup target add wasm32-unknown-unknown"#),
        h3!["設定項目"],
        p!["首先，建立一個新的 cargo 專案："],
        code_block(
            "bash",
            r#"cargo new yew-app
cd yew-app"#,
        ),
        p![
            "為了驗證 Rust 環境是否設定正確，使用 cargo 建置工具執行初始專案。 \
             在關於建置過程的輸出之後，您應該會看到預期的 \"Hello, world!\" 訊息。"
        ],
        code_block("bash", r#"cargo run"#),
        h2!["我們的第一個靜態頁面"],
        p!["為了將這個簡單的命令列應用程式轉換為一個基本的 Yew web 應用程序，需要進行一些更改。"],
        code_block_title(
            "toml",
            "Cargo.toml",
            format!(
                "[package]\nname = \"yew-app\"\nversion = \"0.1.0\"\nedition = \
                 \"2021\"\n\n[dependencies]\n// highlight-next-line\n{yew_dep_csr}"
            ),
        ),
        admonition(
            AdmonitionType::Info,
            None,
            vec![
                p![
                    "如果你只是正在建立一個應用程序，你只需要 ",
                    code("csr"),
                    " 特性。它將啟用 ",
                    code("Renderer"),
                    " 和所有與客戶端渲染相關的程式碼。",
                ],
                p!["如果你正在製作一個函式庫，請不要啟用此特性，\
                    因為它會將客戶端渲染邏輯拉入伺服器端渲染包中。"],
                p![
                    "如果你需要 Renderer 進行測試或範例，你應該在 ",
                    code("dev-dependencies"),
                    " 中啟用它。",
                ],
            ],
        ),
        code_block_title(
            "rust",
            "src/main.rs",
            r#"use yew::prelude::*;

#[component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}"#,
        ),
        p![
            "現在，讓我們在專案的根目錄中建立一個 ",
            code("index.html"),
            "。",
        ],
        code_block_title(
            "html",
            "index.html",
            r#"<!doctype html>
<html lang="en">
    <head></head>
    <body></body>
</html>"#,
        ),
        h3!["啟動開發伺服器"],
        p!["運行以下命令建置並在本地提供應用程式。"],
        code_block("bash", r#"trunk serve --open"#),
        admonition(
            AdmonitionType::Info,
            None,
            vec![p![
                "刪除選項 '--open' 以在執行 ",
                code("trunk serve"),
                " 後不開啟預設瀏覽器。",
            ]],
        ),
        p![
            "Trunk 將在您修改任何原始程式碼檔案時即時重新建立您的應用程式。 \
             預設情況下，伺服器將在位址 '127.0.0.1' 的連接埠 '8080' 上監聽 => ",
            link!["http://127.0.0.1:8080", "http://localhost:8080"],
            "。 若要變更這部分配置，請建立以下檔案並根據需要進行編輯：",
        ],
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
        p![
            "如果您有興趣，您可以執行 ",
            code("trunk help"),
            " 和 ",
            code("trunk help <subcommand>"),
            " 以獲取更多關於正在進行的流程的詳細資訊。",
        ],
        h3!["恭喜"],
        p!["您現在已經成功設定了 Yew 開發環境，並建立了您的第一個 Yew Web 應用程式。"],
        h2!["建立 HTML"],
        p![
            "Yew 利用了 Rust 的過程宏，並為我們提供了一種類似於 JSX（JavaScript 的擴展，可讓您在 \
             JavaScript 中編寫類似 HTML 的程式碼）的語法來建立標記。"
        ],
        h3!["轉換為經典 HTML"],
        p![
            "由於我們已經對我們的網站長什麼樣子有了一個很好的想法，\
             我們可以簡單地將我們的草稿轉換為與 ",
            code("html!"),
            " 相容的表示。如果您習慣於編寫簡單的 HTML，那麼您在 ",
            code("html!"),
            " 中編寫標記時應該沒有問題。要注意的是，這個巨集與 HTML 有一些不同之處：",
        ],
        ol![
            li!["表達式必須用大括號（", code("{ }"), "）括起來",],
            li![
                "只能有一個根節點。如果您想要在不將它們包裝在容器中的情況下擁有多個元素，\
                 可以使用空標籤/片段（",
                code("<> ... </>"),
                "）",
            ],
            li!["元素必須正確關閉。"],
        ],
        p!["我們想要建立一個佈局，原始 HTML 如下："],
        code_block(
            "html",
            r#"<h1>RustConf Explorer</h1>
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
</div>"#,
        ),
        p![
            "現在，讓我們將這個 HTML 轉換為 ",
            code("html!"),
            "。將下列程式碼片段輸入（或複製/貼上）到 ",
            code("app"),
            " 函數的主體中，以便函數傳回 ",
            code("html!"),
            " 的值",
        ],
        code_block(
            "rust",
            r#"html! {
    <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
            <h3>{"Videos to watch"}</h3>
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
}"#,
        ),
        p!["刷新瀏覽器頁面，您應該看到以下輸出："],
        img(
            "/img/tutorial_application_screenshot.png",
            "Running WASM application screenshot",
        ),
        h3!["在標記中使用 Rust 語言結構"],
        p![
            "在 Rust 中編寫標記的一個很大的優勢是，我們在標記中獲得了 Rust 的所有優點。 \
             現在，我們不再在 HTML 中硬編碼影片列表，而是將它們定義為 ",
            code("Vec"),
            " 的 ",
            code("Video"),
            " 結構體。 我們建立一個簡單的 ",
            code("struct"),
            "（在 ",
            code("main.rs"),
            " 或我們選擇的任何檔案中）來保存我們的資料。",
        ],
        code_block(
            "rust",
            r#"struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}"#,
        ),
        p![
            "接下來，我們將在 ",
            code("app"),
            " 函數中建立這個結構體的實例，並使用它們來取代硬編碼的資料：",
        ],
        code_block(
            "rust",
            r#"use website_test::tutorial::Video; // 換成你自己的路徑

let videos = vec![
    Video {
        id: 1,
        title: "Building and breaking things".to_string(),
        speaker: "John Doe".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 2,
        title: "The development process".to_string(),
        speaker: "Jane Smith".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 3,
        title: "The Web 7.0".to_string(),
        speaker: "Matt Miller".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 4,
        title: "Mouseless development".to_string(),
        speaker: "Tom Jerry".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
];"#,
        ),
        p![
            "為了顯示它們，我們需要將 ",
            code("Vec"),
            " 轉換為 ",
            code("Html"),
            "。我們可以透過建立一個迭代器，將其映射到 ",
            code("html!"),
            " 並將其收集為 ",
            code("Html"),
            " 來實現：",
        ],
        code_block(
            "rust",
            r#"let videos = videos.iter().map(|video| html! {
    <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
}).collect::<Html>();"#,
        ),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p![
                "在清單項目上使用鍵有助於 Yew \
                 追蹤清單中哪些項目發生了變化，從而實現更快的重新渲染。 ",
                link![
                    "/concepts/html/lists.mdx#keyed-lists",
                    "始終建議在清單中使用鍵"
                ],
                "。",
            ]],
        ),
        p![
            "最後，我們需要用從資料建立的 ",
            code("Html"),
            " 取代硬編碼的影片清單：",
        ],
        code_block(
            "rust",
            r#"html! {
    <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
            <h3>{ "Videos to watch" }</h3>
// highlight-start
-           <p>{ "John Doe: Building and breaking things" }</p>
-           <p>{ "Jane Smith: The development process" }</p>
-           <p>{ "Matt Miller: The Web 7.0" }</p>
-           <p>{ "Tom Jerry: Mouseless development" }</p>
+           { videos }
// highlight-end
        </div>
        // ...
    </>
}"#,
        ),
        h2!["元件"],
        p![
            "組件是 Yew 應用程式的構建塊。透過組合組件（可以由其他組件組成），\
             我們建立我們的應用程式。透過為可重複使用性建立元件並保持它們的通用性，\
             我們將能夠在應用程式的多個部分中使用它們，而無需重複程式碼或邏輯。"
        ],
        p![
            "到目前為止我們一直在使用的 ",
            code("app"),
            " 函數是一個元件，稱為 ",
            code("App"),
            "。它是一個「函數式元件」。",
        ],
        ol![li!["結構體組件"], li!["函數式組件"],],
        p!["在本教程中，我們將使用函數式元件。"],
        p![
            "現在，讓我們將 ",
            code("App"),
            " 元件拆分為更小的元件。我們首先將影片清單提取到自己的組件中。",
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
}

#[component(VideosList)]
fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos.iter().map(|video| html! {
        <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
    }).collect()
}"#,
        ),
        p![
            "注意我們的 ",
            code("VideosList"),
            " 函數元件的參數。函數元件只接受一個參數，該參數定義了它的 \"props\"（\"properties\" \
             的縮寫）。 Props 用於從父元件傳遞資料到子元件。在這種情況下，",
            code("VideosListProps"),
            " 是一個定義 props 的結構體。",
        ],
        admonition(
            AdmonitionType::Important,
            None,
            vec![p![
                "用於 props 的結構體必須透過派生實作 ",
                code("Properties"),
                "。",
            ]],
        ),
        p![
            "為了讓上面的程式碼編譯通過，我們需要修改 ",
            code("Video"),
            " 結構體如下：",
        ],
        code_block(
            "rust",
            r#"// highlight-next-line
#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}"#,
        ),
        p![
            "現在，我們可以更新我們的 ",
            code("App"),
            " 元件以使用 ",
            code("VideosList"),
            " 元件。",
        ],
        code_block(
            "rust",
            r#"#[component(App)]
fn app() -> Html {
    // ...
// highlight-start
-    let videos = videos.iter().map(|video| html! {
-        <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
-    }).collect::<Html>();
-
// highlight-end
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
// highlight-start
-               { videos }
+               <VideosList videos={videos} />
// highlight-end
            </div>
            // ...
        </>
    }
}"#,
        ),
        p![
            "透過查看瀏覽器窗口，我們可以驗證清單是否按預期呈現。\
             我們已經將清單的渲染邏輯移動到了它的元件中。這縮短了 ",
            code("App"),
            " 元件的原始程式碼，使我們更容易閱讀和理解。",
        ],
        h3!["使應用程式可以交互"],
        p![
            "這裡的最終目標是顯示所選影片。為了做到這一點，",
            code("VideosList"),
            " 元件需要在選擇影片時「通知」其父元件，這是透過 ",
            code("Callback"),
            " 完成的。這個概念稱為「傳遞處理程序」。我們修改其 props 以接受一個 ",
            code("on_click"),
            " 回呼：",
        ],
        code_block(
            "rust",
            r#"#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
// highlight-next-line
+    on_click: Callback<Video>
}"#,
        ),
        p![
            "然後我們修改 ",
            code("VideosList"),
            " 元件以將所選影片傳遞給回呼。",
        ],
        code_block(
            "rust",
            r#"#[component(VideosList)]
// highlight-start
-fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
+fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
+    let on_click = on_click.clone();
// highlight-end
    videos.iter().map(|video| {
// highlight-start
+        let on_video_select = {
+            let on_click = on_click.clone();
+            let video = video.clone();
+            Callback::from(move |_| {
+                on_click.emit(video.clone())
+            })
+        };
// highlight-end

        html! {
// highlight-start
-            <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
+            <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
// highlight-end
        }
    }).collect()
}"#,
        ),
        p![
            "接下來，我們需要修改 ",
            code("VideosList"),
            " 的使用以傳遞該回呼。但在這樣做之前，我們應該建立一個新的元件 ",
            code("VideoDetails"),
            "，當點擊影片時才會顯示。",
        ],
        code_block(
            "rust",
            r#"use website_test::tutorial::Video;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://placehold.co/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}"#,
        ),
        p![
            "現在，修改 ",
            code("App"),
            " 元件以在選擇影片時顯示 ",
            code("VideoDetails"),
            " 元件。",
        ],
        code_block(
            "rust",
            r#"#[component(App)]
fn app() -> Html {
    // ...
// highlight-next-line
+    let selected_video = use_state(|| None);

// highlight-start
+    let on_video_select = {
+        let selected_video = selected_video.clone();
+        Callback::from(move |video: Video| {
+            selected_video.set(Some(video))
+        })
+    };
// highlight-end

// highlight-start
+    let details = selected_video.as_ref().map(|video| html! {
+        <VideoDetails video={video.clone()} />
+    });
// highlight-end

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
// highlight-start
-               <VideosList videos={videos} />
+               <VideosList videos={videos} on_click={on_video_select.clone()} />
// highlight-end
            </div>
// highlight-start
+            { for details }
-            <div>
-                <h3>{ "John Doe: Building and breaking things" }</h3>
-                <img src="https://placehold.co/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
-            </div>
// highlight-end
        </>
    }
}"#,
        ),
        p![
            "現在不用擔心 ",
            code("use_state"),
            "，我們稍後會回到這個問題。注意我們用 ",
            code("{ for details }"),
            " 提取列表資料的技巧。 ",
            code("Option<_>"),
            " 實作了",
            code("Iterator"),
            "，所以我們可以使用特殊的",
            code("{ for ... }"),
            " 語法來逐個顯示",
            code("Iterator"),
            " 返回的唯一元素，而這",
            link!["concepts/html/lists", "由", code("html!"), " 巨集支援"],
            "。",
        ],
        h3!["處理狀態"],
        p![
            "還記得之前使用的 ",
            code("use_state"),
            " 嗎？那是一個特殊的函數，稱為 \"hook\"。 Hooks 用於 \"hook\" \
             到函數元件的生命週期中並執行操作。您可以在",
            link![
                "concepts/function-components/hooks/introduction.mdx#pre-defined-hooks",
                "這裡"
            ],
            "了解更多關於這個 hook 和其他 hook 的資訊。",
        ],
        admonition(
            AdmonitionType::Note,
            None,
            vec![p![
                "結構體組件的行為不同。請查看",
                link!["advanced-topics/struct-components/introduction.mdx", "文件"],
                "以了解有關這些的資訊。",
            ]],
        ),
        h2!["取得資料（使用外部 REST API）"],
        p!["在真實的應用程式中，資料通常來自 API \
            而不是硬編碼。讓我們從外部來源取得我們的影片清單。為此，我們需要添加以下 crate："],
        ul![
            li![
                link!["https://crates.io/crates/gloo-net", code("gloo-net")],
                "\n用於進行 fetch 調用。",
            ],
            li![
                link!["https://serde.rs", code("serde")],
                " 及其衍生特性\n用於反序列化 JSON 回應",
            ],
            li![
                link![
                    "https://crates.io/crates/wasm-bindgen-futures",
                    code("wasm-bindgen-futures")
                ],
                "\n用於將 Rust 的 Future 作為 Promise 執行",
            ],
        ],
        p!["讓我們更新 ", code("Cargo.toml"), " 檔案中的依賴項：",],
        code_block_title(
            "toml",
            "Cargo.toml",
            r#"[dependencies]
gloo-net = "0.6"
serde = { version = "1.0", features = ["derive"] }
wasm-bindgen-futures = "0.4""#,
        ),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p![
                "在選擇依賴項時，請確保它們與 ",
                code("wasm32"),
                " 相容！否則，您將無法運行您的應用程式。",
            ]],
        ),
        p![
            "更新 ",
            code("Video"),
            " 結構體以衍生 ",
            code("Deserialize"),
            " 特性：",
        ],
        code_block(
            "rust",
            r#"// highlight-next-line
+ use serde::Deserialize;

// highlight-start
- #[derive(Clone, PartialEq)]
+ #[derive(Clone, PartialEq, Deserialize)]
// highlight-end
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}"#,
        ),
        p![
            "最後一步，我們需要更新我們的 ",
            code("App"),
            " 元件，以便進行 fetch 請求，而不是使用硬編碼的數據",
        ],
        code_block(
            "rust",
            r#"// highlight-next-line
+ use gloo_net::http::Request;

#[component(App)]
fn app() -> Html {
// highlight-start
-    let videos = vec![
-        // ...
-    ]
+    let videos = use_state(|| vec![]);
+    {
+        let videos = videos.clone();
+        use_effect_with((), move |_| {
+            let videos = videos.clone();
+            wasm_bindgen_futures::spawn_local(async move {
+                let fetched_videos: Vec<Video> = Request::get("https://yew.rs/tutorial/data.json")
+                    .send()
+                    .await
+                    .unwrap()
+                    .json()
+                    .await
+                    .unwrap();
+                videos.set(fetched_videos);
+            });
+            || ()
+        });
+    }
// highlight-end

    // ...

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
// highlight-start
-                <VideosList videos={videos} on_click={on_video_select.clone()} />
+                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
// highlight-end
            </div>
            { for details }
        </>
    }
}"#,
        ),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p![
                "我們在這裡使用 ",
                code("unwrap"),
                "，因為這是一個演示應用程式。在真實的應用程式中，您可能希望有",
                link![
                    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
                    "適當的錯誤處理"
                ],
                "。",
            ]],
        ),
        p![
            "現在，查看瀏覽器，看看一切是否按預期工作……如果不是因為 CORS \
             的話。為了解決這個問題，我們需要一個代理伺服器。幸運的是 trunk 提供了這個功能。"
        ],
        p!["更新這些行："],
        code_block(
            "rust",
            r#"// ...
// highlight-start
-                let fetched_videos: Vec<Video> = Request::get("https://yew.rs/tutorial/data.json")
+                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
// highlight-end
// ..."#,
        ),
        p!["現在，使用以下命令重新運行伺服器："],
        code_block(
            "bash",
            r#"trunk serve --proxy-backend=https://yew.rs/tutorial"#,
        ),
        p!["刷新網頁，一切都應該按預期工作。"],
        h2!["總結"],
        p!["恭喜！您已經建立了一個從外部 API 取得資料並顯示影片清單的 Web 應用程式。"],
        h2!["接下來"],
        p!["這個應用程式離完美或有用還有很長的路要走。完成本教學後，\
            您可以將其作為探索更高級主題的起點。"],
        h3!["樣式"],
        p![
            "我們的應用程式看起來非常醜陋。沒有 CSS 或任何樣式。不幸的是，Yew \
             沒有提供內建的樣式組件。請查看 ",
            link!["https://trunkrs.dev/assets/", "Trunk 的 assets"],
            "，以了解如何新增樣式表。",
        ],
        h3!["更多依賴函式庫"],
        p![
            "我們的應用程式只使用了很少的外部依賴。有很多 crate 可以使用。請查看",
            link!["/community/external-libs", "外部程式庫"],
            "以取得更多詳細資訊。",
        ],
        h3!["了解更多關於 Yew"],
        p![
            "閱讀我們的",
            doc_link![crate::pages::getting_started::introduction, "官方文件"],
            "。它更詳細地解釋了許多概念。要了解有關 Yew API 的更多信息，請查看我們的",
            link!["https://docs.rs/yew", "API 文件"],
            "。",
        ],
    ])
}

pub fn page_content() -> yew_site_lib::Content {
    page_content_versioned(None)
}

crate::doc_page!("教學", "/zh-Hant/docs/tutorial", page_content());
