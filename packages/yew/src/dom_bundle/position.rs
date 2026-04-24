//! Structs for keeping track where in the DOM a node belongs

use std::cell::RefCell;
use std::rc::Rc;

use web_sys::{Element, Node};

/// A position in the list of children of an implicit parent [`Element`].
///
/// This can either be in front of a `DomSlot::at(next_sibling)`, at the end of the list with
/// `DomSlot::at_end()`, or a dynamic position in the list with [`DynamicDomSlot::to_position`].
#[derive(Clone)]
pub(crate) struct DomSlot {
    variant: DomSlotVariant,
}

#[derive(Clone)]
enum DomSlotVariant {
    Node(Option<Node>),
    Chained(DynamicDomSlot),
}

/// A dynamic dom slot can be reassigned. This change is also seen by the [`DomSlot`] from
/// [`Self::to_position`] before the reassignment took place.
#[derive(Clone)]
pub(crate) struct DynamicDomSlot {
    target: Rc<RefCell<DomSlot>>,
}

impl std::fmt::Debug for DomSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.with_next_sibling(|n| {
            let formatted_node = match n {
                None => None,
                Some(n) if trap_impl::is_trap(n) => Some("<not yet initialized />".to_string()),
                Some(n) => Some(crate::utils::print_node(n)),
            };
            write!(f, "DomSlot {{ next_sibling: {formatted_node:?} }}")
        })
    }
}

impl std::fmt::Debug for DynamicDomSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", *self.target.borrow())
    }
}

mod trap_impl {
    use super::Node;
    #[cfg(debug_assertions)]
    thread_local! {
        // A special marker element that should not be referenced
        static TRAP: Node = gloo::utils::document().create_element("div").unwrap().into();
    }
    /// Get a "trap" node, or None if compiled without debug_assertions
    #[cfg(feature = "hydration")]
    pub fn get_trap_node() -> Option<Node> {
        #[cfg(debug_assertions)]
        {
            TRAP.with(|trap| Some(trap.clone()))
        }
        #[cfg(not(debug_assertions))]
        {
            None
        }
    }
    #[inline]
    pub fn is_trap(node: &Node) -> bool {
        #[cfg(debug_assertions)]
        {
            TRAP.with(|trap| node == trap)
        }
        #[cfg(not(debug_assertions))]
        {
            // When not running with debug_assertions, there is no trap node
            let _ = node;
            false
        }
    }
}

impl DomSlot {
    /// Denotes the position just before the given node in its parent's list of children.
    pub fn at(next_sibling: Node) -> Self {
        Self::create(Some(next_sibling))
    }

    /// Denotes the position at the end of a list of children. The parent is implicit.
    pub fn at_end() -> Self {
        Self::create(None)
    }

    pub fn create(next_sibling: Option<Node>) -> Self {
        Self {
            variant: DomSlotVariant::Node(next_sibling),
        }
    }

    /// A new "placeholder" [DomSlot] that should not be used to insert nodes
    #[inline]
    #[cfg(feature = "hydration")]
    pub fn new_debug_trapped() -> Self {
        Self::create(trap_impl::get_trap_node())
    }

    /// Get the [Node] that comes just after the position, or `None` if this denotes the position at
    /// the end
    fn with_next_sibling_check_trap<R>(&self, f: impl FnOnce(Option<&Node>) -> R) -> R {
        let checkedf = |node: Option<&Node>| {
            assert!(
                node.is_none_or(|node| !trap_impl::is_trap(node)),
                "Should not use a trapped DomSlot. Please report this as an internal bug in yew."
            );
            f(node)
        };
        self.with_next_sibling(checkedf)
    }

    fn with_next_sibling<R>(&self, f: impl FnOnce(Option<&Node>) -> R) -> R {
        match &self.variant {
            DomSlotVariant::Node(n) => f(n.as_ref()),
            DomSlotVariant::Chained(chain) => chain.with_next_sibling(f),
        }
    }

    /// Insert a [Node] at the position denoted by this slot. `parent` must be the actual parent
    /// element of the children that this slot is implicitly a part of.
    pub(super) fn insert(&self, parent: &Element, node: &Node) {
        self.with_next_sibling_check_trap(|next_sibling: Option<&Node>| {
            parent
                .insert_before(node, next_sibling)
                .unwrap_or_else(|err| {
                    let msg = if next_sibling.is_some() {
                        "failed to insert node before next sibling"
                    } else {
                        "failed to append child"
                    };
                    // Log normally, so we can inspect the nodes in console
                    gloo::console::error!(msg, err, parent, next_sibling, node);
                    // Log via tracing for consistency
                    tracing::error!(msg);
                    // Panic to short-circuit and fail
                    panic!("{}", msg)
                });
        });
    }

    #[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
    #[cfg(test)]
    fn get(&self) -> Option<Node> {
        self.with_next_sibling(|n| n.cloned())
    }
}

impl DynamicDomSlot {
    /// Create a dynamic dom slot that initially represents ("targets") the same slot as the
    /// argument.
    pub fn new(initial_position: DomSlot) -> Self {
        Self {
            target: Rc::new(RefCell::new(initial_position)),
        }
    }

    #[cfg(feature = "hydration")]
    pub fn new_debug_trapped() -> Self {
        Self::new(DomSlot::new_debug_trapped())
    }

