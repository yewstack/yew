//! This module contains the implementation of a virtual component (`VComp`).

use super::{Key, VDiff, VNode};
use crate::html::{AnyScope, Component, ComponentUpdate, NodeRef, Scope, Scoped};
use std::any::TypeId;
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

use super::Element;
use mountable::{Mountable, PropsWrapper};
pub use vchild::VChild;
pub use vcomp::VComp;
mod vcomp {
    use super::*;
    /// A virtual component.
    pub struct VComp {
        type_id: TypeId,
        pub(crate) scope: Option<Box<dyn Scoped>>,
        props: Option<Box<dyn Mountable>>,
        pub(crate) node_ref: NodeRef,
        pub(crate) key: Option<Key>,
    }

    impl Clone for VComp {
        fn clone(&self) -> Self {
            if self.scope.is_some() {
                panic!("Mounted components are not allowed to be cloned!");
            }

            Self {
                type_id: self.type_id,
                scope: None,
                props: self.props.as_ref().map(|m| m.copy()),
                node_ref: self.node_ref.clone(),
                key: self.key.clone(),
            }
        }
    }

    impl<COMP> From<VChild<COMP>> for VComp
    where
        COMP: Component,
    {
        fn from(vchild: VChild<COMP>) -> Self {
            VComp::new::<COMP>(vchild.props, vchild.node_ref, vchild.key)
        }
    }

    impl VComp {
        /// Creates a new `VComp` instance.
        pub fn new<COMP>(props: COMP::Properties, node_ref: NodeRef, key: Option<Key>) -> Self
        where
            COMP: Component,
        {
            VComp {
                type_id: TypeId::of::<COMP>(),
                node_ref,
                props: Some(Box::new(PropsWrapper::<COMP>::new(props))),
                scope: None,
                key,
            }
        }

        #[allow(unused)]
        pub(crate) fn root_vnode(&self) -> Option<impl Deref<Target = VNode> + '_> {
            self.scope.as_ref().and_then(|scope| scope.root_vnode())
        }
    }

    impl VDiff for VComp {
        fn detach(&mut self, _parent: &Element) {
            self.scope.take().expect("VComp is not mounted").destroy();
        }

        fn apply(
            &mut self,
            parent_scope: &AnyScope,
            parent: &Element,
            next_sibling: NodeRef,
            ancestor: Option<VNode>,
        ) -> NodeRef {
            let mountable = self.props.take().expect("VComp has already been mounted");

            if let Some(mut ancestor) = ancestor {
                if let VNode::VComp(ref mut vcomp) = &mut ancestor {
                    // If the ancestor is the same type, reuse it and update its properties
                    if self.type_id == vcomp.type_id && self.key == vcomp.key {
                        self.node_ref.reuse(vcomp.node_ref.clone());
                        let scope = vcomp.scope.take().expect("VComp is not mounted");
                        mountable.reuse(self.node_ref.clone(), scope.borrow(), next_sibling);
                        self.scope = Some(scope);
                        return vcomp.node_ref.clone();
                    }
                }

                ancestor.detach(parent);
            }

            let placeholder: Node = get_document().create_text_node("").into();
            super::super::insert_node(&placeholder, parent, next_sibling.get());
            self.node_ref.set(Some(placeholder));
            let scope = mountable.mount(
                self.node_ref.clone(),
                parent_scope,
                parent.to_owned(),
                next_sibling,
            );
            self.scope = Some(scope);
            self.node_ref.clone()
        }
    }

    impl PartialEq for VComp {
        fn eq(&self, other: &VComp) -> bool {
            self.type_id == other.type_id
        }
    }

    impl fmt::Debug for VComp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("VComp")
        }
    }

    impl<COMP: Component> fmt::Debug for VChild<COMP> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("VChild<_>")
        }
    }
}

mod vchild {
    use super::*;
    /// A virtual child component.
    pub struct VChild<COMP: Component> {
        /// The component properties
        pub props: COMP::Properties,
        /// Reference to the mounted node
        node_ref: NodeRef,
        key: Option<Key>,
    }

    impl<COMP: Component> Clone for VChild<COMP> {
        fn clone(&self) -> Self {
            VChild {
                props: self.props.clone(),
                node_ref: self.node_ref.clone(),
                key: self.key.clone(),
            }
        }
    }

    impl<COMP: Component> PartialEq for VChild<COMP>
    where
        COMP::Properties: PartialEq,
    {
        fn eq(&self, other: &VChild<COMP>) -> bool {
            self.props == other.props
        }
    }

    impl<COMP> VChild<COMP>
    where
        COMP: Component,
    {
        /// Creates a child component that can be accessed and modified by its parent.
        pub fn new(props: COMP::Properties, node_ref: NodeRef, key: Option<Key>) -> Self {
            Self {
                props,
                node_ref,
                key,
            }
        }
    }
}

mod mountable {
    use super::*;
    pub(crate) trait Mountable {
        fn copy(&self) -> Box<dyn Mountable>;
        fn mount(
            self: Box<Self>,
            node_ref: NodeRef,
            parent_scope: &AnyScope,
            parent: Element,
            next_sibling: NodeRef,
        ) -> Box<dyn Scoped>;
        fn reuse(self: Box<Self>, node_ref: NodeRef, scope: &dyn Scoped, next_sibling: NodeRef);
    }

    pub(crate) struct PropsWrapper<COMP: Component> {
        props: COMP::Properties,
    }

    impl<COMP: Component> PropsWrapper<COMP> {
        pub fn new(props: COMP::Properties) -> Self {
            Self { props }
        }
    }

    impl<COMP: Component> Mountable for PropsWrapper<COMP> {
        fn copy(&self) -> Box<dyn Mountable> {
            let wrapper: PropsWrapper<COMP> = PropsWrapper {
                props: self.props.clone(),
            };
            Box::new(wrapper)
        }

        fn mount(
            self: Box<Self>,
            node_ref: NodeRef,
            parent_scope: &AnyScope,
            parent: Element,
            next_sibling: NodeRef,
        ) -> Box<dyn Scoped> {
            let scope: Scope<COMP> = Scope::new(Some(parent_scope.clone()));
            let scope = scope.mount_in_place(
                parent,
                next_sibling,
                Some(VNode::VRef(node_ref.get().unwrap())),
                node_ref,
                self.props,
            );

            Box::new(scope)
        }

        fn reuse(self: Box<Self>, node_ref: NodeRef, scope: &dyn Scoped, next_sibling: NodeRef) {
            let scope: Scope<COMP> = scope.to_any().downcast();
            scope.update(ComponentUpdate::Properties(
                self.props,
                node_ref,
                next_sibling,
            ));
        }
    }
}
