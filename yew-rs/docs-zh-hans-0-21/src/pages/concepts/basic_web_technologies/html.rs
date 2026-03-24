crate::doc_page!(
    "HTML with html!",
    "/zh-Hans/docs/concepts/basic-web-technologies/html",
    Content::new(vec![
        p![
            text("You can write expressions resembling HTML with the "),
            code("html!"),
            text(
                " macro. Behind the scenes, Yew turns it into Rust code representing the DOM to \
                 generate."
            )
        ],
        p![text(
            "Similar to format expressions, there is an easy way to embed values from the \
             surrounding context into the HTML by applying curly brackets:"
        )],
        h1![text("{header_text}")],
        p![text("{\"My age is: \"}{count}")],
        p![
            text("One major rule comes with the use of "),
            code("html!"),
            code("html!")
        ],
        h3![text("Invalid")],
        p![text("")],
        h3![text("Valid")],
        p![text("")],
        p![
            text("We will introduce Yew and HTML further in depth in "),
            link!("/docs/0.21/concepts/html", text("more HTML")),
            text(".")
        ]
    ])
);
