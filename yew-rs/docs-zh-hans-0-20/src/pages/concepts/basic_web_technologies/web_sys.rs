crate::doc_page!(
    "web-sys",
    "/zh-Hans/docs/concepts/basic-web-technologies/web-sys",
    Content::new(vec![
        p![
            text("The "),
            link!["https://crates.io/crates/web-sys",
                code("web-sys"),
                text(" crate"),
            ],
            text(" provides bindings for Web APIs. This is procedurally generated from browser WebIDL \
              which is why some names are so long and why some types are vague."),
        ],
        h2![text("Features in web-sys")],
        p![
            text("The "),
            code("web-sys"),
            text(" crate with all of it's features enabled can add lots of bloat to a Wasm application, \
              in order to get around this issue most types are feature gated so that you only include the types \
              you require for your application. Yew includes a number of features from "),
            code("web-sys"),
            text(" and exposes some types in it's public API, you will often need to add "),
            code("web-sys"),
            text(" as a dependency yourself."),
        ],
        h2![text("Inheritance in web-sys")],
        p![
            text("In the "),
            link!["/docs/0.20/concepts/basic-web-technologies/wasm-bindgen#simulating-inheritance",
                text("Simulating inheritance section"),
            ],
            text(" you can read how in general Rust provides an approach to simulate inheritance in JavaScript. \
              This is very important in "),
            code("web-sys"),
            text(" as understanding what methods are available on a type means understanding it's inheritance."),
        ],
        p![
            text("This section is going to look at a specific element and list out it's inheritance using Rust by calling "),
            link!["https://doc.rust-lang.org/std/ops/trait.Deref.html#tymethod.deref",
                code("Deref::deref"),
            ],
            text(" until the value is "),
            link!["/docs/0.20/concepts/basic-web-technologies/wasm-bindgen#jsvalue",
                code("JsValue"),
            ],
            text(":"),
        ],
        code_block("rust", r#"use std::ops::Deref;
use web_sys::{
    Element,
    EventTarget,
    HtmlElement,
    HtmlTextAreaElement,
    Node,
};

fn inheritance_of_text_area(text_area: HtmlTextAreaElement) {
    // HtmlTextAreaElement is <textarea> in html.
    let html_element: &HtmlElement = text_area.deref();

    let element: &Element = html_element.deref();

    let node: &Node = element.deref();

    let event_target: &EventTarget = node.deref();

    // Notice we've moved from web-sys types now into built-in
    // JavaScript types which are in the js-sys crate.
    let object: &js_sys::Object = event_target.deref();

    // Notice we've moved from js-sys type to the root JsValue from
    // the wasm-bindgen crate.
    let js_value: &wasm_bindgen::JsValue = object.deref();

    // Using deref like this means we have to manually traverse
    // the inheritance tree, however, you can call JsValue methods
    // on the HtmlTextAreaElement type.
    // The `is_string` method comes from JsValue.
    assert!(!text_area.is_string());

    // empty function just to prove we can pass HtmlTextAreaElement as a
    // &EventTarget.
    fn this_function_only_takes_event_targets(targets: &EventTarget) {};

    // The compiler will walk down the deref chain in order to match the types here.
    this_function_only_takes_event_targets(&text_area);

    // The AsRef implementations allow you to treat the HtmlTextAreaElement
    // as an &EventTarget.

    let event_target: &EventTarget = text_area.as_ref();

}"#),
        p![
            link!["https://wasm-bindgen.github.io/wasm-bindgen/web-sys/inheritance.html",
                text("Inheritance in web-sys in The wasm-bindgen Guide"),
            ],
            text("."),
        ],
        h2![text("The Node in NodeRef")],
        p![
            text("Yew uses a "),
            link!["/docs/0.20/concepts/function-components/node-refs", code("NodeRef")],
            text(" in order to provide a way for keeping a reference to a "),
            code("Node"),
            text(" made by the "),
            link!["/docs/0.20/concepts/html", code("html!")],
            text(" macro. The "),
            code("Node"),
            text(" part of "),
            code("NodeRef"),
            text(" is referring to "),
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/web_sys/struct.Node.html",
                code("web_sys::Node"),
            ],
            text(". The "),
            code("NodeRef::get"),
            text(" method will return a "),
            code("Option<Node>"),
            text(" value, however, most of the time in Yew you want to cast this value to a specific element \
              so you can use it's specific methods. This casting can be done using "),
            link!["/docs/0.20/concepts/basic-web-technologies/wasm-bindgen#JsCast",
                code("JsCast"),
            ],
            text(" on the "),
            code("Node"),
            text(" value, if present, but Yew provides the "),
            code("NodeRef::cast"),
            text(" method to perform this casting for convenience and so that you don't necessarily have to \
              include the "),
            code("wasm-bindgen"),
            text(" dependency for the "),
            code("JsCast"),
            text(" trait."),
        ],
        p![
            text("The two code blocks below do essentially the same thing, the first is using "),
            code("NodeRef::cast"),
            text(" and the second is using "),
            link!["https://wasm-bindgen.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html#method.dyn_into",
                code("JsCast::dyn_into"),
            ],
            text(" on the "),
            code("web_sys::Node"),
            text(" returned from "),
            code("NodeRef::get"),
            text("."),
        ],
        h3![text("Using NodeRef::cast")],
        code_block("rust", r#"use web_sys::HtmlInputElement;
use yew::NodeRef;

fn with_node_ref_cast(node_ref: NodeRef) {
    if let Some(input) = node_ref.cast::<HtmlInputElement>() {
        // do something with HtmlInputElement
    }
}"#),
        h3![text("Using NodeRef::get")],
        code_block("rust", r#"use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::NodeRef;

fn with_jscast(node_ref: NodeRef) {
    if let Some(input) = node_ref
        .get()
        .and_then(|node| node.dyn_into::<HtmlInputElement>().ok()) {
        // do something with HtmlInputElement
    }
}"#),
        h2![text("JavaScript example to Rust")],
        p![
            text("This section is to help show that any examples that use JavaScript to interact with the \
              Web APIs can be adapted and written using Rust with "),
            code("web-sys"),
            text("."),
        ],
        h3![text("JavaScript example")],
        code_block("js", r#"document.getElementById('mousemoveme').onmousemove = (e) => {
    // e = Mouse event.
    var rect = e.target.getBoundingClientRect()
    var x = e.clientX - rect.left //x position within the element.
    var y = e.clientY - rect.top //y position within the element.
    console.log('Left? : ' + x + ' ; Top? : ' + y + '.')
}"#),
        h3![text("web-sys example")],
        p![
            text("Using "),
            code("web-sys"),
            text(" alone the above JavaScript example could be implemented like this:"),
        ],
        code_block("toml", r#"[dependencies]
wasm-bindgen = "0.2"

[dependencies.web-sys]
version = "0.3"
# We need to enable all the web-sys features we want to use!
features = [
    "console",
    "Document",
    "HtmlElement",
    "MouseEvent",
    "DomRect",
]"#),
        code_block("rust", r#"use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{console, Document, HtmlElement, MouseEvent};

let mousemove = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(|e| {
    let rect = e
        .target()
        .expect("mouse event doesn't have a target")
        .dyn_into::<HtmlElement>()
        .expect("event target should be of type HtmlElement")
        .get_bounding_client_rect();
    let x = (e.client_x() as f64) - rect.left();
    let y = (e.client_y() as f64) - rect.top();
    console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
}));

Document::new()
    .expect("global document not set")
    .get_element_by_id("mousemoveme")
    .expect("element with id `mousemoveme` not present")
    .unchecked_into::<HtmlElement>()
    .set_onmousemove(mousemove.as_ref().dyn_ref());

// we now need to save the `mousemove` Closure so that when
// this event fires the closure is still in memory."#),
        p![
            text("This version is much more verbose, but you will probably notice part of that is because of failure \
              types reminding us that some of these function calls have invariants that must be held otherwise will \
              cause a panic in Rust. Another part of the verbosity is the calls to "),
            code("JsCast"),
            text(" in order to cast into different types so that you can call it's specific methods."),
        ],
        h3![text("Yew example")],
        p![
            text("In Yew you will mostly be creating "),
            link!["/docs/0.20/concepts/function-components/callbacks", code("Callback")],
            text("s to use in the "),
            link!["/docs/0.20/concepts/html", code("html!")],
            text(" macro so the example is going to use this approach instead of completely copying \
              the approach above:"),
        ],
        code_block("toml", r#"[dependencies.web-sys]
version = "0.3"
# We need to enable the `DomRect` feature in order to use the
# `get_bounding_client_rect` method.
features = [
    "console",
    "HtmlElement",
    "MouseEvent",
    "DomRect",
]"#),
        code_block("rust", r#"use web_sys::{console, HtmlElement, MouseEvent};
use yew::{
    html,
    Callback, TargetCast,
};

let onmousemove = Callback::from(|e: MouseEvent| {
    if let Some(target) = e.target_dyn_into::<HtmlElement>() {
        let rect = target.get_bounding_client_rect();
        let x = (e.client_x() as f64) - rect.left();
        let y = (e.client_y() as f64) - rect.top();
        console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
    }
});

html! {
    <div id="mousemoveme" {onmousemove}></div>
};"#),
        h2![text("External libraries")],
        p![
            code("web-sys"),
            text(" is a raw binding to the Web API so it comes with some pain in Rust because it was not \
              designed with Rust or even a strong type system in mind, this is where community crates come in to \
              provide abstractions over "),
            code("web-sys"),
            text(" to provide more idiomatic Rust APIs."),
        ],
        p![
            link!["/community/external-libs", text("External libraries page")],
        ],
    ])
);
