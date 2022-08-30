use yew::prelude::*;

use crate::content;
use crate::generator::Generated;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

pub struct Author {
    author: content::Author,
}
impl Component for Author {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            author: content::Author::generate_from_seed(ctx.props().seed),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.author = content::Author::generate_from_seed(ctx.props().seed);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
                                <img alt="The author's profile picture." src={author.image_url.clone()} />
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
