pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition!(AdmonitionType::Note, None,
            p![text("プロパティ (Properties) は通常 \"Props\" と略されます。")],
        ),
        p![text("プロパティ (Properties) はコンポーネントのパラメータであり、Yew はこれらのパラメータを監視できます。")],
        p![
            text("コンポーネントのプロパティで型を使用する前に、その型は "),
            code("Properties"),
            text(" トレイトを実装している必要があります。"),
        ],
        h2![text("リアクティブ性")],
        p![text("再レンダリング時に、Yew は仮想DOMを調整する際にプロパティが変更されたかどうかを確認し、ネストされたコンポーネントを再レンダリングする必要があるかどうかを判断します。これにより、Yew は非常にリアクティブなフレームワークと見なされます。親コンポーネントからの変更は常に下位に伝播し、ビューはプロパティ/状態からのデータと常に同期します。")],
        admonition!(AdmonitionType::Tip, None,
            p![
                text("まだ "),
                link!("/ja/docs/tutorial", text("チュートリアル")),
                text(" を完了していない場合は、このリアクティブ性を自分でテストしてみてください！"),
            ],
        ),
        h2![text("派生マクロ")],
        p![
            text("Yew は、構造体に "),
            code("Properties"),
            text(" トレイトを簡単に実装できる派生マクロを提供します。"),
        ],
        p![
            code("Properties"),
            text(" を派生する型は、Yew がデータ比較を行えるように "),
            code("PartialEq"),
            text(" も実装している必要があります。"),
        ],
        code_block("rust", r#"use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}"#),
        h2![text("関数コンポーネントでの使用")],
        p![
            text("属性 "),
            code("#[component]"),
            text(" は、関数の引数で Props を選択的に受け取ることを可能にします。それらを提供するには、"),
            code("html!"),
            text(" マクロ内の属性を通じて割り当てることができます。"),
        ],
        tabs!(
            "with-props",
            tab!(
                "with-props",
                "With Props",
                code_block(
                    "rust",
                    r#"use yew::{component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[component]
fn HelloWorld(&Props { is_loading }: &Props) -> Html {
    html! { <>{"Am I loading? - "}{is_loading}</> }
}

// そしてプロパティを提供します
#[component]
fn App() -> Html {
    html! { <HelloWorld is_loading=true /> }
}"#,
                ),
            ),
            tab!(
                "no-props",
                "No Props",
                code_block(
                    "rust",
                    r#"use yew::{component, html, Html};

#[component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// 提供するプロパティはありません
#[component]
fn App() -> Html {
    html! { <HelloWorld /> }
}"#,
                ),
            ),
        ),
        h2![text("派生マクロフィールド属性")],
        p![
            code("Properties"),
            text(" を派生する際、デフォルトではすべてのフィールドが必須です。"),
        ],
        p![
            text("以下の属性を使用すると、親コンポーネントがそれらを設定しなかった場合にデフォルト値を提供することができます。"),
        ],
        admonition!(AdmonitionType::Tip, None,
            p![text("属性は Rustdoc によって生成されたドキュメントには表示されません。属性のドキュメント文字列には、その属性がオプションであるかどうか、および特定のデフォルト値があるかどうかを記載する必要があります。")],
        ),
        tabs!(
            "prop_or_default",
            tab!(
                "prop_or_default",
                "#[prop_or_default]",
                p![
                    code("Default"),
                    text(" トレイトを使用して、フィールド型のデフォルト値でプロパティ値を初期化します。"),
                ],
                code_block(
                    "rust",
                    r#"use yew::{component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
}

#[component]
fn HelloWorld(&Props { is_loading }: &Props) -> Html {
    if is_loading {
        html! { "Loading" }
    } else {
        html! { "Hello world" }
    }
}

// デフォルト値を使用する
#[component]
fn Case1() -> Html {
    html! { <HelloWorld /> }
}
// またはデフォルト値を上書きしない
#[component]
fn Case2() -> Html {
    html! { <HelloWorld is_loading=true /> }
}"#,
                ),
            ),
            tab!(
                "prop_or_value",
                "#[prop_or(value)]",
                p![
                    code("value"),
                    text(" を使用してプロパティ値を初期化します。"),
                    code("value"),
                    text(" はフィールド型を返す任意の式である可能性があります。"),
                ],
                p![
                    text("例えば、ブールプロパティをデフォルトで "),
                    code("true"),
                    text(" にするには、属性 "),
                    code("#[prop_or(true)]"),
                    text(" を使用します。プロパティが構築されるときに、式が評価され、明示的な値が与えられていない場合に適用されます。"),
                ],
                code_block(
                    "rust",
                    r#"use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
    #[prop_or(AttrValue::Static("Bob"))]
    pub name: AttrValue,
}

#[component]
fn Hello(&Props { is_loading, ref name }: &Props) -> Html {
    if is_loading {
        html! { "Loading" }
    } else {
        html! { <>{"Hello "}{name} </>}
    }
}

// デフォルト値を使用する
#[component]
fn Case1() -> Html {
    html! { <Hello /> }
}
// またはデフォルト値を上書きしない
#[component]
fn Case2() -> Html {
    html! { <Hello name="Sam" /> }
}"#,
                ),
            ),
            tab!(
                "prop_or_else_function",
                "#[prop_or_else(function)]",
                p![
                    text("属性値を初期化するために "),
                    code("function"),
                    text(" を呼び出します。"),
                    code("function"),
                    text(" は "),
                    code("FnMut() -> T"),
                    text(" シグネチャを持つ必要があり、ここで "),
                    code("T"),
                    text(" はフィールドの型です。このプロパティに明示的な値が与えられていない場合、その関数が呼び出されます。この関数はプロパティが構築されるときに呼び出されます。"),
                ],
                code_block(
                    "rust",
                    r#"use yew::prelude::*;

fn create_default_name() -> AttrValue {
    AttrValue::Static("Bob")
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
    #[prop_or_else(create_default_name)]
    pub name: AttrValue,
}

#[component]
fn Hello(&Props { is_loading, ref name }: &Props) -> Html {
    if is_loading {
        html! { "Loading" }
    } else {
        html! { <>{"Hello "}{name}</> }
    }
}

// デフォルト値を使用する
#[component]
fn Case1() -> Html {
    html! { <Hello /> }
}
// またはデフォルト値を上書きしない
#[component]
fn Case2() -> Html {
    html! { <Hello name="Sam" /> }
}"#,
                ),
            ),
        ),
        h2![text("Properties のパフォーマンスオーバーヘッド")],
        p![text("内部プロパティは参照カウントされたスマートポインタとして渡されます。これにより、コンポーネントツリー内のプロパティに対して共有ポインタが1つだけ渡されるため、プロパティ全体をクローンする高コストを節約できます。")],
        admonition!(AdmonitionType::Tip, None,
            p![
                code("AttrValue"),
                text(" はプロパティ値に使用するカスタムタイプであり、これにより String やその他のクローンコストが高いタイプとして定義する必要がなくなります。"),
            ],
        ),
        h2![text("Props マクロ")],
        p![
            code("yew::props!"),
            text(" マクロを使用すると、"),
            code("html!"),
            text(" マクロと同じ方法でプロパティを構築できます。"),
        ],
        p![
            text("このマクロは構造体の式と同じ構文を使用しますが、プロパティや基本式 ("),
            code("Foo { ..base }"),
            text(") を使用することはできません。タイプパスはプロパティ ("),
            code("path::to::Props"),
            text(") に直接指すことも、コンポーネントの関連プロパティ ("),
            code("MyComp::Properties"),
            text(") に指すこともできます。"),
        ],
        code_block("rust", r#"use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
    #[prop_or(AttrValue::Static("Bob"))]
    pub name: AttrValue,
}

