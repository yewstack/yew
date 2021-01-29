use crate::backend::RenderingBackend;
use anyhow::{anyhow, Error};
use web_sys::{
    Document, Element, HtmlButtonElement as ButtonElement, HtmlInputElement as InputElement,
    HtmlTextAreaElement as TextAreaElement, InputEvent, Node, Text as TextNode, Window,
};

/// The rendering backend for web_sys
#[derive(Debug)]
pub struct Renderer {}

impl RenderingBackend for Renderer {
    type Element = Element;
    type Node = Node;
    type Document = Document;
    type Window = Window;
    type InputEvent = InputEvent;

    type InputElement = InputElement;
    type ButtonElement = ButtonElement;
    type TextAreaElement = TextAreaElement;
    type TextNode = TextNode;

    /// Returns the `host` for the current document. Useful for connecting to the server which serves
    /// the app.
    pub fn get_host() -> Result<String, InvalidRuntimeEnvironmentError> {
        let location = document()
            .location()
            .ok_or(InvalidRuntimeEnvironmentError::NoLocation)?;
        location.host().ok_or(InvalidRuntimeEnvironmentError::NoHost)
    }

    /// Returns the current window. This function will panic if there is no available window.
    fn get_window() -> Window {
        web_sys::window().expect("no window available")
    }

    /// Returns the current document. This function will panic if there is no available document.
    fn get_document() -> Document {
        Self::get_window().document().unwrap()
    }

    /// Returns the `origin` of the current window.
    pub fn get_origin() -> Result<String, InvalidRuntimeEnvironmentError> {
        let location = window().location();
        location.origin().ok_or(InvalidRuntimeEnvironmentError::NoOrigin)
    }
}
