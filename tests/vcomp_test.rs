#[cfg(feature = "wasm_test")]
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::macros::Properties;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[cfg(feature = "wasm_test")]
wasm_bindgen_test_configure!(run_in_browser);

struct Comp;

#[derive(Clone, PartialEq, Properties)]
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

    fn view(&self) -> Html {
        unimplemented!();
    }
}

#[test]
fn set_properties_to_component() {
    html! {
        <Comp />
    };

    html! {
        <Comp field_1=1 />
    };

    html! {
        <Comp field_2=2 />
    };

    html! {
        <Comp field_1=1 field_2=2 />
    };

    let props = Props {
        field_1: 1,
        field_2: 1,
    };

    html! {
        <Comp with props />
    };
}
