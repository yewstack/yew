crate::doc_page!("HTML", "/docs/concepts/html",
    Content::new(vec![
        p![
            text("The "),
            code("html!"),
            text(" macro allows you to write HTML and SVG code declaratively. It is similar to JSX \
              (an extension to JavaScript which allows you to write HTML-like code inside of JavaScript)."),
        ],
        p![bold![text("Important notes")]],
        ol![
            li![
                text("The "),
                code("html!"),
                text(" macro only accepts one root html node (you can counteract this by using "),
                link!["/docs/concepts/html/fragments", text("fragments")],
                text(" or "),
                link!["/docs/concepts/html/lists", text("iterators")],
                text(")"),
            ],
            li![
                text("An empty "),
                code("html! {}"),
                text(" invocation is valid and will not render anything"),
            ],
            li![
                text("Literals must always be quoted and wrapped in braces: "),
                code("html! { <p>{ \"Hello, World\" }</p> }"),
            ],
            li![
                text("The "),
                code("html!"),
                text(" macro will make all tag names lower case. To use upper case characters (which are required for some SVG elements) use "),
                link!["/docs/concepts/html/elements#dynamic-tag-names", text("dynamic tag names")],
                text(": "),
                code("html! { <@{\"myTag\"}></@> }"),
            ],
        ],
        admonition![AdmonitionType::Note, None,
            p![
                text("The "),
                code("html!"),
                text(" macro can reach the default recursion limit of the compiler. \
                  If you encounter compilation errors, add an attribute like "),
                code("#![recursion_limit=\"1024\"]"),
                text(" in the crate root to overcome the problem."),
            ],
        ],
        h2![text("Tag Structure")],
        p![
            text("Tags are based on HTML tags. Components, Elements, and Lists are all based on this tag syntax."),
        ],
        p![
            text("Tags must either self-close "),
            code("<... />"),
            text(" or have a corresponding end tag for each start tag."),
        ],
        code_block("rust", r#"use yew::prelude::*;

html! {
<div id="my_div"></div>
};"#),
        p![text("Invalid (missing close tag):")],
        code_block("rust", r#"use yew::prelude::*;

html! {
<div id="my_div"> // <- MISSING CLOSE TAG
};"#),
        p![text("Self-closing:")],
        code_block("rust", r#"use yew::prelude::*;

html! {
<input id="my_input" />
};"#),
        p![text("Invalid (missing self-close):")],
        code_block("rust", r#"use yew::prelude::*;

html! {
<input id="my_input"> // <- MISSING SELF-CLOSE
};"#),
        admonition![AdmonitionType::Tip, None,
            p![
                text("For convenience, elements which usually require a closing tag are "),
                bold![text("allowed")],
                text(" to self-close. For example, writing "),
                code("html! { <div class=\"placeholder\" /> }"),
                text(" is valid."),
            ],
        ],
        h2![text("Children")],
        p![text("Create complex nested HTML and SVG layouts with ease:")],
        p![text("HTML example:")],
        code_block("rust", r#"use yew::prelude::*;

html! {
<div>
<div data-key="abc"></div>
<div class="parent">
    <span class="child" value="anything"></span>
    <label for="first-name">{ "First Name" }</label>
    <input type="text" id="first-name" value="placeholder" />
    <input type="checkbox" checked=true />
    <textarea value="write a story" />
    <select name="status">
        <option selected=true disabled=false value="">{ "Selected" }</option>
        <option selected=false disabled=true value="">{ "Unselected" }</option>
    </select>
</div>
</div>
};"#),
        p![text("SVG example:")],
        code_block("rust", r##"use yew::prelude::*;

html! {
<svg width="149" height="147" viewBox="0 0 149 147" fill="none" xmlns="http://www.w3.org/2000/svg">
<path d="M60.5776 13.8268L51.8673 42.6431L77.7475 37.331L60.5776 13.8268Z" fill="#DEB819"/>
<path d="M108.361 94.9937L138.708 90.686L115.342 69.8642" stroke="black" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
<g filter="url(#filter0_d)">
    <circle cx="75.3326" cy="73.4918" r="55" fill="#FDD630"/>
    <circle cx="75.3326" cy="73.4918" r="52.5" stroke="black" stroke-width="5"/>
</g>
<circle cx="71" cy="99" r="5" fill="white" fill-opacity="0.75" stroke="black" stroke-width="3"/>
<defs>
    <filter id="filter0_d" x="16.3326" y="18.4918" width="118" height="118" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
        <@{"feGaussianBlur"} stdDeviation="2"/>
        <@{"feColorMatrix"} in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"/>
    </filter>
</defs>
</svg>
};"##),
        h2![text("Lints")],
        p![
            text("If you compile Yew using a nightly version of the Rust compiler, the macro will warn you about some \
              common pitfalls that you might run into. Of course, you may need to use the stable compiler (e.g. \
              your organization might have a policy mandating it) for release builds, but even if you're using a \
              stable toolchain, running "),
            code("cargo +nightly check"),
            text(" might flag some ways that you could improve your HTML code."),
        ],
        p![
            text("At the moment the lints are mostly accessibility-related. If you have ideas for lints, please feel \
              free to "),
            link!["https://github.com/yewstack/yew/issues/1334", text("chime in on this issue")],
            text("."),
        ],
        h2![text("Specifying attributes and properties")],
        p![text("Attributes are set on elements in the same way as in normal HTML:")],
        code_block("rust", r#"use yew::prelude::*;

let value = "something";
html! { <div attribute={value} /> };"#),
        p![
            text("Properties are specified with "),
            code("~"),
            text(" before the element name:"),
        ],
        code_block("rust", r#"use yew::prelude::*;

html! { <my-element ~property="abc" /> };"#),
        admonition![AdmonitionType::Tip, None,
            p![text("The braces around the value can be omitted if the value is a literal.")],
        ],
        admonition![AdmonitionType::Note, Some("What classifies as a literal"),
            p![
                text("Literals are all valid "),
                link!["https://doc.rust-lang.org/reference/expressions/literal-expr.html", text("literal expressions")],
                text(" in Rust. Note that "),
                link!["https://users.rust-lang.org/t/why-are-negative-value-literals-expressions/43333", text("negative numbers are not literals")],
                text(" and thus must be enclosed in curly-braces "),
                code("{-6}"),
            ],
        ],
        admonition![AdmonitionType::Note, Some("Component properties"),
            p![
                text("Component properties are passed as Rust objects and are different from the element attributes/properties described here. \
                  Read more about them at "),
                link!["/docs/concepts/function-components/properties", text("Component Properties")],
            ],
        ],
        h3![text("Special properties")],
        p![
            text("There are special properties which don't directly influence the DOM but instead act as instructions to Yew's virtual DOM. \
              Currently, there are two such special props: "),
            code("ref"),
            text(" and "),
            code("key"),
            text("."),
        ],
        p![
            code("ref"),
            text(" allows you to access and manipulate the underlying DOM node directly. See "),
            link!["/docs/concepts/function-components/node-refs", text("Refs")],
            text(" for more details."),
        ],
        p![
            code("key"),
            text(" on the other hand gives an element a unique identifier which Yew can use for optimization purposes."),
        ],
        admonition![AdmonitionType::Info, None,
            p![
                text("Read more at "),
                link!["/docs/concepts/html/lists", text("Lists")],
            ],
        ],
        h2![text("Conditional Rendering")],
        p![
            text("Markup can be rendered conditionally by using Rust's conditional structures. \
              Currently only "),
            code("if"),
            text(" and "),
            code("if let"),
            text(" are supported."),
        ],
        code_block("rust", r#"use yew::prelude::*;

html! {
if true {
<p>{ "True case" }</p>
}
};"#),
        admonition![AdmonitionType::Info, None,
            p![
                text("Read more at "),
                link!["/docs/concepts/html/conditional-rendering", text("Conditional Rendering")],
            ],
        ],
    ])
);
