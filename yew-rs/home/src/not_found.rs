use stylist::yew::styled_component;
use yew::prelude::*;
use yew_site_lib::Layout;

fn detect_locale() -> &'static str {
    #[cfg(feature = "csr")]
    {
        let path = web_sys::window()
            .and_then(|w| w.location().pathname().ok())
            .unwrap_or_default();
        if path.starts_with("/ja/") || path == "/ja" {
            return "ja";
        }
        if path.starts_with("/zh-Hans/") || path == "/zh-Hans" {
            return "zh-Hans";
        }
        if path.starts_with("/zh-Hant/") || path == "/zh-Hant" {
            return "zh-Hant";
        }
    }
    "en"
}

fn localized_strings(locale: &str) -> (&'static str, &'static str) {
    match locale {
        "ja" => (
            "ページが見つかりません",
            "お探しのページは見つかりませんでした。",
        ),
        "zh-Hans" => ("页面未找到", "我们无法找到您要查找的内容。"),
        "zh-Hant" => ("頁面未找到", "我們無法找到您要查找的內容。"),
        _ => (
            "Page Not Found",
            "We could not find what you were looking for.",
        ),
    }
}

#[styled_component]
pub fn Page() -> Html {
    let locale = detect_locale();
    let (title, message) = localized_strings(locale);

    html! {
        <Layout title="" active_nav="" full_width=true>
            <div class={css!(
        r#"
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 4rem 2rem;
        text-align: center;
        min-height: 40vh;

        .logo {
            width: 80px;
            height: 80px;
            margin-bottom: 1.5rem;
            opacity: 0.6;
        }

        h1 {
            font-size: 2rem;
            margin: 0 0 0.75rem;
        }

        p {
            color: var(--color-text-secondary);
            font-size: 1.125rem;
            margin: 0;
        }
        "#
    )}>
                <img class="logo" src="/img/logo.svg" alt="" />
                <h1>{title}</h1>
                <p>{message}</p>
            </div>
        </Layout>
    }
}