#[component]
fn Hello(&Props { is_loading, ref name }: &Props) -> Html {
    if is_loading {
        html! { "Loading" }
    } else {
        html! { <>{"Hello "}{name}</> }
    }
}

#[component]
fn App() -> Html {
    // highlight-start
    let pre_made_props = yew::props! {
        Props {} // 名前属性を指定する必要はありません
    };
    // highlight-end
    html! { <Hello ..pre_made_props /> }
}"#),
        h2![text("自動生成プロパティ (yew-autoprops)")],
        p![
            text("開発プロセスを簡素化するために、"),
            code("#[autoprops]"),
            text(" マクロ（"),
            code("yew-autoprops"),
            text(" パッケージから）を使用して "),
            code("Properties"),
            text(" 構造体を自動生成することもできます。"),
        ],
        code_block("rust", r#"use yew::prelude::*;
use yew_autoprops::autoprops;

// #[autoprops] マクロは #[component] の前に配置する必要があります。順序が重要です。
#[autoprops]
#[component]
fn Greetings(
    #[prop_or_default]
    is_loading: bool,
    #[prop_or(AttrValue::Static("Hello"))]
    message: &AttrValue,
    #[prop_or(AttrValue::Static("World"))]
    name: &AttrValue,
) -> Html {
    if is_loading {
        html! { "Loading" }
    } else {
        html! { <>{message}{" "}{name}</> }
    }
}

// 構造体 "GreetingsProps" は自動的に生成されます。
//
// `is_loading` は値としてコンポーネントに渡され、`message` と `name` は定義に先行する `&` があるため参照として渡されます。"#),
        h2![text("評価順序")],
        p![text("属性は指定された順序で評価されます。以下の例を参照してください：")],
        code_block("rust", r#"#[derive(yew::Properties, PartialEq)]
struct Props { first: usize, second: usize, last: usize }

fn main() {
    let mut g = 1..=3;
    let props = yew::props!(Props { first: g.next().unwrap(), second: g.next().unwrap(), last: g.next().unwrap() });

    assert_eq!(props.first, 1);
    assert_eq!(props.second, 2);
    assert_eq!(props.last, 3);
}"#),
        h2![text("アンチパターン")],
        p![text("ほとんどのRust型はプロパティとして渡すことができますが、避けるべきアンチパターンがいくつかあります。これらには以下が含まれますが、これに限定されません：")],
        ol![
            li_blocks![
                p![
                    code("String"),
                    text(" 型を "),
                    code("AttrValue"),
                    text(" の代わりに使用する。"),
                ],
                p![
                    bold![text("なぜ悪いのか？")],
                    text(" "),
                    code("String"),
                    text(" のクローンは高コストです。プロパティ値がフックやコールバックと一緒に使用される場合、通常クローンが必要です。"),
                    code("AttrValue"),
                    text(" は参照カウントされた文字列 ("),
                    code("Rc<str>"),
                    text(") または "),
                    code("&'static str"),
                    text(" であり、非常に安価にクローンできます。"),
                ],
                p![
                    bold![text("注意")],
                    text("："),
                    code("AttrValue"),
                    text(" は内部的には "),
                    link!("https://crates.io/crates/implicit-clone", text("implicit-clone")),
                    text(" からの "),
                    code("IString"),
                    text(" です。詳細はそのパッケージを参照してください。"),
                ],
            ],
            li_blocks![
                p![text("内部可変性を使用する。")],
                p![
                    bold![text("なぜ悪いのか？")],
                    text(" 内部可変性（例えば "),
                    code("RefCell"),
                    text("、"),
                    code("Mutex"),
                    text(" など）は "),
                    italic![text("通常")],
                    text(" 避けるべきです。これにより再レンダリングの問題が発生する可能性があり（Yewは状態が変更されたことを認識しません）、手動で再レンダリングを強制する必要があるかもしれません。すべてのものと同様に、適切な使用場所があります。慎重に使用してください。"),
                ],
            ],
            li_blocks![
                p![
                    code("Vec<T>"),
                    text(" 型を "),
                    code("IArray<T>"),
                    text(" の代わりに使用する。"),
                ],
                p![
                    bold![text("なぜ悪いのか？")],
                    text(" "),
                    code("Vec<T>"),
                    text(" も "),
                    code("String"),
                    text(" と同様にクローンのコストが高いです。"),
                    code("IArray<T>"),
                    text(" は参照カウントされたスライス ("),
                    code("Rc<[T]>"),
                    text(") または "),
                    code("&'static [T]"),
                    text(" であり、非常に安価にクローンできます。"),
                ],
                p![
                    bold![text("注意")],
                    text("："),
                    code("IArray"),
                    text(" は "),
                    link!("https://crates.io/crates/implicit-clone", text("implicit-clone")),
                    text(" からインポートできます。詳細はそのパッケージを参照してください。"),
                ],
            ],
            li_blocks![
                p![text("新しい発見があるかもしれません。早く知っておきたかったエッジケースに遭遇しましたか？問題を作成するか、このドキュメントに修正のPRを提供してください。")],
            ],
        ],
        h2![text("yew-autoprops")],
        p![
            link!("https://crates.io/crates/yew-autoprops", text("yew-autoprops")),
            text(" は実験的なパッケージで、関数の引数に基づいて動的にProps構造体を作成することを可能にします。プロパティ構造体が再利用されない場合、これは有用かもしれません。"),
        ],
    ])
}

crate::doc_page!(
    "プロパティ (Properties)",
    "/ja/docs/concepts/function-components/properties",
    page_content()
);
