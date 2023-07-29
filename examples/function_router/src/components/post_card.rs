use std::rc::Rc;

use yew::prelude::*;
use yew_router::components::Link;

use crate::content::PostMeta;
use crate::generator::Generated;
use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub seed: u32,
}

#[derive(PartialEq, Eq, Debug)]
pub struct PostMetaState {
    inner: PostMeta,
}

impl Reducible for PostMetaState {
    type Action = u32;

    fn reduce(self: Rc<Self>, action: u32) -> Rc<Self> {
        Self {
            inner: PostMeta::generate_from_seed(action),
        }
        .into()
    }
}

#[function_component]
pub fn PostCard(props: &Props) -> Html {
    let seed = props.seed;

    let post = use_reducer_eq(|| PostMetaState {
        inner: PostMeta::generate_from_seed(seed),
    });

    {
        let post_dispatcher = post.dispatcher();
        use_effect_with(seed, move |seed| {
            post_dispatcher.dispatch(*seed);

            || {}
        });
    }

    let post = &post.inner;

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
