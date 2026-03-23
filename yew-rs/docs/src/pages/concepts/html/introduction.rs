pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            text("The "),
            code("html!"),
            text(" macro allows you to write HTML and SVG code declaratively. It is similar to JSX \
              (an extension to JavaScript that allows you to write HTML-like code inside of JavaScript)."),
        ]),
        p(vec![bold(vec![text("Important notes")])]),
        ol(vec![
            li(vec![
                text("The "),
                code("html!"),
                text(" macro only accepts one root html node (you can counteract this by using "),
                link("/docs/concepts/html/fragments", vec![text("fragments")]),
                text(" or "),
                link("/docs/concepts/html/lists", vec![text("iterators")]),
                text(")"),
            ]),
            li(vec![
                text("An empty "),
                code("html! {}"),
                text(" invocation is valid and will not render anything"),
            ]),
            li(vec![
                text("Literals must always be quoted and wrapped in braces: "),
                code("html! { <p>{ \"Hello, World\" }</p> }"),
            ]),
            li(vec![
                text("The "),
                code("html!"),
                text(" macro will make all tag names lowercase. To use upper case characters (which are required for some SVG elements) use "),
                link("/docs/concepts/html/elements#dynamic-tag-names", vec![text("dynamic tag names")]),
                text(": "),
                code("html! { <@{\"myTag\"}></@> }"),
            ]),
        ]),
        admonition(AdmonitionType::Note, None, vec![
            p(vec![
                text("The "),
                code("html!"),
                text(" macro can reach the default recursion limit of the compiler. \
                  If you encounter compilation errors, add an attribute like "),
                code("#![recursion_limit=\"1024\"]"),
                text(" in the crate root to overcome the problem."),
            ]),
        ]),
        h2(vec![text("Tag Structure")]),
        p(vec![
            text("Tags are based on HTML tags. Components, Elements, and Lists are all based on this tag syntax."),
        ]),
        p(vec![
            text("Tags must either self-close "),
            code("<... />"),
            text(" or have a corresponding end tag for each start tag."),
        ]),
        tabs("Open - Close", vec![
            tab("Open - Close", "Open - Close", vec![
                code_block("rust", r#"use yew::prelude::*;

html! {
  <div id="my_div"></div>
};"#),
            ]),
            tab("Invalid", "Invalid", vec![
                code_block_compile_fail("rust", r#"use yew::prelude::*;

html! {
  <div id="my_div"> // <- MISSING CLOSE TAG
};"#),
            ]),
        ]),
        tabs("Self-closing", vec![
            tab("Self-closing", "Self-closing", vec![
                code_block("rust", r#"use yew::prelude::*;

html! {
  <input id="my_input" />
};"#),
            ]),
            tab("Invalid", "Invalid", vec![
                code_block_compile_fail("rust", r#"use yew::prelude::*;

html! {
  <input id="my_input"> // <- MISSING SELF-CLOSE
};"#),
            ]),
        ]),
        admonition(AdmonitionType::Tip, None, vec![
            p(vec![
                text("For convenience, elements which "),
                italic(vec![text("usually")]),
                text(" require a closing tag are "),
                bold(vec![text("allowed")]),
                text(" to self-close. For example, writing "),
                code("html! { <div class=\"placeholder\" /> }"),
                text(" is valid."),
            ]),
        ]),
        h2(vec![text("Children")]),
        p(vec![text("Create complex nested HTML and SVG layouts with ease:")]),
        tabs("HTML", vec![
            tab("HTML", "HTML", vec![
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
            ]),
            tab("SVG", "SVG", vec![
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
            ]),
        ]),
        h2(vec![text("Lints")]),
        p(vec![
            text("If you compile Yew using a nightly version of the Rust compiler, the macro will warn you about some \
              common pitfalls that you might run into. Of course, you may need to use the stable compiler (e.g. \
              your organization might have a policy mandating it) for release builds, but even if you're using a \
              stable toolchain, running "),
            code("cargo +nightly check"),
            text(" might flag some ways that you could improve your HTML code."),
        ]),
        p(vec![
            text("At the moment the lints are mostly accessibility-related. If you have ideas for lints, please feel \
              free to "),
            link("https://github.com/yewstack/yew/issues/1334", vec![text("chime in on this issue")]),
            text("."),
        ]),
        h2(vec![text("Specifying attributes and properties")]),
        p(vec![text("Attributes are set on elements in the same way as in normal HTML:")]),
        code_block("rust", r#"use yew::prelude::*;

let value = "something";
html! { <div attribute={value} /> };"#),
        p(vec![
            text("Properties are specified with "),
            code("~"),
            text(" before the element name:"),
        ]),
        code_block_ignore("rust", r#"use yew::prelude::*;

html! { <my-element ~property="abc" /> };"#),
        admonition(AdmonitionType::Tip, None, vec![
            p(vec![text("The braces around the value can be omitted if the value is a literal.")]),
        ]),
        admonition(AdmonitionType::Note, Some("What classifies as a literal"), vec![
            p(vec![
                text("Literals are all valid "),
                link("https://doc.rust-lang.org/reference/expressions/literal-expr.html", vec![text("literal expressions")]),
                text(" in Rust. Note that "),
                link("https://users.rust-lang.org/t/why-are-negative-value-literals-expressions/43333", vec![text("negative numbers are "), bold(vec![text("not")]), text(" literals")]),
                text(" and thus must be enclosed in curly-braces "),
                code("{-6}"),
            ]),
        ]),
        admonition(AdmonitionType::Note, Some("Component properties"), vec![
            p(vec![
                text("Component properties are passed as Rust objects and are different from the element attributes/properties described here. \
                  Read more about them at "),
                link("/docs/concepts/function-components/properties", vec![text("Component Properties")]),
            ]),
        ]),
        h3(vec![text("Special properties")]),
        p(vec![
            text("There are special properties which don't directly influence the DOM but instead act as instructions to Yew's virtual DOM. \
              Currently, there are two such special props: "),
            code("ref"),
            text(" and "),
            code("key"),
            text("."),
        ]),
        p(vec![
            code("ref"),
            text(" allows you to access and manipulate the underlying DOM node directly. See "),
            link("/docs/concepts/function-components/node-refs", vec![text("Refs")]),
            text(" for more details."),
        ]),
        p(vec![
            code("key"),
            text(" on the other hand gives an element a unique identifier which Yew can use for optimization purposes."),
        ]),
        admonition(AdmonitionType::Info, None, vec![
            p(vec![
                text("Read more at "),
                link("/docs/concepts/html/lists", vec![text("Lists")]),
            ]),
        ]),
        h2(vec![text("Comments")]),
        p(vec![text("It is also possible to use Rust style comments as part of the HTML structure:")]),
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
        p(vec![text("Comments will be dropped during the parsing process and will not end up in the final output.")]),
        h2(vec![text("Conditional Rendering")]),
        p(vec![
            text("Markup can be rendered conditionally by using Rust's conditional structures. \
              Currently only "),
            code("if"),
            text(" and "),
            code("if let"),
            text(" are supported."),
        ]),
        code_block("rust", r#"use yew::prelude::*;

html! {
  if true {
      <p>{ "True case" }</p>
  }
};"#),
        admonition(AdmonitionType::Info, None, vec![
            p(vec![
                text("Read more at "),
                link("/docs/concepts/html/conditional-rendering", vec![text("Conditional Rendering")]),
            ]),
        ]),
    ])
}

crate::doc_page!("HTML", "/docs/concepts/html", page_content());
