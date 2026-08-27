//! Structs for keeping track where in the DOM a node belongs

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
    Chained(DynamicDomSlotHandle),
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

mod forest;
use forest::{LinkHandle, LinkOwner, with_forest};

/// A dynamic dom slot can be reassigned. This change is also seen by the [`DomSlot`] from
/// [`Self::to_position`] before the reassignment took place.
pub(crate) struct DynamicDomSlot {
    link: LinkOwner,
}

impl std::fmt::Debug for DynamicDomSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:?} -> {:?}", self.link, self.to_position())
    }
}

#[derive(Clone)]
struct DynamicDomSlotHandle {
    link: LinkHandle,
}

mod trap_impl {
    use std::cell::OnceCell;

    use super::{LinkHandle, Node};

    pub struct TrapContext {
        // A special marker element that should not be referenced
        pub trap: Node,
        #[allow(unused)]
        pub handle: OnceCell<LinkHandle>,
    }
    #[cfg(all(debug_assertions, feature = "hydration"))]
    thread_local! {
        static CTX: TrapContext = TrapContext {
            trap: gloo::utils::document().create_element("div").unwrap().into(),
            handle: OnceCell::new(),
        };
    }
    #[inline]
    pub fn with_trap_ctx<R>(f: impl FnOnce(Option<&TrapContext>) -> R) -> R {
        #[cfg(all(debug_assertions, feature = "hydration"))]
        {
            CTX.with(|ctx| f(Some(ctx)))
        }
        #[cfg(not(all(debug_assertions, feature = "hydration")))]
        {
            f(None)
        }
    }
    #[inline]
    pub fn with_trap_ref<R>(f: impl FnOnce(Option<&Node>) -> R) -> R {
        with_trap_ctx(|ctx| f(ctx.map(|ctx| &ctx.trap)))
    }
    #[inline]
    pub fn is_trap(node: &Node) -> bool {
        with_trap_ref(|trap| trap == Some(node))
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
        let link = with_forest(|slots| slots.insert(initial_position.variant));
        Self { link }
    }

    /// Change the [`DomSlot`] that is targeted. Subsequently, this will behave as if `self` was
    /// created from the passed DomSlot in the first place.
    pub fn reassign(&self, next_position: DomSlot) {
        self.clone_to_handle().reassign_unchecked(next_position);
    }

    /// Get a [`DomSlot`] that gets automatically updated when `self` gets reassigned. All such
    /// slots are equivalent to each other and point to the same position.
    pub fn to_position(&self) -> DomSlot {
        self.clone_to_handle().into_position()
    }

    /// There can only be one owner of a dynamic dom slot. Reassigning a dom slot is only allowed
    /// while that owner is still alive. All other accesses (e.g. through DomSlot) are followers
    /// and should only read the value, but never write to it.
    /// This does not imply that access is always serialized! Followers are allowed to write at any
    /// point without prior synchronization, as long as they ensure that the owner is still alive.
    fn clone_to_handle(&self) -> DynamicDomSlotHandle {
        DynamicDomSlotHandle {
            link: self.link.handle(),
        }
    }
}

impl Drop for DynamicDomSlot {
    fn drop(&mut self) {
        with_forest(|links| links.remove(&mut self.link));
    }
}

impl DynamicDomSlotHandle {
    fn into_position(self) -> DomSlot {
        DomSlot {
            variant: DomSlotVariant::Chained(self),
        }
    }

    /// Reassign through a handle. This is only valid if the owning [DynamicDomSlot] is still alive.
    fn reassign_unchecked(&self, next_position: DomSlot) {
        // TODO: is not defensive against accidental reference loops
        with_forest(|forest| {
            forest.reassign(&self.link, next_position.variant);
        });
    }

    fn with_next_sibling<R>(&self, f: impl FnOnce(Option<&Node>) -> R) -> R {
        let node = with_forest(|forest| forest.find_root(&self.link));
        f(node.as_ref())
    }
}

#[cfg(feature = "hydration")]
mod feat_hydration {
    use std::marker::PhantomData;

    use web_sys::Node;

    use super::{DomSlot, DynamicDomSlot, DynamicDomSlotHandle, with_forest};

    #[inline]
    fn with_trap_handle<R>(f: impl FnOnce(Option<DynamicDomSlotHandle>) -> R) -> R {
        super::trap_impl::with_trap_ctx(|ctx| {
            let handle = ctx.map(|ctx| {
                let trap_link = ctx.handle.get_or_init(|| {
                    with_forest(|forest| {
                        let trap_link = forest.insert(DomSlot::at(ctx.trap.clone()).variant);
                        forest.leak(trap_link)
                    })
                });
                DynamicDomSlotHandle {
                    link: trap_link.clone(),
                }
            });
            f(handle)
        })
    }

    fn trapped_position() -> DomSlot {
        with_trap_handle(|handle| match handle {
            Some(handle) => handle.into_position(),
            None => DomSlot::at_end(),
        })
    }

    impl DynamicDomSlot {
        pub fn new_debug_trapped() -> Self {
            Self::new(trapped_position())
        }

        /// Move out of self, leaving behind a trapped slot. `self` should not be used afterwards.
        /// Used during the transition from a hydrating to a rendered component to move state
        /// between enum variants.
        pub fn take(&mut self) -> Self {
            std::mem::replace(self, Self::new_debug_trapped())
        }
    }

    pub struct SlotBulletin<'tree> {
        prev_next_sibling: Option<DynamicDomSlotHandle>,
        _owner: PhantomData<&'tree mut DynamicDomSlot>,
    }
    impl<'tree> SlotBulletin<'tree> {
        pub fn start(slot: &'tree mut DynamicDomSlot) -> Self {
            // We take a follower, but we are sure the owner is alive
            Self {
                prev_next_sibling: Some(slot.clone_to_handle()),
                _owner: PhantomData,
            }
        }

        pub fn new() -> Self {
            Self {
                prev_next_sibling: None,
                _owner: PhantomData,
            }
        }

        fn write(&mut self, pos: DomSlot) {
            if let Some(slot) = &mut self.prev_next_sibling {
                slot.reassign_unchecked(pos);
            }
        }

        pub fn write_at_node(&mut self, node: Node) {
            self.write(DomSlot::at(node));
            self.prev_next_sibling = None;
        }

        // This method does not track that `inner_next_sibling` (which is the owner) lives for
        // lifetime of this call. This must be done by the caller, which puts it somewhere in
        // its component state
        pub fn write_at_comp(&mut self, slot: DomSlot, inner_next_sibling: &DynamicDomSlot) {
            self.write(slot);
            self.prev_next_sibling = Some(inner_next_sibling.clone_to_handle());
        }
    }
    impl Drop for SlotBulletin<'_> {
        fn drop(&mut self) {
            self.write(DomSlot::at_end())
        }
    }
}
#[cfg(feature = "hydration")]
pub(crate) use feat_hydration::SlotBulletin;

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
        println!("Trapped: {:?}", DynamicDomSlot::new_debug_trapped());
        println!(
            "At element: {:?}",
            DomSlot::at(document().create_element("p").unwrap().into())
        );
    }
}
