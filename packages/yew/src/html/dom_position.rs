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

impl DomPosition {
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

        #[cfg(feature = "csr")]
        #[cfg(debug_assertions)]
        feat_csr::TRAP.with(|trap| {
            assert!(
                node.as_ref() != Some(trap),
                "should not use a trapped node ref"
            )
        });
        node
    }
}

#[cfg(feature = "csr")]
mod feat_csr {
    use super::*;

    impl DomPosition {
        pub fn at(node: Node) -> Self {
            Self::create(Some(node))
        }

        pub fn at_end() -> Self {
            Self::create(None)
        }

        pub fn create(node: Option<Node>) -> Self {
            Self {
                variant: DomPositionVariant::Node(node),
            }
        }

        // A new "placeholder" node ref that should not be accessed
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
    }

    #[cfg(debug_assertions)]
    thread_local! {
        // A special marker element that should not be referenced
        pub(super) static TRAP: Node = gloo::utils::document().create_element("div").unwrap().into();
    }

    impl RetargetableDomPosition {
        pub(crate) fn new(initial_position: DomPosition) -> Self {
            Self {
                target: Rc::new(RefCell::new(initial_position)),
            }
        }

        pub(crate) fn new_debug_trapped() -> Self {
            Self::new(DomPosition::new_debug_trapped())
        }

        pub(crate) fn retarget(&self, next_position: DomPosition) {
            *self.target.borrow_mut() = next_position;
        }

        pub(crate) fn as_position(&self) -> DomPosition {
            DomPosition {
                variant: DomPositionVariant::Chained(self.clone()),
            }
        }
    }
}
