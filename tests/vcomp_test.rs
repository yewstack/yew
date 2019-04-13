extern crate yew;

use yew::virtual_dom::VNode;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

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

impl Component for Comp {
    type Message = ();
    type Properties = Props;

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
fn set_properties_to_component() {
    let _: VNode<Comp> = html! {
        <Comp: />
    };

    let _: VNode<Comp> = html! {
        <Comp: field_1=1, />
    };

    let _: VNode<Comp> = html! {
        <Comp: field_2=2, />
    };

    let _: VNode<Comp> = html! {
        <Comp: field_1=1, field_2=2, />
    };

    let props = Props {
        field_1: 1,
        field_2: 1,
    };

    let _: VNode<Comp> = html! {
        <Comp: with props, field_2=2, />
    };
}
