crate::doc_page!(
    "プロパティ (Properties)",
    "/ja/docs/concepts/function-components/properties",
    Content::new(vec![
        admonition!(
            AdmonitionType::Note,
            None,
            p!["プロパティ (Properties) は通常 \"Props\" と略されます。"]
        ),
        p!["プロパティ (Properties) はコンポーネントのパラメータであり、Yew はこれらのパラメータを監視できます。"],
        p![
            "コンポーネントのプロパティで型を使用する前に、その型は ",
            code("Properties"),
            " トレイトを実装している必要があります。",
        ],
        h2!["リアクティブ性"],
        p!["再レンダリング時に、Yew は仮想DOMを調整する際にプロパティが変更されたかどうかを確認し、\
             ネストされたコンポーネントを再レンダリングする必要があるかどうかを判断します。\
             これにより、Yew は非常にリアクティブなフレームワークと見なされます。\
             親コンポーネントからの変更は常に下位に伝播し、ビューはプロパティ/状態からのデータと常に同期します。",],
        admonition!(
            AdmonitionType::Tip,
            None,
            p![
                "まだ ",
                link!("/ja/docs/tutorial", "チュートリアル"),
                " を完了していない場合は、このリアクティブ性を自分でテストしてみてください！",
            ]
        ),
        h2!["派生マクロ"],
        p![
            "Yew は、構造体に ",
            code("Properties"),
            " トレイトを簡単に実装できる派生マクロを提供します。",
        ],
        p![
            code("Properties"),
            " を派生する型は、Yew がデータ比較を行えるように ",
            code("PartialEq"),
            " も実装している必要があります。",
        ],
        code_block(
            "rust",
            r#"use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}"#
        ),
        h2!["関数コンポーネントでの使用"],
        p![
            "属性 ",
            code("#[function_component]"),
            " は、関数の引数で Props を選択的に受け取ることを可能にします。\
                 それらを提供するには、",
            code("html!"),
            " マクロ内の属性を通じて割り当てることができます。",
        ],
        tabs!("with-props",
            tab!("with-props", "With Props",
                code_block(
                    "rust",
                    r#"use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! { <>{"Am I loading? - "}{props.is_loading.clone()}</> }
}

// そしてプロパティを提供します
#[function_component]
fn App() -> Html {
    html! {<HelloWorld is_loading={true} />}
}
"#
                ),
            ),
            tab!("no-props", "No Props",
                code_block(
                    "rust",
                    r#"use yew::{function_component, html, Html};





#[function_component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}

// 提供するプロパティはありません
#[function_component]
fn App() -> Html {
    html! {<HelloWorld />}
}
"#
                ),
            ),
        ),
        h2!["派生マクロフィールド属性"],
        p![
            code("Properties"),
            " を派生する際、デフォルトではすべてのフィールドが必須です。\
                 以下の属性を使用すると、親コンポーネントがそれらを設定しなかった場合に\
                 デフォルト値を提供することができます。",
        ],
        admonition!(
            AdmonitionType::Tip,
            None,
            p!["属性は Rustdoc によって生成されたドキュメントには表示されません。\
                 属性のドキュメント文字列には、その属性がオプションであるかどうか、\
                 および特定のデフォルト値があるかどうかを記載する必要があります。",]
        ),
        tabs!("prop_or_default",
            tab!("prop_or_default", "#[prop_or_default]",
                p![
                    code("Default"),
                    " トレイトを使用して、フィールド型のデフォルト値でプロパティ値を初期化します。",
                ],
                code_block(
                    "rust",
                    r#"use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    if props.is_loading.clone() {
        html! { "Loading" }
    } else {
        html! { "Hello world" }
    }
}

// デフォルト値を使用する
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// またはデフォルト値を上書きしない
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld is_loading={true} />}
}"#
                ),
            ),
            tab!("prop_or_value", "#[prop_or(value)]",
                p![
                    code("value"),
                    " を使用してプロパティ値を初期化します。",
                    code("value"),
                    " はフィールド型を返す任意の式である可能性があります。\
                         例えば、ブールプロパティをデフォルトで ",
                    code("true"),
                    " にするには、属性 ",
                    code("#[prop_or(true)]"),
                    " を使用します。プロパティが構築されるときに、式が評価され、\
                         明示的な値が与えられていない場合に適用されます。",
                ],
                code_block(
                    "rust",
                    r##"use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("Bob".to_string())]
    pub name: String,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

// デフォルト値を使用する
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// またはデフォルト値を上書きしない
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld name={"Sam".to_string()} />}
}"##
                ),
            ),
            tab!("prop_or_else_function", "#[prop_or_else(function)]",
                p![
                    "属性値を初期化するために ",
                    code("function"),
                    " を呼び出します。",
                    code("function"),
                    " は ",
                    code("FnMut() -> T"),
                    " シグネチャを持つ必要があり、ここで ",
                    code("T"),
                    " はフィールドの型です。このプロパティに明示的な値が与えられていない場合、\
                         その関数が呼び出されます。",
                ],
                code_block(
                    "rust",
                    r##"use yew::{function_component, html, Html, Properties};

fn create_default_name() -> String {
    "Bob".to_string()
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(create_default_name)]
    pub name: String,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

// デフォルト値を使用する
#[function_component]
fn Case1() -> Html {
    html! {<HelloWorld />}
}
// またはデフォルト値を上書きしない
#[function_component]
fn Case2() -> Html {
    html! {<HelloWorld name={"Sam".to_string()} />}
}"##
                ),
            ),
        ),
        h2!["Properties のパフォーマンスオーバーヘッド"],
        p!["内部プロパティは参照カウントされたスマートポインタとして渡されます。\
             これにより、コンポーネントツリー内のプロパティに対して共有ポインタが1つだけ渡されるため、\
             プロパティ全体をクローンする高コストを節約できます。",],
        admonition!(
            AdmonitionType::Tip,
            None,
            p![
                code("AttrValue"),
                " はプロパティ値に使用するカスタムタイプであり、これにより String や\
                     その他のクローンコストが高いタイプとして定義する必要がなくなります。",
            ]
        ),
        h2!["Props マクロ"],
        p![
            code("yew::props!"),
            " マクロを使用すると、",
            code("html!"),
            " マクロと同じ方法でプロパティを構築できます。",
        ],
        p![
            "このマクロは構造体の式と同じ構文を使用しますが、プロパティや基本式 (",
            code("Foo { ..base }"),
            ") を使用することはできません。タイプパスはプロパティ (",
            code("path::to::Props"),
            ") に直接指すことも、コンポーネントの関連プロパティ (",
            code("MyComp::Properties"),
            ") に指すこともできます。",
        ],
        code_block(
            "rust",
            r##"use yew::{function_component, html, Html, Properties, props, virtual_dom::AttrValue};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::from("Bob"))]
    pub name: AttrValue,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    html! {<>{"Hello world"}{props.name.clone()}</>}
}

