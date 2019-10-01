use yew::prelude::*;

pub struct Button {
    title: String,
    onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: String,
    #[props(required)]
    pub onsignal: Callback<()>,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Button {
            title: props.title,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.onsignal.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <button onclick=|_| Msg::Clicked>{ &self.title }</button>
        }
    }
}
