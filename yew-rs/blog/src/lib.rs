pub mod pages;

use yew_site_lib::components::sidebar::{SidebarCategory, SidebarEntry, SidebarItem};

pub fn blog_sidebar() -> Vec<SidebarEntry> {
    let mut years: Vec<(&str, Vec<&BlogMeta>)> = Vec::new();
    for post in BLOG_POSTS {
        let year = &post.date[..4];
        if let Some((_, posts)) = years.iter_mut().find(|(y, _)| *y == year) {
            posts.push(post);
        } else {
            years.push((year, vec![post]));
        }
    }
    years
        .into_iter()
        .map(|(year, posts)| {
            SidebarEntry::Category(SidebarCategory {
                label: match year {
                    "2022" => "2022",
                    "2023" => "2023",
                    "2024" => "2024",
                    "2025" => "2025",
                    "2026" => "2026",
                    _ => "Other",
                },
                link: None,
                items: posts
                    .into_iter()
                    .map(|p| {
                        SidebarEntry::Item(SidebarItem {
                            label: p.title,
                            href: p.url_path_static(),
                        })
                    })
                    .collect(),
            })
        })
        .collect()
}

#[derive(Clone, PartialEq)]
pub struct BlogMeta {
    pub title: &'static str,
    pub date: &'static str,
    pub slug: &'static str,
    pub author_name: &'static str,
    pub author_title: &'static str,
    pub author_url: &'static str,
    pub author_image_url: &'static str,
    pub description: &'static str,
}

impl BlogMeta {
    pub fn url_path(&self) -> String {
        let date_parts: Vec<&str> = self.date.split('-').collect();
        format!(
            "/blog/{}/{}/{}/{}",
            date_parts[0], date_parts[1], date_parts[2], self.slug
        )
    }

    pub fn url_path_static(&self) -> &'static str {
        Box::leak(self.url_path().into_boxed_str())
    }
}

pub const BLOG_POSTS: &[BlogMeta] = &[
    BlogMeta {
        title: "Yew 0.22 - For Real This Time",
        date: "2025-11-29",
        slug: "release-0-22",
        author_name: "Mattuwu",
        author_title: "Maintainer of Yew",
        author_url: "https://github.com/Madoshakalaka",
        author_image_url: "https://github.com/Madoshakalaka.png",
        description: "The Yew team is thrilled to announce the release of Yew 0.22! After a \
                      longer-than-expected journey, this release brings significant improvements \
                      to ergonomics, performance, and developer experience.",
    },
    BlogMeta {
        title: "Announcing Yew 0.22",
        date: "2024-10-14",
        slug: "release-0-22",
        author_name: "langyo",
        author_title: "Contributor of Yew",
        author_url: "https://github.com/langyo",
        author_image_url: "https://github.com/langyo.png",
        description: "Yew 0.22 brings SSR on WASI targets.",
    },
    BlogMeta {
        title: "Announcing Yew 0.21",
        date: "2023-09-23",
        slug: "release-0-21",
        author_name: "Muhammad Hamza",
        author_title: "Maintainer of Yew",
        author_url: "https://github.com/ranile",
        author_image_url: "https://github.com/ranile.png",
        description: "The Yew development team is thrilled to unveil Yew 0.21.0, a significant \
                      milestone in the journey of empowering developers to create dependable and \
                      high-performance web applications with Rust.",
    },
    BlogMeta {
        title: "Releasing Yew 0.20",
        date: "2022-11-24",
        slug: "release-0-20",
        author_name: "Muhammad Hamza",
        author_title: "Maintainer of Yew",
        author_url: "https://github.com/ranile",
        author_image_url: "https://github.com/ranile.png",
        description: "The Yew team is happy to announce a new, long overdue, version of Yew: \
                      v0.20.",
    },
    BlogMeta {
        title: "Hello Yew",
        date: "2022-01-20",
        slug: "hello-yew",
        author_name: "Muhammad Hamza",
        author_title: "Maintainer of Yew",
        author_url: "https://github.com/ranile",
        author_image_url: "https://github.com/ranile.png",
        description: "This is the first Yew blog post.",
    },
];

