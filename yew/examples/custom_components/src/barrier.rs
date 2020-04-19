use crate::button::Button;
use yew::prelude::*;

pub struct Barrier {
    link: ComponentLink<Self>,
    limit: u32,
    counter: u32,
    onsignal: Callback<()>,
}

pub enum Msg {
    ChildClicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub limit: u32,
    pub onsignal: Callback<()>,
}

impl Component for Barrier {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Barrier {
            link,
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
                    self.onsignal.emit(());
                    self.counter = 0;
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

    fn view(&self) -> Html {
        let onsignal = &self.link.callback(|_| Msg::ChildClicked);
        html! {
            <div class="barrier">
                <p>{ format!("{} on {} clicked", self.counter, self.limit) }</p>
                <Button onsignal=onsignal />
                <Button onsignal=onsignal />
                <Button onsignal=onsignal title="Middle" />
                <Button onsignal=onsignal />
                <Button onsignal=onsignal />
            </div>
        }
    }
}
