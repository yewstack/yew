#[macro_use]
extern crate yew;

use yew::html::{Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::virtual_dom::VNode;

// TODO Reuse it from vtag test
type Ctx = ();

struct Comp;

impl Component<Ctx> for Comp {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Ctx, Self>) -> Self {
        Comp
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!();
    }
}

impl Renderable<Ctx, Comp> for Comp {
    fn view(&self) -> Html<Ctx, Self> {
        unimplemented!();
    }
}

#[test]
fn check_fragments() {
    let fragment: VNode<Ctx, Comp> = html! {
        <>
        </>
    };
    let _: VNode<Ctx, Comp> = html! {
        <div>
            { fragment }
        </div>
    };
}
