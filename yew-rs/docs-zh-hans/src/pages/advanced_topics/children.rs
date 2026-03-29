pub fn page_content() -> yew_site_lib::Content {
    use yew_site_lib::content::*;
    Content::new(vec![
        admonition![
            AdmonitionType::Caution,
            None,
            p![
                "检查和操作 ",
                code("Children"),
                " 往往会导致应用程序中令人惊讶且难以解释的行为。这可能导致边缘情况，\
                 并且通常不会产生预期的结果。如果您尝试操作 ",
                code("Children"),
                "，则应考虑其他方法。",
            ],
            p![
                "Yew 支持将 ",
                code("Html"),
                " 用作子组件属性的类型。如果您不需要 ",
                code("Children"),
                " 或 ",
                code("ChildrenRenderer"),
                "，则应使用 ",
                code("Html"),
                " 作为子组件。它没有 ",
                code("Children"),
                " 的缺点，并且性能开销较低。",
            ],
        ],
        h2!["通用用法"],
        p![
            italic!["大多数情况下，"],
            "当允许组件具有子组件时，您不关心组件具有的子组件的类型。在这种情况下，\
             下面的示例就足够了。",
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
}"#,
        ),
        h2!["高级用法"],
        h3!["类型化子组件"],
        p![
            "在您希望将一种类型的组件作为子组件传递给您的组件的情况下，您可以使用 ",
            code("yew::html::ChildrenWithProps<T>"),
            "。",
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
}"#,
        ),
        h2!["带有属性的嵌套子组件"],
        p!["如果包含组件对其子组件进行了类型化，则可以访问和更改嵌套组件的属性。"],
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
        h3!["枚举类型的子组件"],
        p![
            "当然，有时您可能需要将子组件限制为几种不同的组件。在这些情况下，您必须更深入地了解 \
             Yew。"
        ],
        p![
            "这里使用 ",
            link!["https://github.com/JelteF/derive_more", "derive_more",],
            " 来提供更好的人机工程学。如果您不想使用它，您可以为每个变体手动实现 ",
            code("From"),
            "。",
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

// 现在，我们实现 `Into<Html>`，以便 yew 知道如何渲染 `Item`。
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
        h3!["可选类型的子组件"],
        p!["您还可以具有特定类型的单个可选子组件："],
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
        h2!["进一步阅读"],
        ul![li![
            "有关此模式的真实示例，请查阅 yew-router 的源代码。有关更高级的示例，请查看 yew \
             存储库中的",
            link![
                "https://github.com/yewstack/yew/tree/master/examples/nested_list",
                "相关示例清单",
            ],
        ]],
    ])
}

crate::doc_page!(
    "子组件",
    "/zh-Hans/docs/advanced-topics/children",
    page_content()
);