#[macro_export]
macro_rules! blog_page {
    ($meta:expr, $content:expr) => {
        #[allow(unused_imports)]
        use yew::prelude::*;
        #[allow(unused_imports)]
        use yew_site_lib::content::*;
        #[allow(unused_imports)]
        use yew_site_lib::Layout;

        pub const META: &$crate::BlogMeta = $meta;

        #[::stylist::yew::styled_component]
        pub fn Page() -> Html {
            let meta = META;
            let content: yew_site_lib::Content = $content;
            let toc = content.toc_entries();
            let markdown = content.to_markdown();
            let date_display = {
                let parts: Vec<&str> = meta.date.split('-').collect();
                let month = match parts[1] {
                    "01" => "January",
                    "02" => "February",
                    "03" => "March",
                    "04" => "April",
                    "05" => "May",
                    "06" => "June",
                    "07" => "July",
                    "08" => "August",
                    "09" => "September",
                    "10" => "October",
                    "11" => "November",
                    "12" => "December",
                    _ => "",
                };
                let day: u32 = parts[2].parse().unwrap_or(0);
                format!("{} {}, {}", month, day, parts[0])
            };
            let sidebar = $crate::blog_sidebar();
            let active_path = meta.url_path();
            html! {
                <Layout
                    title={meta.title}
                    active_nav="Blog"
                    markdown={markdown}
                    toc={toc}
                    sidebar={sidebar}
                    active_sidebar_path={active_path}
                    sidebar_title="Recent posts"
                    sidebar_all_open={true}
                >
                    <div class={css!(margin-bottom: 1.5rem;)}>
                        <time class={css!(
                            display: block;
                            font-size: 0.875rem;
                            color: var(--color-text-secondary);
                            margin-bottom: 0.75rem;
                        )}>{date_display}</time>
                        <div class={css!(display: flex; align-items: center; gap: 0.625rem;)}>
                            <img
                                class={css!(width: 48px; height: 48px; border-radius: 50%;)}
                                src={meta.author_image_url}
                                alt={meta.author_name}
                                width="48"
                                height="48"
                                loading="lazy"
                            />
                            <div class={css!(display: flex; flex-direction: column;)}>
                                <a class={css!(
                                    font-weight: 600;
                                    color: var(--color-text);
                                    text-decoration: none;
                                    &:hover { color: var(--color-primary); }
                                )} href={meta.author_url}>
                                    {meta.author_name}
                                </a>
                                <span class={css!(
                                    font-size: 0.8125rem;
                                    color: var(--color-text-secondary);
                                )}>{meta.author_title}</span>
                            </div>
                        </div>
                    </div>
                    { content.to_html() }
                </Layout>
            }
        }
    };
}

#[cfg(feature = "ssr")]
pub async fn render_pages() -> Vec<(&'static str, String, String)> {
    let mut pages = Vec::new();
    pages.push(yew_site_lib::render_page!("/blog", pages::index::Page));
    pages.push(yew_site_lib::render_page!(
        "/blog/2022/01/20/hello-yew",
        pages::hello_yew::Page
    ));
    pages.push(yew_site_lib::render_page!(
        "/blog/2022/11/24/release-0-20",
        pages::release_0_20::Page
    ));
    pages.push(yew_site_lib::render_page!(
        "/blog/2023/09/23/release-0-21",
        pages::release_0_21::Page
    ));
    pages.push(yew_site_lib::render_page!(
        "/blog/2024/10/14/release-0-22",
        pages::release_0_22::Page
    ));
    pages.push(yew_site_lib::render_page!(
        "/blog/2025/11/29/release-0-22",
        pages::release_0_22_1::Page
    ));
    pages
}
