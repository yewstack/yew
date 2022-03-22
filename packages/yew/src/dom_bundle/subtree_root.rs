//! Per-subtree state of apps

use super::{test_log, Registry};
use crate::virtual_dom::{Listener, ListenerKind};
use gloo::events::{EventListener, EventListenerOptions, EventListenerPhase};
use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, EventTarget as HtmlEventTarget};

/// DOM-Types that capture (bubbling) events. This generally includes event targets,
/// but also subtree roots.
pub trait EventGrating {
    fn subtree_id(&self) -> Option<TreeId>;
    fn set_subtree_id(&self, tree_id: TreeId);
}

#[wasm_bindgen]
extern "C" {
    // Duck-typing, not a real class on js-side. On rust-side, use impls of EventGrating below
    type EventTargetable;
    #[wasm_bindgen(method, getter = __yew_subtree_root_id, structural)]
    fn subtree_id(this: &EventTargetable) -> Option<TreeId>;
    #[wasm_bindgen(method, setter = __yew_subtree_root_id, structural)]
    fn set_subtree_id(this: &EventTargetable, id: TreeId);
}

macro_rules! impl_event_grating {
    ($($t:ty);* $(;)?) => {
        $(
            impl EventGrating for $t {
                fn subtree_id(&self) -> Option<TreeId> {
                    self.unchecked_ref::<EventTargetable>().subtree_id()
                }
                fn set_subtree_id(&self, tree_id: TreeId) {
                    self.unchecked_ref::<EventTargetable>()
                        .set_subtree_id(tree_id);
                }
            }
        )*
    }
}

impl_event_grating!(
    HtmlEventTarget;
    Event; // We cache the found subtree id on the event. This should speed up repeated searches
);

/// The TreeId is the additional payload attached to each listening element
/// It identifies the host responsible for the target. Events not matching
/// are ignored during handling
type TreeId = i32;

/// Special id for caching the fact that some event should not be handled
static NONE_TREE_ID: TreeId = 0;
static NEXT_ROOT_ID: AtomicI32 = AtomicI32::new(1);

fn next_root_id() -> TreeId {
    NEXT_ROOT_ID.fetch_add(1, Ordering::SeqCst)
}

/// Data kept per controlled subtree. [Portal] and [AppHandle] serve as
/// hosts. Two controlled subtrees should never overlap.
///
/// [Portal]: super::bportal::BPortal
/// [AppHandle]: super::app_handle::AppHandle
#[derive(Debug, Clone)]
pub struct BSubtree(
    Option<Rc<SubtreeData>>, // None during SSR
);

/// The parent is the logical location where a subtree is mounted
/// Used to bubble events through portals, which are physically somewhere else in the DOM tree
/// but should bubble to logical ancestors in the virtual DOM tree
#[derive(Debug)]
struct ParentingInformation {
    parent_root: Rc<SubtreeData>,
    // Logical parent of the subtree. Might be the host element of another subtree,
    // if mounted as a direct child, or a controlled element.
    mount_element: Element,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct EventDescriptor {
    kind: ListenerKind,
    passive: bool,
}

impl From<&dyn Listener> for EventDescriptor {
    fn from(l: &dyn Listener) -> Self {
        Self {
            kind: l.kind(),
            passive: l.passive(),
        }
    }
}

/// Ensures event handler registration.
//
// Separate struct to DRY, while avoiding partial struct mutability.
#[derive(Debug)]
struct HostHandlers {
    /// The host element where events are registered
    host: HtmlEventTarget,

    /// Keep track of all listeners to drop them on registry drop.
    /// The registry is never dropped in production.
    #[cfg(test)]
    registered: Vec<(ListenerKind, EventListener)>,
}

impl HostHandlers {
    fn new(host: HtmlEventTarget) -> Self {
        Self {
            host,
            #[cfg(test)]
            registered: Vec::default(),
        }
    }

    fn add_listener(&mut self, desc: &EventDescriptor, callback: impl 'static + FnMut(&Event)) {
        let cl = {
            let desc = desc.clone();
            let options = EventListenerOptions {
                phase: EventListenerPhase::Capture,
                passive: desc.passive,
            };
            EventListener::new_with_options(&self.host, desc.kind.type_name(), options, callback)
        };

        // Never drop the closure as this event handler is static
        #[cfg(not(test))]
        cl.forget();
        #[cfg(test)]
        self.registered.push((desc.kind.clone(), cl));
    }
}

/// Per subtree data
#[derive(Debug)]

struct SubtreeData {
    /// Data shared between all trees in an app
    app_data: Rc<RefCell<AppData>>,
    /// Parent subtree
    parent: Option<ParentingInformation>,

    subtree_id: TreeId,
    host: HtmlEventTarget,
    event_registry: RefCell<Registry>,
    global: RefCell<HostHandlers>,
}

#[derive(Debug)]
struct WeakSubtree {
    subtree_id: TreeId,
    weak_ref: Weak<SubtreeData>,
}

impl Hash for WeakSubtree {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.subtree_id.hash(state)
    }
}

