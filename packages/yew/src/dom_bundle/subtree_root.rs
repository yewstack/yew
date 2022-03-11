//! Per-subtree state of apps

use super::{test_log, EventDescriptor, Registry};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, EventTarget as HtmlEventTarget};

/// Bubble events during delegation
static BUBBLE_EVENTS: AtomicBool = AtomicBool::new(true);

/// Set, if events should bubble up the DOM tree, calling any matching callbacks.
///
/// Bubbling is enabled by default. Disabling bubbling can lead to substantial improvements in event
/// handling performance.
///
/// Note that yew uses event delegation and implements internal even bubbling for performance
/// reasons. Calling `Event.stopPropagation()` or `Event.stopImmediatePropagation()` in the event
/// handler has no effect.
///
/// This function should be called before any component is mounted.
pub fn set_event_bubbling(bubble: bool) {
    BUBBLE_EVENTS.store(bubble, Ordering::Relaxed);
}

// The TreeId is an additional payload attached to each listening element
// It identifies the host responsible for the target. Events not matching
// are ignored during handling
type TreeId = u32;
/// DOM-Types that capture bubbling events. This generally includes event targets,
/// but also subtree roots.
pub trait EventGrating {
    fn responsible_tree_id(&self) -> Option<TreeId>;
    fn set_responsible_tree_id(&self, tree_id: TreeId);
}

#[wasm_bindgen]
extern "C" {
    // Duck-typing, not a real class on js-side. On rust-side, use impls of EventGrating below
    type EventTargetable;
    #[wasm_bindgen(method, getter = __yew_subtree_root_id, structural)]
    fn subtree_id(this: &EventTargetable) -> Option<u32>;
    #[wasm_bindgen(method, setter = __yew_subtree_root_id, structural)]
    fn set_subtree_id(this: &EventTargetable, id: u32);
}

impl EventGrating for Element {
    fn responsible_tree_id(&self) -> Option<TreeId> {
        self.unchecked_ref::<EventTargetable>().subtree_id()
    }
    fn set_responsible_tree_id(&self, tree_id: TreeId) {
        self.unchecked_ref::<EventTargetable>()
            .set_subtree_id(tree_id);
    }
}

impl EventGrating for HtmlEventTarget {
    fn responsible_tree_id(&self) -> Option<TreeId> {
        self.unchecked_ref::<EventTargetable>().subtree_id()
    }
    fn set_responsible_tree_id(&self, tree_id: TreeId) {
        self.unchecked_ref::<EventTargetable>()
            .set_subtree_id(tree_id);
    }
}

/// We cache the found subtree id on the event. This should speed up repeated searches
impl EventGrating for Event {
    fn responsible_tree_id(&self) -> Option<TreeId> {
        self.unchecked_ref::<EventTargetable>().subtree_id()
    }
    fn set_responsible_tree_id(&self, tree_id: TreeId) {
        self.unchecked_ref::<EventTargetable>()
            .set_subtree_id(tree_id);
    }
}

static NEXT_ROOT_ID: AtomicU32 = AtomicU32::new(1); // Skip 0, used for ssr

fn next_root_id() -> TreeId {
    NEXT_ROOT_ID.fetch_add(1, Ordering::SeqCst)
}

/// Data kept per controlled subtree. [Portal] and [AppHandle] serve as
/// hosts. Two controlled subtrees should never overlap.
///
/// [Portal]: super::bportal::BPortal
/// [AppHandle]: super::app_handle::AppHandle
#[derive(Debug, Clone)]
pub struct BSubtree(/* None during SSR */ Option<Rc<InnerBundleRoot>>);

// The parent is the logical location where a subtree is mounted
// Used to bubble events through portals, which are physically somewhere else in the DOM tree
// but should bubble to logical ancestors in the virtual DOM tree
#[derive(Debug)]
struct ParentingInformation {
    parent_root: Option<Rc<InnerBundleRoot>>,
    mount_element: Element,
}

#[derive(Debug)]

struct InnerBundleRoot {
    host: HtmlEventTarget,
    parent: Option<ParentingInformation>,
    tree_root_id: TreeId,
    event_registry: RefCell<Registry>,
}

struct ClosestInstanceSearchResult {
    root_or_listener: Element,
    responsible_tree_id: TreeId,
    did_bubble: bool,
}

/// Deduce the subtree responsible for handling this event. This already
/// partially starts the bubbling process, as long as no listeners are encountered,
/// but stops at subtree roots.
/// Event listeners are installed only on the subtree roots. Still, those roots can
/// nest [1]. This would lead to events getting handled multiple times. We want event
/// handling to start at the most deeply nested subtree.
///
/// # When nesting occurs
/// The nested subtree portals into a element that is controlled by the user and rendered
/// with VNode::VRef. We get the following nesting:
/// AppRoot > .. > UserControlledVRef > .. > NestedTree(PortalExit) > ..
/// --------------                          ----------------------------
/// The underlined parts of the hierarchy are controlled by Yew.
fn find_closest_responsible_instance(event: &Event) -> Option<ClosestInstanceSearchResult> {
    let target = event.target()?.dyn_into::<web_sys::Element>().ok()?;
    if let Some(cached_id) = event.responsible_tree_id() {
        return Some(ClosestInstanceSearchResult {
            root_or_listener: target,
            responsible_tree_id: cached_id,
            did_bubble: false,
        });
    }

    let mut el = target;
    let mut did_bubble = false;
    let responsible_tree_id = loop {
        if let Some(tree_id) = el.responsible_tree_id() {
            break tree_id;
        }
        el = el.parent_element()?;
        did_bubble = true;
    };
    event.set_responsible_tree_id(responsible_tree_id);
    Some(ClosestInstanceSearchResult {
        root_or_listener: el,
        responsible_tree_id,
        did_bubble,
    })
}

