pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("html!"),
            " 宏允许您声明性地编写 HTML 和 SVG \
                 代码。它类似于 JSX（一种允许您在 JavaScript 中编写类似 HTML 的代码的扩展）。",
        ],
        p![bold!["重要提示"]],
        ol![
            li![
                code("html! {}"),
                " 宏只能接受一个根 HTML 节点（您可以通过使用 ",
                doc_link!(crate::pages::concepts::html::fragments, "fragments"),
                " 或 ",
                doc_link!(crate::pages::concepts::html::lists, "iterators"),
                " 来规避这一点）",
            ],
            li![
                "空的 ",
                code("html! {}"),
                " 调用是有效的，不会渲染任何内容",
            ],
            li![
                "字面量必须始终用引号引起来并用大括号括起来：",
                code("html! { <p>{ \"Hello, World\" }</p> }"),
            ],
            li![
                code("html!"),
                " 宏会将所有标签名称转换为小写。要使用大写字符（某些 SVG 元素所需的字符）请使用",
                doc_link!(
                    crate::pages::concepts::html::elements,
                    #"dynamic-tag-names",
                    "动态标签名称",
                ),
                "：",
                code("html! { <@{\"myTag\"}></@> }"),
            ],
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                code("html!"),
                " 宏可能会达到编译器的默认递归限制。如果遇到编译错误，请在 crate 根目录添加类似 ",
                code("#![recursion_limit=\"1024\"]"),
                " 的属性以解决问题。",
            ],
        ],
        h2!["标签 (Tags) 结构"],
        p!["标签 (Tags) 基于 HTML 标签。组件、元素和列表都基于此标签语法。"],
        p![
            "标签必须要么自闭合 ",
            code("<... />"),
            "，要么对于每个开始标签都有一个相应的结束标签。",
        ],
        tabs!["Open - Close",
            tab!["Open - Close", "Open - Close",
                code_block("rust", r#"use yew::prelude::*;

html! {
  <div id="my_div"></div>
};"#),
            ],
            tab!["Invalid", "Invalid",
                code_block("rust", r#"use yew::prelude::*;

html! {
  <div id="my_div"> // <- 缺少闭合标签
};"#),
            ],
        ],
        tabs!["Self-closing",
            tab!["Self-closing", "Self-closing",
                code_block("rust", r#"use yew::prelude::*;

html! {
  <input id="my_input" />
};"#),
            ],
            tab!["Invalid", "Invalid",
                code_block("rust", r#"use yew::prelude::*;

html! {
  <input id="my_input"> // <- 缺少闭合标签
};"#),
            ],
        ],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                "方便起见，通常需要闭合标签的元素",
                bold!["允许"],
                "自闭合。例如，编写 ",
                code("html! { <div class=\"placeholder\" /> }"),
                " 是有效的。",
            ],
        ],
        p!["创建复杂的嵌套 HTML 和 SVG 布局还是很容易的："],
        tabs!["HTML",
            tab!["HTML", "HTML",
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
            ],
            tab!["SVG", "SVG",
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
            ],
        ],
        h2!["Lints"],
        p![
            "如果您使用 Rust 编译器的开发者版本编译 Yew，宏将警告您可能遇到的一些常见陷阱。当然，您可能需要使用稳定版编译器（例如，您的组织可能有政策要求这样做）进行发布构建，但即使您使用的是稳定工具链，运行 ",
            code("cargo +nightly check"),
            " 也可能会标记一些可以改进 HTML 代码的方法。",
        ],
        p![
            "目前，这些 lint 主要与可访问性相关。如果您有 lint 的想法，请随时",
            link!(
                "https://github.com/yewstack/yew/issues/1334",
                "在此问题中发表意见",
            ),
            "。",
        ],
        h2!["指定属性和属性"],
        p!["属性与普通 HTML 中的元素设置方式相同："],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let value = "something";
html! { <div attribute={value} /> };"#,
        ),
        p![
            "属性在元素名称之前用 ",
            code("~"),
            " 指定：",
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! { <my-element ~property="abc" /> };"#,
        ),
        admonition![
            AdmonitionType::Tip,
            None,
            p!["如果值是一个字面量的话，围绕值的大括号可以省略。"],
        ],
        admonition![
            AdmonitionType::Note,
            Some("什么是字面量"),
            p![
                "字面量是 Rust 中所有有效的",
                link!(
                    "https://doc.rust-lang.org/reference/expressions/literal-expr.html",
                    "字面量表达式",
                ),
                "。请注意，",
                link!(
                    "https://users.rust-lang.org/t/why-are-negative-value-literals-expressions/43333",
                    "负数", bold!["不是"], "字面量",
                ),
                "，因此必须用大括号括起来 ",
                code("{-6}"),
                "。",
            ],
        ],
        admonition![
            AdmonitionType::Note,
            Some("组件属性"),
            p![
                "组件属性作为 Rust 对象传递，与此处描述的元素参数 (Attributes) / 属性 (Properties) 不同。\n在",
                doc_link!(crate::pages::concepts::function_components::properties, "组件属性"),
                "中了解更多信息。",
            ],
        ],
        h3!["特殊属性"],
        p![
            "有一些特殊属性不直接影响 DOM，而是作为 Yew 虚拟 DOM 的指令。目前有两个这样的特殊属性：",
            code("ref"),
            " 和 ",
            code("key"),
            "。",
        ],
        p![
            code("ref"),
            " 允许您直接访问和操作底层 DOM 节点。有关更多详细信息，请参阅",
            doc_link!(crate::pages::concepts::function_components::node_refs, "Refs"),
            "。",
        ],
        p![
            "另一方面，",
            code("key"),
            " 为元素提供了一个唯一标识符，Yew 可以用于优化目的。",
        ],
        admonition![
            AdmonitionType::Info,
            None,
            p![doc_link!(
                crate::pages::concepts::html::lists,
                "了解更多相关内容",
            )],
        ],
        h2!["条件渲染"],
        p![
            "可以通过使用 Rust 的条件结构来条件性地渲染标记。目前只支持 ",
            code("if"),
            " 和 ",
            code("if let"),
            "。",
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! {
  if true {
      <p>{ "True case" }</p>
  }
};"#,
        ),
        admonition![
            AdmonitionType::Info,
            None,
            p![
                "阅读",
                doc_link!(crate::pages::concepts::html::conditional_rendering, "条件渲染"),
                "一节了解更多",
            ],
        ],
    ])
}

crate::doc_page!("HTML", "/zh-Hans/docs/concepts/html", page_content());
