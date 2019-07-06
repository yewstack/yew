use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Button {
    title: String,
    onsignal: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub title: String,
    pub onsignal: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            title: "Send Signal".into(),
            onsignal: None,
        }
    }
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
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<Button> for Button {
    fn view(&self) -> Html<Self> {
        html! {
            <button onclick=|_| Msg::Clicked>{ &self.title }</button>
        }
    }
}
