use yew::html::BaseComponent;
use yew::prelude::*;

pub struct Comp;

impl BaseComponent for Comp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, _ctx: &Context<Self>) -> HtmlResult {
        todo!()
    }
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
    fn destroy(&mut self, _ctx: &Context<Self>) {}
}

fn main() {}