#[function_component]
fn App() -> Html {
    let pre_made_props = props! {
        Props {} // 名前属性を指定する必要はありません
    };
    html! {<HelloWorld ..pre_made_props />}
}"##
        ),
        h2!["評価順序"],
        p!["属性は指定された順序で評価されます。以下の例を参照してください："],
        code_block(
            "rust",
            r#"#[derive(yew::Properties, PartialEq)]
struct Props { first: usize, second: usize, last: usize }

fn main() {
    let mut g = 1..=3;
    let props = yew::props!(Props { first: g.next().unwrap(), second: g.next().unwrap(), last: g.next().unwrap() });

    assert_eq!(props.first, 1);
    assert_eq!(props.second, 2);
    assert_eq!(props.last, 3);
}"#
        ),
        h2!["アンチパターン"],
        p!["ほとんどのRust型はプロパティとして渡すことができますが、避けるべきアンチパターンが\
             いくつかあります。これらには以下が含まれますが、これに限定されません：",],
        ol![
            li_blocks!(
                p![
                    code("String"),
                    " 型を ",
                    code("AttrValue"),
                    " の代わりに使用する。",
                ],
                p![
                    bold!["なぜ悪いのか？"],
                    " ",
                    code("String"),
                    " のクローンは高コストです。プロパティ値がフックやコールバックと一緒に使用される場合、\
                         通常クローンが必要です。",
                    code("AttrValue"),
                    " は参照カウントされた文字列 (",
                    code("Rc<str>"),
                    ") または ",
                    code("&'static str"),
                    " であり、非常に安価にクローンできます。",
                ],
                p![
                    bold!["注意"],
                    "：",
                    code("AttrValue"),
                    " は内部的には ",
                    link!(
                        "https://crates.io/crates/implicit-clone",
                        "implicit-clone"
                    ),
                    " からの ",
                    code("IString"),
                    " です。詳細はそのパッケージを参照してください。",
                ],
            ),
            li_blocks!(
                p!["内部可変性を使用する。"],
                p![
                    bold!["なぜ悪いのか？"],
                    " 内部可変性（例えば ",
                    code("RefCell"),
                    "、",
                    code("Mutex"),
                    " など）は通常避けるべきです。これにより再レンダリングの問題が発生する可能性があり\
                         （Yewは状態が変更されたことを認識しません）、手動で再レンダリングを強制する必要が\
                         あるかもしれません。すべてのものと同様に、適切な使用場所があります。慎重に使用してください。",
                ],
            ),
            li_blocks!(
                p![
                    code("Vec<T>"),
                    " 型を ",
                    code("IArray<T>"),
                    " の代わりに使用する。",
                ],
                p![
                    bold!["なぜ悪いのか？"],
                    " ",
                    code("Vec<T>"),
                    " も ",
                    code("String"),
                    " と同様にクローンのコストが高いです。",
                    code("IArray<T>"),
                    " は参照カウントされたスライス (",
                    code("Rc<[T]>"),
                    ") または ",
                    code("&'static [T]"),
                    " であり、非常に安価にクローンできます。",
                ],
                p![
                    bold!["注意"],
                    "：",
                    code("IArray<T>"),
                    " は ",
                    link!(
                        "https://crates.io/crates/implicit-clone",
                        "implicit-clone"
                    ),
                    " からインポートできます。詳細はそのパッケージを参照してください。",
                ],
            ),
            li_blocks!(p!["新しい発見があるかもしれません。早く知っておきたかったエッジケースに遭遇しましたか？\
                 問題を作成するか、このドキュメントに修正のPRを提供してください。",],),
        ],
        h2!["yew-autoprops"],
        p![
            link!(
                "https://crates.io/crates/yew-autoprops",
                "yew-autoprops"
            ),
            " は実験的なパッケージで、関数の引数に基づいて動的にProps構造体を作成することを可能にします。\
                 プロパティ構造体が再利用されない場合、これは有用かもしれません。",
        ],
    ])
);
