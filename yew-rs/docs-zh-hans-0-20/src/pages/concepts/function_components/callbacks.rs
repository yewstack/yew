crate::doc_page!(
    "Callbacks",
    "/zh-Hans/docs/concepts/function-components/callbacks",
    Content::new(vec![
        p(vec![
            text(
                "Callbacks are used to asynchronously communicate upwards the components tree and \
                 with other things like agents or the DOM during event handling. Internally their \
                 type is just an "
            ),
            code("Fn"),
            text(" wrapped in "),
            code("Rc"),
            text(" to allow them to be cheaply cloned."),
        ]),
        p(vec![
            text("They have an "),
            code("emit"),
            text(" function if you want to call them manually."),
        ]),
        code_block(
            "rust",
            r#"use yew::{html, Component, Context, Html, Callback};

let cb: Callback<String, String> = Callback::from(move |name: String| {
    format!("Bye {}", name)
});

let result = cb.emit(String::from("Bob")); // call the callback
// web_sys::console::log_1(&result.into()); // if uncommented will print "Bye Bob""#
        ),
        h2(vec![text("Passing callbacks as props")]),
        p(vec![text(
            "A common pattern in yew is to create a callback and pass it down as a prop."
        )]),
        code_block(
            "rust",
            r#"use yew::{function_component, html, Html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_name_entry: Callback<String>,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {

    props.on_name_entry.emit(String::from("Bob"));

    html! { "Hello" }
}

// Then supply the prop
#[function_component]
fn App() -> Html {
    let on_name_entry: Callback<String> = Callback::from(move |name: String| {
        let greeting = format!("Hey, {}!", name);
        // web_sys::console::log_1(&greeting.into()); // if uncommented will print
    });

    html! { <HelloWorld {on_name_entry} /> }
}"#
        ),
        h2(vec![text("DOM Events and Callbacks")]),
        p(vec![text(
            "Callbacks are also used to hook into DOM events."
        )]),
        p(vec![text(
            "For example here we define a callback that will be called when the user clicks the \
             button:"
        )]),
        code_block(
            "rust",
            r#"use yew::{function_component, html, Html, Properties, Callback};

#[function_component]
fn App() -> Html {
    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
        // web_sys::console::log_1(&greeting.into()); // if uncommented will print
    });

    html! {
        <button {onclick}>{ "Click" }</button>
    }
}"#
        ),
    ])
);
