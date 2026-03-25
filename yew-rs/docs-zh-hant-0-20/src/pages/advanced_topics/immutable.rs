crate::doc_page!(
    "Immutable Types",
    "/zh-Hant/docs/advanced-topics/immutable",
    Content::new(vec![
        h2![text("What are immutable types?")],
        p![text(
            "These are types that you can instantiate but never mutate the values. In order to \
             update a value, you must instantiate a new value."
        ),],
        h2![text("Why using immutable types?")],
        p![
            text(
                "Properties, like in React, are propagated from ancestors to children. This means \
                 that the properties must live when each component is updated. This is why \
                 properties should ideally be cheap to clone. In order to achieve this we usually \
                 wrap things in "
            ),
            code("Rc"),
            text("."),
        ],
        p![text(
            "Immutable types are a great fit for holding property's values because they can be \
             cheaply cloned when passed from component to component."
        ),],
        h2![text("Further reading")],
        ul![
            li![link![
                "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/immutable",
                text("Immutable example")
            ],],
            li![link![
                "https://docs.rs/implicit-clone/",
                text("Crate implicit-clone")
            ],],
        ],
    ])
);
