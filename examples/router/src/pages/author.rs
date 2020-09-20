use crate::{content, generator::Generated};
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

pub struct Author {
    author: content::Author,
}
impl Component for Author {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            author: content::Author::generate_from_seed(props.seed),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.author.seed == props.seed {
            false
        } else {
            self.author = content::Author::generate_from_seed(props.seed);
            true
        }
    }

    fn view(&self) -> Html {
        let Self { author } = self;

        html! {
            <>
                <h1>{ &author.name }</h1>
                <figure class="image is-128x128">
                    <img class="is-rounded" src=author.image_url />
                </figure>
            </>
        }
    }
}
