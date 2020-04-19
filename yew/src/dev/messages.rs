//! Messages describing the state of an application.

use serde::Serialize;

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

/// Data about a component
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
            #[cfg(feature="web_sys")]
            time: web_sys::window().expect("").performance().unwrap().now(),
            #[cfg(feature="std_web")]
            time: stdweb::web::Date::now(),
            event,
            data,
        }
    }
}
