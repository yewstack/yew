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
        // we use an iterative approach to traverse a possible long chain for references
        // see for example issue #3043 why a recursive call is impossible for large lists in vdom

        // TODO: there could be some data structure that performs better here. E.g. a balanced tree
        // with parent pointers come to mind, but they are a bit fiddly to implement in rust
        let node = match &self.variant {
            DomPositionVariant::Node(ref n) => n.clone(),
            DomPositionVariant::Chained(ref chain) => {
                let mut this = chain.target.clone();
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
        *self.target.borrow_mut() = next_position;
    }

    /// Get a [DomPosition] that gets automatically updated when `self` gets retargeted.
    pub fn as_position(&self) -> DomPosition {
        DomPosition {
            variant: DomPositionVariant::Chained(self.clone()),
        }
    }
}
