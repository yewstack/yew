#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::virtual_dom::VNode;

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
fn check_fragments() {
    let fragment: VNode<Comp> = html! {
        <>
        </>
    };
    let _: VNode<Comp> = html! {
        <div>
            { fragment }
        </div>
    };
}
