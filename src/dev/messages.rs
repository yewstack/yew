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
    /// Sent when a component is created,
    Created,
}

/// Data about a component
#[derive(Serialize, Debug)]
pub struct DebugComponent {
    /// The name of the component
    name: String,
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
            time: web_sys::window().expect("").performance().unwrap().now(),
            event,
            data,
        }
    }
}
