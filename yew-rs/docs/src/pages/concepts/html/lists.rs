pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2(vec![text("Iterators")]),
        p(vec![text("There are 3 ways to build HTML from iterators:")]),
        tabs(
            "`for` loops",
            vec![
                tab(
                    "`for` loops",
                    "`for` loops",
                    vec![
                        p(vec![text(
                            "The main approach is to use for loops, the same for loops that \
                             already exist in Rust, but with 2 key differences:",
                        )]),
                        ol(vec![
                            li(vec![
                                text(
                                    "Unlike standard for loops which can't return anything, for \
                                     loops in ",
                                ),
                                code("html!"),
                                text(" are converted to a list of nodes;"),
                            ]),
                            li(vec![
                                text("Diverging expressions, i.e. "),
                                code("break"),
                                text(", "),
                                code("continue"),
                                text(" are not allowed in the body of for loops in "),
                                code("html!"),
                                text("."),
                            ]),
                        ]),
                        code_block(
                            "rust",
                            r#"use yew::prelude::*;

html! {
    for i in 0 .. 10 {
        <span>{i}</span>
    }
};"#,
                        ),
                    ],
                ),
                tab(
                    "`for` block",
                    "`for` block",
                    vec![
                        p(vec![
                            text("An alternative is to use the "),
                            code("for"),
                            text(
                                " keyword, which is not native Rust syntax and instead is used by \
                                 the HTML macro to output the needed code to display the \
                                 iterator. This approach is better than the first one when the \
                                 iterator is already computed and the only thing left to do is to \
                                 pass it to the macro.",
                            ),
                        ]),
                        code_block(
                            "rust",
                            r#"use yew::prelude::*;

let items = (1..=10).collect::<Vec<_>>();

html! {
    <ul class="item-list">
        { for items.iter() }
    </ul>
};"#,
                        ),
                    ],
                ),
                tab(
                    "`collect` method",
                    "`collect` method",
                    vec![
                        p(vec![
                            text("The last is to call "),
                            code("collect::<Html>()"),
                            text(
                                " on the final transform in your iterator, which returns a list \
                                 that Yew can display.",
                            ),
                        ]),
                        code_block(
                            "rust",
                            r#"use yew::prelude::*;

let items = (1..=10).collect::<Vec<_>>();

html! {
    <ul class="item-list">
        { items.iter().collect::<Html>() }
    </ul>
};"#,
                        ),
                    ],
                ),
            ],
        ),
        h2(vec![text("Keyed lists")]),
        p(vec![
            text("A keyed list is an optimized list that has keys on "),
            bold(vec![text("all")]),
            text(" children. "),
            code("key"),
            text(
                " is a special prop provided by Yew that gives an HTML element or component a \
                 unique identifier that is used for optimization purposes inside Yew.",
            ),
        ]),
        admonition(
            AdmonitionType::Caution,
            None,
            vec![p(vec![
                text(
                    "Key has to be unique only in each list, in contrast to the global uniqueness \
                     of HTML ",
                ),
                code("id"),
                text("s. It must not depend on the order of the list."),
            ])],
        ),
        p(vec![text("It is always recommended to add keys to lists.")]),
        p(vec![
            text("Keys can be added by passing a unique "),
            code("String"),
            text(", "),
            code("str"),
            text(" or integer to the special "),
            code("key"),
            text(" prop:"),
        ]),
        code_block_ignore(
            "rust",
            r#"use yew::prelude::*;

let names = vec!["Sam","Bob","Ray"]

html! {
    <div id="introductions">
        {
            names.into_iter().map(|name| {
                html!{<div key={name}>{ format!("Hello, I'am {}!",name) }</div>}
            }).collect::<Html>()
        }
    </div>
};"#,
        ),
        h3(vec![text("Performance increases")]),
        p(vec![
            text("We have "),
            link(
                "https://github.com/yewstack/yew/tree/master/examples/keyed_list",
                vec![text("Keyed list")],
            ),
            text(
                " example that lets you test the performance improvements, but here is a rough \
                 rundown:",
            ),
        ]),
        ol(vec![
            li(vec![
                text("Go to "),
                link(
                    "https://examples.yew.rs/keyed_list",
                    vec![text("Keyed list hosted demo")],
                ),
            ]),
            li(vec![text("Add 500 elements.")]),
            li(vec![text("Disable keys.")]),
            li(vec![text("Reverse the list.")]),
            li(vec![text(
                "Look at \"The last rendering took Xms\" (At the time of writing this it was \
                 ~60ms)",
            )]),
            li(vec![text("Enable keys.")]),
            li(vec![text("Reverse the list.")]),
            li(vec![text(
                "Look at \"The last rendering took Xms\" (At the time of writing this it was \
                 ~30ms)",
            )]),
        ]),
        p(vec![text(
            "So just at the time of writing this, for 500 components it is a 2x increase of speed.",
        )]),
        h3(vec![text("Detailed explanation")]),
        p(vec![text(
            "Usually, you just need a key on every list item when you iterate and the order of \
             data can change. It's used to speed up the reconciliation process when re-rendering \
             the list.",
        )]),
        p(vec![
            text("Without keys, assume you iterate through "),
            code("[\"bob\", \"sam\", \"rob\"]"),
            text(", ending up with the HTML:"),
        ]),
        code_block(
            "html",
            r#"<div id="bob">My name is Bob</div>
<div id="sam">My name is Sam</div>
<div id="rob">My name is rob</div>"#,
        ),
        p(vec![
            text("Then on the next render, if your list changed to "),
            code("[\"bob\", \"rob\"]"),
            text(
                ", yew could delete the element with id=\"rob\" and update id=\"sam\" to be \
                 id=\"rob\"",
            ),
        ]),
        p(vec![
            text(
                "If you had added a key to each element, the initial HTML would be the same, but \
                 after the render with the modified list, ",
            ),
            code("[\"bob\", \"rob\"]"),
            text(
                ", yew would just delete the second HTML element and leave the rest untouched \
                 since it can use the keys to associate them.",
            ),
        ]),
        p(vec![text(
            "If you ever encounter a bug/\"feature\" where you switch from one component to \
             another but both have a div as the highest rendered element. Yew reuses the rendered \
             HTML div in those cases as an optimization. If you need that div to be recreated \
             instead of reused, then you can add different keys and they will not be reused.",
        )]),
        h2(vec![text("Further reading")]),
        ul(vec![
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/todomvc",
                vec![text("TodoMVC")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/keyed_list",
                vec![text("Keyed list")],
            )]),
            li(vec![link(
                "https://github.com/yewstack/yew/tree/master/examples/router",
                vec![text("Router")],
            )]),
        ]),
    ])
}

crate::doc_page!("Lists", "/docs/concepts/html/lists", page_content());
