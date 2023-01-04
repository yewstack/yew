use std::cell::RefCell;
use std::rc::Rc;

use web_sys::{Element, Node};

#[derive(Clone)]
pub struct RetargetableDomSlot {
    target: Rc<RefCell<DomSlot>>,
}

#[derive(Clone)]
enum DomSlotVariant {
    Node(Option<Node>),
    Chained(RetargetableDomSlot),
}

/// Encode the position between two children of a dom node.
#[derive(Clone)]
pub struct DomSlot {
    variant: DomSlotVariant,
}

impl std::fmt::Debug for DomSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.with_next_sibling(|n| {
            write!(
                f,
                "DomSlot {{ next_sibling: {:?} }}",
                n.map(crate::utils::print_node)
            )
        })
    }
}

impl std::fmt::Debug for RetargetableDomSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", *self.target.borrow())
    }
}

#[cfg(debug_assertions)]
thread_local! {
    // A special marker element that should not be referenced
    static TRAP: Node = gloo::utils::document().create_element("div").unwrap().into();
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
    pub fn new_debug_trapped() -> Self {
        #[cfg(debug_assertions)]
        {
            Self::at(TRAP.with(|trap| trap.clone()))
        }
        #[cfg(not(debug_assertions))]
        {
            Self::at_end()
        }
    }

    /// Get the [Node] that comes just after the position, or `None` if this denotes the position at
    /// the end
    fn with_next_sibling<R>(&self, f: impl FnOnce(Option<&Node>) -> R) -> R {
        let checkedf = |node: Option<&Node>| {
            #[cfg(debug_assertions)]
            TRAP.with(|trap| {
                assert!(
                    node != Some(trap),
                    "Should not use a trapped DomSlot. Please report this as an internal bug in \
                     yew."
                )
            });
            f(node)
        };

        match &self.variant {
            DomSlotVariant::Node(ref n) => checkedf(n.as_ref()),
            DomSlotVariant::Chained(ref chain) => chain.with_next_sibling(checkedf),
        }
    }

    /// Insert a [Node] at the position denoted by this slot. `parent` must be the actual parent
    /// element of the children that this slot is implicitly a part of.
    pub(super) fn insert(&self, parent: &Element, node: &Node) {
        self.with_next_sibling(|next_sibling| {
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
                    // Panic to short-curcuit and fail
                    panic!("{}", msg)
                });
        });
    }

    #[cfg(target_arch = "wasm32")]
    #[cfg(test)]
    fn get(&self) -> Option<Node> {
        self.with_next_sibling(|n| n.cloned())
    }
}

impl RetargetableDomSlot {
    pub fn new(initial_position: DomSlot) -> Self {
        Self {
            target: Rc::new(RefCell::new(initial_position)),
        }
    }

    pub fn new_debug_trapped() -> Self {
        Self::new(DomSlot::new_debug_trapped())
    }

    /// Change the [`DomSlot`] that is targeted. Getting the node from previously obtained
    /// positions from [`Self::to_position`] will subsequently reflect the result of
    /// `next_position.get()`.
    pub fn retarget(&self, next_position: DomSlot) {
        // TODO: is not defensive against accidental reference loops
        *self.target.borrow_mut() = next_position;
    }

    /// Get a [`DomSlot`] that gets automatically updated when `self` gets retargeted.
    pub fn to_position(&self) -> DomSlot {
        DomSlot {
            variant: DomSlotVariant::Chained(self.clone()),
        }
    }

    fn with_next_sibling<R>(&self, f: impl FnOnce(Option<&Node>) -> R) -> R {
        // we use an iterative approach to traverse a possible long chain for references
        // see for example issue #3043 why a recursive call is impossible for large lists in vdom

        // TODO: there could be some data structure that performs better here. E.g. a balanced tree
        // with parent pointers come to mind, but they are a bit fiddly to implement in rust
        let mut this = self.target.clone();
        loop {
            //                          v------- borrow lives for this match expression
            let next_this = match &this.borrow().variant {
                DomSlotVariant::Node(ref n) => break f(n.as_ref()),
                // We clone an Rc here temporarily, so that we don't have to consume stack
                // space. The alternative would be to keep the
                // `Ref<'_, DomSlot>` above in some temporary buffer
                DomSlotVariant::Chained(ref chain) => chain.target.clone(),
            };
            this = next_this;
        }
    }
}

#[cfg(target_arch = "wasm32")]
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
    fn get_through_retargetable() {
        let original = DomSlot::at(document().create_element("p").unwrap().into());
        let target = RetargetableDomSlot::new(original.clone());
        assert_eq!(
            target.to_position().get(),
            original.get(),
            "expected {target:#?} to point to the same position as {original:#?}"
        );
    }

    #[test]
    fn get_after_retarget() {
        let target = RetargetableDomSlot::new(DomSlot::at_end());
        let target_pos = target.to_position();
        // We retarget *after* we called `to_position` here to be strict in the test
        let replacement = DomSlot::at(document().create_element("p").unwrap().into());
        target.retarget(replacement.clone());
        assert_eq!(
            target_pos.get(),
            replacement.get(),
            "expected {target:#?} to point to the same position as {replacement:#?}"
        );
    }

    #[test]
    fn get_chain_after_retarget() {
        let middleman = RetargetableDomSlot::new(DomSlot::at_end());
        let target = RetargetableDomSlot::new(middleman.to_position());
        let target_pos = target.to_position();
        assert!(
            target.to_position().get().is_none(),
            "should not yet point to a node"
        );
        // Now retarget the middle man, but get the node from `target`
        let replacement = DomSlot::at(document().create_element("p").unwrap().into());
        middleman.retarget(replacement.clone());
        assert_eq!(
            target_pos.get(),
            replacement.get(),
            "expected {target:#?} to point to the same position as {replacement:#?}"
        );
    }
}
