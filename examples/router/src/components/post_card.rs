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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            post: Post::generate_from_seed(props.seed),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.post.seed == props.seed {
            false
        } else {
            self.post = Post::generate_from_seed(props.seed);
            true
        }
    }

    fn view(&self) -> Html {
        let Self { post } = self;
        html! {
            <div class="card">
                <div class="card-image">
                    <figure class="image is-2by1">
                        <img src=post.image_url.clone() loading="lazy" />
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
