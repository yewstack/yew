use yew::prelude::*;
use yew_site_proc::comp;

use crate::content::*;
use crate::Content;

struct ExampleData {
    name: &'static str,
    display_name: &'static str,
    description: &'static str,
    component_type: &'static str,
}

const EXAMPLES: &[ExampleData] = include!(concat!(env!("OUT_DIR"), "/examples_data.rs"));

pub fn examples_page_content(is_next: bool) -> Content {
    let mut blocks = Vec::new();

    if !is_next {
        blocks.push(admonition!(
            AdmonitionType::Info,
            None,
            p![
                "The hosted demos at ",
                code("examples.yew.rs"),
                " are built from the ",
                code("master"),
                " branch and may use features not available in this version of Yew.",
            ],
        ));
    }

    blocks.push(p!["The Yew repository contains many examples that \
                    demonstrate different features and patterns of the \
                    framework. Each example is a standalone project you \
                    can run locally, and most have a live deployment you \
                    can try instantly.",]);

    let mut md = String::from("| Example | Type | Description |\n| --- | --- | --- |\n");
    for ex in EXAMPLES {
        md.push_str(&format!(
            "| [{}](https://examples.yew.rs/{}) | {} | {} |\n",
            ex.display_name, ex.name, ex.component_type, ex.description,
        ));
    }
    blocks.push(custom(html! { <ExamplesGallery /> }, md));

    Content::new(blocks)
}

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Function,
    Struct,
}

fn matches_filter(ct: &str, filter: Filter) -> bool {
    match filter {
        Filter::All => true,
        Filter::Function => ct.contains('F'),
        Filter::Struct => ct.contains('S'),
    }
}

#[comp]
fn ExamplesGallery() {
    let search = use_state(String::new);
    let filter = use_state(|| Filter::All);

    let query = search.to_lowercase();
    let filtered: Vec<&ExampleData> = EXAMPLES
        .iter()
        .filter(|ex| matches_filter(ex.component_type, *filter))
        .filter(|ex| {
            query.is_empty()
                || ex.display_name.to_lowercase().contains(&query)
                || ex.name.contains(&query)
                || ex.description.to_lowercase().contains(&query)
        })
        .collect();

    let total = EXAMPLES.len();
    let shown = filtered.len();

    #[cfg(feature = "csr")]
    let oninput = {
        let search = search.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search.set(input.value());
        })
    };
    #[cfg(not(feature = "csr"))]
    let oninput = Callback::<InputEvent>::noop();

    let make_filter = |f: Filter| {
        let filter = filter.clone();
        Callback::from(move |_: MouseEvent| filter.set(f))
    };

    html! {
        <div class={css!(
            margin-top: 1.5rem;
        )}>
            <div class={css!(
                display: flex;
                flex-wrap: wrap;
                gap: 0.75rem;
                align-items: center;
                margin-bottom: 1rem;
            )}>
                <div class={css!(
                    flex: 1 1 280px;
                    position: relative;
                    min-width: 0;
                )}>
                    <svg viewBox="0 0 24 24" width="18" height="18" class={css!(
                        position: absolute;
                        left: 0.75rem;
                        top: 50%;
                        transform: translateY(-50%);
                        color: var(--color-text-secondary);
                        pointer-events: none;
                    )}>
                        <path fill="currentColor" d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0016 9.5 6.5 6.5 0 109.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
                    </svg>
                    <input
                        type="text"
                        placeholder="Search examples..."
                        value={(*search).clone()}
                        oninput={oninput}
                        class={css!(
                            width: 100%;
                            padding: 0.625rem 0.75rem 0.625rem 2.5rem;
                            border: 1px solid var(--color-border);
                            border-radius: 8px;
                            background: var(--color-bg);
                            color: var(--color-text);
                            font-size: 0.9375rem;
                            font-family: var(--font-sans);
                            outline: none;
                            box-sizing: border-box;
                            transition: border-color 0.2s, box-shadow 0.2s;
                            &:focus {
                                border-color: var(--color-primary);
                                box-shadow: 0 0 0 3px rgba(0, 117, 90, 0.12);
                            }
                            &::placeholder {
                                color: var(--color-text-secondary);
                            }
                        )}
                    />
                </div>
                <div class={css!(
                    display: flex;
                    gap: 0.25rem;
                    background: var(--color-bg-secondary);
                    border-radius: 8px;
                    padding: 0.25rem;
                    border: 1px solid var(--color-border);
                )}>
                    <FilterButton label="All" active={*filter == Filter::All} onclick={make_filter(Filter::All)} />
                    <FilterButton label="Function" active={*filter == Filter::Function} onclick={make_filter(Filter::Function)} />
                    <FilterButton label="Struct" active={*filter == Filter::Struct} onclick={make_filter(Filter::Struct)} />
                </div>
            </div>

            <p class={css!(
                color: var(--color-text-secondary);
                font-size: 0.8125rem;
                margin: 0 0 1rem 0;
            )}>
                {format!("Showing {shown} of {total} examples")}
            </p>

            if filtered.is_empty() {
                <div class={css!(
                    text-align: center;
                    padding: 3rem 1rem;
                    color: var(--color-text-secondary);
                )}>
                    <p class={css!(font-size: 1.125rem; margin: 0 0 0.5rem 0;)}>
                        {"No examples match your search."}
                    </p>
                    <p class={css!(font-size: 0.875rem; margin: 0;)}>
                        {"Try adjusting your search or filter."}
                    </p>
                </div>
            } else {
                <div class={css!(
                    display: grid;
                    grid-template-columns: 1fr;
                    gap: 1rem;
                    @media (min-width: 600px) {
                        grid-template-columns: repeat(2, 1fr);
                    }
                    @media (min-width: 960px) {
                        grid-template-columns: repeat(3, 1fr);
                    }
                )}>
                    { for filtered.iter().map(|ex| html! {
                        <ExampleCard
                            name={ex.name}
                            display_name={ex.display_name}
                            description={ex.description}
                            component_type={ex.component_type}
                        />
                    }) }
                </div>
            }
        </div>
    }
}

