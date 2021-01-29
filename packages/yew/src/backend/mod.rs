//! When imported, this module represents the selected backend target.
//! This may be selected via feature flag:
//!     - (default) web_sys
//!     - (not yet supported) static_render

use cfg_if::cfg_if;
use thiserror::Error as ThisError;

/// A backend with which Yew communicates to render applications. This trait is implemented
/// by all of the possible rendering backends.
pub trait RenderingBackend {
    /// A window on screen, analogous to the `window` global object in JavaScript.
    type Window;

    /// The document residing in a window. Analogous to the `document` global in Javascript.
    type Document;

    /// A nonspecific DOM element.
    type Element;

    /// A nonspecific DOM node.
    type Node;

    /// An event that notifies of editable content changes.
    type InputEvent;

    /// A DOM text node.
    type TextNode;

    /// A DOM element that represents a button, analogous to HTML's <button />
    type ButtonElement;

    /// A DOM element that represents a user input, analogous to HTML's <input />
    type InputElement;

    /// A DOM element representing a text area.
    type TextAreaElement;

    /// Returns the current window. This function will panic if there is no available window.
    fn get_window() -> Self::Window;

    /// Returns the current document. This function will panic if there is no available document.
    fn get_document() -> Self::Document;

    /// Returns the `origin` of the current window.
    fn get_origin() -> Result<String, InvalidRuntimeEnvironmentError>;

    /// Returns the `host` for the current document. Useful for connecting to the server which serves the app.
    fn get_host() -> Result<String, InvalidRuntimeEnvironmentError>;
}

#[derive(ThisError, Debug)]
pub enum InvalidRuntimeEnvironmentError {
    #[error("no window available")]
    NoWindow,

    #[error("could not access document's location")]
    NoLocation,

    #[error("could not extract host from location")]
    NoHost,

    #[error("could not extract origin from location")]
    NoOrigin,
}

cfg_if! {
    if #[cfg(feature = "static_render")] {
        unimplemented!("SSR and SSG are not implemented yet");
    } else {
        mod web_sys_backend;
        pub use self::web_sys_backend::{ Renderer };
    }
}

// Re-export types from the specific rendering backend

/// The re-exported Document from the active rendering backend.
pub type Document = <Renderer as RenderingBackend>::Document;

/// The re-exported Window from the active rendering backend.
pub type Window = <Renderer as RenderingBackend>::Window;

/// The re-exported Element from the active rendering backend.
pub type Element = <Renderer as RenderingBackend>::Element;

/// The re-exported Node from the active rendering backend.
pub type Node = <Renderer as RenderingBackend>::Node;

/// The re-exported TextNode from the active rendering backend.
pub type TextNode = <Renderer as RenderingBackend>::TextNode;

/// The re-exported InputElement from the active rendering backend.
pub type InputElement = <Renderer as RenderingBackend>::InputElement;

/// The re-exported InputEvent from the active rendering backend.
pub type InputEvent = <Renderer as RenderingBackend>::InputEvent;

/// The re-exported ButtonElement from the active rendering backend.
pub type ButtonElement = <Renderer as RenderingBackend>::ButtonElement;

/// The re-exported TextAreaElement from the active rendering backend.
pub type TextAreaElement = <Renderer as RenderingBackend>::TextAreaElement;
