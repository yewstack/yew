use html::Html;

pub trait Component: Default {
    type Msg;
    fn update(&mut self, msg: Self::Msg);
    fn view(&self) -> Html<Self::Msg>;
}

