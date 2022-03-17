//! Per-subtree state of apps

use super::{test_log, EventDescriptor, Registry};
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
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

type KnownSubtrees = HashMap<TreeId, Weak<SubtreeData>>;
thread_local! {
    static KNOWN_ROOTS: RefCell<KnownSubtrees> = RefCell::default();
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

// The parent is the logical location where a subtree is mounted
// Used to bubble events through portals, which are physically somewhere else in the DOM tree
// but should bubble to logical ancestors in the virtual DOM tree
#[derive(Debug)]
struct ParentingInformation {
    parent_root: Option<Rc<SubtreeData>>,
    // Logical parent of the subtree. Might be the host element of another subtree,
    // if mounted as a direct child, or a controlled element.
    mount_element: Element,
}

#[derive(Debug)]

struct SubtreeData {
    subtree_id: TreeId,
    host: HtmlEventTarget,
    parent: Option<ParentingInformation>,
    event_registry: RefCell<Registry>,
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

struct SubtreeHierarchyIterator<'tree> {
    current: Option<(&'tree Rc<SubtreeData>, &'tree Element)>,
}

impl<'tree> Iterator for SubtreeHierarchyIterator<'tree> {
    type Item = (&'tree Rc<SubtreeData>, &'tree Element);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current.take()?;
        if let Some(parenting_info) = next.0.parent.as_ref() {
            let parent_root = parenting_info
                .parent_root
                .as_ref()
                .expect("Not in SSR, this shouldn't be None");
            self.current = Some((parent_root, &parenting_info.mount_element));
        }
        Some(next)
    }
}

impl<'tree> SubtreeHierarchyIterator<'tree> {
    fn start_from(subtree: &'tree Rc<SubtreeData>, el: &'tree Element) -> Self {
        Self {
            current: Some((subtree, el)),
        }
    }
}

impl SubtreeData {
    fn new_ref(host_element: &HtmlEventTarget, parent: Option<ParentingInformation>) -> Rc<Self> {
        let tree_root_id = next_root_id();
        let event_registry = Registry::new(host_element.clone());
        let subtree = Rc::new(SubtreeData {
            subtree_id: tree_root_id,
            host: host_element.clone(),
            parent,
            event_registry: RefCell::new(event_registry),
        });
        KNOWN_ROOTS.with(|roots| {
            roots
                .borrow_mut()
                .insert(tree_root_id, Rc::downgrade(&subtree))
        });
        subtree
    }

    fn event_registry(&self) -> &RefCell<Registry> {
        &self.event_registry
    }

    fn find_by_id(tree_id: TreeId) -> Option<Rc<Self>> {
        KNOWN_ROOTS.with(|roots| {
            let mut roots = roots.borrow_mut();
            let subtree = match roots.entry(tree_id) {
                Entry::Occupied(subtree) => subtree,
                _ => return None,
            };
            match subtree.get().upgrade() {
                Some(subtree) => Some(subtree),
                None => {
                    // Remove stale entry
                    subtree.remove();
                    None
                }
            }
        })
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
            let parent_root = parent
                .parent_root
                .as_ref()
                .expect("Not in SSR, this shouldn't be None");
            next_subtree = parent_root;
            next_el = parent.mount_element.clone();
        }
        Some((next_subtree, next_el))
    }

    #[allow(clippy::needless_lifetimes)] // I don't see a way to omit the lifetimes here
    fn start_bubbling_if_responsible<'s>(
        self: &'s Rc<Self>,
        event: &'s Event,
        desc: &'s EventDescriptor,
    ) -> Option<BubblingIterator<'s>> {
        // Note: the event is not necessarily indentically the same object for all installed handlers
        // hence this cache can be unreliable.
        let self_is_responsible = match event.subtree_id() {
            Some(responsible_tree_id) if responsible_tree_id == self.subtree_id => true,
            None => false,
            // some other handler has determined (via this function, but other `self`) a subtree that is
            // responsible for handling this event, and it's not this subtree.
            Some(_) => return None,
        };
        // We're tasked with finding the subtree that is reponsible with handling the event, and/or
        // run the handling if that's `self`. The process is very similar
        let target = event.target()?.dyn_into::<Element>().ok()?;
        let should_bubble = BUBBLE_EVENTS.load(Ordering::Relaxed);
        let BrandingSearchResult {
            branding,
            closest_branded_ancestor,
        } = find_closest_branded_element(target.clone(), should_bubble)?;
        // The branded element can be in a subtree that has no handler installed for the event.
        // We say that the most deeply nested subtree that does have a handler installed is "responsible"
        // for handling the event.
        let (responsible_tree_id, bubble_start) = if branding == self.subtree_id {
            // since we're currently in this handler, `self` has a handler installed and is the most
            // deeply nested one. This usual case saves a look-up in the global KNOWN_ROOTS.
            if self.host.eq(&target) {
                // One more special case: don't handle events that get fired directly on a subtree host
                // but we still want to cache this fact
                (NONE_TREE_ID, closest_branded_ancestor)
            } else {
                (self.subtree_id, closest_branded_ancestor)
            }
        } else {
            // bubble through subtrees until we find one that has a handler installed for the event descriptor
            let target_subtree = Self::find_by_id(branding)
                .expect("incorrectly branded element: subtree already removed");
            if target_subtree.host.eq(&target) {
                (NONE_TREE_ID, closest_branded_ancestor)
            } else {
                let responsible_tree = SubtreeHierarchyIterator::start_from(
                    &target_subtree,
                    &closest_branded_ancestor,
                )
                .find(|(candidate, _)| {
                    if candidate.subtree_id == self.subtree_id {
                        true
                    } else if !self_is_responsible {
                        // only do this check if we aren't sure which subtree is responsible for handling
                        candidate.event_registry().borrow().has_any_listeners(desc)
                    } else {
                        false
                    }
                })
                .expect("nesting error: current subtree should show up in hierarchy");
                (responsible_tree.0.subtree_id, responsible_tree.1.clone())
            }
        };
        event.set_subtree_id(responsible_tree_id); // cache it for other event handlers
        (responsible_tree_id == self.subtree_id)
            .then(|| BubblingIterator::start_from(self, bubble_start, event, should_bubble))
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
        if let Some(bubbling_it) = self.start_bubbling_if_responsible(&event, &desc) {
            test_log!("Running handler on subtree {}", self.subtree_id);
            for (subtree, el) in bubbling_it {
                run_handler(subtree, &el);
            }
        }
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
        el.set_subtree_id(inner.subtree_id);
    }
}
