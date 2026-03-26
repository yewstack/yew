pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                code("Children"),
                " をチェックおよび操作すると、アプリケーションで驚くべきかつ説明が難しい動作が発生することがよくあります。これにより、エッジケースが発生し、通常は予期しない結果が生じる可能性があります。",
            ],
            p![
                code("Children"),
                " を操作しようとする場合は、他の方法を検討する必要があります。",
            ],
            p![
                "Yew は、子コンポーネントのプロパティの型として ",
                code("Html"),
                " を使用することをサポートしています。",
                code("Children"),
                " または ",
                code("ChildrenRenderer"),
                " が必要ない場合は、子コンポーネントとして ",
                code("Html"),
                " を使用することをお勧めします。これは ",
                code("Children"),
                " の欠点がなく、パフォーマンスのオーバーヘッドも低くなります。",
            ],
        ],
        h2!["一般的な使用法"],
        p![
            italic!["ほとんどの場合、"],
            " コンポーネントに子コンポーネントを持たせる場合、子コンポーネントの型を気にする必要はありません。この場合、以下の例で十分です。",
        ],
        code_block(
            "rust",
            r#"use yew::{html, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ListProps {
    #[prop_or_default]
    pub children: Html,
}

pub struct List;

impl Component for List {
    type Message = ();
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="list">
                {ctx.props().children.clone()}
            </div>
        }
    }
}"#
        ),
        h2!["高度な使用法"],
        h3!["型指定された子コンポーネント"],
        p![
            "特定のタイプのコンポーネントを子コンポーネントとして渡したい場合は、",
            code("yew::html::ChildrenWithProps<T>"),
            " を使用できます。",
        ],
        code_block(
            "rust",
            r#"use yew::{html, ChildrenWithProps, Component, Context, Html, Properties};

pub struct Item;

impl Component for Item {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            { "item" }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ListProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Item>,
}

pub struct List;

impl Component for List {
    type Message = ();
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="list">
                { for ctx.props().children.iter() }
            </div>
        }
    }
}"#
        ),
        h2!["プロパティを持つネストされた子コンポーネント"],
        p!["コンポーネントがその子コンポーネントを型指定している場合、ネストされたコンポーネントのプロパティにアクセスして変更することができます。",],
        code_block(
            "rust",
            r#"use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ListItemProps {
    value: String,
}

#[component]
fn ListItem(props: &ListItemProps) -> Html {
    let ListItemProps { value } = props.clone();
    html! {
        <span>
            {value}
        </span>
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenWithProps<ListItem>,
}

#[component]
fn List(props: &Props) -> Html {
    let modified_children = props.children.iter().map(|mut item| {
            let mut props = Rc::make_mut(&mut item.props);
            props.value = format!("item-{}", props.value);
            item
    });
    html! {{for modified_children}}
}

html! {
    <List>
        <ListItem value="a" />
        <ListItem value="b" />
        <ListItem value="c" />
    </List>
};"#
        ),
        h3!["列挙型の子コンポーネント"],
        p!["もちろん、時には子コンポーネントをいくつかの異なるコンポーネントに制限する必要がある場合があります。そのような場合には、Yewについてさらに深く理解する必要があります。",],
        p![
            "ここでは、より良いエルゴノミクスを提供するために ",
            link![
                "https://github.com/JelteF/derive_more",
                "derive_more"
            ],
            " を使用しています。使用したくない場合は、各バリアントに対して手動で ",
            code("From"),
            " を実装することができます。",
        ],
        code_block(
            "rust",
            r#"use yew::{
    html, html::ChildrenRenderer, virtual_dom::VChild, Component,
    Context, Html, Properties,
};

pub struct Primary;

impl Component for Primary {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            { "Primary" }
        }
    }
}

pub struct Secondary;

impl Component for Secondary {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            { "Secondary" }
        }
    }
}

#[derive(Clone, derive_more::From, PartialEq)]
pub enum Item {
    Primary(VChild<Primary>),
    Secondary(VChild<Secondary>),
}

// 現在、`Into<Html>` を実装して、yew が `Item` をどのようにレンダリングするかを知ることができるようにします。
#[allow(clippy::from_over_into)]
impl Into<Html> for Item {
    fn into(self) -> Html {
        match self {
            Self::Primary(child) => child.into(),
            Self::Secondary(child) => child.into(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ListProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<Item>,
}

pub struct List;

impl Component for List {
    type Message = ();
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="list">
                { for ctx.props().children.iter() }
            </div>
        }
    }
}"#
        ),
        h3!["オプションの型の子コンポーネント"],
        p!["特定の型の単一のオプションの子コンポーネントを持つこともできます：",],
        code_block(
            "rust",
            r#"use yew::{
    html, html_nested, virtual_dom::VChild, Component,
    Context, Html, Properties
};

pub struct PageSideBar;

impl Component for PageSideBar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            { "sidebar" }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct PageProps {
    #[prop_or_default]
    pub sidebar: Option<VChild<PageSideBar>>,
}

struct Page;

impl Component for Page {
    type Message = ();
    type Properties = PageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="page">
                { ctx.props().sidebar.clone().map(Html::from).unwrap_or_default() }
                // ... ページ内容
            </div>
        }
    }
}

// ページコンポーネントはサイドバーを含むかどうかを選択できます：

pub fn render_page(with_sidebar: bool) -> Html {
    if with_sidebar {
        // サイドバーを含むページ
        html! {
            <Page sidebar={html_nested! {
                <PageSideBar />
            }} />
        }
    } else {
        // サイドバーを含まないページ
        html! {
            <Page />
        }
    }
}"#
        ),
        h2!["さらに読む"],
        ul![li![
            "このパターンの実際の例については、yew-router のソースコードを参照してください。より高度な例については、yew リポジトリの ",
            link![
                "https://github.com/yewstack/yew/tree/master/examples/nested_list",
                "関連する例のリスト"
            ],
            " を参照してください。",
        ],],
    ])
}

crate::doc_page!(
    "子コンポーネント",
    "/ja/docs/advanced-topics/children",
    page_content()
);
