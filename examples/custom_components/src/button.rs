use yew::prelude::*;
use yew::html::PureComponent;

pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Properties)]
pub struct Button {
    pub title: String,
    #[props(required)]
    pub onsignal: Callback<()>,
}

impl PureComponent for Button {
    type Message = Msg;

    fn render(&self) -> Html<Self> {
        html! {
            <button onclick=|_| Msg::Clicked>{ &self.title }</button>
        }
    }

    fn emit(&self, _: Self::Message) {
        self.onsignal.emit(())
    }
}
