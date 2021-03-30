use crate::{content, generator::Generated, Routes};
use content::PostPart;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Post {
    post: content::Post,
}
impl Component for Post {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let seed = match RouterService::current_route().route() {
            Routes::Post { id } => *id,
            _ => unreachable!()
        };

        Self {
            post: content::Post::generate_from_seed(seed),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let Self { post } = self;

        let keywords = post
            .keywords
            .iter()
            .map(|keyword| html! { <span class="tag is-info">{ keyword }</span> });

        html! {
            <>
                <section class="hero is-medium is-light has-background">
                    <img class="hero-background is-transparent" src=post.image_url />
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">
                                { &post.title }
                            </h1>
                            <h2 class="subtitle">
                                { "by " }
                                <Link<Routes> classes="has-text-weight-semibold" route=Routes::Author { id: post.author.seed }>
                                    { &post.author.name }
                                </Link<Routes>>
                            </h2>
                            <div class="tags">
                                { for keywords }
                            </div>
                        </div>
                    </div>
                </section>
                <div class="section container">
                    { self.view_content() }
                </div>
            </>
        }
    }
}
impl Post {
    fn render_quote(&self, quote: &content::Quote) -> Html {
        html! {
            <article class="media block box my-6">
                <figure class="media-left">
                    <p class="image is-64x64">
                        <img src=quote.author.image_url loading="lazy" />
                    </p>
                </figure>
                <div class="media-content">
                    <div class="content">
                        <Link<Routes> classes="is-size-5" route=Routes::Author { id: quote.author.seed }>
                            <strong>{ &quote.author.name }</strong>
                        </Link<Routes>>
                        <p class="is-family-secondary">
                            { &quote.content }
                        </p>
                    </div>
                </div>
            </article>
        }
    }

    fn render_section_hero(&self, section: &content::Section) -> Html {
        html! {
            <section class="hero is-dark has-background mt-6 mb-3">
                <img class="hero-background is-transparent" src=section.image_url loading="lazy" />
                <div class="hero-body">
                    <div class="container">
                        <h2 class="subtitle">{ &section.title }</h2>
                    </div>
                </div>
            </section>
        }
    }

    fn render_section(&self, section: &content::Section, show_hero: bool) -> Html {
        let hero = if show_hero {
            self.render_section_hero(section)
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
    }

    fn view_content(&self) -> Html {
        // don't show hero for the first section
        let mut show_hero = false;

        let parts = self.post.content.iter().map(|part| match part {
            PostPart::Section(section) => {
                let html = self.render_section(section, show_hero);
                // show hero between sections
                show_hero = true;
                html
            }
            PostPart::Quote(quote) => {
                // don't show hero after a quote
                show_hero = false;
                self.render_quote(quote)
            }
        });
        html! { for parts }
    }
}
