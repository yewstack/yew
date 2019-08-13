use yew::prelude::*;

pub struct Child {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub on_click: Callback<()>,
    #[props(required)]
    pub name: String,
}

pub enum Msg {
    Click,
}

impl Component for Child {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Child { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.props.on_click.emit(());
            }
        }
        false
    }
}

impl Renderable<Child> for Child {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="child">
                { format!("My name is {}", self.props.name) }
                <button onclick=|_| Msg::Click>
                    { "Child button" }
                </button>
            </div>
        }
    }
}
