crate::doc_page!(
    "Tutorial",
    "/zh-Hans/docs/tutorial",
    Content::new(vec![
        h2(vec![text("Introduction")]),
        p(vec![
            text(
                "In this hands-on tutorial, we will take a look at how we can use Yew to build \
                 web applications. "
            ),
            bold(vec![text("Yew")]),
            text(" is a modern "),
            link("https://www.rust-lang.org/", vec![text("Rust")]),
            text(" framework for building front-end web apps using "),
            link("https://webassembly.org/", vec![text("WebAssembly")]),
            link(
                "https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html",
                vec![text("crates")]
            ),
            text(", provide components for commonly-used patterns such as state management. "),
            link("https://doc.rust-lang.org/cargo/", vec![text("Cargo")]),
            text(
                ", the package manager for Rust, allows us to take advantage of the numerous \
                 crates available on "
            ),
            link("https://crates.io", vec![text("crates.io")]),
            text(", such as Yew.")
        ]),
        h3(vec![text("What we are going to build")]),
        p(vec![text(
            "Rustconf is an intergalactic gathering of the Rust community that happens annually. \
             Rustconf 2020 had a plethora of talks that provided a good amount of information. In \
             this hands-on tutorial, we will be building a web application to help fellow \
             Rustaceans get an overview of the talks and watch them all from one page."
        )]),
        h2(vec![text("Setting up")]),
        h3(vec![text("Prerequisites")]),
        p(vec![
            link(
                "https://doc.rust-lang.org/book/ch00-00-introduction.html",
                vec![text("Rust Book")]
            ),
            text(
                " offers a great starting point for beginners and continues to be an excellent \
                 resource even for experienced Rust developers."
            )
        ]),
        p(vec![
            text("Ensure the latest version of Rust is installed by running "),
            code("rustup update"),
            text(" or by "),
            link(
                "https://www.rust-lang.org/tools/install",
                vec![text("installing rust")]
            ),
            text(" if you have not already done so.")
        ]),
        p(vec![text(
            "After installing Rust, you can use Cargo to install trunk by running:"
        )]),
        code_block("bash", "cargo install trunk"),
        p(vec![text(
            "We will also need to add the WASM build target by running:"
        )]),
        code_block("bash", "rustup target add wasm32-unknown-unknown"),
        h3(vec![text("Setting up the project")]),
        p(vec![text("First, create a new cargo project:")]),
        code_block(
            "bash",
            r#"cargo new yew-app
cd yew-app"#
        ),
        p(vec![text(
            "To verify the Rust environment is set up properly, run the initial project using the \
             cargo build tool. After the output about the build process, you should see the \
             expected \"Hello, world!\" message."
        )]),
        code_block("bash", "cargo run"),
        h2(vec![text("Our first static page")]),
        p(vec![text(
            "To convert this simple command line application to a basic Yew web application, a \
             few changes are needed. Update the files as follows:"
        )]),
        code_block(
            "toml",
            r##"[package]
name = "yew-app"
version = "0.1.0"
edition = "2021"

[dependencies]
// highlight-next-line
yew = { version = "0.21", features = ["csr"] }"##
        ),
        admonition(
            AdmonitionType::Info,
            None,
            vec![
                p(vec![
                    text("You only need the feature "),
                    code("csr"),
                    code("Renderer"),
                    text(" and all client-side rendering-related code.")
                ]),
                p(vec![text(
                    "If you are making a library, do not enable this feature as it will pull in \
                     client-side rendering logic into the server-side rendering bundle."
                )]),
                p(vec![code("dev-dependencies"), text(" instead.")])
            ]
        ),
        code_block(
            "rust",
            r##"use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}"##
        ),
        p(vec![
            text("Now, let's create an "),
            code("index.html"),
            text(" at the root of the project.")
        ]),
        code_block(
            "html",
            r#"<!doctype html>
<html lang="en">
    <head></head>
    <body></body>
</html>"#
        ),
        h3(vec![text("Start the development server")]),
        p(vec![text(
            "Run the following command to build and serve the application locally."
        )]),
        code_block("bash", "trunk serve --open"),
        admonition(
            AdmonitionType::Info,
            None,
            vec![p(vec![
                text("Remove option '--open' to not open your default browser "),
                code("trunk serve"),
                text(".")
            ])]
        ),
        p(vec![
            link("http://127.0.0.1:8080", vec![text("http://localhost:8080")]),
            text(". To change it, create the following file and edit as needed:")
        ]),
        code_block(
            "toml",
            r##"[serve]
# The address to serve on LAN.
address = "127.0.0.1"
# The address to serve on WAN.
# address = "0.0.0.0"
# The port to serve on.
port = 8000"##
        ),
        p(vec![
            text("If you are curious, you can run "),
            code("trunk help"),
            text(" and "),
            code("trunk help <subcommand>"),
            text(" for more details on what is happening.")
        ]),
        h3(vec![text("Congratulations")]),
        p(vec![text(
            "You have now successfully set up your Yew development environment and built your \
             first Yew web application."
        )]),
        h2(vec![text("Building HTML")]),
        p(vec![text(
            "Yew makes use of Rust's procedural macros and provides us with a syntax similar to \
             JSX (an extension to JavaScript which allows you to write HTML-like code inside \
             JavaScript) to create the markup."
        )]),
        h3(vec![text("Converting classic HTML")]),
        p(vec![
            code("html!"),
            code("html!"),
            text(". It is important to note that the macro does differ from HTML in a few ways:")
        ]),
        ol(vec![
            li(vec![
                text("Expressions must be wrapped in curly braces ("),
                code("{{ }}"),
                text(")")
            ]),
            li(vec![code("<> ... </>"), text(") is used")]),
            li(vec![text("Elements must be closed properly.")])
        ]),
        p(vec![text(
            "We want to build a layout that looks something like this in raw HTML:"
        )]),
        code_block(
            "html",
            r##"<h1>RustConf Explorer</h1>
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
</div>"##
        ),
        p(vec![
            text("Now, let's convert this HTML into "),
            code("html!"),
            text(". Type (or copy/paste) the following snippet into the body of "),
            code("app"),
            text(" function such that the value of "),
            code("html!"),
            text(" is returned by the function")
        ]),
        code_block(
            "rust",
            r##"html! {
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
}"##
        ),
        p(vec![text(
            "Refresh the browser page, and you should see the following output displayed:"
        )]),
        img(
            "/img/tutorial_application_screenshot.png",
            "Running WASM application screenshot"
        ),
        h3(vec![text("Using Rust language constructs in the markup")]),
        p(vec![
            code("Vec"),
            text(" of "),
            code("Video"),
            text(" structs. We create a simple "),
            code("struct"),
            text(" (in "),
            code("main.rs"),
            text(" or any file of our choice) that will hold our data.")
        ]),
        code_block(
            "rust",
            r##"struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}"##
        ),
        p(vec![
            text("Next, we will create instances of this struct in our "),
            code("app"),
            text(" function and use those instead of hardcoding the data:")
        ]),
        code_block(
            "rust",
            r##"let videos = vec![
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
];"##
        ),
        p(vec![
            text("To display them, we need to convert the "),
            code("Vec"),
            text(" into "),
            code("Html"),
            text(". We can do that by creating an iterator, mapping it to "),
            code("html!"),
            text(" and collecting it as "),
            code("Html"),
            text(":")
        ]),
        code_block(
            "rust",
            r##"let videos = videos.iter().map(|video| html! {
    <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
}).collect::<Html>();"##
        ),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![
                text(
                    "Keys on list items help Yew keep track of which items have changed in the \
                     list, resulting in faster re-renders. "
                ),
                link(
                    "/docs/0.21/concepts/html/lists#keyed-lists",
                    vec![text("It is always recommended to use keys in lists")]
                ),
                text(".")
            ])]
        ),
        p(vec![
            text("And finally, we need to replace the hardcoded list of videos with the "),
            code("Html"),
            text(" we created from the data:")
        ]),
        code_block(
            "rust",
            r##"html! {
    <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
            <h3>{ "Videos to watch" }</h3>
// highlight-next-line
            { videos }
        </div>
        // ...
    </>
}"##
        ),
        h2(vec![text("Components")]),
        p(vec![text(
            "Components are the building blocks of Yew applications. By combining components, \
             which can be made of other components, we build our application. By structuring our \
             components for re-usability and keeping them generic, we will be able to use them in \
             multiple parts of our application without having to duplicate code or logic."
        )]),
        p(vec![
            text("The "),
            code("app"),
            text(" function we have been using so far is a component, called "),
            code("App"),
            text(
                ". It is a \"function component\". There are two different types of components in \
                 Yew."
            )
        ]),
        ol(vec![
            li(vec![text("Struct Components")]),
            li(vec![text("Function Components")])
        ]),
        p(vec![text(
            "In this tutorial, we will be using function components."
        )]),
        p(vec![
            text("Now, let's split up our "),
            code("App"),
            text(
                " component into smaller components. We begin by extracting the videos list into \
                 its own component."
            )
        ]),
        code_block(
            "rust",
            r##"use yew::prelude::*;

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

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos.iter().map(|video| html! {
        <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
    }).collect()
}"##
        ),
        p(vec![
            text("Notice the parameters of our "),
            code("VideosList"),
            code("VideosListProps"),
            text(" is a struct that defines the props.")
        ]),
        admonition(
            AdmonitionType::Warning,
            Some("Important"),
            vec![p(vec![
                text("The struct used for props must implement "),
                code("Properties"),
                text(" by deriving it.")
            ])]
        ),
        p(vec![text(
            "For the above code to compile, we need to modify the Video struct like this:"
        )]),
        code_block(
            "rust",
            r##"// highlight-next-line
#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}"##
        ),
        p(vec![
            text("Now, we can update our "),
            code("App"),
            text(" component to make use of "),
            code("VideosList"),
            text(" component.")
        ]),
        code_block(
            "rust",
            r##"#[function_component(App)]
fn app() -> Html {
    // ...
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
// highlight-next-line
                <VideosList videos={videos} />
            </div>
            // ...
        </>
    }
}"##
        ),
        p(vec![
            code("App"),
            text(" component's source code, making it easier for us to read and understand.")
        ]),
        h3(vec![text("Making it interactive")]),
        p(vec![
            text("The final goal here is to display the selected video. To do that, "),
            code("VideosList"),
            text(
                " component needs to \"notify\" its parent when a video is selected, which is \
                 done via a "
            ),
            code("Callback"),
            text(". This concept is called \"passing handlers\". We modify its props to take an "),
            code("on_click"),
            text(" callback:")
        ]),
        code_block(
            "rust",
            r##"#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
// highlight-next-line
    on_click: Callback<Video>,
}"##
        ),
        p(vec![
            text("Then we modify the "),
            code("VideosList"),
            text(" component to \"emit\" the selected video to the callback.")
        ]),
        code_block(
            "rust",
            r##"#[function_component(VideosList)]
// highlight-start
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
// highlight-end
    videos.iter().map(|video| {
// highlight-start
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        };
// highlight-end

        html! {
// highlight-next-line
            <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect()
}"##
        ),
        p(vec![
            text("Next, we need to modify the usage of "),
            code("VideosList"),
            code("VideoDetails"),
            text(", that is displayed when a video is clicked.")
        ]),
        code_block(
            "rust",
            r##"#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://placehold.co/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}"##
        ),
        p(vec![
            text("Now, modify the "),
            code("App"),
            text(" component to display "),
            code("VideoDetails"),
            text(" component whenever a video is selected.")
        ]),
        code_block(
            "rust",
            r#"// highlight-next-line
let selected_video = use_state(|| None);

// highlight-start
let on_video_select = {
    let selected_video = selected_video.clone();
    Callback::from(move |video: Video| {
        selected_video.set(Some(video))
    })
};
// highlight-end

// highlight-start
let details = selected_video.as_ref().map(|video| html! {
    <VideoDetails video={video.clone()} />
});
// highlight-end

html! {
    <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
            <h3>{"Videos to watch"}</h3>
// highlight-next-line
            <VideosList videos={videos} on_click={on_video_select.clone()} />
        </div>
// highlight-next-line
        { for details }
    </>
}"#
        ),
        p(vec![
            text("Do not worry about the "),
            code("use_state"),
            code("{{ for details }}"),
            text(". "),
            code("Option<_>"),
            text(" implements "),
            code("Iterator"),
            text(" so we can use it to display the only element returned by the "),
            code("Iterator"),
            text(" with a special "),
            code("{{ for ... }}"),
            text(" syntax "),
            link(
                "/docs/0.21/concepts/html/lists",
                vec![text("supported by the html! macro")]
            ),
            text(".")
        ]),
        h3(vec![text("Handling state")]),
        p(vec![
            text("Remember the "),
            code("use_state"),
            link(
                "/docs/0.21/concepts/function-components/hooks/introduction#pre-defined-hooks",
                vec![text("here")]
            ),
            text(".")
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("Struct components act differently. See "),
                link(
                    "/docs/0.21/advanced-topics/struct-components",
                    vec![text("the documentation")]
                ),
                text(" to learn about those.")
            ])]
        ),
        h2(vec![text("Fetching data (using external REST API)")]),
        p(vec![text(
            "In a real-world application, data will usually come from an API instead of being \
             hardcoded. Let's fetch our videos list from an external source. For this we will \
             need to add the following crates:"
        )]),
        ul(vec![
            li(vec![
                link("https://crates.io/crates/gloo-net", vec![code("gloo-net")]),
                text(" - For making the fetch call.")
            ]),
            li(vec![
                link("https://serde.rs", vec![code("serde")]),
                text(" with derive features - For de-serializing the JSON response")
            ]),
            li(vec![
                link(
                    "https://crates.io/crates/wasm-bindgen-futures",
                    vec![code("wasm-bindgen-futures")]
                ),
                text(" - For executing Rust Future as a Promise")
            ])
        ]),
        p(vec![
            text("Let's update the dependencies in "),
            code("Cargo.toml"),
            text(" file:")
        ]),
        code_block(
            "toml",
            r##"[dependencies]
gloo-net = "0.2"
serde = { version = "1.0", features = ["derive"] }
wasm-bindgen-futures = "0.4""##
        ),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("When choosing dependencies make sure they are "),
                code("wasm32"),
                text(" compatible! Otherwise you won't be able to run your application.")
            ])]
        ),
        p(vec![
            text("Update the "),
            code("Video"),
            text(" struct to derive the "),
            code("Deserialize"),
            text(" trait:")
        ]),
        code_block(
            "rust",
            r##"// highlight-next-line
use serde::Deserialize;

// highlight-next-line
#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}"##
        ),
        p(vec![
            text("Now as the last step, we need to update our "),
            code("App"),
            text(" component to make the fetch request instead of using hardcoded data")
        ]),
        code_block(
            "rust",
            r##"// highlight-next-line
use gloo_net::http::Request;

#[function_component(App)]
fn app() -> Html {
// highlight-start
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("https://yew.rs/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
            || ()
        });
    }
