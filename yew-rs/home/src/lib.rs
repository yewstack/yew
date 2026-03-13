pub mod not_found;
pub mod search;

use stylist::yew::styled_component;
use yew::prelude::*;
use yew_site_lib::Layout;

#[styled_component]
fn Hero() -> Html {
    let style = css!(
        r#"
        background: var(--color-hero-bg);
        padding: 4rem 2rem;
        text-align: center;

        .header {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: calc(var(--h1-font-size) / 2);
            padding: var(--h1-font-size);
        }

        .logo {
            width: calc(var(--h1-font-size) * 5);
            height: calc(var(--h1-font-size) * 5);
        }

        .title {
            font-size: calc(var(--h1-font-size) * 3);
            margin: 0;
        }

        .subtitle {
            font-size: 1.25rem;
            color: var(--color-text-secondary);
            padding: 1rem;
            max-width: 600px;
            margin: 0 auto;
        }

        .actions {
            display: flex;
            gap: 1rem;
            justify-content: center;
            margin-top: 2rem;
            flex-wrap: wrap;
        }

        .button {
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.75rem 1.5rem;
            border-radius: 6px;
            font-weight: 600;
            font-size: 1rem;
            cursor: pointer;
            transition: background 0.2s, color 0.2s, border-color 0.2s;
            text-decoration: none;
            border: 2px solid transparent;
        }

        .button:hover {
            text-decoration: none;
        }

        .button.primary {
            background: var(--color-primary);
            color: #fff;
            border-color: var(--color-primary);
        }

        .button.primary:hover {
            background: var(--color-primary-dark);
            border-color: var(--color-primary-dark);
            color: #fff;
        }

        .button.outline {
            background: transparent;
            color: var(--color-primary);
            border-color: var(--color-primary);
        }

        .button.outline:hover {
            background: var(--color-primary);
            color: #fff;
        }

        .button.secondary {
            background: var(--color-bg-secondary);
            color: var(--color-text);
            border-color: var(--color-border);
        }

        .button.secondary:hover {
            background: var(--color-border);
        }

        .button.lg {
            padding: 1rem 2rem;
            font-size: 1.125rem;
        }

        @media (max-width: 700px) {
            .header {
                flex-direction: column;
            }

            .title {
                font-size: calc(var(--h1-font-size) * 1.5);
            }

            .logo {
                width: calc(var(--h1-font-size) * 3);
                height: calc(var(--h1-font-size) * 3);
            }

            .actions {
                flex-direction: column;
                align-items: center;
            }
        }
    "#
    );

    html! {
        <div class={style}>
            <div class="header">
                <img class="logo" src="/img/logo.svg" alt="Yew" />
                <h1 class="title">{"Yew"}</h1>
            </div>
            <p class="subtitle">
                {"A framework for creating reliable and efficient web applications."}
            </p>
            <div class="actions">
                <a class="button lg outline primary" href="/docs/getting-started">
                    {"Get Started"}
                </a>
                <a class="button lg outline secondary" href="https://play.yew.rs" target="_blank" rel="noopener noreferrer">
                    {"Playground"}
                    <svg viewBox="0 0 24 24" width="16" height="16">
                        <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                    </svg>
                </a>
            </div>
        </div>
    }
}

struct Feature {
    header: &'static str,
    body: &'static str,
    href: &'static str,
}

const FEATURES: &[Feature] = &[
    Feature {
        header: "Component Based",
        body: "Features a component-based framework which makes it easy to create interactive \
               UIs. Developers who have experience with frameworks like React and Elm should feel \
               quite at home when using Yew.",
        href: "/docs/concepts/function-components",
    },
    Feature {
        header: "HTML macro",
        body: "Features a macro for declaring interactive HTML with Rust expressions. Developers \
               who have experience using JSX in React should feel quite at home when using Yew.",
        href: "/docs/concepts/html",
    },
    Feature {
        header: "Server Side Rendering",
        body: "Features server side rendering for all the SEO and enhancements of server-rendered \
               app while keeping the feel of an SPA.",
        href: "/docs/advanced-topics/server-side-rendering",
    },
];

#[styled_component]
fn Features() -> Html {
    let style = css!(
        r#"
        padding: 3rem 2rem;
        max-width: 1200px;
        margin: 0 auto;

        & h2 {
            font-size: calc(1.5rem * 1.5);
            margin-bottom: 1.5rem;
        }

        .grid {
            display: grid;
            grid-template-columns: 1fr;
            gap: 2rem;
            padding: 1rem 0;
        }

        @media (min-width: 700px) {
            .grid {
                grid-template-columns: repeat(3, 1fr);
            }
        }

        .card {
            background: var(--color-bg);
            border: 1px solid var(--color-border);
            border-radius: 8px;
            overflow: hidden;
            transition: box-shadow 0.2s;
        }

        .card:hover {
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
        }

        .card-header {
            padding: 1.25rem 1.25rem 0;
        }

        .card-header h3 {
            margin: 0;
        }

        .card-body {
            padding: 0.75rem 1.25rem;
            color: var(--color-text-secondary);
        }

        .card-body p {
            margin: 0;
        }

        .card-footer {
            padding: 0 1.25rem 1.25rem;
        }

        .button {
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.75rem 1.5rem;
            border-radius: 6px;
            font-weight: 600;
            font-size: 1rem;
            cursor: pointer;
            transition: background 0.2s, color 0.2s, border-color 0.2s;
            text-decoration: none;
            border: 2px solid transparent;
        }

        .button:hover {
            text-decoration: none;
        }

        .button.secondary {
            background: var(--color-bg-secondary);
            color: var(--color-text);
            border-color: var(--color-border);
        }

        .button.secondary:hover {
            background: var(--color-border);
        }
    "#
    );

    html! {
        <article class={style}>
            <h2>{"Features"}</h2>
            <section class="grid">
                { for FEATURES.iter().map(|f| html! {
                    <div class="card">
                        <div class="card-header">
                            <h3>{f.header}</h3>
                        </div>
                        <div class="card-body">
                            <p>{f.body}</p>
                        </div>
                        <div class="card-footer">
                            <a class="button secondary" href={f.href}>
                                {"Learn more"}
                            </a>
                        </div>
                    </div>
                })}
            </section>
        </article>
    }
}

#[component]
pub fn Page() -> Html {
    html! {
        <Layout title="" description="A framework for creating reliable and efficient web applications." full_width=true>
            <Hero />
            <Features />
        </Layout>
    }
}

#[cfg(feature = "ssr")]
pub async fn render_pages() -> Vec<(&'static str, String, String)> {
    let mut pages = Vec::new();
    pages.push(yew_site_lib::render_page!("/", Page));
    pages.push(yew_site_lib::render_page!("/search", search::Page));
    pages.push(yew_site_lib::render_page!("/404", not_found::Page));
    pages
}
