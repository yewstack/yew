crate::doc_page!(
    "HTML with html!",
    "/ja/docs/concepts/basic-web-technologies/html",
    Content::new(vec![
        p![
            "You can write expressions resembling HTML with the ",
            code("html!"),
            " macro. Behind the scenes, Yew turns it into Rust code representing the DOM to \
             generate.",
        ],
        code_block(
            "rust",
            "use yew::prelude::*;

let my_header: Html = html! {
    <img src=\"img_girl.jpg\" alt=\"Girl in a jacket\" width=\"500\" height=\"600\" />
};"
        ),
        p![
            "Similar to format expressions, there is an easy way to embed values from the \
             surrounding context into the HTML by applying curly brackets:"
        ],
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
            "One major rule comes with the use of ",
            code("html!"),
            " - you can only return 1 wrapping node. To render a list of multiple elements, ",
            code("html!"),
            " allows fragments. Fragments are tags without a name, that produce no HTML element \
             by themselves.",
        ],
        h3!["Invalid"],
        code_block(
            "rust",
            "use yew::html;

// error: only one root HTML element allowed
html! {

    <div></div>
    <p></p>

};"
        ),
        h3!["Valid"],
        code_block(
            "rust",
            "use yew::html;

// fixed: using HTML fragments
html! {
    <>
        <div></div>
        <p></p>
    </>
};"
        ),
        p![
            "We will introduce Yew and HTML further in depth in ",
            doc_link![crate::pages::concepts::html::introduction, "more HTML"],
            ".",
        ],
    ])
    .with_description("It is HTML but not quite!")
);
