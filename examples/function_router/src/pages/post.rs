use std::rc::Rc;

use content::PostPart;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::generator::Generated;
use crate::{content, Route};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub seed: u32,
}

#[derive(PartialEq, Eq, Debug)]
pub struct PostState {
    pub inner: content::Post,
}

impl Reducible for PostState {
    type Action = u32;

    fn reduce(self: Rc<Self>, action: u32) -> Rc<Self> {
        Self {
            inner: content::Post::generate_from_seed(action),
        }
        .into()
    }
}

#[function_component]
pub fn Post(props: &Props) -> Html {
    let seed = props.seed;

    let post = use_reducer(|| PostState {
        inner: content::Post::generate_from_seed(seed),
    });

    {
        let post_dispatcher = post.dispatcher();
        use_effect_with_deps(
            move |seed| {
                post_dispatcher.dispatch(*seed);

                || {}
            },
            seed,
        );
    }

    let post = &post.inner;

    let render_quote = |quote: &content::Quote| {
        html! {
            <article class="media block box my-6">
                <figure class="media-left">
                    <p class="image is-64x64">
                        <img alt="The author's profile" src={quote.author.image_url.clone()} loading="lazy" />
                    </p>
                </figure>
                <div class="media-content">
                    <div class="content">
                        <Link<Route> classes={classes!("is-size-5")} to={Route::Author { id: quote.author.seed }}>
                            <strong>{ &quote.author.name }</strong>
                        </Link<Route>>
                        <p class="is-family-secondary">
                            { &quote.content }
                        </p>
                    </div>
                </div>
            </article>
        }
    };

    let render_section_hero = |section: &content::Section| {
        html! {
            <section class="hero is-dark has-background mt-6 mb-3">
                <img alt="This section's image" class="hero-background is-transparent" src={section.image_url.clone()} loading="lazy" />
                <div class="hero-body">
                    <div class="container">
                        <h2 class="subtitle">{ &section.title }</h2>
                    </div>
                </div>
            </section>
        }
    };

    let render_section = |section, show_hero| {
        let hero = if show_hero {
            render_section_hero(section)
        } else {
            html! {}
        };
        let paragraphs = section.paragraphs.iter().map(|paragraph| {
            html! {
                <p>{ paragraph }</p>
            }
        });
        html! {
            <section>
                { hero }
                <div>{ for paragraphs }</div>
            </section>
        }
    };

    let view_content = {
        // don't show hero for the first section
        let mut show_hero = false;

        let parts = post.content.iter().map(|part| match part {
            PostPart::Section(section) => {
                let html = render_section(section, show_hero);
                // show hero between sections
                show_hero = true;
                html
            }
            PostPart::Quote(quote) => {
                // don't show hero after a quote
                show_hero = false;
                render_quote(quote)
            }
        });
        html! { for parts }
    };

    let keywords = post
        .meta
        .keywords
        .iter()
        .map(|keyword| html! { <span class="tag is-info">{ keyword }</span> });

    html! {
        <>
            <section class="hero is-medium is-light has-background">
                <img alt="The hero's background" class="hero-background is-transparent" src={post.meta.image_url.clone()} />
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">
                            { &post.meta.title }
                        </h1>
                        <h2 class="subtitle">
                            { "by " }
                            <Link<Route> classes={classes!("has-text-weight-semibold")} to={Route::Author { id: post.meta.author.seed }}>
                                { &post.meta.author.name }
                            </Link<Route>>
                        </h2>
                        <div class="tags">
                            { for keywords }
                        </div>
                    </div>
                </div>
            </section>
            <div class="section container">
                { view_content }
            </div>
        </>
    }
}
