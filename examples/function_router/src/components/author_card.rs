use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::content::Author;
use crate::generator::Generated;
use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub seed: u32,
}

#[derive(PartialEq, Eq, Debug)]
pub struct AuthorState {
    pub inner: Author,
}

impl Reducible for AuthorState {
    type Action = u32;

    fn reduce(self: Rc<Self>, action: u32) -> Rc<Self> {
        Self {
            inner: Author::generate_from_seed(action),
        }
        .into()
    }
}

#[function_component]
pub fn AuthorCard(props: &Props) -> Html {
    let seed = props.seed;

    let author = use_reducer_eq(|| AuthorState {
        inner: Author::generate_from_seed(seed),
    });

    {
        let author_dispatcher = author.dispatcher();
        use_effect_with_deps(
            move |seed| {
                author_dispatcher.dispatch(*seed);

                || {}
            },
            seed,
        );
    }

    let author = &author.inner;

    html! {
        <div class="card">
            <div class="card-content">
                <div class="media">
                    <div class="media-left">
                        <figure class="image is-128x128">
                            <img alt="Author's profile picture" src={author.image_url.clone()} />
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
                <Link<Route> classes={classes!("card-footer-item")} to={Route::Author { id: author.seed }}>
                    { "Profile" }
                </Link<Route>>
            </footer>
        </div>
    }
}
