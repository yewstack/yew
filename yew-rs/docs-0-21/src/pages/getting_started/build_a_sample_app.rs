crate::doc_page!(
    "Build a sample app",
    "/docs/getting-started/build-a-sample-app",
    Content::new(vec![
        p![
            "Once you have the environment ready, you can either choose to use a starter template \
             that contains the boilerplate needed for a basic Yew app or manually set up a small \
             project."
        ],
        h2!["Using a starter template"],
        p![
            "Install ",
            link!(
                "https://github.com/cargo-generate/cargo-generate",
                code("cargo-generate")
            ),
            " by following their installation instructions then take the following steps:",
        ],
        h3!["Checkout and customize project"],
        code_block(
            "shell",
            "cargo generate yewstack/yew-trunk-minimal-template"
        ),
        h3!["Run project"],
        code_block("shell", "trunk serve"),
        admonition![
            AdmonitionType::Note,
            None,
            p![
                "Trunk ",
                link!("https://github.com/trunk-rs/trunk/issues/852", "has a bug"),
                " on windows when ",
                code("trunk serve"),
                " command fails. To workaround the issue you can run ",
                code("trunk build"),
                " before running ",
                code("trunk serve"),
                ".",
            ],
        ],
        h2!["Setting up the application manually"],
        h3!["Create Project"],
        p!["To get started, create a new cargo project."],
        code_block("bash", "cargo new yew-app"),
        p!["Open the newly created directory."],
        code_block("bash", "cd yew-app"),
        h3!["Run a hello world example"],
        p![
            "To verify the Rust environment is set up, run the initial project using ",
            code("cargo run"),
            ". You should see a \"Hello World!\" message.",
        ],
        code_block("bash", "cargo run\n# output: Hello World!"),
        h3!["Setting up the project as a Yew web application"],
        p![
            "To convert this simple command line application to a basic Yew web application, a \
             few changes are needed."
        ],
        h4!["Update Cargo.toml"],
        p!["Add ", code("yew"), " to the list of dependencies.",],
        code_block_title(
            "toml",
            "Cargo.toml",
            r#"[package]
name = "yew-app"
version = "0.1.0"
edition = "2021"

[dependencies]
yew = { version = "0.21", features = ["csr"] }"#
        ),
        admonition![
            AdmonitionType::Info,
            None,
            p![
                "You only need the feature ",
                code("csr"),
                " if you are building an application. It will enable the ",
                code("Renderer"),
                " and all client-side rendering-related code.",
            ],
            p![
                "If you are making a library, do not enable this feature as it will pull in \
                 client-side rendering logic into the server-side rendering bundle."
            ],
            p![
                "If you need the Renderer for testing or examples, you should enable it in the ",
                code("dev-dependencies"),
                " instead.",
            ],
        ],
        h4!["Update main.rs"],
        p![
            "We need to generate a template that sets up a root Component called ",
            code("App"),
            " which renders a button that updates its value when clicked. Replace the contents of ",
            code("src/main.rs"),
            " with the following code.",
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                "The call to ",
                code("yew::Renderer::<App>::new().render()"),
                " inside the ",
                code("main"),
                " function starts your application and mounts it to the page's ",
                code("<body>"),
                " tag. If you would like to start your application with any dynamic properties, \
                 you can instead use ",
                code("yew::Renderer::<App>::with_props(..).render()"),
                ".",
            ],
        ],
        code_block_title(
            "rust",
            "main.rs",
            r#"use yew::prelude::*;

#[function_component]
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
}"#
        ),
        h4!["Create index.html"],
        p![
            "Finally, add an ",
            code("index.html"),
            " file in the root directory of your app.",
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
</html>"#
        ),
        h2!["View your web application"],
        p!["Run the following command to build and serve the application locally."],
        code_block("bash", "trunk serve"),
        admonition![
            AdmonitionType::Info,
            None,
            p![
                "Add option '--open' to open your default browser ",
                code("trunk serve --open"),
                ".",
            ],
        ],
        p![
            "Trunk will rebuild your application if you modify any of its source code files. By \
             default server will be listening at address '127.0.0.1' and port '8080' => ",
            link!("http://127.0.0.1:8080", "http://localhost:8080"),
            ". To change it, create the following file and edit as needed:",
        ],
        code_block_title(
            "toml",
            "Trunk.toml",
            r#"[serve]
# The address to serve on LAN.
address = "127.0.0.1"
# The address to serve on WAN.
# address = "0.0.0.0"
# The port to serve on.
port = 8000"#
        ),
        h2!["Congratulations"],
        p![
            "You have now successfully set up your Yew development environment, and built your \
             first web application."
        ],
        p![
            "Experiment with this application and review the ",
            link!("/docs/getting-started/examples", "examples"),
            " to further your learning.",
        ],
    ])
);
