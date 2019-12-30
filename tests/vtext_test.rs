#[cfg(feature = "wasm_test")]
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::virtual_dom::{VText, VNode};

#[cfg(feature = "wasm_test")]
wasm_bindgen_test_configure!(run_in_browser);

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

    fn view(&self) -> Html {
        unimplemented!();
    }
}

#[test]
fn text_as_root() {
    let no_braces = html! {
        "Text Node As Root"
    };

    let with_braces = html! {
        { "Text Node As Root" }
    };

    let expected_tree = VNode::VText(
        VText::new("Text Node As Root".to_string())
    );

    assert_eq!(no_braces, expected_tree);
    assert_eq!(with_braces, expected_tree);
}