#[comp]
fn FilterButton(label: AttrValue, active: bool, onclick: Callback<MouseEvent>) {
    let bg = if active {
        "var(--color-bg)"
    } else {
        "transparent"
    };
    let color = if active {
        "var(--color-text)"
    } else {
        "var(--color-text-secondary)"
    };
    let weight = if active { "600" } else { "400" };
    html! {
        <button
            {onclick}
            class={css!(
                padding: 0.375rem 0.75rem;
                border: none;
                border-radius: 6px;
                background: ${bg};
                color: ${color};
                font-weight: ${weight};
                font-size: 0.8125rem;
                font-family: var(--font-sans);
                cursor: pointer;
                transition: all 0.15s;
                &:hover { color: var(--color-text); }
            )}
        >
            {&label}
        </button>
    }
}

#[comp]
fn ExampleCard(
    name: AttrValue,
    display_name: AttrValue,
    description: AttrValue,
    component_type: AttrValue,
) {
    let badge_color = match component_type.as_str() {
        ct if ct.contains('S') && ct.contains('F') => "#8b5cf6",
        ct if ct.contains('S') => "#6366f1",
        _ => "var(--color-primary)",
    };
    let badge_label = match component_type.as_str() {
        ct if ct.contains('S') && ct.contains('F') => "SC+FC",
        ct if ct.contains('S') => "SC",
        _ => "FC",
    };
    let demo_url = format!("https://examples.yew.rs/{}", name);
    let source_url = format!(
        "https://github.com/yewstack/yew/tree/master/examples/{}",
        name
    );
    html! {
        <div class={css!(
            background: var(--color-bg);
            border: 1px solid var(--color-border);
            border-radius: 10px;
            display: flex;
            flex-direction: column;
            position: relative;
            min-width: 0;
            transition: box-shadow 0.2s, border-color 0.2s, transform 0.2s;
            &:hover {
                box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
                border-color: var(--color-primary);
                transform: translateY(-2px);
            }
        )}>
            <span class={css!(
                position: absolute;
                top: 1.25rem;
                right: 1.25rem;
                font-size: 0.6875rem;
                font-weight: 600;
                padding: 0.125rem 0.5rem;
                border-radius: 9999px;
                color: ${badge_color};
                background: ${format!("color-mix(in srgb, {badge_color} 12%, transparent)")};
                white-space: nowrap;
                line-height: 1.3;
            )}>
                {badge_label}
            </span>
            <div class={css!(padding: 1.25rem 3.5rem 0 1.25rem; flex: 1;)}>
                <h3 class={css!(
                    margin: 0 0 0.5rem 0;
                    font-size: 1rem;
                    font-weight: 600;
                    line-height: 1.3;
                )}>
                    {&display_name}
                </h3>
                if !description.is_empty() {
                    <p class={css!(
                        margin: 0;
                        font-size: 0.8125rem;
                        color: var(--color-text-secondary);
                        line-height: 1.5;
                    )}>
                        {&description}
                    </p>
                }
            </div>
            <div class={css!(
                padding: 1rem 1.25rem;
                display: flex;
                gap: 0.75rem;
                align-items: center;
            )}>
                <a
                    href={demo_url}
                    target="_blank"
                    rel="noopener noreferrer"
                    class={css!(
                        display: inline-flex;
                        align-items: center;
                        gap: 0.375rem;
                        padding: 0.5rem 1rem;
                        background: var(--color-primary);
                        color: white;
                        border-radius: 6px;
                        font-size: 0.8125rem;
                        font-weight: 600;
                        font-family: var(--font-sans);
                        text-decoration: none;
                        transition: background 0.15s, transform 0.15s;
                        &:hover {
                            background: var(--color-primary-dark);
                            transform: scale(1.02);
                        }
                    )}
                >
                    <svg viewBox="0 0 24 24" width="14" height="14">
                        <path fill="currentColor" d="M8 5v14l11-7z"/>
                    </svg>
                    {"Demo"}
                </a>
                <a
                    href={source_url}
                    target="_blank"
                    rel="noopener noreferrer"
                    class={css!(
                        display: inline-flex;
                        align-items: center;
                        gap: 0.375rem;
                        color: var(--color-text-secondary);
                        font-size: 0.8125rem;
                        text-decoration: none;
                        transition: color 0.15s;
                        &:hover { color: var(--color-text); }
                    )}
                >
                    <svg viewBox="0 0 24 24" width="14" height="14">
                        <path fill="currentColor" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"/>
                    </svg>
                    {"Source"}
                </a>
            </div>
        </div>
    }
}
