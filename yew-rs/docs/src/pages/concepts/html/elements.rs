pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["DOM nodes"],
        p![
            "There are many reasons why you might want to create or manage DOM nodes manually in \
             Yew, such as when integrating with JS libraries that can cause conflicts with \
             managed components."
        ],
        p![
            "Using ",
            code("web-sys"),
            ", you can create DOM elements and convert them into a ",
            code("Node"),
            " - which can then be used as an ",
            code("Html"),
            " value using ",
            code("VRef"),
            ":",
        ],
        code_block(
            "rust",
            r#"use web_sys::{Element, Node};
use yew::prelude::*;
use gloo::utils::document;

#[component]
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
}"#,
        ),
        h2_id!("dynamic-tag-names", "Dynamic tag names"),
        p![
            "When building a higher-order component you might find yourself in a situation where \
             the element's tag name is not static. For example, you might have a ",
            code("Title"),
            " component that can render anything from ",
            code("h1"),
            " to ",
            code("h6"),
            " depending on a level prop. Instead of having to use a big match expression, Yew \
             allows you to set the tag name dynamically using ",
            code("@{name}"),
            " where ",
            code("name"),
            " can be any expression that returns a string.",
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let level = 5;
let text = "Hello World!".to_owned();

html! {
    <@{format!("h{}", level)} class="title">{ text }</@>
};"#,
        ),
        h2!["Boolean Attributes"],
        p![
            "Some content attributes (e.g checked, hidden, required) are called boolean \
             attributes. In Yew, boolean attributes need to be set to a bool value:"
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! {
    <div hidden=true>
        { "This div is hidden." }
    </div>
};"#,
        ),
        p![
            "This will result in ",
            bold!["HTML"],
            " that is functionally equivalent to this:",
        ],
        code_block("html", r#"<div hidden>This div is hidden.</div>"#),
        p![
            "Setting a boolean attribute to false is equivalent to not using the attribute at \
             all; values from boolean expressions can be used:"
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let no = 1 + 1 != 2;

html! {
    <div hidden={no}>
        { "This div is NOT hidden." }
    </div>
};"#,
        ),
        p!["This will result in the following ", bold!["HTML"], ":",],
        code_block("html", r#"<div>This div is NOT hidden.</div>"#),
        h2!["String-like attributes"],
        p![
            "But apart from a select few boolean attributes, you will probably be dealing with a \
             lot of string-like HTML attributes and Yew has a few options to pass string-like \
             values to components."
        ],
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
};"#,
        ),
        p![
            "They are all valid ",
            bold!["but"],
            " we encourage you to favor Yew's custom ",
            code("AttrValue"),
            ", especially if you need to clone or pass them as properties to another component.",
        ],
        h2!["Optional attributes for HTML elements"],
        p![
            "Most HTML attributes can use optional values (Some(x) or None). This allows us to \
             omit the attribute if the attribute is marked as optional."
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let maybe_id = Some("foobar");

html! {
    <div id={maybe_id}></div>
};"#,
        ),
        p![
            "If the attribute is set to ",
            code("None"),
            ", the attribute will not be set in the DOM.",
        ],
        h2!["Children"],
        p![
            "Most HTML elements accept arbitrary HTML as children, however, there is a set of \
             them that doesn't accept any children at all. These elements are called ",
            italic!["void"],
            " elements, and they are:",
        ],
        ul![
            li![code("<area />")],
            li![code("<base />")],
            li![code("<base />")],
            li![code("<br />")],
            li![code("<col />")],
            li![code("<embed />")],
            li![code("<hr />")],
            li![code("<img />")],
            li![code("<input />")],
            li![code("<link />")],
            li![code("<meta />")],
            li![code("<param />")],
            li![code("<source />")],
            li![code("<track />")],
            li![code("<wbr />")],
            li![code("<textarea />")],
        ],
        p![
            "Attempting to provide children to these elements will result in a compilation error \
             or, if the element tag is chosen dynamically, in a panic."
        ],
        h3!["The case of ", code("<textarea>")],
        p![
            "The ",
            code("<textarea>"),
            " element is special; The modern HTML specification states that children of ",
            code("<textarea>"),
            " define its default value, however in Yew it's specified differently. Instead of \
             writing",
        ],
        code_block("html", r#"<textarea>{"default value"}</textarea>"#),
        p!["Which would fail to compile, it's customary to write"],
        code_block("html", r#"<textarea defaultvalue="default value" />"#),
        h2!["Relevant examples"],
        ul![li![link!(
            "https://github.com/yewstack/yew/tree/master/examples/inner_html",
            "Inner HTML",
        )]],
    ])
}

crate::doc_page!("Elements", "/docs/concepts/html/elements", page_content());
