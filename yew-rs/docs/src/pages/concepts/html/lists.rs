pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        h2!["Iterators"],
        p!["There are 3 ways to build HTML from iterators:"],
        tabs!(
            "`for` loops",
            tab!(
                "`for` loops",
                "`for` loops",
                p![
                    "The main approach is to use for loops, the same for loops that already exist \
                     in Rust, but with 2 key differences:"
                ],
                ol![
                    li![
                        "Unlike standard for loops which can't return anything, for loops in ",
                        code("html!"),
                        " are converted to a list of nodes;",
                    ],
                    li![
                        "Diverging expressions, i.e. ",
                        code("break"),
                        ", ",
                        code("continue"),
                        " are not allowed in the body of for loops in ",
                        code("html!"),
                        ".",
                    ],
                ],
                code_block(
                    "rust",
                    r#"use yew::prelude::*;

html! {
    for i in 0 .. 10 {
        <span>{i}</span>
    }
};"#,
                ),
            ),
            tab!(
                "`for` block",
                "`for` block",
                p![
                    "An alternative is to use the ",
                    code("for"),
                    " keyword, which is not native Rust syntax and instead is used by the HTML \
                     macro to output the needed code to display the iterator. This approach is \
                     better than the first one when the iterator is already computed and the only \
                     thing left to do is to pass it to the macro.",
                ],
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
            ),
            tab!(
                "`collect` method",
                "`collect` method",
                p![
                    "The last is to call ",
                    code("collect::<Html>()"),
                    " on the final transform in your iterator, which returns a list that Yew can \
                     display.",
                ],
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
            ),
        ),
        h2!["Keyed lists"],
        p![
            "A keyed list is an optimized list that has keys on ",
            bold!["all"],
            " children. ",
            code("key"),
            " is a special prop provided by Yew that gives an HTML element or component a unique \
             identifier that is used for optimization purposes inside Yew.",
        ],
        admonition!(
            AdmonitionType::Caution,
            None,
            p![
                "Key has to be unique only in each list, in contrast to the global uniqueness of \
                 HTML ",
                code("id"),
                "s. It must not depend on the order of the list.",
            ],
        ),
        p!["It is always recommended to add keys to lists."],
        p![
            "Keys can be added by passing a unique ",
            code("String"),
            ", ",
            code("str"),
            " or integer to the special ",
            code("key"),
            " prop:",
        ],
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
        h3!["Performance increases"],
        p![
            "We have ",
            link!(
                "https://github.com/yewstack/yew/tree/master/examples/keyed_list",
                "Keyed list",
            ),
            " example that lets you test the performance improvements, but here is a rough \
             rundown:",
        ],
        ol![
            li![
                "Go to ",
                link!(
                    "https://examples.yew.rs/keyed_list",
                    "Keyed list hosted demo",
                ),
            ],
            li!["Add 500 elements."],
            li!["Disable keys."],
            li!["Reverse the list."],
            li![
                "Look at \"The last rendering took Xms\" (At the time of writing this it was \
                 ~60ms)"
            ],
            li!["Enable keys."],
            li!["Reverse the list."],
            li![
                "Look at \"The last rendering took Xms\" (At the time of writing this it was \
                 ~30ms)"
            ],
        ],
        p!["So just at the time of writing this, for 500 components it is a 2x increase of speed."],
        h3!["Detailed explanation"],
        p![
            "Usually, you just need a key on every list item when you iterate and the order of \
             data can change. It's used to speed up the reconciliation process when re-rendering \
             the list."
        ],
        p![
            "Without keys, assume you iterate through ",
            code("[\"bob\", \"sam\", \"rob\"]"),
            ", ending up with the HTML:",
        ],
        code_block(
            "html",
            r#"<div id="bob">My name is Bob</div>
<div id="sam">My name is Sam</div>
<div id="rob">My name is rob</div>"#,
        ),
        p![
            "Then on the next render, if your list changed to ",
            code("[\"bob\", \"rob\"]"),
            ", yew could delete the element with id=\"rob\" and update id=\"sam\" to be id=\"rob\"",
        ],
        p![
            "If you had added a key to each element, the initial HTML would be the same, but \
             after the render with the modified list, ",
            code("[\"bob\", \"rob\"]"),
            ", yew would just delete the second HTML element and leave the rest untouched since \
             it can use the keys to associate them.",
        ],
        p![
            "If you ever encounter a bug/\"feature\" where you switch from one component to \
             another but both have a div as the highest rendered element. Yew reuses the rendered \
             HTML div in those cases as an optimization. If you need that div to be recreated \
             instead of reused, then you can add different keys and they will not be reused."
        ],
        h2!["Further reading"],
        ul![
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/todomvc",
                "TodoMVC",
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/keyed_list",
                "Keyed list",
            )],
            li![link!(
                "https://github.com/yewstack/yew/tree/master/examples/router",
                "Router",
            )],
        ],
    ])
}

crate::doc_page!("Lists", "/docs/concepts/html/lists", page_content());
