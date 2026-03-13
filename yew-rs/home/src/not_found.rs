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
            "\u{30da}\u{30fc}\u{30b8}\u{304c}\u{898b}\u{3064}\u{304b}\u{308a}\u{307e}\u{305b}\\
             u{3093}",
            "\u{304a}\u{63a2}\u{3057}\u{306e}\u{30da}\u{30fc}\u{30b8}\u{306f}\u{898b}\u{3064}\\
             u{304b}\u{308a}\u{307e}\u{305b}\u{3093}\u{3067}\u{3057}\u{305f}\u{3002}",
        ),
        "zh-Hans" => (
            "\u{9875}\u{9762}\u{672a}\u{627e}\u{5230}",
            "\u{6211}\u{4eec}\u{65e0}\u{6cd5}\u{627e}\u{5230}\u{60a8}\u{8981}\u{67e5}\u{627e}\\
             u{7684}\u{5185}\u{5bb9}\u{3002}",
        ),
        "zh-Hant" => (
            "\u{9801}\u{9762}\u{672a}\u{627e}\u{5230}",
            "\u{6211}\u{5011}\u{7121}\u{6cd5}\u{627e}\u{5230}\u{60a8}\u{8981}\u{67e5}\u{627e}\\
             u{7684}\u{5167}\u{5bb9}\u{3002}",
        ),
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

    let style = css!(
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
    );

    html! {
        <Layout title="" active_nav="" full_width=true>
            <div class={style}>
                <img class="logo" src="/img/logo.svg" alt="" />
                <h1>{title}</h1>
                <p>{message}</p>
            </div>
        </Layout>
    }
}