impl InnerBundleRoot {
    fn event_registry(&self) -> &RefCell<Registry> {
        &self.event_registry
    }
    /// Handle a global event firing
    fn handle(self: &Rc<Self>, desc: EventDescriptor, event: Event) {
        let closest_instance = match find_closest_responsible_instance(&event) {
            Some(closest_instance) if closest_instance.responsible_tree_id == self.tree_root_id => {
                closest_instance
            }
            _ => return, // Don't handle this event
        };
        test_log!("Running handler on subtree {}", self.tree_root_id);
        if self.host.eq(&closest_instance.root_or_listener) {
            let (self_, target) = match self.bubble_at_root() {
                Some(bubbled_target) => bubbled_target,
                None => return, // No relevant listener
            };
            self_.run_handlers(desc, event, target, true);
        } else {
            let target = closest_instance.root_or_listener;
            let did_bubble = closest_instance.did_bubble;
            self.run_handlers(desc, event, target, did_bubble);
        }
    }

    #[allow(clippy::needless_lifetimes)] // I don't see a way to omit the lifetimes here
    fn bubble_at_root<'s>(self: &'s Rc<Self>) -> Option<(&'s Rc<Self>, Element)> {
        // we've reached the physical host, delegate to a parent if one exists
        let parent = self.parent.as_ref()?;
        let parent_root = parent
            .parent_root
            .as_ref()
            .expect("Can't access listeners in SSR");
        Some((parent_root, parent.mount_element.clone()))
    }

    #[allow(clippy::needless_lifetimes)] // I don't see a way to omit the lifetimes here
    fn bubble<'s>(self: &'s Rc<Self>, el: Element) -> Option<(&'s Rc<Self>, Element)> {
        let parent = el.parent_element()?;
        if self.host.eq(&parent) {
            self.bubble_at_root()
        } else {
            Some((self, parent))
        }
    }

    fn run_handlers(
        self: &Rc<Self>,
        desc: EventDescriptor,
        event: Event,
        closest_target: Element,
        did_bubble: bool, // did bubble to find the closest target?
    ) {
        let run_handler = |root: &Rc<Self>, el: &Element| {
            let handler = Registry::get_handler(root.event_registry(), el, &desc);
            if let Some(handler) = handler {
                handler(&event)
            }
        };

        let should_bubble = BUBBLE_EVENTS.load(Ordering::Relaxed);

        // If we bubbled to find closest_target, respect BUBBLE_EVENTS setting
        if should_bubble || !did_bubble {
            run_handler(self, &closest_target);
        }

        let mut current_root = self;
        if should_bubble {
            let mut el = closest_target;
            while !event.cancel_bubble() {
                let next = match current_root.bubble(el) {
                    Some(next) => next,
                    None => break,
                };
                // Destructuring assignments are unstable
                current_root = next.0;
                el = next.1;

                run_handler(self, &el);
            }
        }
    }
}

impl BSubtree {
    fn do_create_root(
        host_element: &HtmlEventTarget,
        parent: Option<ParentingInformation>,
    ) -> Self {
        let event_registry = Registry::new(host_element.clone());
        let root = BSubtree(Some(Rc::new(InnerBundleRoot {
            host: host_element.clone(),
            parent,
            tree_root_id: next_root_id(),
            event_registry: RefCell::new(event_registry),
        })));
        root.brand_element(host_element);
        root
    }
    /// Create a bundle root at the specified host element
    pub fn create_root(host_element: &HtmlEventTarget) -> Self {
        Self::do_create_root(host_element, None)
    }
    /// Create a bundle root at the specified host element, that is logically
    /// mounted under the specified element in this tree.
    pub fn create_subroot(&self, mount_point: Element, host_element: &HtmlEventTarget) -> Self {
        let parent_information = ParentingInformation {
            parent_root: self.0.clone(),
            mount_element: mount_point,
        };
        Self::do_create_root(host_element, Some(parent_information))
    }
    /// Create a bundle root for ssr
    #[cfg(feature = "ssr")]
    pub fn create_ssr() -> Self {
        BSubtree(None)
    }
    /// Run f with access to global Registry
    #[inline]
    pub fn with_listener_registry<R>(&self, f: impl FnOnce(&mut Registry) -> R) -> R {
        let inner = self.0.as_deref().expect("Can't access listeners in SSR");
        f(&mut *inner.event_registry().borrow_mut())
    }
    /// Return a closure that should be installed as an event listener on the root element for a specific
    /// kind of event.
    pub fn event_listener(&self, desc: EventDescriptor) -> impl 'static + FnMut(&Event) {
        let inner = self.0.clone().expect("Can't access listeners in SSR"); // capture the registry
        move |e: &Event| {
            inner.handle(desc.clone(), e.clone());
        }
    }

    pub fn brand_element(&self, el: &dyn EventGrating) {
        let inner = self.0.as_deref().expect("Can't access listeners in SSR");
        el.set_responsible_tree_id(inner.tree_root_id);
    }
}