impl PartialEq for WeakSubtree {
    fn eq(&self, other: &Self) -> bool {
        self.subtree_id == other.subtree_id
    }
}
impl Eq for WeakSubtree {}

/// Per tree data, shared between all subtrees in the hierarchy
#[derive(Debug, Default)]
struct AppData {
    subtrees: HashSet<WeakSubtree>,
    listening: HashSet<EventDescriptor>,
}

impl AppData {
    fn add_subtree(&mut self, subtree: &Rc<SubtreeData>) {
        for event in self.listening.iter() {
            subtree.add_listener(event);
        }
        self.subtrees.insert(WeakSubtree {
            subtree_id: subtree.subtree_id,
            weak_ref: Rc::downgrade(subtree),
        });
    }
    fn ensure_handled(&mut self, desc: &EventDescriptor) {
        if !self.listening.insert(desc.clone()) {
            return;
        }
        self.subtrees.retain(|subtree| {
            if let Some(subtree) = subtree.weak_ref.upgrade() {
                subtree.add_listener(desc);
                true
            } else {
                false
            }
        })
    }
}

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

struct BrandingSearchResult {
    branding: TreeId,
    closest_branded_ancestor: Element,
}

/// Deduce the subtree an element is part of. This already partially starts the bubbling
/// process, as long as no listeners are encountered.
/// Subtree roots are always branded with their own subtree id.
fn find_closest_branded_element(mut el: Element, do_bubble: bool) -> Option<BrandingSearchResult> {
    if !do_bubble {
        Some(BrandingSearchResult {
            branding: el.subtree_id()?,
            closest_branded_ancestor: el,
        })
    } else {
        let responsible_tree_id = loop {
            if let Some(tree_id) = el.subtree_id() {
                break tree_id;
            }
            el = el.parent_element()?;
        };
        Some(BrandingSearchResult {
            branding: responsible_tree_id,
            closest_branded_ancestor: el,
        })
    }
}

/// Iterate over all potentially listening elements in bubbling order.
/// If bubbling is turned off, yields at most a single element.
struct BubblingIterator<'tree> {
    event: &'tree Event,
    subtree: &'tree Rc<SubtreeData>,
    next_el: Option<Element>,
    should_bubble: bool,
}

impl<'tree> Iterator for BubblingIterator<'tree> {
    type Item = (&'tree Rc<SubtreeData>, Element);

    fn next(&mut self) -> Option<Self::Item> {
        let candidate = self.next_el.take()?;
        let candidate_parent = self.subtree;
        if self.event.cancel_bubble() {
            return None;
        }
        if self.should_bubble {
            if let Some((next_subtree, parent)) = candidate
                .parent_element()
                .and_then(|parent| self.subtree.bubble_to_inner_element(parent, true))
            {
                self.subtree = next_subtree;
                self.next_el = Some(parent);
            }
        }
        Some((candidate_parent, candidate))
    }
}

impl<'tree> BubblingIterator<'tree> {
    fn start_from(
        subtree: &'tree Rc<SubtreeData>,
        root_or_listener: Element,
        event: &'tree Event,
        should_bubble: bool,
    ) -> Self {
        let start = match subtree.bubble_to_inner_element(root_or_listener, should_bubble) {
            Some((subtree, next_el)) => (subtree, Some(next_el)),
            None => (subtree, None),
        };
        Self {
            event,
            subtree: start.0,
            next_el: start.1,
            should_bubble,
        }
    }
}

impl SubtreeData {
    fn new_ref(host_element: &HtmlEventTarget, parent: Option<ParentingInformation>) -> Rc<Self> {
        let tree_root_id = next_root_id();
        let event_registry = Registry::new();
        let host_handlers = HostHandlers::new(host_element.clone());
        let app_data = match parent {
            Some(ref parent) => parent.parent_root.app_data.clone(),
            None => Rc::default(),
        };
        let subtree = Rc::new(SubtreeData {
            parent,
            app_data,

            subtree_id: tree_root_id,
            host: host_element.clone(),
            event_registry: RefCell::new(event_registry),
            global: RefCell::new(host_handlers),
        });
        subtree.app_data.borrow_mut().add_subtree(&subtree);
        subtree
    }

    fn event_registry(&self) -> &RefCell<Registry> {
        &self.event_registry
    }

    fn host_handlers(&self) -> &RefCell<HostHandlers> {
        &self.global
    }

    // Bubble a potential parent until it reaches an internal element
    #[allow(clippy::needless_lifetimes)] // I don't see a way to omit the lifetimes here
    fn bubble_to_inner_element<'s>(
        self: &'s Rc<Self>,
        parent_el: Element,
        should_bubble: bool,
    ) -> Option<(&'s Rc<Self>, Element)> {
        let mut next_subtree = self;
        let mut next_el = parent_el;
        if !should_bubble && next_subtree.host.eq(&next_el) {
            return None;
        }
        while next_subtree.host.eq(&next_el) {
            // we've reached the host, delegate to a parent if one exists
            let parent = next_subtree.parent.as_ref()?;
            next_subtree = &parent.parent_root;
            next_el = parent.mount_element.clone();
        }
        Some((next_subtree, next_el))
    }