// highlight-end

    // ...

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
// highlight-next-line
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}"##
        ),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                text("We are using "),
                code("unwrap"),
                text(
                    "s here because this is a demo application. In a real-world app, you would \
                     likely want to have "
                ),
                link(
                    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
                    vec![text("proper error handling")]
                ),
                text(".")
            ])]
        ),
        p(vec![text(
            "Now, look at the browser to see everything working as expected... which would have \
             been the case if it were not for CORS. To fix that, we need a proxy server. Luckily \
             trunk provides that."
        )]),
        p(vec![text("Update the following line:")]),
        code_block(
            "rust",
            r##"// highlight-next-line
let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")"##
        ),
        p(vec![text(
            "Now, rerun the server with the following command:"
        )]),
        code_block(
            "bash",
            "trunk serve --proxy-backend=https://yew.rs/tutorial"
        ),
        p(vec![text(
            "Refresh the tab and everything should work as expected."
        )]),
        h2(vec![text("Wrapping up")]),
        p(vec![text(
            "Congratulations! You've created a web application that fetches data from an external \
             API and displays a list of videos."
        )]),
        h2(vec![text("What's next")]),
        p(vec![text(
            "This application is very far from perfect or useful. After going through this \
             tutorial, you can use it as a jumping-off point to explore more advanced topics."
        )]),
        h3(vec![text("Styles")]),
        p(vec![
            link("https://trunkrs.dev/assets/", vec![text("Trunk's assets")]),
            text(" to learn how to add style sheets.")
        ]),
        h3(vec![text("More libraries")]),
        p(vec![
            link("/community/external-libs", vec![text("external libraries")]),
            text(" for more details.")
        ]),
        h3(vec![text("Learning more about Yew")]),
        p(vec![
            text("Read our "),
            link(
                "/docs/0.21/getting-started",
                vec![text("official documentation")]
            ),
            link("https://docs.rs/yew", vec![text("API docs")]),
            text(".")
        ])
    ])
);
