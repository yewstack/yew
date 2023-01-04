use std::cell::RefCell;
use std::rc::Rc;

use web_sys::Node;

#[derive(Clone)]
pub struct RetargetableDomPosition {
    target: Rc<RefCell<DomPosition>>,
}

#[derive(Clone)]
enum DomPositionVariant {
    Node(Option<Node>),
    Chained(RetargetableDomPosition),
}

/// Encode the position between two children of a dom node.
#[derive(Clone)]
pub struct DomPosition {
    variant: DomPositionVariant,
}

impl std::fmt::Debug for DomPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DomPosition {{ next_sibling: {:?} }}",
            self.get().map(|n| crate::utils::print_node(&n))
        )
    }
}

impl std::fmt::Debug for RetargetableDomPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", *self.target.borrow())
    }
}

#[cfg(debug_assertions)]
thread_local! {
    // A special marker element that should not be referenced
    static TRAP: Node = gloo::utils::document().create_element("div").unwrap().into();
}

impl DomPosition {
    /// Denotes the position just before the given node in its parent's list of children.
    pub fn at(node: Node) -> Self {
        Self::create(Some(node))
    }

    /// Denotes the position at the end of a list of children. The parent is implicit.
    pub fn at_end() -> Self {
        Self::create(None)
    }

    pub fn create(node: Option<Node>) -> Self {
        Self {
            variant: DomPositionVariant::Node(node),
        }
    }

    /// A new "placeholder" node ref that should not be accessed
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
    pub fn get(&self) -> Option<Node> {
        #[allow(clippy::let_and_return)]
        let node = match &self.variant {
            DomPositionVariant::Node(ref n) => n.clone(),
            DomPositionVariant::Chained(ref chain) => chain.get(),
        };

        #[cfg(debug_assertions)]
        TRAP.with(|trap| {
            assert!(
                node.as_ref() != Some(trap),
                "Should not use a trapped node ref. Please report this as an internal bug in yew."
            )
        });
        node
    }
}

impl RetargetableDomPosition {
    pub fn new(initial_position: DomPosition) -> Self {
        Self {
            target: Rc::new(RefCell::new(initial_position)),
        }
    }

    pub fn new_debug_trapped() -> Self {
        Self::new(DomPosition::new_debug_trapped())
    }

    /// Change the [DomPosition] that is targeted. Getting the node from previously obtained
    /// positions from [`Self::as_position`] will subsequently reflect the result of
    /// `next_position.get()`.
    pub fn retarget(&self, next_position: DomPosition) {
        // TODO: is not defensive against accidental reference loops
        *self.target.borrow_mut() = next_position;
    }

    /// Get a [DomPosition] that gets automatically updated when `self` gets retargeted.
    pub fn as_position(&self) -> DomPosition {
        DomPosition {
            variant: DomPositionVariant::Chained(self.clone()),
        }
    }

    fn get(&self) -> Option<Node> {
        // we use an iterative approach to traverse a possible long chain for references
        // see for example issue #3043 why a recursive call is impossible for large lists in vdom

        // TODO: there could be some data structure that performs better here. E.g. a balanced tree
        // with parent pointers come to mind, but they are a bit fiddly to implement in rust
        let mut this = self.target.clone();
        loop {
            //                          v------- borrow lives for this match expression
            let next_this = match &this.borrow().variant {
                DomPositionVariant::Node(ref n) => break n.clone(),
                // We clone an Rc here temporarily, so that we don't have to consume stack
                // space. The alternative would be to keep the
                // `Ref<'_, DomPosition>` above in some temporary buffer
                DomPositionVariant::Chained(ref chain) => chain.target.clone(),
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
        let position = DomPosition::at(node.clone().into());
        assert_eq!(
            position.get().unwrap(),
            node.clone().into(),
            "expected the DomPosition to be at {node:#?}"
        );
    }

    #[test]
    fn new_at_end_and_get() {
        let position = DomPosition::at_end();
        assert!(
            position.get().is_none(),
            "expected the DomPosition to not have a next sibling"
        );
    }

    #[test]
    fn get_through_retargetable() {
        let original = DomPosition::at(document().create_element("p").unwrap().into());
        let target = RetargetableDomPosition::new(original.clone());
        assert_eq!(
            target.as_position().get(),
            original.get(),
            "expected {target:#?} to point to the same position as {original:#?}"
        );
    }

    #[test]
    fn get_after_retarget() {
        let target = RetargetableDomPosition::new(DomPosition::at_end());
        let target_pos = target.as_position();
        // We retarget *after* we called `as_position` here to be strict in the test
        let replacement = DomPosition::at(document().create_element("p").unwrap().into());
        target.retarget(replacement.clone());
        assert_eq!(
            target_pos.get(),
            replacement.get(),
            "expected {target:#?} to point to the same position as {replacement:#?}"
        );
    }

    #[test]
    fn get_chain_after_retarget() {
        let middleman = RetargetableDomPosition::new(DomPosition::at_end());
        let target = RetargetableDomPosition::new(middleman.as_position());
        let target_pos = target.as_position();
        assert!(
            target.as_position().get().is_none(),
            "should not yet point to a node"
        );
        // Now retarget the middle man, but get the node from `target`
        let replacement = DomPosition::at(document().create_element("p").unwrap().into());
        middleman.retarget(replacement.clone());
        assert_eq!(
            target_pos.get(),
            replacement.get(),
            "expected {target:#?} to point to the same position as {replacement:#?}"
        );
    }
}
