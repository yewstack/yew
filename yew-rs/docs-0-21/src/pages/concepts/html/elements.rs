crate::doc_page!(
    "Elements",
    "/docs/concepts/html/elements",
    Content::new(vec![
        h2(vec![text("DOM nodes")]),
        p(vec![text(
            "There are many reasons why you might want to create or manage DOM nodes manually in \
             Yew, such as when integrating with JS libraries that can cause conflicts with \
             managed components."
        ),]),
        p(vec![
            text("Using "),
            code("web-sys"),
            text(", you can create DOM elements and convert them into a "),
            code("Node"),
            text(" - which can then be used as an "),
            code("Html"),
            text(" value using "),
            code("VRef"),
            text(":"),
        ]),
        code_block(
            "rust",
            r#"use web_sys::{Element, Node};
use yew::prelude::*;
use gloo::utils::document;

#[function_component]
fn MyComponent() -> Html {
    // memoize as this only needs to be executed once
    let node = use_memo(
        (),
        |_| {
            // Create a div element from the document
            let div: Element = document().create_element("div").unwrap();
            // Add content, classes etc.
            div.set_inner_html("Hello, World!");
            // Convert Element into a Node
            let node: Node = div.into();
            // Return that Node as a Html value
            Html::VRef(node)
        },
    );

    // use_memo return Rc so we need to deref and clone
    (*node).clone()
}
"#
        ),
        h2_id("dynamic-tag-names", vec![text("Dynamic tag names")]),
        p(vec![
            text(
                "When building a higher-order component you might find yourself in a situation \
                 where the element's tag name is not static. For example, you might have a "
            ),
            code("Title"),
            text(" component that can render anything from "),
            code("h1"),
            text(" to "),
            code("h6"),
            text(
                " depending on a level prop. Instead of having to use a big match expression, Yew \
                 allows you to set the tag name dynamically using "
            ),
            code("@{name}"),
            text(" where "),
            code("name"),
            text(" can be any expression that returns a string."),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let level = 5;
let text = "Hello World!".to_owned();

html! {
    <@{format!("h{}", level)} class="title">{ text }</@>
};"#
        ),
        h2(vec![text("Boolean Attributes")]),
        p(vec![text(
            "Some content attributes (e.g checked, hidden, required) are called boolean \
             attributes. In Yew, boolean attributes need to be set to a bool value:"
        ),]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! {
    <div hidden=true>
        { "This div is hidden." }
    </div>
};"#
        ),
        p(vec![
            text("This will result in "),
            bold(vec![text("HTML")]),
            text(" that is functionally equivalent to this:"),
        ]),
        code_block("html", r#"<div hidden>This div is hidden.</div>"#),
        p(vec![text(
            "Setting a boolean attribute to false is equivalent to not using the attribute at \
             all; values from boolean expressions can be used:"
        ),]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let no = 1 + 1 != 2;

html! {
    <div hidden={no}>
        { "This div is NOT hidden." }
    </div>
};"#
        ),
        p(vec![
            text("This will result in the following "),
            bold(vec![text("HTML")]),
            text(":"),
        ]),
        code_block("html", r#"<div>This div is NOT hidden.</div>"#),
        h2(vec![text("String-like attributes")]),
        p(vec![text(
            "But apart from a select few boolean attributes, you will probably be dealing with a \
             lot of string-like HTML attributes and Yew has a few options to pass string-like \
             values to components."
        ),]),
        code_block(
            "rust",
            r#"use yew::{html, virtual_dom::AttrValue};

let str_placeholder = "I'm a str!";
let string_placeholder = String::from("I'm a String!");
let attrvalue_placeholder = AttrValue::from("I'm an AttrValue!");

html! {
    <div>
        <input placeholder={str_placeholder} />
        <input placeholder={string_placeholder} />
        <input placeholder={attrvalue_placeholder} />
    </div>
};"#
        ),
        p(vec![
            text("They are all valid "),
            bold(vec![text("but")]),
            text(" we encourage you to favor Yew's custom "),
            code("AttrValue"),
            text(
                ", especially if you need to clone or pass them as properties to another \
                 component."
            ),
        ]),
        h2(vec![text("Optional attributes for HTML elements")]),
        p(vec![text(
            "Most HTML attributes can use optional values (Some(x) or None). This allows us to \
             omit the attribute if the attribute is marked as optional."
        ),]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let maybe_id = Some("foobar");

html! {
    <div id={maybe_id}></div>
};"#
        ),
        p(vec![
            text("If the attribute is set to "),
            code("None"),
            text(", the attribute will not be set in the DOM."),
        ]),
        h2(vec![text("Relevant examples")]),
        ul(vec![li(vec![link(
            "https://github.com/yewstack/yew/tree/master/examples/inner_html",
            vec![text("Inner HTML")]
        )]),]),
    ])
);
