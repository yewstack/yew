use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub route: Option<String>,
}

pub struct PageNotFound {
    props: Props,
}
impl Component for PageNotFound {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
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
