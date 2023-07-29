use yew::prelude::*;
use yew_router::components::Link;

use crate::content::PostMeta;
use crate::generator::Generated;
use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
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

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.post = PostMeta::generate_from_seed(ctx.props().seed);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { post } = self;
        html! {
            <div class="card">
                <div class="card-image">
                    <figure class="image is-2by1">
                        <img alt="This post's image" src={post.image_url.clone()} loading="lazy" />
                    </figure>
                </div>
                <div class="card-content">
                    <Link<Route> classes={classes!("title", "is-block")} to={Route::Post { id: post.seed }}>
                        { &post.title }
                    </Link<Route>>
                    <Link<Route> classes={classes!("subtitle", "is-block")} to={Route::Author { id: post.author.seed }}>
                        { &post.author.name }
                    </Link<Route>>
                </div>
            </div>
        }
    }
}
