crate::doc_page!(
    "HTML with html!",
    "/zh-Hant/docs/concepts/basic-web-technologies/html",
    Content::new(vec![
        p![
            text("You can write expressions resembling HTML with the "),
            code("html!"),
            text(
                " macro. Behind the scenes Yew turns it into rust code representing the DOM to \
                 generate."
            ),
        ],
        code_block(
            "rust",
            "use yew::prelude::*;

let my_header: Html = html! {
<img src=\"img_girl.jpg\" alt=\"Girl in a jacket\" width=\"500\" height=\"600\" />
};"
        ),
        p![text(
            "Similar to format expressions, there is an easy way to embed values from the \
             surrounding context into the html by applying curly brackets:"
        )],
        code_block(
            "rust",
            "use yew::prelude::*;

let header_text = \"Hello world\".to_string();
let header_html: Html = html! {
<h1>{header_text}</h1>
};

let count: usize = 5;
let counter_html: Html = html! {
<p>{\"My age is: \"}{count}</p>
};

let combined_html: Html = html! {
<div>{header_html}{counter_html}</div>
};"
        ),
        p![
            text("One major rule comes with the use of "),
            code("html!"),
            text(" - you can only return 1 wrapping node. To render a list of multiple elements, "),
            code("html!"),
            text(
                " allows fragments. Fragments are tags without a name, that produce no html \
                 element by themselves."
            ),
        ],
        h3![text("Invalid")],
        code_block(
            "rust",
            "use yew::html;

// error: only one root html element allowed
html! {

<div></div>
<p></p>

};"
        ),
        h3![text("Valid")],
        code_block(
            "rust",
            "use yew::html;

// fixed: using html fragments
html! {
<>
<div></div>
<p></p>
</>
};"
        ),
        p![
            text("We will introduce Yew and HTML further in depth in "),
            link!["/zh-Hant/docs/concepts/html", text("more HTML")],
            text("."),
        ],
    ])
);
