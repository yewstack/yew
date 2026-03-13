use stylist::css;
use yew::prelude::*;
use yew_site_lib::Layout;

use crate::BLOG_POSTS;

#[component]
pub fn Page() -> Html {
    let style = css!(
        r#"
        .blog-list {
            list-style: none;
            padding: 0;
            margin: 0;
        }

        .blog-list-item {
            margin-bottom: 2rem;
            padding-bottom: 2rem;
            border-bottom: 1px solid var(--color-border);
        }

        .blog-list-item:last-child {
            border-bottom: none;
        }

        .blog-list-title {
            font-size: 1.5rem;
            font-weight: 700;
            margin: 0 0 0.5rem 0;
        }

        .blog-list-title a {
            color: var(--color-text);
            text-decoration: none;
        }

        .blog-list-title a:hover {
            color: var(--color-primary);
        }

        .blog-list-meta {
            display: flex;
            align-items: center;
            gap: 0.75rem;
            margin-bottom: 0.75rem;
        }

        .blog-list-date {
            color: var(--color-text-secondary);
            font-size: 0.875rem;
        }

        .blog-list-author {
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }

        .blog-list-avatar {
            width: 24px;
            height: 24px;
            border-radius: 50%;
        }

        .blog-list-author-name {
            font-size: 0.875rem;
            color: var(--color-text-secondary);
            text-decoration: none;
        }

        .blog-list-author-name:hover {
            color: var(--color-primary);
        }

        .blog-list-description {
            color: var(--color-text-secondary);
            margin: 0;
            line-height: 1.6;
        }

        .blog-list-read-more {
            display: inline-block;
            margin-top: 0.5rem;
            color: var(--color-primary);
            text-decoration: none;
            font-weight: 600;
            font-size: 0.875rem;
        }

        .blog-list-read-more:hover {
            text-decoration: underline;
        }
        "#
    );

    let format_date = |date: &str| -> String {
        let parts: Vec<&str> = date.split('-').collect();
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

    html! {
        <Layout title="Blog" active_nav="Blog">
            <div class={style}>
                <ul class="blog-list">
                    { for BLOG_POSTS.iter().map(|post| {
                        let url = post.url_path();
                        html! {
                            <li class="blog-list-item">
                                <h2 class="blog-list-title">
                                    <a href={url.clone()}>{post.title}</a>
                                </h2>
                                <div class="blog-list-meta">
                                    <time class="blog-list-date">{format_date(post.date)}</time>
                                    <div class="blog-list-author">
                                        <img
                                            class="blog-list-avatar"
                                            src={post.author_image_url}
                                            alt={post.author_name}
                                            width="24"
                                            height="24"
                                            loading="lazy"
                                        />
                                        <a class="blog-list-author-name" href={post.author_url}>
                                            {post.author_name}
                                        </a>
                                    </div>
                                </div>
                                <p class="blog-list-description">{post.description}</p>
                                <a class="blog-list-read-more" href={url}>{"Read more"}</a>
                            </li>
                        }
                    })}
                </ul>
            </div>
        </Layout>
    }
}
