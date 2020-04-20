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
/// connection to the browser extension.
pub fn selector(dom_node: &Node) -> String {
    let mut current_node = Some(dom_node.clone());
    let mut selector_string = String::new();
    loop {
        match current_node { 
            Some(node) => {
                cfg_if! {
                    if #[cfg(feature="std_web")] {
                        selector_string = node.node_name() + "/" + &selector_string;
                        current_node = node.parent_node();
                    } else if #[cfg(feature="web_sys")] {
                        selector_string = node.node_name() + "/" + &selector_string;
                        current_node = node.parent_node();
                    }
                }
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
}

impl DebugComponent {
    /// Creates a new instance of `DebugComponent`
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// A message sent to describe a change in a component's state.
#[derive(Serialize, Debug)]
pub struct ComponentMessage {
    /// Time in seconds since the page was loaded.
    time: f64,
    /// The event which has happened
    event: ComponentEvent,
    /// Optional additional data about the event.
    data: Option<DebugComponent>,
}

impl ComponentMessage {
    /// Creates a new `ComponentMessage`.
    pub fn new(event: ComponentEvent, data: Option<DebugComponent>) -> Self {
        Self {
            #[cfg(feature = "web_sys")]
            time: web_sys::window().expect("").performance().unwrap().now(),
            #[cfg(feature = "std_web")]
            time: stdweb::web::Date::now(),
            event,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use cfg_if::cfg_if;
    #[cfg(feature="wasm_test")]
    use wasm_bindgen_test::*;
    cfg_if! {
        if #[cfg(feature = "std_web")] {
            use stdweb::web::{Node, INode};
        } else if #[cfg(feature = "web_sys")] {
            use web_sys::Node;
        }
    }

    #[cfg(feature="wasm_test")]
    #[cfg(feature="std_web")]
    #[wasm_bindgen_test]
    fn test_dom_selector() {
        use super::selector;
        let node = Node::from_html("<html><head></head><body><h1>Hello World!</h1></body></html>");
        match node {
            Ok(n) => {
                let dom_selector = selector(&n);
                assert_eq!(dom_selector, "SPAN/H1/")
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
    #[cfg(feature="wasm_test")]
    #[cfg(feature="web_sys")]
    #[wasm_bindgen_test]
    fn test_dom_selector() {

    }
}