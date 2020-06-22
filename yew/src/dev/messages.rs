//! Messages describing the state of an application.

use cfg_if::cfg_if;
use serde::Serialize;

cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Node, INode};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::Node;
    }
}

/// Sent when something happens to a component.
#[derive(Serialize, Debug)]
pub enum ComponentEvent {
    /// Sent when a component mounts to the DOM
    Mounted,
    /// Sent when a component unmounts from the DOM
    Unmounted,
    /// Sent when a component updates itself
    Updated,
    /// Sent when a component is created
    Created,
    /// Sent when a component is destroyed
    Destroyed,
}

/// Generates a selector for a given node which can then be sent over a WebSocket
/// connection to the browser extension. This works by climbing to the top of the
/// DOM tree, adding each DOM node to a string as it does so.
pub fn selector(dom_node: &Node) -> String {
    let mut current_node = Some(dom_node.clone());
    let mut selector_string = String::new();
    loop {
        match current_node {
            Some(node) => {
                selector_string = node.node_name() + "/" + &selector_string;
                current_node = node.parent_node();
            }
            None => {
                return selector_string;
            }
        };
    }
}

/// Stores data about a component (currently only the name of the struct).
#[derive(Serialize, Debug)]
pub struct DebugComponent {
    /// The name of the component
    name: String,
    selector: Option<String>,
}

impl DebugComponent {
    /// Creates a new instance of `DebugComponent`
    pub fn new(name: String, selector: Option<String>) -> Self {
        Self { name, selector }
    }
}

/// A message sent to describe a change in a component's state.
#[derive(Serialize, Debug)]
pub struct ComponentMessage {
    /// The event which is to be logged.
    event: ComponentEvent,
    /// Optional additional data about the event (e.g. the component's location in the DOM).
    data: Option<DebugComponent>,
}

impl ComponentMessage {
    /// Creates a new instance of `ComponentMessage`
    pub fn new(event: ComponentEvent, data: Option<DebugComponent>) -> Self {
        Self { event, data }
    }
}

#[cfg(test)]
pub mod tests {
    use cfg_if::cfg_if;
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::*;
    cfg_if! {
        if #[cfg(feature = "std_web")] {
            use stdweb::web::{Node, INode};
        } else if #[cfg(feature = "web_sys")] {
            use web_sys::Node;
        }
    }

    #[cfg(feature = "wasm_test")]
    #[cfg(feature = "std_web")]
    #[wasm_bindgen_test]
    fn test_dom_selector() {
        use super::selector;
        let node = Node::from_html("<html><head></head><body><h1>Hello World!</h1></body></html>");
        match node {
            Ok(n) => {
                let dom_selector = selector(&n);
                assert_eq!(dom_selector, "SPAN/H1/")
            }
            Err(e) => panic!("{:?}", e),
        }
    }
    #[cfg(feature = "wasm_test")]
    #[cfg(feature = "web_sys")]
    #[wasm_bindgen_test]
    fn test_dom_selector() {
        let document = crate::utils::document();
        use super::selector;
        let element = document.create_element("div").unwrap();
        let element2 = document.create_element("h1").unwrap();
        element.append_child(&element2).unwrap();
        assert_eq!(selector(element2.as_ref()), "DIV/H1/");
    }
}
