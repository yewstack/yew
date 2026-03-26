pub mod pages;
pub mod sidebar_data;

#[macro_export]
macro_rules! doc_page {
    ($title:expr, $path:expr, $content:expr) => {
        yew_site_lib::doc_page_impl! { $title, $path, $content, $crate::sidebar_data::docs_sidebar, "0.22", "" }
    };
}

yew_site_lib::doc_render_pages! { "/docs/0.22" }
