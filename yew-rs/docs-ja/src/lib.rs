pub mod pages;
pub mod sidebar_data;

#[macro_export]
macro_rules! doc_page {
    ($title:expr, $path:expr, $content:expr) => {
        yew_site_lib::doc_page_impl! { $title, $path, $content, $crate::sidebar_data::docs_sidebar, "Next", "ja" }
    };
}

yew_site_lib::doc_render_pages! { "/ja/docs/next", with_migration_guides: "/ja/docs/migration-guides" }
