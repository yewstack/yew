use crate::{
    content::Post,
    generator::Generated,
    switch::{AppAnchor, AppRoute},
};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

pub struct PostCard {
    post: Post,
}

impl Component for PostCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            post: Post::generate_from_seed(ctx.props.seed),
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, new_props: &Self::Properties) -> ShouldRender {
        self.post = Post::generate_from_seed(new_props.seed);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { post } = self;
        html! {
            <div class="card">
                <div class="card-image">
                    <figure class="image is-2by1">
                        <img src={ &post.image_url } loading="lazy" />
                    </figure>
                </div>
                <div class="card-content">
                    <AppAnchor classes="title is-block" route=AppRoute::Post(post.seed)>
                        { &post.title }
                    </AppAnchor>
                    <AppAnchor classes="subtitle is-block" route=AppRoute::Author(post.author.seed)>
                        { &post.author.name }
                    </AppAnchor>
                </div>
            </div>
        }
    }
}
