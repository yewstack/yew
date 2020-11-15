use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub route: Option<String>,
}

pub struct PageNotFound;
impl Component for PageNotFound {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="hero is-danger is-bold is-large">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">
                            { "Page not found" }
                        </h1>
                        <h2 class="subtitle">
                            { "This page does not seem to exist" }
                        </h2>
                    </div>
                </div>
            </section>
        }
    }
}
