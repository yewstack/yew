use html::{Html, LocalSender};

pub trait Component<CTX>: Default {
    type Msg;
    fn update(&mut self, msg: Self::Msg, context: LocalSender<Self::Msg, CTX>);
    fn view(&self) -> Html<Self::Msg, CTX>;
}