    #[allow(clippy::needless_lifetimes)] // I don't see a way to omit the lifetimes here
    fn start_bubbling_if_responsible<'s>(
        self: &'s Rc<Self>,
        event: &'s Event,
    ) -> Option<BubblingIterator<'s>> {
        // Note: the event is not necessarily indentically the same object for all installed handlers
        // hence this cache can be unreliable.
        let cached_responsible_tree_id = event.subtree_id();
        if matches!(cached_responsible_tree_id, Some(responsible_tree_id) if responsible_tree_id != self.subtree_id)
        {
            // some other handler has determined (via this function, but other `self`) a subtree that is
            // responsible for handling this event, and it's not this subtree.
            return None;
        }
        // We're tasked with finding the subtree that is reponsible with handling the event, and/or
        // run the handling if that's `self`.
        let target = event.composed_path().get(0).dyn_into::<Element>().ok()?;
        let should_bubble = BUBBLE_EVENTS.load(Ordering::Relaxed);
        // We say that the most deeply nested subtree is "responsible" for handling the event.
        let (responsible_tree_id, bubbling_start) =
            if let Some(branding) = cached_responsible_tree_id {
                (branding, target)
            } else if let Some(branding) = find_closest_branded_element(target, should_bubble) {
                let BrandingSearchResult {
                    branding,
                    closest_branded_ancestor,
                } = branding;
                event.set_subtree_id(branding);
                (branding, closest_branded_ancestor)
            } else {
                // Possible only? if bubbling is disabled
                // No tree should handle this event
                event.set_subtree_id(NONE_TREE_ID);
                return None;
            };
        if self.subtree_id != responsible_tree_id {
            return None;
        }
        if self.host.eq(&bubbling_start) {
            // One more special case: don't handle events that get fired directly on a subtree host
            return None;
        }
        Some(BubblingIterator::start_from(
            self,
            bubbling_start,
            event,
            should_bubble,
        ))
        // # More details: When nesting occurs
        //
        // Event listeners are installed only on the subtree roots. Still, those roots can
        // nest. This could lead to events getting handled multiple times. We want event handling to start
        // at the most deeply nested subtree.
        //
        // A nested subtree portals into an element that is controlled by the user and rendered
        // with VNode::VRef. We get the following dom nesting:
        //
        // AppRoot > .. > UserControlledVRef > .. > NestedTree(PortalExit) > ..
        // --------------                          ----------------------------
        // The underlined parts of the hierarchy are controlled by Yew.
        //
        // from the following virtual_dom
        // <AppRoot>
        //   {VNode::VRef(<div><div id="portal_target" /></div>)}
        //   {create_portal(<NestedTree />, #portal_target)}
        // </AppRoot>
    }
    /// Handle a global event firing
    fn handle(self: &Rc<Self>, desc: EventDescriptor, event: Event) {
        let run_handler = |root: &Rc<Self>, el: &Element| {
            let handler = Registry::get_handler(root.event_registry(), el, &desc);
            if let Some(handler) = handler {
                handler(&event)
            }
        };
        if let Some(bubbling_it) = self.start_bubbling_if_responsible(&event) {
            test_log!("Running handler on subtree {}", self.subtree_id);
            for (subtree, el) in bubbling_it {
                run_handler(subtree, &el);
            }
        }
    }
    fn add_listener(self: &Rc<Self>, desc: &EventDescriptor) {
        let this = self.clone();
        let listener = {
            let desc = desc.clone();
            move |e: &Event| {
                this.handle(desc.clone(), e.clone());
            }
        };
        self.host_handlers()
            .borrow_mut()
            .add_listener(desc, listener);
    }
}

impl BSubtree {
    fn do_create_root(
        host_element: &HtmlEventTarget,
        parent: Option<ParentingInformation>,
    ) -> Self {
        let shared_inner = SubtreeData::new_ref(host_element, parent);
        let root = BSubtree(Some(shared_inner));
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
        let parent_information = self.0.as_ref().map(|parent_info| ParentingInformation {
            parent_root: parent_info.clone(),
            mount_element: mount_point,
        });
        Self::do_create_root(host_element, parent_information)
    }
    /// Ensure the event described is handled on all subtrees
    pub fn ensure_handled(&self, desc: &EventDescriptor) {
        let inner = self.0.as_deref().expect("Can't access listeners in SSR");
        inner.app_data.borrow_mut().ensure_handled(desc);
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
    pub fn brand_element(&self, el: &dyn EventGrating) {
        let inner = self.0.as_deref().expect("Can't access listeners in SSR");
        el.set_subtree_id(inner.subtree_id);
    }
}
