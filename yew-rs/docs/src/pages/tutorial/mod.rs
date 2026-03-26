pub fn page_content_versioned(version: Option<&str>) -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    let yew_dep_csr = match version {
        Some(v) => format!("yew = {{ version = \"{v}\", features = [\"csr\"] }}"),
        None => {
            "yew = { git = \"https://github.com/yewstack/yew/\", features = [\"csr\"] }".to_string()
        }
    };
    let yew_dep_csr_serde = match version {
        Some(v) => format!("yew = {{ version = \"{v}\", features = [\"csr\", \"serde\"] }}"),
        None => "yew = { git = \"https://github.com/yewstack/yew/\", features = [\"csr\", \
                 \"serde\"] }"
            .to_string(),
    };
    Content::new(vec![
        h2!["Introduction"],
        p![
            "In this hands-on tutorial, we will take a look at how we can use Yew to build web \
             applications. ",
            bold!["Yew"],
            " is a modern ",
            link!("https://www.rust-lang.org/", "Rust"),
            " framework for building front-end web apps using ",
            link!("https://webassembly.org/", "WebAssembly"),
            ". Yew encourages a reusable, maintainable, and well-structured architecture by \
             leveraging Rust's powerful type system. A large ecosystem of community-created \
             libraries, known in Rust as ",
            link!(
                "https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html",
                "crates",
            ),
            ", provide components for commonly-used patterns such as state management. ",
            link!("https://doc.rust-lang.org/cargo/", "Cargo"),
            ", the package manager for Rust, allows us to take advantage of the numerous crates \
             available on ",
            link!("https://crates.io", "crates.io"),
            ", such as Yew.",
        ],
        h3!["What we are going to build"],
        p![
            "Rustconf is an intergalactic gathering of the Rust community that happens annually. \
             Rustconf 2020 had a plethora of talks that provided a good amount of information. In \
             this hands-on tutorial, we will be building a web application to help fellow \
             Rustaceans get an overview of the talks and watch them all from one page."
        ],
        h2!["Setting up"],
        h3!["Prerequisites"],
        p![
            "This tutorial assumes you are already familiar with Rust. If you are new to Rust, \
             the free ",
            link!(
                "https://doc.rust-lang.org/book/ch00-00-introduction.html",
                "Rust Book",
            ),
            " offers a great starting point for beginners and continues to be an excellent \
             resource even for experienced Rust developers.",
        ],
        p![
            "Ensure the latest version of Rust is installed by running ",
            code("rustup update"),
            " or by ",
            link!("https://www.rust-lang.org/tools/install", "installing rust",),
            " if you have not already done so.",
        ],
        p![
            "After installing Rust, you can use Cargo to install ",
            code("trunk"),
            " by running:",
        ],
        code_block("bash", "cargo install trunk"),
        p!["We will also need to add the WASM build target by running:"],
        code_block("bash", "rustup target add wasm32-unknown-unknown"),
        h3!["Setting up the project"],
        p!["First, create a new cargo project:"],
        code_block(
            "bash",
            r#"cargo new yew-app
cd yew-app"#,
        ),
        p![
            "To verify the Rust environment is set up properly, run the initial project using the \
             cargo build tool. After the output about the build process, you should see the \
             expected \"Hello, world!\" message."
        ],
        code_block("bash", "cargo run"),
        h2!["Our first static page"],
        p![
            "To convert this simple command line application to a basic Yew web application, a \
             few changes are needed. Update the files as follows:"
        ],
        code_block_title(
            "toml",
            "Cargo.toml",
            format!(
                "[package]\nname = \"yew-app\"\nversion = \"0.1.0\"\nedition = \
                 \"2021\"\n\n[dependencies]\n// highlight-next-line\n{yew_dep_csr}"
            ),
        ),
        admonition!(
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
        ),
        code_block_title_no_run(
            "rust",
            "src/main.rs",
            r##"use yew::prelude::*;

#[component]
fn App() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}"##,
        ),
        p![
            "Now, let's create an ",
            code("index.html"),
            " at the root of the project.",
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
        h3!["Start the development server"],
        p!["Run the following command to build and serve the application locally."],
        code_block("bash", "trunk serve --open"),
        admonition!(
            AdmonitionType::Info,
            None,
            p![
                "Remove option '--open' to not open your default browser ",
                code("trunk serve"),
                ".",
            ],
        ),
        p![
            "Trunk will open your application in your default browser, watch the project \
             directory and helpfully rebuild your application if you modify any source files. \
             This will fail if the socket is being used by another application. By default server \
             will be listening at address '127.0.0.1' and port '8080' => ",
            link!("http://127.0.0.1:8080", "http://localhost:8080"),
            ". To change it, create the following file and edit as needed:",
        ],
        code_block_title(
            "toml",
            "Trunk.toml",
            r##"[serve]
# The address to serve on LAN.
address = "127.0.0.1"
# The address to serve on WAN.
# address = "0.0.0.0"
# The port to serve on.
port = 8000"##,
        ),
        p![
            "If you are curious, you can run ",
            code("trunk help"),
            " and ",
            code("trunk help <subcommand>"),
            " for more details on what is happening.",
        ],
        h3!["Congratulations"],
        p![
            "You have now successfully set up your Yew development environment and built your \
             first Yew web application."
        ],
        h2!["Building HTML"],
        p![
            "Yew makes use of Rust's procedural macros and provides us with a syntax similar to \
             JSX (an extension to JavaScript which allows you to write HTML-like code inside \
             JavaScript) to create the markup."
        ],
        h3!["Converting classic HTML"],
        p![
            "Since we already have a pretty good idea of what our website will look like, we can \
             simply translate our mental draft into a representation compatible with ",
            code("html!"),
            ". If you are comfortable writing simple HTML, you should have no problem writing \
             marking inside ",
            code("html!"),
            ". It is important to note that the macro does differ from HTML in a few ways:",
        ],
        ol![
            li![
                "Expressions must be wrapped in curly braces (",
                code("{ }"),
                ")",
            ],
            li![
                "There must only be one root node. If you want to have multiple elements without \
                 wrapping them in a container, an empty tag/fragment (",
                code("<> ... </>"),
                ") is used",
            ],
            li!["Elements must be closed properly."],
        ],
        p!["We want to build a layout that looks something like this in raw HTML:"],
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
</div>"##,
        ),
        p![
            "Now, let's convert this HTML into ",
            code("html!"),
            ". Type (or copy/paste) the following snippet into the body of ",
            code("app"),
            " function such that the value of ",
            code("html!"),
            " is returned by the function",
        ],
        code_block(
            "rust",
            r##"#[component]
fn App() -> Html {
-   html! {
-       <h1>{ "Hello World" }</h1>
-   }
+   html! {
+       <>
+           <h1>{ "RustConf Explorer" }</h1>
+           <div>
+               <h3>{ "Videos to watch" }</h3>
+               <p>{ "John Doe: Building and breaking things" }</p>
+               <p>{ "Jane Smith: The development process" }</p>
+               <p>{ "Matt Miller: The Web 7.0" }</p>
+               <p>{ "Tom Jerry: Mouseless development" }</p>
+           </div>
+           <div>
+               <h3>{ "John Doe: Building and breaking things" }</h3>
+               <img src="https://placehold.co/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
+           </div>
+       </>
+   }
}"##,
        ),
        p!["Refresh the browser page, and you should see the following output displayed:"],
        img(
            "/img/tutorial_application_screenshot.png",
            "Running WASM application screenshot",
        ),
        h3!["Using Rust language constructs in the markup"],
        p![
            "A big advantage of writing markup in Rust is that we get all the coolness of Rust in \
             our markup. Now, instead of hardcoding the list of videos in the HTML, let's define \
             them as a ",
            code("Vec"),
            " of ",
            code("Video"),
            " structs. We create a simple ",
            code("struct"),
            " (in ",
            code("main.rs"),
            " or any file of our choice) that will hold our data.",
        ],
        code_block(
            "rust",
            r##"#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: AttrValue,
    speaker: AttrValue,
    url: AttrValue,
}"##,
        ),
        p![
            "Next, we will create instances of this struct in our ",
            code("app"),
            " function and use those instead of hardcoding the data:",
        ],
        code_block(
            "rust",
            r##"#[component]
fn App() -> Html {
+   let videos = vec![
+       Video {
+           id: 1,
+           title: "Building and breaking things".into(),
+           speaker: "John Doe".into(),
+           url: "https://youtu.be/PsaFVLr8t4E".into(),
+       },
+       Video {
+           id: 2,
+           title: "The development process".into(),
+           speaker: "Jane Smith".into(),
+           url: "https://youtu.be/PsaFVLr8t4E".into(),
+       },
+       Video {
+           id: 3,
+           title: "The Web 7.0".into(),
+           speaker: "Matt Miller".into(),
+           url: "https://youtu.be/PsaFVLr8t4E".into(),
+       },
+       Video {
+           id: 4,
+           title: "Mouseless development".into(),
+           speaker: "Tom Jerry".into(),
+           url: "https://youtu.be/PsaFVLr8t4E".into(),
+       },
+   ];
+"##,
        ),
        p![
            "To display them, we can use a ",
            code("for"),
            " loop right in the macro in place of the hardcoded HTML:",
        ],
        code_block(
            "rust",
            r#"    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
-               <p>{ "John Doe: Building and breaking things" }</p>
-               <p>{ "Jane Smith: The development process" }</p>
-               <p>{ "Matt Miller: The Web 7.0" }</p>
-               <p>{ "Tom Jerry: Mouseless development" }</p>
+               for video in &videos {
+                   <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
+               }
            </div>
            // ...
        </>
    }"#,
        ),
        admonition!(
            AdmonitionType::Tip,
            None,
            p![
                "Keys on list items help Yew keep track of which items have changed in the list, \
                 resulting in faster re-renders. ",
                link!(
                    "/docs/concepts/html/lists#keyed-lists",
                    "It is always recommended to use keys in lists",
                ),
                ".",
            ],
        ),
        h2!["Components"],
        p![
            "Components are the building blocks of Yew applications. By combining components, \
             which can be made of other components, we build our application. By structuring our \
             components for re-usability and keeping them generic, we will be able to use them in \
             multiple parts of our application without having to duplicate code or logic."
        ],
        p![
            "The ",
            code("app"),
            " function we have been using so far is a component, called ",
            code("App"),
            ". It is a \"function component\". There are two different types of components in Yew.",
        ],
        ol![li!["Struct Components"], li!["Function Components"],],
        p!["In this tutorial, we will be using function components."],
        p![
            "Now, let's split up our ",
            code("App"),
            " component into smaller components. We begin by extracting the videos list into its \
             own component.",
        ],
        code_block(
            "rust",
            r##"#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
}

