//! Per-subtree state of apps

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::{intern, JsCast, UnwrapThrowExt};
use web_sys::{
    AddEventListenerOptions, Element, Event, EventTarget as HtmlEventTarget, ShadowRoot,
};

use super::{test_log, Registry};
use crate::virtual_dom::{Listener, ListenerKind};

/// DOM-Types that capture (bubbling) events. This generally includes event targets,
/// but also subtree roots.
pub trait EventGrating {
    fn subtree_id(&self) -> Option<TreeId>;
    fn set_subtree_id(&self, tree_id: TreeId);
    // When caching, we key on the length of the `composed_path`. Important to check
    // considering event retargeting!
    fn cache_key(&self) -> Option<u32>;
    fn set_cache_key(&self, key: u32);
}

#[wasm_bindgen]
extern "C" {
    // Duck-typing, not a real class on js-side. On rust-side, use impls of EventGrating below
    type EventTargetable;
    #[wasm_bindgen(method, getter = __yew_subtree_id, structural)]
    fn subtree_id(this: &EventTargetable) -> Option<TreeId>;
    #[wasm_bindgen(method, setter = __yew_subtree_id, structural)]
    fn set_subtree_id(this: &EventTargetable, id: TreeId);
    #[wasm_bindgen(method, getter = __yew_subtree_cache_key, structural)]
    fn cache_key(this: &EventTargetable) -> Option<u32>;
    #[wasm_bindgen(method, setter = __yew_subtree_cache_key, structural)]
    fn set_cache_key(this: &EventTargetable, key: u32);
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
                fn cache_key(&self) -> Option<u32> {
                    self.unchecked_ref::<EventTargetable>().cache_key()
                }
                fn set_cache_key(&self, key: u32) {
                    self.unchecked_ref::<EventTargetable>().set_cache_key(key)
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
type TreeId = u32;

/// Special id for caching the fact that some event should not be handled
static NONE_TREE_ID: TreeId = 0;
static NEXT_ROOT_ID: AtomicU32 = AtomicU32::new(1);

fn next_root_id() -> TreeId {
    NEXT_ROOT_ID.fetch_add(1, Ordering::SeqCst)
}

/// Data kept per controlled subtree. [Portal] and [AppHandle] serve as
/// hosts. Two controlled subtrees should never overlap.
///
/// [Portal]: super::bportal::BPortal
/// [AppHandle]: super::app_handle::AppHandle
#[derive(Debug, Clone)]
pub struct BSubtree(Rc<SubtreeData>);

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

// FIXME: this is a reproduction of gloo's EventListener to work around #2989
// change back to gloo's implementation once it has been decided how to fix this upstream
// The important part is that we use `Fn` instead of `FnMut` below!
type EventClosure = Closure<dyn Fn(&Event)>;
#[derive(Debug)]
#[must_use = "event listener will never be called after being dropped"]
struct EventListener {
    target: HtmlEventTarget,
    event_type: Cow<'static, str>,
    callback: Option<EventClosure>,
}

impl Drop for EventListener {
    #[inline]
    fn drop(&mut self) {
        if let Some(ref callback) = self.callback {
            self.target
                .remove_event_listener_with_callback_and_bool(
                    &self.event_type,
                    callback.as_ref().unchecked_ref(),
                    true, // Always capture
                )
                .unwrap_throw();
        }
    }
}

impl EventListener {
    fn new(
        target: &HtmlEventTarget,
        desc: &EventDescriptor,
        callback: impl 'static + Fn(&Event),
    ) -> Self {
        let event_type = desc.kind.type_name();

        let callback = Closure::wrap(Box::new(callback) as Box<dyn Fn(&Event)>);
        // defaults: { once: false }
        let mut options = AddEventListenerOptions::new();
        options.capture(true).passive(desc.passive);

        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                intern(&event_type),
                callback.as_ref().unchecked_ref(),
                &options,
            )
            .unwrap_throw();

        EventListener {
            target: target.clone(),
            event_type,
            callback: Some(callback),
        }
    }

    #[cfg(not(test))]
    fn forget(mut self) {
        if let Some(callback) = self.callback.take() {
            // Should always match, but no need to introduce a panic path here
            callback.forget();
        }
    }
}

/// Ensures event handler registration.
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

    fn add_listener(&mut self, desc: &EventDescriptor, callback: impl 'static + Fn(&Event)) {
        let cl = EventListener::new(&self.host, desc, callback);

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
/// This function should be called before any component is mounted.
#[cfg(feature = "csr")]
pub fn set_event_bubbling(bubble: bool) {
    BUBBLE_EVENTS.store(bubble, Ordering::Relaxed);
}

struct BrandingSearchResult {
    branding: TreeId,
    closest_branded_ancestor: Element,
}

fn shadow_aware_parent(el: &Element) -> Option<Element> {
    match el.parent_element() {
        s @ Some(_) => s,
        None => el.parent_node()?.dyn_ref::<ShadowRoot>().map(|h| h.host()),
    }
}

/// Deduce the subtree an element is part of. This already partially starts the bubbling
/// process, as long as no listeners are encountered.
/// Subtree roots are always branded with their own subtree id.
fn find_closest_branded_element(mut el: Element, do_bubble: bool) -> Option<BrandingSearchResult> {
    if !do_bubble {
        let branding = el.subtree_id()?;
        Some(BrandingSearchResult {
            branding,
            closest_branded_ancestor: el,
        })
    } else {
        let responsible_tree_id = loop {
            if let Some(tree_id) = el.subtree_id() {
                break tree_id;
            }
            el = shadow_aware_parent(&el)?;
        };
        Some(BrandingSearchResult {
            branding: responsible_tree_id,
            closest_branded_ancestor: el,
        })
    }
}

/// Iterate over all potentially listening elements in bubbling order.
/// If bubbling is turned off, yields at most a single element.
fn start_bubbling_from(
    subtree: &SubtreeData,
    root_or_listener: Element,
    should_bubble: bool,
) -> impl '_ + Iterator<Item = (&'_ SubtreeData, Element)> {
    let start = subtree.bubble_to_inner_element(root_or_listener, should_bubble);

    std::iter::successors(start, move |(subtree, element)| {
        if !should_bubble {
            return None;
        }
        let parent = shadow_aware_parent(element)?;
        subtree.bubble_to_inner_element(parent, true)
    })
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
    fn bubble_to_inner_element(
        &self,
        parent_el: Element,
        should_bubble: bool,
    ) -> Option<(&Self, Element)> {
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

    fn start_bubbling_if_responsible<'s>(
        &'s self,
        event: &'s Event,
    ) -> Option<impl 's + Iterator<Item = (&'s SubtreeData, Element)>> {
        // Note: the event is not necessarily indentically the same object for all installed
        // handlers hence this cache can be unreliable. Hence the cached repsonsible_tree_id
        // might be missing. On the other hand, due to event retargeting at shadow roots,
        // the cache might be wrong! Keep in mind that we handle events in the capture
        // phase, so top-down. When descending and retargeting into closed shadow-dom, the
        // event might have been handled 'prematurely'. TODO: figure out how to prevent this
        // and establish correct event handling for closed shadow root. Note: Other
        // frameworks also get this wrong and dispatch such events multiple times.
        let event_path = event.composed_path();
        let derived_cached_key = event_path.length();
        let cached_branding = if matches!(event.cache_key(), Some(cache_key) if cache_key == derived_cached_key)
        {
            event.subtree_id()
        } else {
            None
        };
        if matches!(cached_branding, Some(responsible_tree_id) if responsible_tree_id != self.subtree_id)
        {
            // some other handler has determined (via this function, but other `self`) a subtree
            // that is responsible for handling this event, and it's not this subtree.
            return None;
        }
        // We're tasked with finding the subtree that is reponsible with handling the event, and/or
        // run the handling if that's `self`.
        let target = event_path.get(0).dyn_into::<Element>().ok()?;
        let should_bubble = BUBBLE_EVENTS.load(Ordering::Relaxed) && event.bubbles();
        // We say that the most deeply nested subtree is "responsible" for handling the event.
        let (responsible_tree_id, bubbling_start) = if let Some(branding) = cached_branding {
            (branding, target.clone())
        } else if let Some(branding) = find_closest_branded_element(target.clone(), should_bubble) {
            let BrandingSearchResult {
                branding,
                closest_branded_ancestor,
            } = branding;
            event.set_subtree_id(branding);
            event.set_cache_key(derived_cached_key);
            (branding, closest_branded_ancestor)
        } else {
            // Possible only? if bubbling is disabled
            // No tree should handle this event
            event.set_subtree_id(NONE_TREE_ID);
            event.set_cache_key(derived_cached_key);
            return None;
        };
        if self.subtree_id != responsible_tree_id {
            return None;
        }
        if self.host.eq(&target) {
            // One more special case: don't handle events that get fired directly on a subtree host
            return None;
        }
        Some(start_bubbling_from(self, bubbling_start, should_bubble))
        // # More details: When nesting occurs
        //
        // Event listeners are installed only on the subtree roots. Still, those roots can
        // nest. This could lead to events getting handled multiple times. We want event handling to
        // start at the most deeply nested subtree.
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
    fn handle(&self, desc: EventDescriptor, event: Event) {
        let run_handler = |root: &Self, el: &Element| {
            let handler = Registry::get_handler(root.event_registry(), el, &desc);
            if let Some(handler) = handler {
                handler(&event)
            }
        };
        if let Some(bubbling_it) = self.start_bubbling_if_responsible(&event) {
            test_log!("Running handler on subtree {}", self.subtree_id);
            for (subtree, el) in bubbling_it {
                if event.cancel_bubble() {
                    break;
                }
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
        let root = BSubtree(shared_inner);
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

    /// Ensure the event described is handled on all subtrees
    pub fn ensure_handled(&self, desc: &EventDescriptor) {
        self.0.app_data.borrow_mut().ensure_handled(desc);
    }

    /// Run f with access to global Registry
    #[inline]
    pub fn with_listener_registry<R>(&self, f: impl FnOnce(&mut Registry) -> R) -> R {
        f(&mut self.0.event_registry().borrow_mut())
    }

    pub fn brand_element(&self, el: &dyn EventGrating) {
        el.set_subtree_id(self.0.subtree_id);
    }
}
