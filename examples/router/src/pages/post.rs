use crate::{
    content,
    generator::Generated,
    switch::{AppAnchor, AppRoute},
};
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

pub struct Post {
    post: content::Post,
}
impl Component for Post {
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

        let author_route = AppRoute::Author(post.author.seed);
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
                                <AppAnchor classes="has-text-weight-semibold" route=author_route>
                                    { &post.author.name }
                                </AppAnchor>
                            </h2>
                            <div class="tags">
                                { for keywords }
                            </div>
                        </div>
                    </div>
                </section>
                <section class="section">
                    <p class="content">
                        { &post.content }
                    </p>
                </section>
            </>
        }
    }
}
