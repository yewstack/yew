pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        p![
            "プロパティ (Properties) は、子コンポーネントと親コンポーネントの間で通信を可能にします。各コンポーネントには、親コンポーネントから渡される内容を記述するための関連プロパティ型があります。理論的には、これは ",
            code("Properties"),
            " トレイトを実装した任意の型である可能性がありますが、実際には、各フィールドがプロパティを表す構造体であるべきです。",
        ],
        h2!["派生マクロ"],
        p![
            code("Properties"),
            " トレイトを自分で実装する必要はありません。",
            code("#[derive(Properties)]"),
            " を使用して実装を自動生成できます。",
            code("Properties"),
            " を派生する型は ",
            code("PartialEq"),
            " も実装する必要があります。",
        ],
        h3!["フィールド属性"],
        p![
            code("Properties"),
            " を派生する際、デフォルトではすべてのフィールドが必須です。以下の属性を使用すると、他の値が設定されていない限り、プロパティに初期値を提供できます。",
        ],
        admonition!(AdmonitionType::Tip, None,
            p!["プロパティは Rustdoc によって生成されたドキュメントには表示されません。プロパティのドキュメント文字列には、そのプロパティがオプションであるかどうか、または特別なデフォルト値があるかどうかを記載する必要があります。"],
        ),
        h4!["#[prop_or_default]"],
        p![
            "フィールド型のデフォルト値を使用してプロパティ値を初期化します。これは ",
            code("Default"),
            " トレイトを使用します。",
        ],
        h4!["#[prop_or(value)]"],
        p![
            code("value"),
            " を使用してプロパティ値を初期化します。",
            code("value"),
            " はフィールド型を返す任意の式である可能性があります。例えば、ブールプロパティをデフォルトで ",
            code("true"),
            " にするには、属性 ",
            code("#[prop_or(true)]"),
            " を使用します。",
        ],
        h4!["#[prop_or_else(function)]"],
        p![
            code("function"),
            " を呼び出してプロパティ値を初期化します。",
            code("function"),
            " は ",
            code("FnMut() -> T"),
            " のシグネチャを持つ必要があります。ここで、",
            code("T"),
            " はフィールド型です。",
        ],
        h2!["PartialEq"],
        p![
            code("Properties"),
            " は ",
            code("PartialEq"),
            " を実装する必要があります。これにより、Yew はそれらを比較し、変更があった場合に ",
            code("changed"),
            " メソッドを呼び出すことができます。",
        ],
        h2!["Properties のパフォーマンスオーバーヘッド"],
        p!["内部プロパティは参照カウントされたポインタに基づいて格納されます。これにより、コンポーネントツリーに渡されるプロパティにはポインタのみが渡され、プロパティ全体をクローンすることによる高価なパフォーマンスオーバーヘッドを回避できます。"],
        admonition!(AdmonitionType::Tip, None,
            p![
                code("AttrValue"),
                " を使用してください。これは、クローンが必要な String やその他の類似の型を使用せずに済むようにするために提供されているカスタムプロパティ値型です。",
            ],
        ),
        h2!["例"],
        code_block("rust", r##"use yew::Properties;
/// virtual_dom から AttrValue をインポート
use yew::virtual_dom::AttrValue;

#[derive(Clone, PartialEq)]
pub enum LinkColor {
    Blue,
    Red,
    Green,
    Black,
    Purple,
}

fn create_default_link_color() -> LinkColor {
    LinkColor::Blue
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    /// リンクにはターゲットが必要です
    href: AttrValue,
    /// また、String ではなく AttrValue を使用していることに注意してください
    text: AttrValue,
    /// リンクの色、デフォルトは `Blue`
    #[prop_or_else(create_default_link_color)]
    color: LinkColor,
    /// 値が None の場合、ビュー関数はサイズを指定しません
    #[prop_or_default]
    size: Option<u32>,
    /// ビュー関数がアクティブを指定しない場合、デフォルトは true
    #[prop_or(true)]
    active: bool,
}"##),
        h2!["Props マクロ"],
        p![
            code("yew::props!"),
            " マクロを使用すると、",
            code("html!"),
            " マクロと同じ方法でプロパティを構築できます。",
        ],
        p![
            "このマクロは構造体の式と同じ構文を使用しますが、属性や基本式（",
            code("Foo { ..base }"),
            "）を使用することはできません。型パスはプロパティ（",
            code("path::to::Props"),
            "）に直接指すことも、コンポーネントの関連プロパティ（",
            code("MyComp::Properties"),
            "）に指すこともできます。",
        ],
        code_block("rust", r##"use yew::{props, Properties, virtual_dom::AttrValue};

#[derive(Clone, PartialEq)]
pub enum LinkColor {
    Blue,
    Red,
    Green,
    Black,
    Purple,
}

fn create_default_link_color() -> LinkColor {
    LinkColor::Blue
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    /// リンクにはターゲットが必要です
    href: AttrValue,
    /// また、String ではなく AttrValue を使用していることに注意してください
    text: AttrValue,
    /// リンクの色、デフォルトは `Blue`
    #[prop_or_else(create_default_link_color)]
    color: LinkColor,
    /// 値が None の場合、ビュー関数はサイズを指定しません
    #[prop_or_default]
    size: Option<u32>,
    /// ビュー関数がアクティブを指定しない場合、デフォルトは true
    #[prop_or(true)]
    active: bool,
}

impl LinkProps {
    /// この関数は href と text を String として受け取ります
    /// `AttrValue::from` を使用してそれらを `AttrValue` に変換できます
    pub fn new_link_with_size(href: String, text: String, size: u32) -> Self {
        // highlight-start
        props! {LinkProps {
            href: AttrValue::from(href),
            text: AttrValue::from(text),
            size,
        }}
        // highlight-end
    }
}"##),
    ])
    .with_description("Parent to child communication")
}

crate::doc_page!(
    "プロパティ (Props)",
    "/ja/docs/advanced-topics/struct-components/properties",
    page_content()
);
