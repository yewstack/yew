//! This module contains the items required for a statically typed VDOM

use std::collections::HashMap;
use std::rc::Rc;

use yew_macro::generate_element;

use crate::virtual_dom::{AttrValue, Key, Listener, VNode};
use crate::{Children, NodeRef};

generate_element! {
    button;
    props: {
        autofocus: AttrValue,
        disabled: AttrValue,
        form: AttrValue,
        formaction: AttrValue,
        formenctype: AttrValue,
        formmethod: AttrValue,
        formnovalidate: AttrValue,
        formtarget: AttrValue,
        name: AttrValue,
        type_: AttrValue,
        value: AttrValue,
        node_ref: NodeRef,
        key: Key,
        children: Children,
    }
}

/// Metadata of an HTML element
///
/// A [Component](crate::html::Component) is generated using this data for every element.
#[derive(Debug)]
pub struct ElementData {
    node_ref: NodeRef,
    attributes: HashMap<&'static str, AttrValue>,
    listeners: Vec<Option<Rc<dyn Listener>>>,
    key: Option<Key>,
    children: Vec<VNode>,
}

#[cfg(all(test, feature = "wasm_test"))]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use crate::{function_component, html, props, Callback, Html, Properties};

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn it_works() {
        use super::button as Btn;

        const TEXT: &'static str = "Inner Text";
        const CLICKED_TEXT: &'static str = "Clicked Text";

        #[derive(PartialEq, Properties)]
        struct Props {
            on_click: Callback<web_sys::MouseEvent>,
        }

        #[function_component]
        fn Comp(props: &Props) -> Html {
            html! {
                <Btn class="ccc" name="yes" on_click={props.on_click.clone()}>{ TEXT }</Btn>
            }
        }

        let event_data = Rc::new(RefCell::new(None));

        let on_click = {
            let event_data = Rc::clone(&event_data);
            Callback::from(move |_| {
                (*event_data).borrow_mut().replace(CLICKED_TEXT);
            })
        };

        let document = gloo_utils::document();
        yew::start_app_with_props_in_element::<Comp>(
            document.get_element_by_id("output").unwrap(),
            props!(Props { on_click: on_click }),
        );
        let button = gloo_utils::document()
            .query_selector("#output button")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlElement>();
        assert_eq!(button.get_attribute("class").unwrap(), "ccc");
        assert_eq!(button.get_attribute("name").unwrap(), "yes");
        assert_eq!(button.inner_text(), TEXT);

        button.click();
        let data = *event_data.borrow();
        assert_eq!(data, Some(CLICKED_TEXT))
    }
}
