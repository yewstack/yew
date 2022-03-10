//! Per-subtree state of apps

use super::{EventDescriptor, Registry};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{Event, EventTarget};

thread_local! {
    /// Global event listener registry
    static GLOBAL: BundleRoot = {
        let event_registry = RefCell::new(Registry::new_global());
        BundleRoot(Rc::new(InnerBundleRoot {
            event_registry: Some(event_registry),
        }))
    }
}

/// Data kept per controlled subtree. [Portal] and [AppHandle] serve as
/// hosts. Two controlled subtrees should never overlap.
///
/// [Portal]: super::bportal::BPortal
/// [AppHandle]: super::app_handle::AppHandle
#[derive(Debug, Clone)]
pub struct BundleRoot(Rc<InnerBundleRoot>);

#[derive(Debug)]

struct InnerBundleRoot {
    /// None only during ssr.
    event_registry: Option<RefCell<Registry>>,
}

impl BundleRoot {
    /// Create a bundle root at the specified host element
    pub fn create_root(_root_element: &EventTarget) -> Self {
        GLOBAL.with(|root| root.clone())
    }
    /// Create a bundle root for ssr
    #[cfg(feature = "ssr")]
    pub fn create_ssr() -> Self {
        BundleRoot(Rc::new(InnerBundleRoot {
            event_registry: None,
        }))
    }

    fn event_registry(&self) -> &RefCell<Registry> {
        self.0
            .event_registry
            .as_ref()
            .expect("can't access event registry during SSR")
    }
    /// Run f with access to global Registry
    #[inline]
    pub fn with_listener_registry<R>(&self, f: impl FnOnce(&mut Registry) -> R) -> R {
        f(&mut *self.event_registry().borrow_mut())
    }
    /// Return a closure that should be installed as an event listener on the root element for a specific
    /// kind of event.
    pub fn event_listener(&self, desc: EventDescriptor) -> impl 'static + FnMut(&Event) {
        move |e: &Event| {
            GLOBAL.with(|root| Registry::handle(root.event_registry(), desc.clone(), e.clone()));
        }
    }
    #[cfg(all(test, feature = "wasm_test"))]
    pub fn clear_global_listeners() {
        GLOBAL.with(|root| *root.event_registry().borrow_mut() = Registry::new_global());
    }
}