#[component]
fn VideosList(VideosListProps { videos }: &VideosListProps) -> Html {
    html! {
        for video in videos {
            <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }
}"##,
        ),
        p![
            "Notice the parameters of our ",
            code("VideosList"),
            " function component. A function component takes only one argument which defines its \
             \"props\" (short for \"properties\"). Props are used to pass data down from a parent \
             component to a child component. In this case, ",
            code("VideosListProps"),
            " is a struct that defines the props.",
        ],
        admonition!(
            AdmonitionType::Important,
            None,
            p![
                "The struct used for props must implement ",
                code("Properties"),
                " by deriving it.",
            ],
        ),
        p![
            "Now, we can update our ",
            code("App"),
            " component to make use of ",
            code("VideosList"),
            " component.",
        ],
        code_block(
            "rust",
            r#"#[component]
fn App() -> Html {
    // ...
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
-               for video in &videos {
-                   <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
-               }
+               <VideosList {videos} />
            </div>
            // ...
        </>
    }
}"#,
        ),
        p![
            "By looking at the browser window, we can verify that the lists are rendered as they \
             should be. We have moved the rendering logic of lists to its component. This \
             shortens the ",
            code("App"),
            " component's source code, making it easier for us to read and understand.",
        ],
        h3!["Making it interactive"],
        p![
            "The final goal here is to display the selected video. To do that, ",
            code("VideosList"),
            " component needs to \"notify\" its parent when a video is selected, which is done \
             via a ",
            code("Callback"),
            ". This concept is called \"passing handlers\". We modify its props to take an ",
            code("on_click"),
            " callback:",
        ],
        code_block(
            "rust",
            r##"#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
+   on_click: Callback<Video>,
}"##,
        ),
        p![
            "Then we modify the ",
            code("VideosList"),
            " component to \"emit\" the selected video to the callback.",
        ],
        code_block(
            "rust",
            r##"#[component]
-fn VideosList(VideosListProps { videos }: &VideosListProps) -> Html {
+fn VideosList(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
+   let on_select = |video: &Video| {
+       let on_click = on_click.clone();
+       let video = video.clone();
+       Callback::from(move |_| {
+           on_click.emit(video.clone())
+       })
+   };
+
    html! {
        for video in videos {
-           <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
+           <p key={video.id} onclick={on_select(video)}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }
}"##,
        ),
        p![
            "Next, we need to modify the usage of ",
            code("VideosList"),
            " to pass that callback. But before doing that, we should create a new component, ",
            code("VideoDetails"),
            ", that is displayed when a video is clicked.",
        ],
        code_block(
            "rust",
            r##"#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[component]
fn VideoDetails(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ &*video.title }</h3>
            <img src="https://placehold.co/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}"##,
        ),
        p![
            "Now, modify the ",
            code("App"),
            " component to display ",
            code("VideoDetails"),
            " component whenever a video is selected.",
        ],
        code_block(
            "rust",
            r#"        },
    ];
+
+   let selected_video = use_state(|| None);
+
+   let on_video_select = {
+       let selected_video = selected_video.clone();
+       Callback::from(move |video: Video| {
+           selected_video.set(Some(video))
+       })
+   };

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
-               <VideosList {videos} />
+               <VideosList {videos} on_click={on_video_select} />
            </div>
+           if let Some(video) = &*selected_video {
+               <VideoDetails video={video.clone()} />
+           }
-           <div>
-               <h3>{ "John Doe: Building and breaking things" }</h3>
-               <img src="https://placehold.co/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
-           </div>
        </>
    }"#,
        ),
        h3!["Handling state"],
        p![
            "Remember the ",
            code("use_state"),
            " used earlier? That is a special function, called a \"hook\". Hooks are used to \
             \"hook\" into the lifecycle of a function component and perform actions. You can \
             learn more about this hook, and others ",
            link!(
                "/docs/concepts/function-components/hooks/introduction#pre-defined-hooks",
                "here",
            ),
            ".",
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "Struct components act differently. See ",
                link!(
                    "/docs/advanced-topics/struct-components",
                    "the documentation",
                ),
                " to learn about those.",
            ],
        ),
        h2!["Fetching data (using external REST API)"],
        p![
            "In a real-world application, data will usually come from an API instead of being \
             hardcoded. Let's fetch our videos list from an external source. For this we will \
             need to add the following crates:"
        ],
        ul![
            li![
                link!("https://crates.io/crates/gloo-net", code("gloo-net")),
                " - For making the fetch call.",
            ],
            li![
                link!("https://serde.rs", code("serde")),
                " with derive features - For de-serializing the JSON response",
            ],
            li![
                link!(
                    "https://crates.io/crates/wasm-bindgen-futures",
                    code("wasm-bindgen-futures"),
                ),
                " - For executing Rust Future as a Promise",
            ],
        ],
        p![
            "Let's update the dependencies in ",
            code("Cargo.toml"),
            " file:",
        ],
        code_block_title(
            "toml",
            "Cargo.toml",
            format!(
                "[dependencies]\n// highlight-start\n{yew_dep_csr_serde}\ngloo-net = \
                 \"0.6\"\nserde = {{ version = \"1.0\", features = [\"derive\"] \
                 }}\nwasm-bindgen-futures = \"0.4\"\n// highlight-end"
            ),
        ),
        p![
            "Yew's ",
            code("serde"),
            " feature enables integration with the ",
            code("serde"),
            " crate, the important point for us is that it adds a ",
            code("serde::Deserialize"),
            " impl to ",
            code("AttrValue"),
            ".",
        ],
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "When choosing dependencies make sure they are ",
                code("wasm32"),
                " compatible! Otherwise you won't be able to run your application.",
            ],
        ),
        p![
            "Update the ",
            code("Video"),
            " struct to derive the ",
            code("Deserialize"),
            " trait:",
        ],
        code_block(
            "rust",
            r##"use yew::prelude::*;
+use serde::Deserialize;
// ...
-#[derive(Clone, PartialEq)]
+#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: AttrValue,
    speaker: AttrValue,
    url: AttrValue,
}"##,
        ),
        p![
            "Now as the last step, we need to update our ",
            code("App"),
            " component to make the fetch request instead of using hardcoded data",
        ],
        code_block(
            "rust",
            r##"use yew::prelude::*;
+use gloo_net::http::Request;

#[component]
fn App() -> Html {
-   let videos = vec![
-       Video {
-           id: 1,
-           title: "Building and breaking things".into(),
-           speaker: "John Doe".into(),
-           url: "https://youtu.be/PsaFVLr8t4E".into(),
-       },
-       Video {
-           id: 2,
-           title: "The development process".into(),
-           speaker: "Jane Smith".into(),
-           url: "https://youtu.be/PsaFVLr8t4E".into(),
-       },
-       Video {
-           id: 3,
-           title: "The Web 7.0".into(),
-           speaker: "Matt Miller".into(),
-           url: "https://youtu.be/PsaFVLr8t4E".into(),
-       },
-       Video {
-           id: 4,
-           title: "Mouseless development".into(),
-           speaker: "Tom Jerry".into(),
-           url: "https://youtu.be/PsaFVLr8t4E".into(),
-       },
-   ];
-
+   let videos = use_state(|| vec![]);
+   {
+       let videos = videos.clone();
+       use_effect_with((), move |_| {
+           let videos = videos.clone();
+           wasm_bindgen_futures::spawn_local(async move {
+               let fetched_videos: Vec<Video> = Request::get("https://yew.rs/tutorial/data.json")
+                   .send()
+                   .await
+                   .unwrap()
+                   .json()
+                   .await
+                   .unwrap();
+               videos.set(fetched_videos);
+           });
+           || ()
+       });
+   }

    // ...

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
-               <VideosList {videos} on_click={on_video_select} />
+               <VideosList videos={(*videos).clone()} on_click={on_video_select} />
            </div>
            // ...
        </>
    }
}"##,
        ),
        admonition!(
            AdmonitionType::Note,
            None,
            p![
                "We are using ",
                code("unwrap"),
                "s here because this is a demo application. In a real-world app, you would likely \
                 want to have ",
                link!(
                    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
                    "proper error handling",
                ),
                ".",
            ],
        ),
        p![
            "Now, look at the browser to see everything working as expected... which would have \
             been the case if it were not for CORS. To fix that, we need a proxy server. Luckily \
             trunk provides that."
        ],
        p!["Update the following line:"],
        code_block(
            "rust",
            r##"-               let fetched_videos: Vec<Video> = Request::get("https://yew.rs/tutorial/data.json")
+               let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")"##,
        ),
        p!["Now, rerun the server with the following command:"],
        code_block(
            "bash",
            "trunk serve --proxy-backend=https://yew.rs/tutorial",
        ),
        p!["Refresh the tab and everything should work as expected."],
        h2!["Wrapping up"],
        p![
            "Congratulations! You've created a web application that fetches data from an external \
             API and displays a list of videos."
        ],
        h2!["What's next"],
        p![
            "This application is very far from perfect or useful. After going through this \
             tutorial, you can use it as a jumping-off point to explore more advanced topics."
        ],
        h3!["Styles"],
        p![
            "Our apps look very ugly. There is no CSS or any kind of style. Unfortunately, Yew \
             does not offer a built-in way to style components. See ",
            link!("https://trunkrs.dev/assets/", "Trunk's assets"),
            " to learn how to add style sheets.",
        ],
        h3!["More libraries"],
        p![
            "Our app made use of only a few external dependencies. There are lots of crates out \
             there that can be used. See ",
            link!("/community/external-libs", "external libraries"),
            " for more details.",
        ],
        h3!["Learning more about Yew"],
        p![
            "Read our ",
            doc_link!(
                crate::pages::getting_started::introduction,
                "official documentation",
            ),
            ". It explains a lot of concepts in much more detail. To learn more about the Yew \
             API, see our ",
            link!("https://docs.rs/yew", "API docs"),
            ".",
        ],
    ])
}

pub fn page_content() -> yew_site_lib::Content {
    page_content_versioned(None)
}

crate::doc_page!("Tutorial", "/docs/tutorial", page_content());
// COMBINE CODE BLOCKS
