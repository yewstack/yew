use yew::prelude::*;

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

impl<CTX: 'static> Component<CTX> for Button {
    type Msg = Msg;
    type Properties = Props;

    fn create(_: &mut ScopeRef<CTX, Self>) -> Self {
        Button {
            title: "Send Signal".into(),
            onsignal: None,
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut ScopeRef<CTX, Self>) -> ShouldUpdate {
        match msg {
            Msg::Clicked => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties, _: &mut ScopeRef<CTX, Self>) -> ShouldUpdate {
        self.title = props.title;
        self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html<CTX, Self> {
        html! {
            <button onclick=|_| Msg::Clicked,>{ &self.title }</button>
        }
    }
}

