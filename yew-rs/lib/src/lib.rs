pub mod components;
pub mod content;
pub mod styles;

pub use components::footer::Footer;
pub use components::layout::Layout;
pub use components::navbar::Navbar;
pub use components::sidebar::{
    flatten_sidebar, migration_guides_sidebar, Sidebar, SidebarCategory, SidebarEntry, SidebarItem,
};
pub use content::Content;
use yew::prelude::*;

pub const LATEST_STABLE: &str = "0.23";

#[derive(Clone, PartialEq)]
pub struct NavigationContext {
    pub navigate: Callback<(MouseEvent, AttrValue)>,
}

pub struct PageData {
    pub title: &'static str,
    pub sidebar_path: &'static str,
    pub doc_version: &'static str,
    pub sidebar: Vec<SidebarEntry>,
    pub content: Content,
}

pub const VERSIONS: &[(&str, &str)] = &[
    ("Next", "next"),
    ("0.23", ""),
    ("0.22", "0.22"),
    ("0.21", "0.21"),
    ("0.20", "0.20"),
];

#[cfg(feature = "ssr")]
pub mod ssr_reexports {
    pub use stylist::manager::{render_static, StyleManager};
    pub use stylist::yew::ManagerProvider;
    pub use yew::ServerRenderer;
}

#[macro_export]
macro_rules! page_main {
    ($page:ty) => {
        fn main() {
            yew::Renderer::<$page>::new().render();
        }
    };
}

#[cfg(feature = "ssr")]
#[macro_export]
macro_rules! render_spa_page {
    ($url:expr, $title:expr, $sidebar:expr, $sidebar_path:expr, $doc_version:expr, $lang:expr, $content_expr:expr) => {{
        use ::yew::prelude::*;
        use $crate::ssr_reexports::{render_static, ManagerProvider, ServerRenderer, StyleManager};

        #[derive(Properties, PartialEq)]
        struct __SpaProps {
            manager: StyleManager,
            title: &'static str,
            sidebar: Vec<$crate::SidebarEntry>,
            sidebar_path: &'static str,
            doc_version: &'static str,
            lang: &'static str,
        }

        fn __make_content() -> $crate::Content {
            $content_expr
        }

        #[component]
        fn __SpaWrapper(props: &__SpaProps) -> Html {
            let content = __make_content();
            let toc = content.toc_entries();
            let markdown = content.to_markdown();
            html! {
                <ManagerProvider manager={props.manager.clone()}>
                    <$crate::Layout
                        title={props.title}
                        sidebar={props.sidebar.clone()}
                        active_sidebar_path={props.sidebar_path}
                        active_nav="Docs"
                        doc_version={props.doc_version}
                        lang={props.lang}
                        markdown={markdown}
                        toc={toc}
                    >
                        { content.to_html() }
                    </$crate::Layout>
                </ManagerProvider>
            }
        }

        let (writer, reader) = render_static();
        let body = ServerRenderer::<__SpaWrapper>::with_props(move || {
            let mgr = StyleManager::builder().writer(writer).build().unwrap();
            __SpaProps {
                manager: mgr,
                title: $title,
                sidebar: $sidebar,
                sidebar_path: $sidebar_path,
                doc_version: $doc_version,
                lang: $lang,
            }
        })
        .hydratable(false)
        .render()
        .await;

        let style_data = reader.read_style_data();
        let mut styles = String::new();
        style_data.write_static_markup(&mut styles).unwrap();

        ($url, body, styles)
    }};
}

#[cfg(feature = "ssr")]
#[macro_export]
macro_rules! render_page {
    ($url:expr, $page:ty) => {{
        use ::yew::prelude::*;
        use $crate::ssr_reexports::{render_static, ManagerProvider, ServerRenderer, StyleManager};

        #[derive(Properties, PartialEq)]
        struct __SsrWrapperProps {
            manager: StyleManager,
        }

        #[component]
        fn __SsrWrapper(props: &__SsrWrapperProps) -> Html {
            html! {
                <ManagerProvider manager={props.manager.clone()}>
                    <$page />
                </ManagerProvider>
            }
        }

        let (writer, reader) = render_static();
        let body = ServerRenderer::<__SsrWrapper>::with_props(move || {
            let mgr = StyleManager::builder().writer(writer).build().unwrap();
            __SsrWrapperProps { manager: mgr }
        })
        .hydratable(false)
        .render()
        .await;

        let style_data = reader.read_style_data();
        let mut styles = String::new();
        style_data.write_static_markup(&mut styles).unwrap();

        ($url, body, styles)
    }};
}
