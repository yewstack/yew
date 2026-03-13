pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition(
            AdmonitionType::Caution,
            None,
            vec![
                p(vec![
                    text("檢查和操作 "),
                    code("Children"),
                    text(
                        " 往往會導致應用程式中令人驚訝且難以解釋的行為。這可能導致邊緣情況，\
                         並且通常不會產生預期的結果。如果您嘗試操作 ",
                    ),
                    code("Children"),
                    text("，則應考慮其他方法。"),
                ]),
                p(vec![
                    text("Yew 支援將 "),
                    code("Html"),
                    text(" 用作子元件屬性的類型。如果您不需要 "),
                    code("Children"),
                    text(" 或 "),
                    code("ChildrenRenderer"),
                    text("，則應使用 "),
                    code("Html"),
                    text(" 作為子元件。它沒有 "),
                    code("Children"),
                    text(" 的缺點，且效能開銷較低。"),
                ]),
            ],
        ),
        h2(vec![text("一般用法")]),
        p(vec![
            italic(vec![text("大多數情況下，")]),
            text(
                "當允許元件具有子元件時，您不關心元件具有的子元件的類型。在這種情況下，\
                 下面的範例就足夠了。",
            ),
        ]),
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
}"#,
        ),
        h2(vec![text("進階用法")]),
        h3(vec![text("類型化子元件")]),
        p(vec![
            text("在您希望將一種類型的元件作為子元件傳遞給您的元件的情況下，您可以使用 "),
            code("yew::html::ChildrenWithProps<T>"),
            text("。"),
        ]),
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
}"#,
        ),
        h2(vec![text("帶有屬性的巢狀子元件")]),
        p(vec![text(
            "如果包含元件對其子元件進行了類型化，則可以存取和變更巢狀元件的屬性。",
        )]),
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
    html! { for modified_children }
}

html! {
    <List>
        <ListItem value="a" />
        <ListItem value="b" />
        <ListItem value="c" />
    </List>
};"#,
        ),
        h3(vec![text("枚舉類型的子元件")]),
        p(vec![text(
            "當然，有時您可能需要將子元件限制為幾種不同的元件。在這些情況下，您必須更深入地了解 \
             Yew。",
        )]),
        p(vec![
            text("這裡使用 "),
            link(
                "https://github.com/JelteF/derive_more",
                vec![code("derive_more")],
            ),
            text(" 來提供更好的人體工學。如果您不想使用它，您可以為每個變體手動實現 "),
            code("From"),
            text("。"),
        ]),
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

// 現在，我們實現 `Into<Html>`，以便 yew 知道如何渲染 `Item`。
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
}"#,
        ),
        h3(vec![text("可選類型的子元件")]),
        p(vec![text("您也可以具有特定類型的單一可選子元件：")]),
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
                // ... 页面内容
            </div>
        }
    }
}

// 页面组件可以选择是否附带侧边栏：

pub fn render_page(with_sidebar: bool) -> Html {
    if with_sidebar {
        // 附带侧边栏的页面
        html! {
            <Page sidebar={html_nested! {
                <PageSideBar />
            }} />
        }
    } else {
        // 不附带侧边栏的页面
        html! {
            <Page />
        }
    }
}"#,
        ),
        h2(vec![text("進一步閱讀")]),
        ul(vec![li(vec![
            text(
                "有關此模式的真實範例，請查閱 yew-router 的原始程式碼。有關更高級的範例，請查看 \
                 yew 儲存庫中的",
            ),
            link(
                "https://github.com/yewstack/yew/tree/master/examples/nested_list",
                vec![text("相關範例清單")],
            ),
        ])]),
    ])
}

crate::doc_page!(
    "子組件",
    "/zh-Hant/docs/advanced-topics/children",
    page_content()
);
