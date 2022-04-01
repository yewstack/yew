//! This module contains the implementation of a portal `VPortal`.

use super::VNode;
#[cfg(feature = "csr")]
use crate::html::NodeRef;
use web_sys::{Element, Node};

#[derive(Debug, Clone)]
pub struct VPortal {
    /// The element under which the content is inserted.
    pub host: Element,
    /// The next sibling after the inserted content. Most be a child of `host`.
    #[cfg(feature = "csr")]
    pub(crate) inner_sibling: NodeRef,
    /// The inserted node
    pub node: Box<VNode>,
}

impl VPortal {
    /// Creates a [VPortal] rendering `content` in the DOM hierarchy under `host`.
    pub fn new(content: VNode, host: Element) -> Self {
        Self {
            host,
            #[cfg(feature = "csr")]
            inner_sibling: NodeRef::default(),
            node: Box::new(content),
        }
    }
    /// Creates a [VPortal] rendering `content` in the DOM hierarchy under `host`.
    /// If `next_sibling` is given, the content is inserted before that [Node].
    /// The parent of `next_sibling`, if given, must be `host`.
    pub fn new_before(content: VNode, host: Element, inner_sibling: Option<Node>) -> Self {
        #[cfg(not(feature = "csr"))]
        drop(inner_sibling);

        Self {
            host,
            #[cfg(feature = "csr")]
            inner_sibling: {
                let sib_ref = NodeRef::default();
                sib_ref.set(inner_sibling);
                sib_ref
            },
            node: Box::new(content),
        }
    }
}
