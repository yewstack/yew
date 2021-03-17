use crate::{content, generator::Generated};
use yew::prelude::*;
use yew_router::RouterService;

pub struct Author {
    author: content::Author,
}
impl Component for Author {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let seed = RouterService::current_route()
            .parmas()
            .find("id")
            .unwrap()
            .parse()
            .unwrap();
        Self {
            author: content::Author::generate_from_seed(seed),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let Self { author } = self;

        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-light">
                            <p class="title">{ &author.name }</p>
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent is-3">
                            <article class="tile is-child notification">
                                <p class="title">{ "Interests" }</p>
                                <div class="tags">
                                    { for author.keywords.iter().map(|tag| html! { <span class="tag is-info">{ tag }</span> }) }
                                </div>
                            </article>
                        </div>
                        <div class="tile is-parent">
                            <figure class="tile is-child image is-square">
                                <img src=author.image_url />
                            </figure>
                        </div>
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-info">
                                <div class="content">
                                    <p class="title">{ "About me" }</p>
                                    <div class="content">
                                        { "This author has chosen not to reveal anything about themselves" }
                                    </div>
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
