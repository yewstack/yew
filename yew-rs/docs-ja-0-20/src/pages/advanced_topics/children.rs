crate::doc_page!(
    "Children",
    "/ja/docs/advanced-topics/children",
    Content::new(vec![
        h2(vec![text("General usage")]),
        p(vec![
            italic(vec![text("Most of the time,")]),
            text(
                " when allowing a component to have children, you don't care what type of \
                 children the component has. In such cases, the below example will suffice."
            ),
        ]),
        code_block(
            "rust",
            r#"use yew::{html, Children, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ListProps {
    #[prop_or_default]
    pub children: Children,
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
        h2(vec![text("Advanced usage")]),
        h3(vec![text("Typed children")]),
        p(vec![
            text(
                "In cases where you want one type of component to be passed as children to your \
                 component, you can use "
            ),
            code("yew::html::ChildrenWithProps<T>"),
            text("."),
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
}"#
        ),
        h3(vec![text("Enum typed children")]),
        p(vec![text(
            "Of course, sometimes you might need to restrict the children to a few different \
             components. In these cases, you have to get a little more hands-on with Yew."
        ),]),
        p(vec![
            text("The "),
            link(
                "https://github.com/JelteF/derive_more",
                vec![text("derive_more")]
            ),
            text(
                " crate is used here for better ergonomics. If you don't want to use it, you can \
                 manually implement "
            ),
            code("From"),
            text(" for each variant."),
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

// Now, we implement `Into<Html>` so that yew knows how to render `Item`.
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
        h3(vec![text("Optional typed child")]),
        p(vec![text(
            "You can also have a single optional child component of a specific type too:"
        ),]),
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
                // ... page content
            </div>
        }
    }
}

// The page component can be called either with the sidebar or without:

pub fn render_page(with_sidebar: bool) -> Html {
    if with_sidebar {
        // Page with sidebar
        html! {
            <Page sidebar={html_nested! {
                <PageSideBar />
            }} />
        }
    } else {
        // Page without sidebar
        html! {
            <Page />
        }
    }
}"#
        ),
    ])
);
