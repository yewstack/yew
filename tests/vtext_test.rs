extern crate yew;

use yew::virtual_dom::VNode;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

struct Comp;

impl Component for Comp {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Comp
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!();
    }
}

impl Renderable<Comp> for Comp {
    fn view(&self) -> Html<Self> {
        unimplemented!();
    }
}

#[test]
fn text_as_root() {
    let _: VNode<Comp> = html! {
        { "Text Node As Root" }
    };
}
