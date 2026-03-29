pub fn page_content_versioned(version: Option<&str>) -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    let yew_dep_csr = match version {
        Some(v) => format!("yew = {{ version = \"{v}\", features = [\"csr\"] }}"),
        None => {
            "yew = { git = \"https://github.com/yewstack/yew/\", features = [\"csr\"] }".to_string()
        }
    };
    Content::new(vec![
        h2!["介绍"],
        p![
            "在这个实践教程中，我们将学习如何使用 Yew 构建 Web 应用程序。 ",
            bold!["Yew"],
            " 是一个现代的 ",
            link!["https://www.rust-lang.org/", "Rust"],
            " 框架，用于使用 ",
            link!["https://webassembly.org/", "WebAssembly"],
            " 构建前端 Web 应用程序。 Yew 通过利用 Rust \
             强大的类型系统，鼓励可重用、可维护和良好结构化的架构。 \
             一个庞大的社区创建的库生态系统，称为 Rust 中的 ",
            link![
                "https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html",
                "crates"
            ],
            "，为常用模式（如状态管理）提供了组件。 Rust 的包管理器 ",
            link!["https://doc.rust-lang.org/cargo/", "Cargo"],
            " 允许我们利用 ",
            link!["https://crates.io", "crates.io"],
            " 上提供的大量 crate，例如 Yew。",
        ],
        h3!["我们将要构建的内容"],
        p!["Rustconf 是 Rust 社区每年举办的星际聚会。 Rustconf 2020 \
            有大量的演讲，提供了大量的信息。 在这个实践教程中，我们将构建一个 Web \
            应用程序，帮助其他 Rustaceans 了解这些演讲并从一个页面观看它们。"],
        h2!["设置"],
        h3!["先决条件"],
        p![
            "这个教程假设您已经熟悉 Rust。如果您是 Rust 的新手，免费的 ",
            link![
                "https://doc.rust-lang.org/book/ch00-00-introduction.html",
                "Rust 书"
            ],
            " 为初学者提供了一个很好的起点，并且即使对于有经验的 Rust \
             开发人员来说，它仍然是一个很好的资源。",
        ],
        p![
            "确保安装了最新版本的 Rust，方法是运行 ",
            code("rustup update"),
            " 或者",
            link!["https://www.rust-lang.org/tools/install", "安装 Rust"],
            "。",
        ],
        p![
            "安装 Rust 后，您可以使用 Cargo 运行以下命令安装 ",
            code("trunk"),
            "：",
        ],
        code_block("bash", r#"cargo install trunk"#),
        p!["我们还需要添加 WASM 构建目标，运行以下命令："],
        code_block("bash", r#"rustup target add wasm32-unknown-unknown"#),
        h3!["设置项目"],
        p!["首先，创建一个新的 cargo 项目："],
        code_block(
            "bash",
            r#"cargo new yew-app
cd yew-app"#,
        ),
        p![
            "为了验证 Rust 环境是否设置正确，使用 cargo 构建工具运行初始项目。 \
             在关于构建过程的输出之后，您应该看到预期的 \"Hello, world!\" 消息。"
        ],
        code_block("bash", r#"cargo run"#),
        h2!["我们的第一个静态页面"],
        p!["为了将这个简单的命令行应用程序转换为一个基本的 Yew web 应用程序，需要进行一些更改。"],
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
                    "如果你只是正在构建一个应用程序，你只需要 ",
                    code("csr"),
                    " 特性。它将启用 ",
                    code("Renderer"),
                    " 和所有与客户端渲染相关的代码。",
                ],
                p!["如果你正在制作一个库，请不要启用此特性，\
                    因为它会将客户端渲染逻辑拉入服务器端渲染包中。"],
                p![
                    "如果你需要 Renderer 进行测试或示例，你应该在 ",
                    code("dev-dependencies"),
                    " 中启用它。",
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
            "现在，让我们在项目的根目录创建一个 ",
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
        h3!["启动开发服务器"],
        p!["运行以下命令构建并在本地提供应用程序。"],
        code_block("bash", r#"trunk serve --open"#),
        admonition(
            AdmonitionType::Info,
            None,
            vec![p![
                "删除选项 '--open' 以在运行 ",
                code("trunk serve"),
                " 后不打开默认浏览器。",
            ]],
        ),
        p![
            "Trunk 将在您修改任何源代码文件时实时重新构建您的应用程序。 \
             默认情况下，服务器将在地址 '127.0.0.1' 的端口 '8080' 上监听 => ",
            link!["http://127.0.0.1:8080", "http://localhost:8080"],
            "。 要更改这部分配置，请创建以下文件并根据需要进行编辑：",
        ],
        code_block_title(
            "toml",
            "Trunk.toml",
            r#"[serve]
# 局域网上的监听地址
address = "127.0.0.1"
# 广域网上的监听地址
# address = "0.0.0.0"
# 监听的端口
port = 8000"#,
        ),
        p![
            "如果您感兴趣，您可以运行 ",
            code("trunk help"),
            " 和 ",
            code("trunk help <subcommand>"),
            " 以获取更多关于正在进行的流程的详细信息。",
        ],
        h3!["恭喜"],
        p!["您现在已经成功设置了 Yew 开发环境，并构建了您的第一个 Yew Web 应用程序。"],
        h2!["构建 HTML"],
        p![
            "Yew 利用了 Rust 的过程宏，并为我们提供了一种类似于 JSX（JavaScript 的扩展，允许您在 \
             JavaScript 中编写类似 HTML 的代码）的语法来创建标记。"
        ],
        h3!["转换为经典 HTML"],
        p![
            "由于我们已经对我们的网站长什么样有了一个很好的想法，\
             我们可以简单地将我们的草稿转换为与 ",
            code("html!"),
            " 兼容的表示。如果您习惯于编写简单的 HTML，那么您在 ",
            code("html!"),
            " 中编写标记时应该没有问题。需要注意的是，这个宏与 HTML 有一些不同之处：",
        ],
        ol![
            li!["表达式必须用大括号（", code("{ }"), "）括起来",],
            li![
                "只能有一个根节点。如果您想要在不将它们包装在容器中的情况下拥有多个元素，\
                 可以使用空标签/片段（",
                code("<> ... </>"),
                "）",
            ],
            li!["元素必须正确关闭。"],
        ],
        p!["我们想要构建一个布局，原始 HTML 如下："],
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
            "现在，让我们将这个 HTML 转换为 ",
            code("html!"),
            "。将以下代码片段输入（或复制/粘贴）到 ",
            code("app"),
            " 函数的主体中，以便函数返回 ",
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
        p!["刷新浏览器页面，您应该看到以下输出："],
        img(
            "/img/tutorial_application_screenshot.png",
            "Running WASM application screenshot",
        ),
        h3!["在标记中使用 Rust 语言结构"],
        p![
            "在 Rust 中编写标记的一个很大的优势是，我们在标记中获得了 Rust 的所有优点。 \
             现在，我们不再在 HTML 中硬编码视频列表，而是将它们定义为 ",
            code("Vec"),
            " 的 ",
            code("Video"),
            " 结构体。 我们创建一个简单的 ",
            code("struct"),
            "（在 ",
            code("main.rs"),
            " 或我们选择的任何文件中）来保存我们的数据。",
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
            "接下来，我们将在 ",
            code("app"),
            " 函数中创建这个结构体的实例，并使用它们来替代硬编码的数据：",
        ],
        code_block(
            "rust",
            r#"use website_test::tutorial::Video; // 换成你自己的路径

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
            "为了显示它们，我们需要将 ",
            code("Vec"),
            " 转换为 ",
            code("Html"),
            "。我们可以通过创建一个迭代器，将其映射到 ",
            code("html!"),
            " 并将其收集为 ",
            code("Html"),
            " 来实现：",
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
                "在列表项上使用键有助于 Yew 跟踪列表中哪些项发生了变化，从而实现更快的重新渲染。",
                link![
                    "/concepts/html/lists.mdx#keyed-lists",
                    "始终建议在列表中使用键"
                ],
                "。",
            ]],
        ),
        p![
            "最后，我们需要用从数据创建的 ",
            code("Html"),
            " 替换硬编码的视频列表：",
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
        h2!["组件"],
        p![
            "组件是 Yew 应用程序的构建块。通过组合组件（可以由其他组件组成），\
             我们构建我们的应用程序。通过为可重用性构建组件并保持它们的通用性，\
             我们将能够在应用程序的多个部分中使用它们，而无需重复代码或逻辑。"
        ],
        p![
            "到目前为止我们一直在使用的 ",
            code("app"),
            " 函数是一个组件，称为 ",
            code("App"),
            "。它是一个\"函数式组件\"。",
        ],
        ol![li!["结构体组件"], li!["函数式组件"],],
        p!["在本教程中，我们将使用函数式组件。"],
        p![
            "现在，让我们将 ",
            code("App"),
            " 组件拆分为更小的组件。我们首先将视频列表提取到自己的组件中。",
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
            "注意我们的 ",
            code("VideosList"),
            " 函数组件的参数。函数组件只接受一个参数，该参数定义了它的 \"props\"（\"properties\" \
             的缩写）。Props 用于从父组件传递数据到子组件。在这种情况下，",
            code("VideosListProps"),
            " 是一个定义 props 的结构体。",
        ],
        admonition(
            AdmonitionType::Important,
            None,
            vec![p![
                "用于 props 的结构体必须通过派生实现 ",
                code("Properties"),
                "。",
            ]],
        ),
        p![
            "为了使上面的代码编译通过，我们需要修改 ",
            code("Video"),
            " 结构体如下：",
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
            "现在，我们可以更新我们的 ",
            code("App"),
            " 组件以使用 ",
            code("VideosList"),
            " 组件。",
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
            "通过查看浏览器窗口，我们可以验证列表是否按预期呈现。\
             我们已经将列表的渲染逻辑移动到了它的组件中。这缩短了 ",
            code("App"),
            " 组件的源代码，使我们更容易阅读和理解。",
        ],
        h3!["使应用可以交互"],
        p![
            "这里的最终目标是显示所选视频。为了做到这一点，",
            code("VideosList"),
            " 组件需要在选择视频时\"通知\"其父组件，这是通过 ",
            code("Callback"),
            " 完成的。这个概念称为\"传递处理程序\"。我们修改其 props 以接受一个 ",
            code("on_click"),
            " 回调：",
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
            "然后我们修改 ",
            code("VideosList"),
            " 组件以将所选视频传递给回调。",
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
            "接下来，我们需要修改 ",
            code("VideosList"),
            " 的使用以传递该回调。但在这样做之前，我们应该创建一个新的组件 ",
            code("VideoDetails"),
            "，当点击视频时才会显示。",
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
            "现在，修改 ",
            code("App"),
            " 组件以在选择视频时显示 ",
            code("VideoDetails"),
            " 组件。",
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
            "现在不用担心 ",
            code("use_state"),
            "，我们稍后会回到这个问题。注意我们用 ",
            code("{ for details }"),
            " 提取列表数据的技巧。 ",
            code("Option<_>"),
            " 实现了 ",
            code("Iterator"),
            "，所以我们可以使用特殊的 ",
            code("{ for ... }"),
            " 语法来逐个显示 ",
            code("Iterator"),
            " 返回的唯一元素，而这",
            link!["concepts/html/lists", "由 ", code("html!"), " 宏支持"],
            "。",
        ],
        h3!["处理状态"],
        p![
            "还记得之前使用的 ",
            code("use_state"),
            " 吗？那是一个特殊的函数，称为 \"hook\"。Hooks 用于 \"hook\" \
             到函数组件的生命周期中并执行操作。您可以在",
            link![
                "concepts/function-components/hooks/introduction.mdx#pre-defined-hooks",
                "这里"
            ],
            "了解更多关于这个 hook 和其他 hook 的信息。",
        ],
        admonition(
            AdmonitionType::Note,
            None,
            vec![p![
                "结构体组件的行为不同。请查看",
                link!["advanced-topics/struct-components/introduction.mdx", "文档"],
                "了解有关这些的信息。",
            ]],
        ),
        h2!["获取数据（使用外部 REST API）"],
        p!["在真实的应用程序中，数据通常来自 API \
            而不是硬编码。让我们从外部源获取我们的视频列表。为此，我们需要添加以下 crate："],
        ul![
            li![
                link!["https://crates.io/crates/gloo-net", code("gloo-net")],
                " 用于进行 fetch 调用。",
            ],
            li![
                link!["https://serde.rs", code("serde")],
                " 和其派生特性 用于反序列化 JSON 响应",
            ],
            li![
                link![
                    "https://crates.io/crates/wasm-bindgen-futures",
                    code("wasm-bindgen-futures")
                ],
                " 用于将 Rust 的 Future 作为 Promise 执行",
            ],
        ],
        p!["让我们更新 ", code("Cargo.toml"), " 文件中的依赖项：",],
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
                "在选择依赖项时，请确保它们与 ",
                code("wasm32"),
                " 兼容！否则，您将无法运行您的应用程序。",
            ]],
        ),
        p![
            "更新 ",
            code("Video"),
            " 结构体以派生 ",
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
            "作为最后一步，我们需要更新我们的 ",
            code("App"),
            " 组件，以便进行 fetch 请求，而不是使用硬编码的数据",
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
                "我们在这里使用 ",
                code("unwrap"),
                "，因为这是一个演示应用程序。在真实的应用程序中，您可能希望有",
                link![
                    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
                    "适当的错误处理"
                ],
                "。",
            ]],
        ),
        p![
            "现在，查看浏览器，看看一切是否按预期工作……如果不是因为 CORS \
             的话。为了解决这个问题，我们需要一个代理服务器。幸运的是 trunk 提供了这个功能。"
        ],
        p!["更新这些行："],
        code_block(
            "rust",
            r#"// ...
// highlight-start
-                let fetched_videos: Vec<Video> = Request::get("https://yew.rs/tutorial/data.json")
+                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
// highlight-end
// ..."#,
        ),
        p!["现在，使用以下命令重新运行服务器："],
        code_block(
            "bash",
            r#"trunk serve --proxy-backend=https://yew.rs/tutorial"#,
        ),
        p!["刷新网页，一切应该按预期工作。"],
        h2!["总结"],
        p!["恭喜！您已经创建了一个从外部 API 获取数据并显示视频列表的 Web 应用程序。"],
        h2!["接下来"],
        p!["这个应用程序离完美或有用还有很长的路要走。在完成本教程后，\
            您可以将其作为探索更高级主题的起点。"],
        h3!["样式"],
        p![
            "我们的应用程序看起来非常丑陋。没有 CSS 或任何样式。不幸的是，Yew \
             没有提供内置的样式组件。请查看 ",
            link!["https://trunkrs.dev/assets/", "Trunk 的 assets"],
            "，了解如何添加样式表。",
        ],
        h3!["更多依赖库"],
        p![
            "我们的应用程序只使用了很少的外部依赖。有很多 crate 可以使用。请查看",
            link!["/community/external-libs", "外部库"],
            "以获取更多详细信息。",
        ],
        h3!["了解更多关于 Yew"],
        p![
            "阅读我们的",
            doc_link![crate::pages::getting_started::introduction, "官方文档"],
            "。它更详细地解释了许多概念。要了解有关 Yew API 的更多信息，请查看我们的",
            link!["https://docs.rs/yew", "API 文档"],
            "。",
        ],
    ])
}

pub fn page_content() -> yew_site_lib::Content {
    page_content_versioned(None)
}

crate::doc_page!("教程", "/zh-Hans/docs/tutorial", page_content());
