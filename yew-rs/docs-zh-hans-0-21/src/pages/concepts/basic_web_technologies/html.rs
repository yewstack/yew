crate::doc_page!(
    "HTML with html!",
    "/zh-Hans/docs/concepts/basic-web-technologies/html",
    Content::new(vec![
        p![
            "You can write expressions resembling HTML with the ",
            code("html!"),
            " macro. Behind the scenes, Yew turns it into Rust code representing the DOM to \
             generate."
        ],
        p![
            "Similar to format expressions, there is an easy way to embed values from the \
             surrounding context into the HTML by applying curly brackets:"
        ],
        h1!["{header_text}"],
        p!["{\"My age is: \"}{count}"],
        p![
            "One major rule comes with the use of ",
            code("html!"),
            code("html!")
        ],
        h3!["Invalid"],
        p![""],
        h3!["Valid"],
        p![""],
        p![
            "We will introduce Yew and HTML further in depth in ",
            doc_link!(crate::pages::concepts::html::introduction, "more HTML"),
            "."
        ]
    ])
    .with_description("It is HTML but not quite!")
);
