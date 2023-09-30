//! This module contains the implementation of a portal `VPortal`.

use std::rc::Rc;

use web_sys::{Element, Node};

use super::VNode;
use crate::html::ImplicitClone;

#[derive(Debug, Clone, PartialEq)]
pub struct VPortal {
    /// The element under which the content is inserted.
    pub host: Element,
    /// The next sibling after the inserted content. Must be a child of `host`.
    pub inner_sibling: Option<Node>,
    /// The inserted node
    pub node: Rc<VNode>,
}

impl ImplicitClone for VPortal {}

impl VPortal {
    /// Creates a [VPortal] rendering `content` in the DOM hierarchy under `host`.
    pub fn new(content: VNode, host: Element) -> Self {
        Self {
            host,
            inner_sibling: None,
            node: Rc::new(content),
        }
    }

    /// Creates a [VPortal] rendering `content` in the DOM hierarchy under `host`.
    /// If `inner_sibling` is given, the content is inserted before that [Node].
    /// The parent of `inner_sibling`, if given, must be `host`.
    pub fn new_before(content: VNode, host: Element, inner_sibling: Option<Node>) -> Self {
        Self {
            host,
            inner_sibling,
            node: Rc::new(content),
        }
    }
}
