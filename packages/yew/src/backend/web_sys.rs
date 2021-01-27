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
    fn get_host() -> Result<String, Error> {
        let location = Self::get_document()
            .location()
            .ok_or_else(|| anyhow!("can't get location"))?;

        let host = location.host().map_err(|e| {
            anyhow!(e
                .as_string()
                .unwrap_or_else(|| String::from("error not recoverable")),)
        })?;

        Ok(host)
    }

    /// Returns the current window. This function will panic if there is no available window.
    fn get_window() -> Window {
        web_sys::window().expect("no window available")
    }

    /// Returns the current document.
    fn get_document() -> Document {
        Self::get_window().document().unwrap()
    }

    /// Returns the `origin` of the current window.
    fn get_origin() -> Result<String, Error> {
        let location = Self::get_window().location();

        let origin = location.origin().map_err(|e| {
            anyhow!(e
                .as_string()
                .unwrap_or_else(|| String::from("error not recoverable")),)
        })?;

        Ok(origin)
    }
}
