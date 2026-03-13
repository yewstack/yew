pub mod pages;

#[macro_export]
macro_rules! community_page {
    ($title:expr, $content:expr) => {
        #[allow(unused_imports)]
        use yew::prelude::*;
        #[allow(unused_imports)]
        use yew_site_lib::content::*;
        #[allow(unused_imports)]
        use yew_site_lib::Layout;

        #[component]
        pub fn Page() -> Html {
            let content: yew_site_lib::Content = $content;
            let toc = content.toc_entries();
            let markdown = content.to_markdown();
            html! {
                <Layout title={$title} active_nav="Community" markdown={markdown} toc={toc}>
                    { content.to_html() }
                </Layout>
            }
        }
    };
}

#[cfg(feature = "ssr")]
pub async fn render_pages() -> Vec<(&'static str, String, String)> {
    let mut pages = Vec::new();
    pages.push(yew_site_lib::render_page!(
        "/community/awesome",
        pages::awesome::Page
    ));
    pages.push(yew_site_lib::render_page!(
        "/community/external-libs",
        pages::external_libs::Page
    ));
    pages
}
