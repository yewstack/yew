// NOTE: Support for component is tested in vcom_test.rs
#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::virtual_dom::VNode;

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
fn test_selected_value() {
    let selected_value: Option<String> = Some("val_1".to_string());
    let _: VNode<Comp> = html! {
        <select selected_value=selected_value,>
            <option value="val_1",>{"Value 1"}</option>
        </select>
    };
    
    let selected_value = None;
    let _: VNode<Comp> = html! {
        <select selected_value=selected_value,>
            <option value="val_1",>{"Value 1"}</option>
        </select>
    };
}

#[test]
fn test_selected_index() {
    let selected_index = Some(1_usize);
    let _: VNode<Comp> = html! {
        <select selected_index=selected_index,>
            <option value="val_1",>{"Value 1"}</option>
            <option value="val_2",>{"Value 2"}</option>
        </select>
    };

    let selected_index = None;
    let _: VNode<Comp> = html! {
        <select selected_index=selected_index,>
            <option value="val_1",>{"Value 1"}</option>
            <option value="val_2",>{"Value 2"}</option>
        </select>
    };
}