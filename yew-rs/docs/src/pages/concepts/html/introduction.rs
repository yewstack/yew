pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "The ",
            code("html!"),
            " macro allows you to write HTML and SVG code declaratively. It is similar to JSX \
              (an extension to JavaScript that allows you to write HTML-like code inside of JavaScript).",
        ],
        p![bold!["Important notes"]],
        ol![
            li![
                "The ",
                code("html!"),
                " macro only accepts one root html node (you can counteract this by using ",
                doc_link!(crate::pages::concepts::html::fragments, "fragments"),
                " or ",
                doc_link!(crate::pages::concepts::html::lists, "iterators"),
                ")",
            ],
            li![
                "An empty ",
                code("html! {}"),
                " invocation is valid and will not render anything",
            ],
            li![
                "Literals must always be quoted and wrapped in braces: ",
                code("html! { <p>{ \"Hello, World\" }</p> }"),
            ],
            li![
                "The ",
                code("html!"),
                " macro will make all tag names lowercase. To use upper case characters (which are required for some SVG elements) use ",
                doc_link!(crate::pages::concepts::html::elements, #"dynamic-tag-names", "dynamic tag names"),
                ": ",
                code("html! { <@{\"myTag\"}></@> }"),
            ],
        ],
        admonition!(AdmonitionType::Note, None,
            p![
                "The ",
                code("html!"),
                " macro can reach the default recursion limit of the compiler. \
                  If you encounter compilation errors, add an attribute like ",
                code("#![recursion_limit=\"1024\"]"),
                " in the crate root to overcome the problem.",
            ],
        ),
        h2!["Tag Structure"],
        p![
            "Tags are based on HTML tags. Components, Elements, and Lists are all based on this tag syntax.",
        ],
        p![
            "Tags must either self-close ",
            code("<... />"),
            " or have a corresponding end tag for each start tag.",
        ],
        tabs!("Open - Close",
            tab!("Open - Close", "Open - Close",
                code_block("rust", r#"use yew::prelude::*;

html! {
  <div id="my_div"></div>
};"#),
            ),
            tab!("Invalid", "Invalid",
                code_block_compile_fail("rust", r#"use yew::prelude::*;

html! {
  <div id="my_div"> // <- MISSING CLOSE TAG
};"#),
            ),
        ),
        tabs!("Self-closing",
            tab!("Self-closing", "Self-closing",
                code_block("rust", r#"use yew::prelude::*;

html! {
  <input id="my_input" />
};"#),
            ),
            tab!("Invalid", "Invalid",
                code_block_compile_fail("rust", r#"use yew::prelude::*;

html! {
  <input id="my_input"> // <- MISSING SELF-CLOSE
};"#),
            ),
        ),
        admonition!(AdmonitionType::Tip, None,
            p![
                "For convenience, elements which ",
                italic!["usually"],
                " require a closing tag are ",
                bold!["allowed"],
                " to self-close. For example, writing ",
                code("html! { <div class=\"placeholder\" /> }"),
                " is valid.",
            ],
        ),
        h2!["Children"],
        p!["Create complex nested HTML and SVG layouts with ease:"],
        tabs!("HTML",
            tab!("HTML", "HTML",
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
            ),
            tab!("SVG", "SVG",
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
            ),
        ),
        h2!["Lints"],
        p![
            "If you compile Yew using a nightly version of the Rust compiler, the macro will warn you about some \
              common pitfalls that you might run into. Of course, you may need to use the stable compiler (e.g. \
              your organization might have a policy mandating it) for release builds, but even if you're using a \
              stable toolchain, running ",
            code("cargo +nightly check"),
            " might flag some ways that you could improve your HTML code.",
        ],
        p![
            "At the moment the lints are mostly accessibility-related. If you have ideas for lints, please feel \
              free to ",
            link!("https://github.com/yewstack/yew/issues/1334", "chime in on this issue"),
            ".",
        ],
        h2!["Specifying attributes and properties"],
        p!["Attributes are set on elements in the same way as in normal HTML:"],
        code_block("rust", r#"use yew::prelude::*;

let value = "something";
html! { <div attribute={value} /> };"#),
        p![
            "Properties are specified with ",
            code("~"),
            " before the element name:",
        ],
        code_block_ignore("rust", r#"use yew::prelude::*;

html! { <my-element ~property="abc" /> };"#),
        admonition!(AdmonitionType::Tip, None,
            p!["The braces around the value can be omitted if the value is a literal."],
        ),
        admonition!(AdmonitionType::Note, Some("What classifies as a literal"),
            p![
                "Literals are all valid ",
                link!("https://doc.rust-lang.org/reference/expressions/literal-expr.html", "literal expressions"),
                " in Rust. Note that ",
                link!("https://users.rust-lang.org/t/why-are-negative-value-literals-expressions/43333", "negative numbers are ", bold!["not"], " literals"),
                " and thus must be enclosed in curly-braces ",
                code("{-6}"),
            ],
        ),
        admonition!(AdmonitionType::Note, Some("Component properties"),
            p![
                "Component properties are passed as Rust objects and are different from the element attributes/properties described here. \
                  Read more about them at ",
                doc_link!(crate::pages::concepts::function_components::properties, "Component Properties"),
            ],
        ),
        h3!["Special properties"],
        p![
            "There are special properties which don't directly influence the DOM but instead act as instructions to Yew's virtual DOM. \
              Currently, there are two such special props: ",
            code("ref"),
            " and ",
            code("key"),
            ".",
        ],
        p![
            code("ref"),
            " allows you to access and manipulate the underlying DOM node directly. See ",
            doc_link!(crate::pages::concepts::function_components::node_refs, "Refs"),
            " for more details.",
        ],
        p![
            code("key"),
            " on the other hand gives an element a unique identifier which Yew can use for optimization purposes.",
        ],
        admonition!(AdmonitionType::Info, None,
            p![
                "Read more at ",
                doc_link!(crate::pages::concepts::html::lists, "Lists"),
            ],
        ),
        h2!["Comments"],
        p!["It is also possible to use Rust style comments as part of the HTML structure:"],
        code_block("rust", r#"use yew::prelude::*;

html! {
  <>
    <h1>{ "My heading" }</h1>
    // here comes the content
    <main>
      { "…" }
    </main>
  </>
};"#),
        p!["Comments will be dropped during the parsing process and will not end up in the final output."],
        h2!["Conditional Rendering"],
        p![
            "Markup can be rendered conditionally by using Rust's conditional structures. \
              Currently only ",
            code("if"),
            " and ",
            code("if let"),
            " are supported.",
        ],
        code_block("rust", r#"use yew::prelude::*;

html! {
  if true {
      <p>{ "True case" }</p>
  }
};"#),
        admonition!(AdmonitionType::Info, None,
            p![
                "Read more at ",
                doc_link!(crate::pages::concepts::html::conditional_rendering, "Conditional Rendering"),
            ],
        ),
    ])
}

crate::doc_page!("HTML", "/docs/concepts/html", page_content());
