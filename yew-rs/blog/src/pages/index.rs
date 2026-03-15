use stylist::yew::styled_component;
use yew::prelude::*;
use yew_site_lib::Layout;

use crate::BLOG_POSTS;

#[styled_component]
pub fn Page() -> Html {
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
            <ul class={css!(list-style: none; padding: 0; margin: 0;)}>
                { for BLOG_POSTS.iter().map(|post| {
                    let url = post.url_path();
                    html! {
                        <li class={css!(r#"
                            margin-bottom: 2rem;
                            padding-bottom: 2rem;
                            border-bottom: 1px solid var(--color-border);
                            &:last-child {
                                border-bottom: none;
                            }
                        "#)}>
                            <h2 class={css!(font-size: 1.5rem; font-weight: 700; margin: 0 0 0.5rem 0;)}>
                                <a class={css!(r#"
                                    color: var(--color-text);
                                    text-decoration: none;
                                    &:hover { color: var(--color-primary); }
                                "#)} href={url.clone()}>{post.title}</a>
                            </h2>
                            <div class={css!(r#"
                                display: flex;
                                align-items: center;
                                gap: 0.75rem;
                                margin-bottom: 0.75rem;
                            "#)}>
                                <time class={css!(color: var(--color-text-secondary); font-size: 0.875rem;)}>
                                    {format_date(post.date)}
                                </time>
                                <div class={css!(display: flex; align-items: center; gap: 0.5rem;)}>
                                    <img
                                        class={css!(width: 24px; height: 24px; border-radius: 50%;)}
                                        src={post.author_image_url}
                                        alt={post.author_name}
                                        width="24"
                                        height="24"
                                        loading="lazy"
                                    />
                                    <a class={css!(r#"
                                        font-size: 0.875rem;
                                        color: var(--color-text-secondary);
                                        text-decoration: none;
                                        &:hover { color: var(--color-primary); }
                                    "#)} href={post.author_url}>
                                        {post.author_name}
                                    </a>
                                </div>
                            </div>
                            <p class={css!(r#"
                                color: var(--color-text-secondary);
                                margin: 0;
                                line-height: 1.6;
                            "#)}>{post.description}</p>
                            <a class={css!(r#"
                                display: inline-block;
                                margin-top: 0.5rem;
                                color: var(--color-primary);
                                text-decoration: none;
                                font-weight: 600;
                                font-size: 0.875rem;
                                &:hover { text-decoration: underline; }
                            "#)} href={url}>{"Read more"}</a>
                        </li>
                    }
                })}
            </ul>
        </Layout>
    }
}
