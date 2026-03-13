pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            code("html!"),
            text(
                " 宏允许您声明性地编写 HTML 和 SVG \
                 代码。它类似于 JSX（一种允许您在 JavaScript 中编写类似 HTML 的代码的扩展）。",
            ),
        ]),
        p(vec![bold(vec![text("重要提示")])]),
        ol(vec![
            li(vec![
                code("html! {}"),
                text(" 宏只能接受一个根 HTML 节点（您可以通过使用 "),
                link("/zh-Hans/docs/concepts/html/fragments", vec![text("fragments")]),
                text(" 或 "),
                link("/zh-Hans/docs/concepts/html/lists", vec![text("iterators")]),
                text(" 来规避这一点）"),
            ]),
            li(vec![
                text("空的 "),
                code("html! {}"),
                text(" 调用是有效的，不会渲染任何内容"),
            ]),
            li(vec![
                text("字面量必须始终用引号引起来并用大括号括起来："),
                code("html! { <p>{ \"Hello, World\" }</p> }"),
            ]),
            li(vec![
                code("html!"),
                text(" 宏会将所有标签名称转换为小写。要使用大写字符（某些 SVG 元素所需的字符）请使用"),
                link(
                    "/zh-Hans/docs/concepts/html/elements#dynamic-tag-names",
                    vec![text("动态标签名称")],
                ),
                text("："),
                code("html! { <@{\"myTag\"}></@> }"),
            ]),
        ]),
        admonition(
            AdmonitionType::Note,
            None,
            vec![p(vec![
                code("html!"),
                text(" 宏可能会达到编译器的默认递归限制。如果遇到编译错误，请在 crate 根目录添加类似 "),
                code("#![recursion_limit=\"1024\"]"),
                text(" 的属性以解决问题。"),
            ])],
        ),
        h2(vec![text("标签 (Tags) 结构")]),
        p(vec![text(
            "标签 (Tags) 基于 HTML 标签。组件、元素和列表都基于此标签语法。",
        )]),
        p(vec![
            text("标签必须要么自闭合 "),
            code("<... />"),
            text("，要么对于每个开始标签都有一个相应的结束标签。"),
        ]),
        tabs("Open - Close", vec![
            tab("Open - Close", "Open - Close", vec![
                code_block("rust", r#"use yew::prelude::*;

html! {
  <div id="my_div"></div>
};"#),
            ]),
            tab("Invalid", "Invalid", vec![
                code_block("rust", r#"use yew::prelude::*;

html! {
  <div id="my_div"> // <- 缺少闭合标签
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
                code_block("rust", r#"use yew::prelude::*;

html! {
  <input id="my_input"> // <- 缺少闭合标签
};"#),
            ]),
        ]),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![
                text("方便起见，通常需要闭合标签的元素"),
                bold(vec![text("允许")]),
                text("自闭合。例如，编写 "),
                code("html! { <div class=\"placeholder\" /> }"),
                text(" 是有效的。"),
            ])],
        ),
        p(vec![text(
            "创建复杂的嵌套 HTML 和 SVG 布局还是很容易的：",
        )]),
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
            text("如果您使用 Rust 编译器的开发者版本编译 Yew，宏将警告您可能遇到的一些常见陷阱。当然，您可能需要使用稳定版编译器（例如，您的组织可能有政策要求这样做）进行发布构建，但即使您使用的是稳定工具链，运行 "),
            code("cargo +nightly check"),
            text(" 也可能会标记一些可以改进 HTML 代码的方法。"),
        ]),
        p(vec![
            text("目前，这些 lint 主要与可访问性相关。如果您有 lint 的想法，请随时"),
            link(
                "https://github.com/yewstack/yew/issues/1334",
                vec![text("在此问题中发表意见")],
            ),
            text("。"),
        ]),
        h2(vec![text("指定属性和属性")]),
        p(vec![text(
            "属性与普通 HTML 中的元素设置方式相同：",
        )]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

let value = "something";
html! { <div attribute={value} /> };"#,
        ),
        p(vec![
            text("属性在元素名称之前用 "),
            code("~"),
            text(" 指定："),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! { <my-element ~property="abc" /> };"#,
        ),
        admonition(
            AdmonitionType::Tip,
            None,
            vec![p(vec![text(
                "如果值是一个字面量的话，围绕值的大括号可以省略。",
            )])],
        ),
        admonition(
            AdmonitionType::Note,
            Some("什么是字面量"),
            vec![p(vec![
                text("字面量是 Rust 中所有有效的"),
                link(
                    "https://doc.rust-lang.org/reference/expressions/literal-expr.html",
                    vec![text("字面量表达式")],
                ),
                text("。请注意，"),
                link(
                    "https://users.rust-lang.org/t/why-are-negative-value-literals-expressions/43333",
                    vec![text("负数"), bold(vec![text("不是")]), text("字面量")],
                ),
                text("，因此必须用大括号括起来 "),
                code("{-6}"),
                text("。"),
            ])],
        ),
        admonition(
            AdmonitionType::Note,
            Some("组件属性"),
            vec![p(vec![
                text("组件属性作为 Rust 对象传递，与此处描述的元素参数 (Attributes) / 属性 (Properties) 不同。\n在"),
                link("/zh-Hans/docs/concepts/function-components/properties", vec![text("组件属性")]),
                text("中了解更多信息。"),
            ])],
        ),
        h3(vec![text("特殊属性")]),
        p(vec![
            text("有一些特殊属性不直接影响 DOM，而是作为 Yew 虚拟 DOM 的指令。目前有两个这样的特殊属性："),
            code("ref"),
            text(" 和 "),
            code("key"),
            text("。"),
        ]),
        p(vec![
            code("ref"),
            text(" 允许您直接访问和操作底层 DOM 节点。有关更多详细信息，请参阅"),
            link("/zh-Hans/docs/concepts/function-components/node-refs", vec![text("Refs")]),
            text("。"),
        ]),
        p(vec![
            text("另一方面，"),
            code("key"),
            text(" 为元素提供了一个唯一标识符，Yew 可以用于优化目的。"),
        ]),
        admonition(
            AdmonitionType::Info,
            None,
            vec![p(vec![link(
                "/zh-Hans/docs/concepts/html/lists",
                vec![text("了解更多相关内容")],
            )])],
        ),
        h2(vec![text("条件渲染")]),
        p(vec![
            text("可以通过使用 Rust 的条件结构来条件性地渲染标记。目前只支持 "),
            code("if"),
            text(" 和 "),
            code("if let"),
            text("。"),
        ]),
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! {
  if true {
      <p>{ "True case" }</p>
  }
};"#,
        ),
        admonition(
            AdmonitionType::Info,
            None,
            vec![p(vec![
                text("阅读"),
                link("/zh-Hans/docs/concepts/html/conditional-rendering", vec![text("条件渲染")]),
                text("一节了解更多"),
            ])],
        ),
    ])
}

crate::doc_page!("HTML", "/zh-Hans/docs/concepts/html", page_content());
