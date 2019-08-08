#[cfg(feature = "wasm_test")]
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::macros::Properties;
use yew::virtual_dom::VNode;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

#[cfg(feature = "wasm_test")]
wasm_bindgen_test_configure!(run_in_browser);

struct Comp;

#[derive(PartialEq, Properties)]
struct Props {
    field_1: u32,
    field_2: u32,
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
        <Comp />
    };

    let _: VNode<Comp> = html! {
        <Comp field_1=1 />
    };

    let _: VNode<Comp> = html! {
        <Comp field_2=2 />
    };

    let _: VNode<Comp> = html! {
        <Comp field_1=1 field_2=2 />
    };

    let props = Props {
        field_1: 1,
        field_2: 1,
    };

    let _: VNode<Comp> = html! {
        <Comp with props />
    };
}
