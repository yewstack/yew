crate::doc_page!(
    "HTML with html!",
    "/zh-Hans/docs/concepts/basic-web-technologies/html",
    Content::new(vec![
        p(vec![
            text("You can write expressions resembling HTML with the "),
            code("html!"),
            text(
                " macro. Behind the scenes, Yew turns it into Rust code representing the DOM to \
                 generate."
            )
        ]),
        p(vec![text(
            "Similar to format expressions, there is an easy way to embed values from the \
             surrounding context into the HTML by applying curly brackets:"
        )]),
        h1(vec![text("{header_text}")]),
        p(vec![text("{\"My age is: \"}{count}")]),
        p(vec![
            text("One major rule comes with the use of "),
            code("html!"),
            code("html!")
        ]),
        h3(vec![text("Invalid")]),
        p(vec![text("")]),
        h3(vec![text("Valid")]),
        p(vec![text("")]),
        p(vec![
            text("We will introduce Yew and HTML further in depth in "),
            link("/docs/0.21/concepts/html", vec![text("more HTML")]),
            text(".")
        ])
    ])
);
