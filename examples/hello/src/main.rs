use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
    value: String,
}

impl Component for Model {
    type Message = ChangeData;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ChangeData::Value(value) => self.value = value,
            _ => unreachable!(),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input onchange=self.link.callback(|x| x) />
                <Hello name=self.value.clone() />
                <Hello name="world!" />
            </div>
        }
    }
}

struct Hello {
    props: HelloProps,
}

#[derive(Clone, PartialEq, Properties)]
struct HelloProps {
    name: yew::virtual_dom::VNode,
}

impl Component for Hello {
    type Message = ();
    type Properties = HelloProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        return html! {
            <div>
                {"Hello "}
                {self.props.name.clone()}
            </div>
        };
    }
}

fn main() {
    yew::start_app::<Model>();
}
