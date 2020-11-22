use crate::{
    content::Author,
    generator::Generated,
    switch::{AppAnchor, AppRoute},
};
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

pub struct AuthorCard {
    author: Author,
}
impl Component for AuthorCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            author: Author::generate_from_seed(ctx.props.seed),
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, new_props: &Self::Properties) -> ShouldRender {
        self.author = Author::generate_from_seed(new_props.seed);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { author } = self;
        html! {
            <div class="card">
                <div class="card-content">
                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-128x128">
                                <img src=author.image_url />
                            </figure>
                        </div>
                        <div class="media-content">
                            <p class="title is-3">{ &author.name }</p>
                            <p>
                                { "I like " }
                                <b>{ author.keywords.join(", ") }</b>
                            </p>
                        </div>
                    </div>
                </div>
                <footer class="card-footer">
                    <AppAnchor classes="card-footer-item" route=AppRoute::Author(author.seed)>
                        { "Profile" }
                    </AppAnchor>
                </footer>
            </div>
        }
    }
}
