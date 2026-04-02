pub fn page_content_versioned(version: Option<&str>) -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    let yew_dep = match version {
        Some(v) => format!("yew = {{ version = \"{v}\", features = [\"csr\"] }}"),
        None => "# 开发版本的 Yew\nyew = { git = \"https://github.com/yewstack/yew/\", features = \
                 [\"csr\"] }"
            .to_string(),
    };
    Content::new(vec![
        p!["当您的环境准备好后，您可以选择使用一个包含基本 Yew \
            应用所需样板的起始模板，或手动设置一个小项目。"],
        h2!["使用模板快速起步"],
        p![
            "按照 ",
            link!(
                "https://github.com/cargo-generate/cargo-generate",
                code("cargo-generate"),
            ),
            " 的安装说明安装该工具，然后运行以下命令：",
        ],
        code_block(
            "shell",
            r#"cargo generate yewstack/yew-trunk-minimal-template"#,
        ),
        h2!["手动配置应用"],
        h3!["创建项目"],
        p!["首先，请创建一个新的 cargo 项目。"],
        code_block("bash", r#"cargo new yew-app"#),
        p!["打开新创建的目录。"],
        code_block("bash", r#"cd yew-app"#),
        h3!["运行一个 hello world 示例"],
        p![
            "为了验证 Rust 环境是否设置正确，使用 ",
            code("cargo run"),
            " 运行初始项目。您应该看到一个 \"Hello World!\" 消息。",
        ],
        code_block(
            "bash",
            r#"cargo run
# output: Hello World!"#,
        ),
        h3!["将项目设置为 Yew web 应用"],
        p!["为了将这个简单的命令行应用程序转换为一个基本的 Yew web 应用程序，需要进行一些更改。"],
        h4!["更新 Cargo.toml"],
        p!["将 ", code("yew"), " 添加到依赖列表中。"],
        code_block_title(
            "toml",
            "Cargo.toml",
            format!(
                "[package]\nname = \"yew-app\"\nversion = \"0.1.0\"\nedition = \
                 \"2021\"\n\n[dependencies]\n{yew_dep}"
            ),
        ),
        admonition!(
            AdmonitionType::Info,
            None,
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
        ),
        h4!["更新 main.rs"],
        p![
            "我们需要生成一个模板，设置一个名为 ",
            code("App"),
            " 的根组件，该组件渲染一个按钮，当点击时更新其值。用以下代码替换 ",
            code("src/main.rs"),
            " 的内容。",
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                code("main"),
                " 函数中的 ",
                code("yew::Renderer::<App>::new().render()"),
                " 调用启动您的应用程序并将其挂载到页面的 ",
                code("<body>"),
                " 标签上。如果您想要使用任何动态属性启动您的应用程序，您可以使用 ",
                code("yew::Renderer::<App>::with_props(..).render()"),
                "。",
            ],
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
        h4!["创建 index.html"],
        p![
            "最后，在应用程序的根目录中添加一个 ",
            code("index.html"),
            " 文件。",
        ],
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
        h2!["查看您的 Web 应用"],
        p!["运行以下命令在本地构建和提供应用程序。"],
        code_block("bash", r#"trunk serve"#),
        admonition!(
            AdmonitionType::Info,
            None,
            p![
                "添加选项 '--open' 来打开您的默认浏览器 ",
                code("trunk serve --open"),
                "。",
            ],
        ),
        p![
            "Trunk 将在您修改任何源代码文件时实时重新构建您的应用程序。 \
             默认情况下，服务器将在地址 '127.0.0.1' 的端口 '8080' 上监听 => ",
            link!("http://127.0.0.1:8080", "http://localhost:8080"),
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
        h2!["恭喜"],
        p!["您现在已经成功设置了您的 Yew 开发环境，并构建了您的第一个 Web 应用程序。"],
        p![
            "尝试这个应用程序，并查看",
            doc_link!(crate::pages::getting_started::examples, "示例"),
            "以进一步学习。",
        ],
    ])
}

pub fn page_content() -> yew_site_lib::Content {
    page_content_versioned(None)
}

crate::doc_page!(
    "构建一个示例应用",
    "/zh-Hans/docs/getting-started/build-a-sample-app",
    page_content()
);
