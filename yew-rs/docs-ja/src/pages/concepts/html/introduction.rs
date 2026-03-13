pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p(vec![
            code("html!"),
            text(" マクロを使用すると、宣言的に HTML および SVG コードを記述できます。これは、JavaScript で HTML に似たコードを記述できる拡張機能である JSX に似ています。"),
        ]),
        p(vec![bold(vec![text("重要な注意点")])]),
        ol(vec![
            li(vec![
                code("html!"),
                text(" マクロは 1 つのルート HTML ノードしか受け入れません（これを回避するには、"),
                link("/ja/docs/concepts/html/fragments", vec![text("fragments")]),
                text(" または "),
                link("/ja/docs/concepts/html/lists", vec![text("iterators")]),
                text(" を使用できます）"),
            ]),
            li(vec![
                text("空の "),
                code("html! {}"),
                text(" 呼び出しは有効で、何もレンダリングしません"),
            ]),
            li(vec![
                text("リテラルは常に引用符で囲み、中括弧で囲む必要があります："),
                code("html! { <p>{ \"Hello, World\" }</p> }"),
            ]),
            li(vec![
                code("html!"),
                text(" マクロはすべてのタグ名を小文字に変換します。大文字の文字（特定の SVG 要素に必要な文字）を使用するには、"),
                link("/ja/docs/concepts/html/elements#dynamic-tag-names", vec![text("動的タグ名")]),
                text(" を使用してください："),
                code("html! { <@{\"myTag\"}></@> }"),
            ]),
        ]),
        admonition(AdmonitionType::Note, None, vec![
            p(vec![
                code("html!"),
                text(" マクロはコンパイラのデフォルトの再帰制限に達する可能性があります。コンパイル エラーが発生した場合は、クレートのルートに "),
                code("#![recursion_limit=\"1024\"]"),
                text(" のような属性を追加して問題を解決してください。"),
            ]),
        ]),
        h2(vec![text("タグ (Tags) 構造")]),
        p(vec![text("タグ (Tags) は HTML タグに基づいています。コンポーネント、要素、およびリストはすべてこのタグ構文に基づいています。")]),
        p(vec![
            text("タグは自己閉鎖 "),
            code("<... />"),
            text(" であるか、開始タグごとに対応する終了タグが必要です。"),
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
  <div id="my_div"> // <- 閉じタグがありません
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
  <input id="my_input"> // <- 閉じタグがありません
};"#),
            ]),
        ]),
        admonition(AdmonitionType::Tip, None, vec![
            p(vec![
                text("便利のために、通常は閉じタグが必要な要素も"),
                bold(vec![text("自己閉じ")]),
                text("が許可されています。例えば、"),
                code("html! { <div class=\"placeholder\" /> }"),
                text(" と記述することができます。"),
            ]),
        ]),
        p(vec![text("複雑なネストされた HTML および SVG レイアウトを作成するのは依然として簡単です：")]),
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
            text("もし、Rust コンパイラの開発者バージョンを使用して Yew をコンパイルする場合、マクロは一般的な落とし穴について警告します。もちろん、安定版コンパイラを使用してリリースビルドを行う必要があるかもしれません（例えば、組織のポリシーでそうする必要がある場合など）。しかし、安定版ツールチェーンを使用している場合でも、"),
            code("cargo +nightly check"),
            text(" を実行すると、HTML コードを改善する方法がいくつか示されるかもしれません。"),
        ]),
        p(vec![
            text("現在、これらのリントは主にアクセシビリティに関連しています。リントに関するアイデアがあれば、"),
            link("https://github.com/yewstack/yew/issues/1334", vec![text("この問題")]),
            text("に自由にコメントしてください。"),
        ]),
        h2(vec![text("属性とプロパティの指定")]),
        p(vec![text("属性は通常の HTML と同じ方法で要素に設定されます：")]),
        code_block("rust", r#"use yew::prelude::*;

let value = "something";
html! { <div attribute={value} /> };"#),
        p(vec![
            text("属性は要素名の前に "),
            code("~"),
            text(" を使用して指定されます："),
        ]),
        code_block("rust", r#"use yew::prelude::*;

html! { <my-element ~property="abc" /> };"#),
        admonition(AdmonitionType::Tip, None, vec![
            p(vec![text("値がリテラルの場合、値を囲む中括弧は省略できます。")]),
        ]),
        admonition(AdmonitionType::Note, Some("リテラルとは"), vec![
            p(vec![
                text("リテラルは、Rust のすべての有効な"),
                link("https://doc.rust-lang.org/reference/expressions/literal-expr.html", vec![text("リテラル式")]),
                text("です。注意してください、"),
                link("https://users.rust-lang.org/t/why-are-negative-value-literals-expressions/43333", vec![text("負の数は"), bold(vec![text("リテラルではありません")])]),
                text("、したがって中括弧で囲む必要があります "),
                code("{-6}"),
            ]),
        ]),
        admonition(AdmonitionType::Note, Some("コンポーネント属性"), vec![
            p(vec![
                text("コンポーネント属性は Rust オブジェクトとして渡され、ここで説明されている要素のパラメータ (Attributes) / 属性 (Properties) とは異なります。"),
                link("/ja/docs/concepts/function-components/properties", vec![text("コンポーネント属性")]),
                text("で詳細を確認してください。"),
            ]),
        ]),
        h3(vec![text("特殊属性")]),
        p(vec![
            text("いくつかの特殊な属性があり、これらは直接 DOM に影響を与えるのではなく、Yew 仮想 DOM の指示として機能します。現在、2 つの特殊な属性があります："),
            code("ref"),
            text(" と "),
            code("key"),
            text("。"),
        ]),
        p(vec![
            code("ref"),
            text(" は、基礎となる DOM ノードに直接アクセスして操作することを可能にします。詳細については、"),
            link("/ja/docs/concepts/function-components/node-refs", vec![text("Refs")]),
            text("を参照してください。"),
        ]),
        p(vec![
            text("一方、"),
            code("key"),
            text(" は要素に一意の識別子を提供し、Yew が最適化のために使用できます。"),
        ]),
        admonition(AdmonitionType::Info, None, vec![
            p(vec![
                link("/ja/docs/concepts/html/lists", vec![text("詳細はこちら")]),
            ]),
        ]),
        h2(vec![text("条件付きレンダリング")]),
        p(vec![
            text("Rust の条件構造を使用して、条件付きでマークアップをレンダリングできます。現在、"),
            code("if"),
            text(" と "),
            code("if let"),
            text(" のみがサポートされています。"),
        ]),
        code_block("rust", r#"use yew::prelude::*;

html! {
  if true {
      <p>{ "True case" }</p>
  }
};"#),
        admonition(AdmonitionType::Info, None, vec![
            p(vec![
                text("条件付きレンダリングの詳細については、"),
                link("/ja/docs/concepts/html/conditional-rendering", vec![text("条件付きレンダリング")]),
                text("のセクションを参照してください。"),
            ]),
        ]),
    ])
}

crate::doc_page!("HTML", "/ja/docs/concepts/html", page_content());
