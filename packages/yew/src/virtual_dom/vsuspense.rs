use super::{Key, VNode};
use web_sys::Element;

/// This struct represents a suspendable DOM fragment.
#[derive(Clone, Debug, PartialEq)]
pub struct VSuspense {
    /// Child nodes.
    pub(crate) children: Box<VNode>,
    /// Fallback nodes when suspended.
    pub(crate) fallback: Box<VNode>,
    /// The element to attach to when children is not attached to DOM
    pub(crate) detached_parent: Element,
    /// Whether the current status is suspended.
    pub(crate) suspended: bool,
    /// The Key.
    pub(crate) key: Option<Key>,
}

impl VSuspense {
    pub(crate) fn new(
        children: VNode,
        fallback: VNode,
        detached_parent: Element,
        suspended: bool,
        key: Option<Key>,
    ) -> Self {
        Self {
            children: children.into(),
            fallback: fallback.into(),
            detached_parent,
            suspended,
            key,
        }
    }
}
