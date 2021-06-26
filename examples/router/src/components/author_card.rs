use crate::{content::Author, generator::Generated, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

pub struct AuthorCard {
    author: Author,
}
impl Component for AuthorCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            author: Author::generate_from_seed(props.seed),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.author.seed == props.seed {
            false
        } else {
            self.author = Author::generate_from_seed(props.seed);
            true
        }
    }

    fn view(&self) -> Html {
        let Self { author } = self;
        html! {
            <div class="card">
                <div class="card-content">
                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-128x128">
                                <img src={author.image_url.clone()} />
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
                    <Link<Route> classes={classes!("card-footer-item")} route={Route::Author { id: author.seed }}>
                        { "Profile" }
                    </Link<Route>>
                </footer>
            </div>
        }
    }
}
