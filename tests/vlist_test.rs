#[macro_use]
extern crate yew;

use yew::html::{Html, Component, Renderable, Env, ShouldRender};
use yew::virtual_dom::VNode;

// TODO Reuse it from vtag test
type Ctx = ();

struct Comp;

impl Component<Ctx> for Comp {
    type Msg = ();
    type Properties = ();

    fn create(_: &mut Env<Ctx, Self>) -> Self {
        Comp
    }

    fn update(&mut self, _: Self::Msg, _: &mut Env<Ctx, Self>) -> ShouldRender {
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
