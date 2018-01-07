use html::Html;

pub trait Component<CTX>: Default {
    type Msg;
    fn update(&mut self, msg: Self::Msg, context: &mut CTX);
    fn view(&self) -> Html<Self::Msg, CTX>;
}

