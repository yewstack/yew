pub mod pages;
pub mod sidebar_data;

#[macro_export]
macro_rules! doc_page {
    ($title:expr, $path:expr, $content:expr) => {
        yew_site_lib::doc_page_impl! { $title, $path, $content, $crate::sidebar_data::docs_sidebar, "Next", "zh-Hans" }
    };
}

yew_site_lib::doc_render_pages! { "/zh-Hans/docs/next", with_migration_guides: "/zh-Hans/docs/migration-guides" }
