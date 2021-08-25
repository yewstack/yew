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

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            post: PostMeta::generate_from_seed(ctx.props().seed),
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> ShouldRender {
        self.post = PostMeta::generate_from_seed(ctx.props().seed);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
