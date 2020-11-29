//! When imported, this module represents the selected backend target.
//! This may be selected via feature flag:
//!     - std_web
//!     - web_sys
//!     - static_render

use crate::NodeRef;

cfg_if::cfg_if! {
    if #[cfg(feature = "std_web")] {
        mod std_web;
    } else if #[cfg(feature = "web_sys")] {
        mod web_sys;
    } else if #[cfg(feature = "static_render")] {
        mod smr;
        pub use smr::*;
        pub use smr::{
            Document, Element, Node, Renderer, Window, EventListener,
            InputElement, SelectElement, TextAreaElement, FileList, InputEvent,
            ButtonElement, Text, get_window, get_document, get_origin, get_host
        };
    }
}

pub trait DomBackend {
    type Element;
    type Node;
    type Document;
    type Window;
    type InputEvent;
    type InputData;
    type ChangeData;

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
