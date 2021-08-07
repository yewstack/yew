use crate::{content::PostMeta, generator::Generated, Route};
use yew::prelude::*;
use yew_router::components::Link;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

pub struct PostCard {
    post: PostMeta,
}
impl Component for PostCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            post: PostMeta::generate_from_seed(props.seed),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.post.seed == props.seed {
            false
        } else {
            self.post = PostMeta::generate_from_seed(props.seed);
            true
        }
    }

    fn view(&self) -> Html {
        let Self { post } = self;
        html! {
            <div class="card">
                <div class="card-image">
                    <figure class="image is-2by1">
                        <img src={post.image_url.clone()} loading="lazy" />
                    </figure>
                </div>
                <div class="card-content">
                    <Link<Route> classes={classes!("title", "is-block")} route={Route::Post { id: post.seed }}>
                        { &post.title }
                    </Link<Route>>
                    <Link<Route> classes={classes!("subtitle", "is-block")} route={Route::Author { id: post.author.seed }}>
                        { &post.author.name }
                    </Link<Route>>
                </div>
            </div>
        }
    }
}
