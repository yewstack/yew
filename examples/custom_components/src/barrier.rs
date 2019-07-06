use crate::button::Button;
use yew::{html, Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Barrier {
    limit: u32,
    counter: u32,
    onsignal: Option<Callback<()>>,
}

pub enum Msg {
    ChildClicked,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub limit: u32,
    pub onsignal: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            limit: 0,
            onsignal: None,
        }
    }
}


impl Component for Barrier {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Barrier {
            limit: props.limit,
            counter: 0,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChildClicked => {
                self.counter += 1;
                if self.counter >= self.limit {
                    if let Some(ref mut callback) = self.onsignal {
                        callback.emit(());
                        self.counter = 0;
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.limit = props.limit;
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<Barrier> for Barrier {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="barrier">
                <p>{ format!("{} on {} clicked", self.counter, self.limit) }</p>
                <Button onsignal=|_| Msg::ChildClicked />
                <Button onsignal=|_| Msg::ChildClicked />
                <Button onsignal=|_| Msg::ChildClicked title="Middle" />
                <Button onsignal=|_| Msg::ChildClicked />
                <Button onsignal=|_| Msg::ChildClicked />
            </div>
        }
    }
}
