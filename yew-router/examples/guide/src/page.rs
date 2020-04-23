use yew::{prelude::*, virtual_dom::VNode};

pub struct Page;

#[derive(Properties, Clone)]
pub struct PageProps {
    pub uri: String,
    pub page_url: String,
    pub title: String,
}

impl Component for Page {
    type Message = ();
    type Properties = PageProps;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Page
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> VNode {
        unimplemented!()
    }
}