    /// Move out of self, leaving behind a trapped slot. `self` should not be used afterwards.
    /// Used during the transition from a hydrating to a rendered component to move state between
    /// enum variants.
    #[cfg(feature = "hydration")]
    pub fn take(&mut self) -> Self {
        std::mem::replace(self, Self::new(DomSlot::new_debug_trapped()))
    }

    /// Change the [`DomSlot`] that is targeted. Subsequently, this will behave as if `self` was
    /// created from the passed DomSlot in the first place.
    pub fn reassign(&self, next_position: DomSlot) {
        // TODO: is not defensive against accidental reference loops
        *self.target.borrow_mut() = next_position;
    }

    /// Get a [`DomSlot`] that gets automatically updated when `self` gets reassigned. All such
    /// slots are equivalent to each other and point to the same position.
    pub fn to_position(&self) -> DomSlot {
        DomSlot {
            variant: DomSlotVariant::Chained(self.clone()),
        }
    }

    fn with_next_sibling<R>(&self, f: impl FnOnce(Option<&Node>) -> R) -> R {
        // We use an iterative approach to traverse a possible long chain of references.
        // See issue #3043 for why a recursive call is impossible for large lists in vdom.
        //
        // TODO: there could be some data structure that performs better here. E.g. a balanced tree
        // with parent pointers come to mind, but they are a bit fiddly to implement in rust
        //
        // We traverse via raw pointers to avoid Rc refcount overhead (clone + drop) per hop, then
        // clone the terminal next-sibling out of the chain before invoking `f`. Invoking `f` with
        // no borrow held and no reliance on chain structure keeps the traversal sound: `f` runs
        // arbitrary code (panic drop glue, `gloo::console::error`, tracing subscribers) that
        // could, in principle, reassign a link in the chain and drop the last strong reference
        // to the RefCell we would otherwise still borrow from.
        //
        // SAFETY: All RefCells visited by the loop remain live while we dereference them:
        // - `self.target` (Rc) is alive because `self` is borrowed
        // - Each DomSlot::Chained(DynamicDomSlot { target }) in the chain holds a strong Rc to the
        //   next RefCell, so all links are transitively kept alive
        // - Yew is single-threaded and the loop body does not run user code, so no mutable borrow
        //   (e.g. from reassign()) can occur on any RefCell in the chain during traversal
        // - Each RefCell::borrow() is dropped before advancing to the next hop
        let node: Option<Node> = {
            let mut ptr: *const RefCell<DomSlot> = Rc::as_ptr(&self.target);
            loop {
                let cell = unsafe { &*ptr };
                let slot_ref = cell.borrow();
                match &slot_ref.variant {
                    DomSlotVariant::Node(n) => break n.clone(),
                    DomSlotVariant::Chained(chain) => {
                        ptr = Rc::as_ptr(&chain.target);
                    }
                }
            }
        };
        f(node.as_ref())
    }
}

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
#[cfg(test)]
mod layout_tests {
    use gloo::utils::document;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn new_at_and_get() {
        let node = document().create_element("p").unwrap();
        let position = DomSlot::at(node.clone().into());
        assert_eq!(
            position.get().unwrap(),
            node.clone().into(),
            "expected the DomSlot to be at {node:#?}"
        );
    }

    #[test]
    fn new_at_end_and_get() {
        let position = DomSlot::at_end();
        assert!(
            position.get().is_none(),
            "expected the DomSlot to not have a next sibling"
        );
    }

    #[test]
    fn get_through_dynamic() {
        let original = DomSlot::at(document().create_element("p").unwrap().into());
        let target = DynamicDomSlot::new(original.clone());
        assert_eq!(
            target.to_position().get(),
            original.get(),
            "expected {target:#?} to point to the same position as {original:#?}"
        );
    }

    #[test]
    fn get_after_reassign() {
        let target = DynamicDomSlot::new(DomSlot::at_end());
        let target_pos = target.to_position();
        // We reassign *after* we called `to_position` here to be strict in the test
        let replacement = DomSlot::at(document().create_element("p").unwrap().into());
        target.reassign(replacement.clone());
        assert_eq!(
            target_pos.get(),
            replacement.get(),
            "expected {target:#?} to point to the same position as {replacement:#?}"
        );
    }

    #[test]
    fn get_chain_after_reassign() {
        let middleman = DynamicDomSlot::new(DomSlot::at_end());
        let target = DynamicDomSlot::new(middleman.to_position());
        let target_pos = target.to_position();
        assert!(
            target.to_position().get().is_none(),
            "should not yet point to a node"
        );
        // Now reassign the middle man, but get the node from `target`
        let replacement = DomSlot::at(document().create_element("p").unwrap().into());
        middleman.reassign(replacement.clone());
        assert_eq!(
            target_pos.get(),
            replacement.get(),
            "expected {target:#?} to point to the same position as {replacement:#?}"
        );
    }

    #[test]
    fn debug_printing() {
        // basic tests that these don't panic. We don't enforce any specific format.
        println!("At end: {:?}", DomSlot::at_end());
        println!("Trapped: {:?}", DomSlot::new_debug_trapped());
        println!(
            "At element: {:?}",
            DomSlot::at(document().create_element("p").unwrap().into())
        );
    }
}
