crate::doc_page!(
    "Build a sample app",
    "/docs/getting-started/build-a-sample-app",
    Content::new(vec![
        p(vec![text(
            "Once you have the environment ready, you can either choose to use a starter template \
             that contains the boilerplate needed for a basic Yew app or manually set up a small \
             project."
        ),]),
        h2(vec![text("Using a starter template")]),
        p(vec![
            text("Install "),
            link(
                "https://github.com/cargo-generate/cargo-generate",
                vec![code("cargo-generate"),]
            ),
            text(" by following their installation instructions then take the following steps:"),
        ]),
        h3(vec![text("Checkout and customize project")]),
        code_block(
            "shell",
            "cargo generate yewstack/yew-trunk-minimal-template"
        ),
        h3(vec![text("Run project")]),
        code_block("shell", "trunk serve"),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("Trunk "),
                link(
                    "https://github.com/trunk-rs/trunk/issues/852",
                    vec![text("has a bug")]
                ),
                text(" on windows when "),
                code("trunk serve"),
                text(" command fails. To workaround the issue you can run "),
                code("trunk build"),
                text(" before running "),
                code("trunk serve"),
                text("."),
            ]),]
        ),
        h2(vec![text("Setting up the application manually")]),
        h3(vec![text("Create Project")]),
        p(vec![text("To get started, create a new cargo project.")]),
        code_block("bash", "cargo new yew-app"),
        p(vec![text("Open the newly created directory.")]),
        code_block("bash", "cd yew-app"),
        h3(vec![text("Run a hello world example")]),
        p(vec![
            text("To verify the Rust environment is set up, run the initial project using "),
            code("cargo run"),
            text(". You should see a \"Hello World!\" message."),
        ]),
        code_block("bash", "cargo run\n# output: Hello World!"),
        h3(vec![text(
            "Setting up the project as a Yew web application"
        )]),
        p(vec![text(
            "To convert this simple command line application to a basic Yew web application, a \
             few changes are needed."
        ),]),
        h4(vec![text("Update Cargo.toml")]),
        p(vec![
            text("Add "),
            code("yew"),
            text(" to the list of dependencies."),
        ]),
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
        admonition(
            AdmonitionType::Info,
            None,
            vec![
                p(vec![
                    text("You only need the feature "),
                    code("csr"),
                    text(" if you are building an application. It will enable the "),
                    code("Renderer"),
                    text(" and all client-side rendering-related code."),
                ]),
                p(vec![text(
                    "If you are making a library, do not enable this feature as it will pull in \
                     client-side rendering logic into the server-side rendering bundle."
                ),]),
                p(vec![
                    text(
                        "If you need the Renderer for testing or examples, you should enable it \
                         in the "
                    ),
                    code("dev-dependencies"),
                    text(" instead."),
                ]),
            ]
        ),
        h4(vec![text("Update main.rs")]),
        p(vec![
            text("We need to generate a template that sets up a root Component called "),
            code("App"),
            text(
                " which renders a button that updates its value when clicked. Replace the \
                 contents of "
            ),
            code("src/main.rs"),
            text(" with the following code."),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("The call to "),
                code("yew::Renderer::<App>::new().render()"),
                text(" inside the "),
                code("main"),
                text(" function starts your application and mounts it to the page's "),
                code("<body>"),
                text(
                    " tag. If you would like to start your application with any dynamic \
                     properties, you can instead use "
                ),
                code("yew::Renderer::<App>::with_props(..).render()"),
                text("."),
            ]),]
        ),
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
        h4(vec![text("Create index.html")]),
        p(vec![
            text("Finally, add an "),
            code("index.html"),
            text(" file in the root directory of your app."),
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
</html>"#
        ),
        h2(vec![text("View your web application")]),
        p(vec![text(
            "Run the following command to build and serve the application locally."
        )]),
        code_block("bash", "trunk serve"),
        admonition(
            AdmonitionType::Info,
            None,
            vec![p(vec![
                text("Add option '--open' to open your default browser "),
                code("trunk serve --open"),
                text("."),
            ]),]
        ),
        p(vec![
            text(
                "Trunk will rebuild your application if you modify any of its source code files. \
                 By default server will be listening at address '127.0.0.1' and port '8080' => "
            ),
            link("http://127.0.0.1:8080", vec![text("http://localhost:8080")]),
            text(". To change it, create the following file and edit as needed:"),
        ]),
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
        h2(vec![text("Congratulations")]),
        p(vec![text(
            "You have now successfully set up your Yew development environment, and built your \
             first web application."
        ),]),
        p(vec![
            text("Experiment with this application and review the "),
            link("/docs/getting-started/examples", vec![text("examples")]),
            text(" to further your learning."),
        ]),
    ])
);
