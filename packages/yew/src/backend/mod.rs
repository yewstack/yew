//! When imported, this module represents the selected backend target.
//! This may be selected via feature flag:
//!     - (default) web_sys
//!     - static_render

use crate::NodeRef;
use cfg_if::cfg_if;
use wasm_bindgen::JsCast;

pub trait DomBackend {
    type ButtonElement;
    type Document;
    type Element;
    type InputElement;
    type InputEvent;
    type Node;
    type TextAreaElement;
    type TextNode;
    type Window;

    /// Returns the current window. This function will panic if there is no available window.
    fn get_window() -> Self::Window;

    /// Returns the current document.
    fn get_document() -> Self::Document;

    /// Returns the `origin` of the current window.
    fn get_origin() -> Result<String, anyhow::Error>;

    /// Returns the `host` for the current document. Useful for connecting to the server which serves the app.
    fn get_host() -> Result<String, anyhow::Error>;
}

cfg_if! {
    if #[cfg(feature = "static_render")] {
        unimplemented!("SSR and SSG are not implemented yet");
    } else {
        use ::web_sys::{FileList, HtmlSelectElement as SelectElement};
        mod web_sys;
        pub use self::web_sys::{ Renderer };
    }
}

// Re-export types from the specific renderer backend

pub type Element = <Renderer as DomBackend>::Element;
pub type Node = <Renderer as DomBackend>::Node;
pub type TextNode = <Renderer as DomBackend>::TextNode;
pub type InputElement = <Renderer as DomBackend>::InputElement;
pub type InputEvent = <Renderer as DomBackend>::InputEvent;
pub type ButtonElement = <Renderer as DomBackend>::ButtonElement;
pub type TextAreaElement = <Renderer as DomBackend>::TextAreaElement;
