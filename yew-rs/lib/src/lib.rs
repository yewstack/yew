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
