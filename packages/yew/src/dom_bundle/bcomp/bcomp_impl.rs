//! This module contains the bundle implementation of a virtual component [BComp].

use super::Scoped;
use crate::dom_bundle::{BNode, DomBundle, Reconcilable};
use crate::html::{AnyScope, BaseComponent, Scope};
use crate::virtual_dom::{Key, VComp};
use crate::NodeRef;
#[cfg(feature = "ssr")]
use futures::future::{FutureExt, LocalBoxFuture};
use std::{any::TypeId, borrow::Borrow};
use std::{fmt, rc::Rc};
use web_sys::Element;

thread_local! {
    #[cfg(debug_assertions)]
     static EVENT_HISTORY: std::cell::RefCell<std::collections::HashMap<u64, Vec<String>>>
        = Default::default();
}

/// Push [VComp] event to lifecycle debugging registry
#[cfg(debug_assertions)]
pub fn log_event(vcomp_id: u64, event: impl ToString) {
    EVENT_HISTORY.with(|h| {
        h.borrow_mut()
            .entry(vcomp_id)
            .or_default()
            .push(event.to_string())
    });
}

/// Get [VComp] event log from lifecycle debugging registry
#[cfg(debug_assertions)]
#[allow(dead_code)]
pub fn get_event_log(vcomp_id: u64) -> Vec<String> {
    EVENT_HISTORY.with(|h| {
        h.borrow()
            .get(&vcomp_id)
            .map(|l| (*l).clone())
            .unwrap_or_default()
    })
}

/// A virtual component. Compare with [VComp].
pub struct BComp {
    type_id: TypeId,
    scope: Box<dyn Scoped>,
    node_ref: NodeRef,
    key: Option<Key>,
}

impl BComp {
    /// Get the key of the underlying component
    pub(in crate::dom_bundle) fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }
}

impl fmt::Debug for BComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BComp {{ root: {:?} }}",
            self.scope.root_bnode().as_deref()
        )
    }
}

pub trait Mountable {
    fn copy(&self) -> Box<dyn Mountable>;
    fn mount(
        self: Box<Self>,
        node_ref: NodeRef,
        parent_scope: &AnyScope,
        parent: Element,
        next_sibling: NodeRef,
    ) -> Box<dyn Scoped>;
    fn reuse(self: Box<Self>, node_ref: NodeRef, scope: &dyn Scoped, next_sibling: NodeRef);

    #[cfg(feature = "ssr")]
    fn render_to_string<'a>(
        &'a self,
        w: &'a mut String,
        parent_scope: &'a AnyScope,
    ) -> LocalBoxFuture<'a, ()>;
}

pub struct PropsWrapper<COMP: BaseComponent> {
    props: Rc<COMP::Properties>,
}

impl<COMP: BaseComponent> PropsWrapper<COMP> {
    pub fn new(props: Rc<COMP::Properties>) -> Self {
        Self { props }
    }
}

impl<COMP: BaseComponent> Mountable for PropsWrapper<COMP> {
    fn copy(&self) -> Box<dyn Mountable> {
        let wrapper: PropsWrapper<COMP> = PropsWrapper {
            props: Rc::clone(&self.props),
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
        scope.mount_in_place(parent, next_sibling, node_ref, self.props);

        Box::new(scope)
    }

    fn reuse(self: Box<Self>, node_ref: NodeRef, scope: &dyn Scoped, next_sibling: NodeRef) {
        let scope: Scope<COMP> = scope.to_any().downcast();
        scope.reuse(self.props, node_ref, next_sibling);
    }

    #[cfg(feature = "ssr")]
    fn render_to_string<'a>(
        &'a self,
        w: &'a mut String,
        parent_scope: &'a AnyScope,
    ) -> LocalBoxFuture<'a, ()> {
        async move {
            let scope: Scope<COMP> = Scope::new(Some(parent_scope.clone()));
            scope.render_to_string(w, self.props.clone()).await;
        }
        .boxed_local()
    }
}

impl DomBundle for BComp {
    fn detach(self, _parent: &Element) {
        self.scope.destroy_boxed();
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) {
        self.scope.shift_node(next_parent.clone(), next_sibling);
    }
}

impl Reconcilable for VComp {
    type Bundle = BComp;

    fn attach(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let VComp {
            type_id,
            mountable,
            node_ref,
            key,
        } = self;

        let scope = mountable.mount(
            node_ref.clone(),
            parent_scope,
            parent.to_owned(),
            next_sibling,
        );

        (
            node_ref.clone(),
            BComp {
                type_id,
                node_ref,
                key,
                scope,
            },
        )
    }

    fn reconcile(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        let bcomp = match bundle {
            // If the existing bundle is the same type, reuse it and update its properties
            BNode::BComp(ref mut bcomp)
                if self.type_id == bcomp.type_id && self.key == bcomp.key =>
            {
                bcomp
            }
            _ => {
                return self.replace(parent_scope, parent, next_sibling, bundle);
            }
        };
        let VComp {
            mountable,
            node_ref,
            key,
            type_id: _,
        } = self;
        bcomp.key = key;
        let old_ref = std::mem::replace(&mut bcomp.node_ref, node_ref.clone());
        bcomp.node_ref.reuse(old_ref);
        mountable.reuse(node_ref.clone(), bcomp.scope.borrow(), next_sibling);
        node_ref
    }
}
