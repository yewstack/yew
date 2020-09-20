use crate::{
    content,
    generator::Generated,
    switch::{AppAnchor, AppRoute},
};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

pub struct PostCard {
    post: content::Post,
}
impl Component for PostCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            post: content::Post::generate_from_seed(props.seed),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.post.seed == props.seed {
            false
        } else {
            self.post = content::Post::generate_from_seed(props.seed);
            true
        }
    }

    fn view(&self) -> Html {
        let Self { post } = self;
        html! {
            <div class="card">
                <div class="card-image">
                    <figure class="image is-2by1">
                        <img src={ &post.image_url } loading="lazy" />
                    </figure>
                </div>
                <div class="card-content">
                    <p class="title">{ &post.title }</p>
                    <p class="subtitle">{ &post.author.name }</p>
                </div>
                <footer class="card-footer">
                    <AppAnchor classes="card-footer-item" route=AppRoute::Post(post.seed)>
                        { "Read" }
                    </AppAnchor>
                </footer>
            </div>
        }
    }
}
