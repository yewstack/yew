#[macro_use]
extern crate yew;

use yew::html::{Component, Env, Html, Renderable, ShouldRender};
use yew::virtual_dom::VNode;

type Ctx = ();

struct Comp;

#[derive(PartialEq, Clone)]
struct Props {
    field_1: u32,
    field_2: u32,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            field_1: 0,
            field_2: 0,
        }
    }
}

impl Component<Ctx> for Comp {
    type Message = ();
    type Properties = Props;

    fn create(_: Self::Properties, _: &mut Env<Ctx, Self>) -> Self {
        Comp
    }

    fn update(&mut self, _: Self::Message, _: &mut Env<Ctx, Self>) -> ShouldRender {
        unimplemented!();
    }
}

impl Renderable<Ctx, Comp> for Comp {
    fn view(&self) -> Html<Ctx, Self> {
        unimplemented!();
    }
}

#[test]
fn set_properties_to_component() {
    let _: VNode<Ctx, Comp> = html! {
        <Comp: />
    };

    let _: VNode<Ctx, Comp> = html! {
        <Comp: field_1=1, />
    };

    let _: VNode<Ctx, Comp> = html! {
        <Comp: field_2=2, />
    };

    let _: VNode<Ctx, Comp> = html! {
        <Comp: field_1=1, field_2=2, />
    };

    let props = Props {
        field_1: 1,
        field_2: 1,
    };

    let _: VNode<Ctx, Comp> = html! {
        <Comp: with props, field_2=2, />
    };
}
