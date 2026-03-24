crate::doc_page!(
    "Lists",
    "/docs/concepts/html/lists",
    Content::new(vec![
        h2![text("Iterators")],
        p![text(
            "Yew supports two different syntaxes for building HTML from an iterator."
        )],
        p![
            text("The first is to call "),
            code("collect::<Html>()"),
            text(
                " on the final transform in your iterator, which returns a list that Yew can \
                 display."
            ),
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let items = (1..=10).collect::<Vec<_>>();

html! {
<ul class="item-list">
{ items.iter().collect::<Html>() }
</ul>
};"#
        ),
        p![
            text("The alternative is to use the "),
            code("for"),
            text(
                " keyword, which is not native Rust syntax and instead is used by the HTML macro \
                 to output the needed code to display the iterator."
            ),
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let items = (1..=10).collect::<Vec<_>>();

html! {
<ul class="item-list">
{ for items.iter() }
</ul>
};"#
        ),
        h2![text("Keyed lists")],
        p![
            text("A keyed list is an optimized list that has keys on "),
            bold![text("all")],
            text(" children. "),
            code("key"),
            text(
                " is a special prop provided by Yew which gives an html element or component a \
                 unique identifier which is used for optimization purposes inside Yew."
            ),
        ],
        admonition![
            AdmonitionType::Warning,
            None,
            p![
                text(
                    "Key has to be unique only in each list, in contrast to the global uniqueness \
                     of html "
                ),
                code("id"),
                text("s. It must not depend on the order of the list."),
            ],
        ],
        p![text("It is always recommended to add keys to lists.")],
        p![
            text("Keys can be added by passing a unique "),
            code("String"),
            text(", "),
            code("str"),
            text(" or integer to the special "),
            code("key"),
            text(" prop:"),
        ],
        code_block(
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
};"#
        ),
        h3![text("Performance increases")],
        p![
            text("We have "),
            link![
                "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/keyed_list",
                text("Keyed list"),
            ],
            text(
                " example that lets you test the performance improvements, but here is rough \
                 rundown:"
            ),
        ],
        ol![
            li![
                text("Go to "),
                link![
                    "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/keyed_list",
                    text("Keyed list"),
                ],
                text(" hosted demo"),
            ],
            li![text("Add 500 elements.")],
            li![text("Disable keys.")],
            li![text("Reverse the list.")],
            li![text(
                "Look at \"The last rendering took Xms\" (At the time of writing this it was \
                 ~60ms)"
            )],
            li![text("Enable keys.")],
            li![text("Reverse the list.")],
            li![text(
                "Look at \"The last rendering took Xms\" (At the time of writing this it was \
                 ~30ms)"
            )],
        ],
        p![text(
            "So just at the time of writing this, for 500 components its a x2 increase of speed."
        )],
        h3![text("Detailed explanation")],
        p![text(
            "Usually you just need a key on every list item when you iterate and the order of \
             data can change. It's used to speed up the reconciliation process when re-rendering \
             the list."
        )],
        p![
            text("Without keys, lets assume you iterate through "),
            code("[\"bob\",\"sam\",\"rob\"]"),
            text(", ending up with the html:"),
        ],
        code_block(
            "html",
            r#"<div id="bob">My name is Bob</div>
<div id="sam">My name is Sam</div>
<div id="rob">My name is rob</div>"#
        ),
        p![
            text("Then on the next render, if your list changed to "),
            code("[\"bob\",\"rob\"]"),
            text(
                ", yew could delete the element with id=\"rob\" and update id=\"sam\" to be \
                 id=\"rob\""
            ),
        ],
        p![
            text(
                "If you had added a key to each element, the initial html would be the same, but \
                 after the render with the modified list, "
            ),
            code("[\"bob\",\"rob\"]"),
            text(
                ", yew would just delete the second html element and leave the rest untouched \
                 since it can use the keys to associate them."
            ),
        ],
        p![text(
            "If you ever encounter a bug/\"feature\" where you switch from one component to \
             another but both have a div as the highest rendered element. Yew reuses the rendered \
             html div in those cases as an optimization. If you need that div to be recreated \
             instead of reused, then you can add different keys and they wont be reused"
        )],
        h2![text("Further reading")],
        ul![
            li![link![
                "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/todomvc",
                text("TodoMVC"),
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/keyed_list",
                text("Keyed list"),
            ]],
            li![link![
                "https://github.com/yewstack/yew/tree/yew-v0.20.0/examples/router",
                text("Router"),
            ]],
        ],
    ])
);
