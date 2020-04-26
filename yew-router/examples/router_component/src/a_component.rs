use crate::{c_component::CModel, ARoute, AppRoute};
use yew::{prelude::*, virtual_dom::VNode, Properties};
use yew_router::{prelude::*, switch::AllowMissing};

pub struct AModel {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub route: Option<ARoute>,
}

pub enum Msg {}

impl Component for AModel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        AModel { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                { "I am the A component" }
                <div>
                    <RouterButton<AppRoute>
                        route=AppRoute::A(AllowMissing(Some(ARoute)))
                    />
                    // {"Go to a/c"}
                    // </RouterButton<AppRoute>>
                </div>
                <div>
                {
                    match self.props.route {
                        Some(_) => html!{<CModel/>},
                        None => html!{}
                    }
                }
                </div>
            </div>
        }
    }
}
