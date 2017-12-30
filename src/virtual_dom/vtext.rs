//! This module contains the implementation of a virtual text node `VText`.

use std::fmt;
use stdweb::web::{INode, TextNode};

/// A type for a virtual
/// [TextNode](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
/// represenation.
pub struct VText {
    /// Contains a text of the node.
    pub text: String,
}

impl VText {
    /// Creates new virtual text node with a content.
    pub fn new<T: ToString>(text: T) -> Self {
        VText { text: text.to_string() }
    }

    /// Renders virtual node over existent `TextNode`, but
    /// only if value of text had changed.
    pub fn render(&mut self, subject: &TextNode, opposite: Option<Self>) {
        if let Some(opposite) = opposite {
            if self.text != opposite.text {
                subject.set_node_value(Some(&self.text));
            }
        } else {
            subject.set_node_value(Some(&self.text));
        }
    }
}

impl fmt::Debug for VText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VText {{ text: {} }}", self.text)
    }
}

