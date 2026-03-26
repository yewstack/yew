pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "Callbacks are used to asynchronously communicate upwards the components tree and \
             with other things like agents or the DOM during event handling. Internally their \
             type is just an ",
            code("Fn"),
            " wrapped in ",
            code("Rc"),
            " to allow them to be cheaply cloned.",
        ],
        p![
            "They have an ",
            code("emit"),
            " function if you want to call them manually.",
        ],
        code_block(
            "rust",
            r#"use yew::{html, Component, Context, Html, Callback};

let cb: Callback<String, String> = Callback::from(move |name: String| {
    format!("Bye {}", name)
});

let result = cb.emit(String::from("Bob")); // call the callback
// web_sys::console::log_1(&result.into()); // if uncommented will print "Bye Bob""#,
        ),
        h2!["Passing callbacks as props"],
        p!["A common pattern in yew is to create a callback and pass it down as a prop."],
        code_block(
            "rust",
            r#"use yew::{component, html, Html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_name_entry: Callback<String>,
}

#[component]
fn HelloWorld(props: &Props) -> Html {

    props.on_name_entry.emit(String::from("Bob"));

    html! { "Hello" }
}

// Then supply the prop
#[component]
fn App() -> Html {
    let on_name_entry: Callback<String> = Callback::from(move |name: String| {
        let greeting = format!("Hey, {}!", name);
        // web_sys::console::log_1(&greeting.into()); // if uncommented will print
    });

    html! { <HelloWorld {on_name_entry} /> }
}"#,
        ),
        h2!["DOM Events and Callbacks"],
        p!["Callbacks are also used to hook into DOM events."],
        p![
            "For example, here we define a callback that will be called when the user clicks the \
             button:"
        ],
        code_block(
            "rust",
            r#"use yew::{component, html, Html, Properties, Callback};

#[component]
fn App() -> Html {
    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
        // web_sys::console::log_1(&greeting.into()); // if uncommented will print
    });

    html! {
        <button {onclick}>{ "Click" }</button>
    }
}"#,
        ),
    ])
}

crate::doc_page!(
    "Callbacks",
    "/docs/concepts/function-components/callbacks",
    page_content()
);
