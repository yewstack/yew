pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            code("html!"),
            text(" 巨集可讓您聲明性地編寫 HTML 和 SVG 程式碼。它類似於 JSX（一種允許您在 JavaScript 中編寫類似 HTML 的程式碼的擴充）。"),
        ],
        p![bold![text("重要提示")]],
        ol![
            li![
                code("html! {}"),
                text(" 巨集只能接受一個根HTML 節點（您可以透過使用"),
                link!("/zh-Hant/docs/concepts/html/fragments", text("fragments")),
                text(" 或"),
                link!("/zh-Hant/docs/concepts/html/lists", text("iterators")),
                text(" 來規避這一點）"),
            ],
            li![
                text("空的 "),
                code("html! {}"),
                text(" 呼叫是有效的，不會渲染任何內容"),
            ],
            li![
                text("字面量必須永遠用引號引起來並用大括號括起來："),
                code("html! { <p>{ \"Hello, World\" }</p> }"),
            ],
            li![
                code("html!"),
                text(" 巨集會將所有標籤名稱轉換為小寫。若要使用大寫字元（某些SVG 元素所需的字元）請使用"),
                link!(
                    "/zh-Hant/docs/concepts/html/elements#dynamic-tag-names",
                    text("動態標籤名稱"),
                ),
                text("："),
                code("html! { <@{\"myTag\"}>< /@> }"),
            ],
        ],
        admonition![
            AdmonitionType::Note,
            None,
            p![
                code("html!"),
                text(" 巨集可能會達到編譯器的預設遞歸限制。如果遇到編譯錯誤，請在 crate 根目錄中新增類似 "),
                code("#![recursion_limit=\"1024\"]"),
                text(" 的屬性以解決問題。"),
            ],
        ],
        h2![text("標籤 (Tags) 結構")],
        p![text(
            "標籤 (Tags) 是基於 HTML 標籤。元件、元素和清單都基於此標籤語法。",
        )],
        p![
            text("標籤必須或自閉合 "),
            code("<... />"),
            text("，或對於每個開始標籤都有一個對應的結束標籤。"),
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
  <div id="my_div"> // <- 缺少閉合標籤
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
  <input id="my_input"> // <- 缺少閉合標籤
};"#),
            ],
        ],
        admonition![
            AdmonitionType::Tip,
            None,
            p![
                text("方便起見，通常需要閉合標籤的元素"),
                bold![text("允許")],
                text("自閉合。例如，編寫 "),
                code("html! { <div class=\"placeholder\" /> }"),
                text(" 是有效的。"),
            ],
        ],
        p![text(
            "建立複雜的巢狀 HTML 和 SVG 佈局還是很容易的：",
        )],
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
        h2![text("Lints")],
        p![
            text("如果您使用 Rust 編譯器的開發者版本編譯 Yew，巨集將警告您可能遇到的一些常見陷阱。當然，您可能需要使用穩定版編譯器（例如，您的組織可能有政策要求這樣做）進行發布構建，但即使您使用的是穩定工具鏈，運行"),
            code("cargo +nightly check"),
            text(" 也可能會標記一些可以改進HTML 程式碼的方法。"),
        ],
        p![
            text("目前，這些 lint 主要與可訪問性相關。如果您有 lint 的想法，請隨時"),
            link!(
                "https://github.com/yewstack/yew/issues/1334",
                text("在此問題中發表意見"),
            ),
            text("。"),
        ],
        h2![text("指定屬性和屬性")],
        p![text(
            "屬性與普通 HTML 中的元素設定方式相同：",
        )],
        code_block(
            "rust",
            r#"use yew::prelude::*;

let value = "something";
html! { <div attribute={value} /> };"#,
        ),
        p![
            text("屬性在元素名稱之前用 "),
            code("~"),
            text(" 指定："),
        ],
        code_block(
            "rust",
            r#"use yew::prelude::*;

html! { <my-element ~property="abc" /> };"#,
        ),
        admonition![
            AdmonitionType::Tip,
            None,
            p![text(
                "如果值是一個字面量的話，圍繞值的大括號可以省略。",
            )],
        ],
        admonition![
            AdmonitionType::Note,
            Some("什麼是字面量"),
            p![
                text("字面量是 Rust 中所有有效的"),
                link!(
                    "https://doc.rust-lang.org/reference/expressions/literal-expr.html",
                    text("字面量表達式"),
                ),
                text("。請注意，"),
                link!(
                    "https://users.rust-lang.org/t/why-are-negative-value-literals-expressions/43333",
                    text("負數"), bold![text("不是")], text("字面量"),
                ),
                text("，因此必須用大括號括起來 "),
                code("{-6}"),
                text("。"),
            ],
        ],
        admonition![
            AdmonitionType::Note,
            Some("元件屬性"),
            p![
                text("元件屬性以 Rust 物件傳遞，與此處所述的元素參數 (Attributes) / 屬性 (Properties) 不同。\n在"),
                link!("/zh-Hant/docs/concepts/function-components/properties", text("元件屬性")),
                text("中了解更多。"),
            ],
        ],
        h3![text("特殊屬性")],
        p![
            text("有一些特殊屬性不會直接影響 DOM，而是作為 Yew 虛擬 DOM 的指令。目前有兩個這樣的特殊屬性："),
            code("ref"),
            text(" 和 "),
            code("key"),
            text("。"),
        ],
        p![
            code("ref"),
            text(" 可讓您直接存取和操作底層 DOM 節點。有關更多詳細信息，請參閱 "),
            link!("/zh-Hant/docs/concepts/function-components/node-refs", text("Refs")),
            text("。"),
        ],
        p![
            text("另一方面，"),
            code("key"),
            text(" 為元素提供了一個唯一標識符，Yew 可以用於最佳化目的。"),
        ],
        admonition![
            AdmonitionType::Info,
            None,
            p![link!(
                "/zh-Hant/docs/concepts/html/lists",
                text("了解更多相關內容"),
            )],
        ],
        h2![text("條件渲染")],
        p![
            text("可以透過使用 Rust 的條件結構來條件性地渲染標記。目前只支援 "),
            code("if"),
            text(" 和 "),
            code("if let"),
            text("。"),
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
                text("閱讀"),
                link!("/zh-Hant/docs/concepts/html/conditional-rendering", text("條件渲染")),
                text("一節以了解更多"),
            ],
        ],
    ])
}

crate::doc_page!("HTML", "/zh-Hant/docs/concepts/html", page_content());
