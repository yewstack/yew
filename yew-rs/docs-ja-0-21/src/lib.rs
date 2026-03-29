pub mod pages;
pub mod sidebar_data;

#[macro_export]
macro_rules! doc_page {
    ($title:expr, $path:expr, $content:expr) => {
        yew_site_lib::doc_page_with_content_fn_impl! { $title, $path, $content, $crate::sidebar_data::docs_sidebar, "0.21", "ja" }
    };
}

yew_site_lib::doc_render_pages! { "/ja/docs/0.21" }
