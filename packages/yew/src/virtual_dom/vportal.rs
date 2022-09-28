//! This module contains the implementation of a portal `VPortal`.

use web_sys::{Element, Node};

use super::VNode;

#[derive(Debug, Clone)]
pub struct VPortal {
    /// The element under which the content is inserted.
    pub host: Element,
    /// The next sibling after the inserted content. Most be a child of `host`.
    pub inner_sibling: Option<Node>,
    /// The inserted node
    pub node: Box<VNode>,
}

impl VPortal {
    /// Creates a [VPortal] rendering `content` in the DOM hierarchy under `host`.
    pub fn new(content: VNode, host: Element) -> Self {
        Self {
            host,
            inner_sibling: None,
            node: Box::new(content),
        }
    }

    /// Creates a [VPortal] rendering `content` in the DOM hierarchy under `host`.
    /// If `next_sibling` is given, the content is inserted before that [Node].
    /// The parent of `next_sibling`, if given, must be `host`.
    pub fn new_before(content: VNode, host: Element, inner_sibling: Option<Node>) -> Self {
        Self {
            host,
            inner_sibling,
            node: Box::new(content),
        }
    }
}
