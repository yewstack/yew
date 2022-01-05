//! This module contains the implementation of a virtual component (`VComp`).

use super::Key;
use crate::dom_bundle::{Mountable, PropsWrapper};
use crate::html::{BaseComponent, NodeRef};
use std::any::TypeId;
use std::fmt;
use std::rc::Rc;

/// A virtual component.
pub struct VComp {
    pub(crate) type_id: TypeId,
    pub(crate) props: Box<dyn Mountable>,
    pub(crate) node_ref: NodeRef,
    pub(crate) key: Option<Key>,
}

impl fmt::Debug for VComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VComp")
            .field("type_id", &self.type_id)
            .field("node_ref", &self.node_ref)
            .field("props", &"..")
            .field("key", &self.key)
            .finish()
    }
}

impl Clone for VComp {
    fn clone(&self) -> Self {
        Self {
            type_id: self.type_id,
            props: self.props.copy(),
            node_ref: self.node_ref.clone(),
            key: self.key.clone(),
        }
    }
}

/// A virtual child component.
pub struct VChild<COMP: BaseComponent> {
    /// The component properties
    pub props: Rc<COMP::Properties>,
    /// Reference to the mounted node
    node_ref: NodeRef,
    key: Option<Key>,
}

impl<COMP: BaseComponent> Clone for VChild<COMP> {
    fn clone(&self) -> Self {
        VChild {
            props: Rc::clone(&self.props),
            node_ref: self.node_ref.clone(),
            key: self.key.clone(),
        }
    }
}

impl<COMP: BaseComponent> PartialEq for VChild<COMP>
where
    COMP::Properties: PartialEq,
{
    fn eq(&self, other: &VChild<COMP>) -> bool {
        self.props == other.props
    }
}

impl<COMP> VChild<COMP>
where
    COMP: BaseComponent,
{
    /// Creates a child component that can be accessed and modified by its parent.
    pub fn new(props: COMP::Properties, node_ref: NodeRef, key: Option<Key>) -> Self {
        Self {
            props: Rc::new(props),
            node_ref,
            key,
        }
    }
}

impl<COMP> From<VChild<COMP>> for VComp
where
    COMP: BaseComponent,
{
    fn from(vchild: VChild<COMP>) -> Self {
        VComp::new::<COMP>(vchild.props, vchild.node_ref, vchild.key)
    }
}

impl VComp {
    /// Creates a new `VComp` instance.
    pub fn new<COMP>(props: Rc<COMP::Properties>, node_ref: NodeRef, key: Option<Key>) -> Self
    where
        COMP: BaseComponent,
    {
        VComp {
            type_id: TypeId::of::<COMP>(),
            node_ref,
            props: Box::new(PropsWrapper::<COMP>::new(props)),
            key,
        }
    }
}

impl PartialEq for VComp {
    fn eq(&self, other: &VComp) -> bool {
        self.type_id == other.type_id
    }
}

impl<COMP: BaseComponent> fmt::Debug for VChild<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("VChild<_>")
    }
}
