//! When imported, this module represents the selected backend target.
//! This may be selected via feature flag:
//!     - std_web
//!     - web_sys
//!     - static_render

use crate::NodeRef;

pub trait DomBackend {
    type Element: DomElement;
    type Node: DomNode;
    type Document;
    type Window;
    type InputEvent;
    type InputData;
    type ChangeData;
    type EventListener;

    /// Returns the current window. This function will panic if there is no available window.
    fn get_window() -> Self::Window;

    /// Returns the current document.
    fn get_document() -> Self::Document;

    /// Returns the `origin` of the current window.
    fn get_origin() -> Result<String, anyhow::Error>;

    /// Returns the `host` for the current document. Useful for connecting to the server which serves the app.
    fn get_host() -> Result<String, anyhow::Error>;

    // Element-related methods
    fn element_as_node(element: &Self::Element) -> Self::Node;
    fn element_last_child(element: &Self::Element) -> Option<Self::Element>;
    fn element_remove_child(
        element: &Self::Element,
        child: &Self::Element,
    ) -> Result<Self::Node, ()>;
    fn cast_node_ref<INTO>(node_ref: &NodeRef) -> Option<INTO>;

    fn oninput_handler(this: &Self::Element, event: Self::InputEvent) -> Self::InputData;
    fn onchange_handler(this: &Self::Element) -> Self::ChangeData;
}

pub(crate) trait DomElement {}
pub(crate) trait DomNode {}
