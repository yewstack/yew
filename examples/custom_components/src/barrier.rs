use yew::prelude::*;
use button::Button;

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


impl<CTX: 'static> Component<CTX> for Barrier {
    type Msg = Msg;
    type Properties = Props;

    fn create(_: &mut Env<CTX, Self>) -> Self {
        Barrier {
            limit: 0,
            counter: 0,
            onsignal: None,
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<CTX, Self>) -> ShouldRender {
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

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.limit = props.limit;
        self.onsignal = props.onsignal;
        true
    }
}

impl<CTX: 'static> Renderable<CTX, Barrier> for Barrier {
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div class="barrier",>
                <p>{ format!("{} on {} clicked", self.counter, self.limit) }</p>
                <Button: onsignal=|_| Msg::ChildClicked, />
                <Button: onsignal=|_| Msg::ChildClicked, />
                <Button: onsignal=|_| Msg::ChildClicked, title="Middle", />
                <Button: onsignal=|_| Msg::ChildClicked, />
                <Button: onsignal=|_| Msg::ChildClicked, />
            </div>
        }
    }
}

