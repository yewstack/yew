use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub value: String,
    pub onchange: Callback<String>,
}

pub struct Model {
    props: Props,
    link: ComponentLink<Self>,
}

pub type TextInput = Model;

impl Component for TextInput {
    type Message = String;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextInput { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props.onchange.emit(msg);
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let props = self.props.clone();

        html! {
            <input
                type="text"
                oninput={self.link.callback(|input: InputData| input.value)}
                value={props.value}
            />
        }
    }
}
