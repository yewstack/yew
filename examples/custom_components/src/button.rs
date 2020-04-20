use yew::prelude::*;

pub struct Button {
    link: ComponentLink<Self>,
    title: String,
    onsignal: Callback<()>,
}

pub enum Msg {
    Clicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
    pub onsignal: Callback<()>,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Button {
            link,
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

    fn view(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| Msg::Clicked)>
                { &self.title }
            </button>
        }
    }
}
