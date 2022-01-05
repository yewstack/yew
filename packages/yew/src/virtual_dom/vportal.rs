//! This module contains the implementation of a portal `VPortal`.

use super::VNode;
use crate::html::NodeRef;
use web_sys::{Element, Node};

#[derive(Debug, Clone)]
pub struct VPortal {
    /// The element under which the content is inserted.
    pub host: Element,
    /// The next sibling after the inserted content
    pub next_sibling: NodeRef,
    /// The inserted node
    pub node: Box<VNode>,
}

impl VPortal {
    /// Creates a [VPortal] rendering `content` in the DOM hierarchy under `host`.
    pub fn new(content: VNode, host: Element) -> Self {
        Self {
            host,
            next_sibling: NodeRef::default(),
            node: Box::new(content),
        }
    }
    /// Creates a [VPortal] rendering `content` in the DOM hierarchy under `host`.
    /// If `next_sibling` is given, the content is inserted before that [Node].
    /// The parent of `next_sibling`, if given, must be `host`.
    pub fn new_before(content: VNode, host: Element, next_sibling: Option<Node>) -> Self {
        Self {
            host,
            next_sibling: {
                let sib_ref = NodeRef::default();
                sib_ref.set(next_sibling);
                sib_ref
            },
            node: Box::new(content),
        }
    }
}
