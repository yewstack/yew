use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsValue;
use web_sys::Node;

#[derive(Default, Clone)]
pub struct DomPosition(Rc<RefCell<NodeRefInner>>);

impl PartialEq for DomPosition {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr() == other.0.as_ptr() || Some(self) == other.0.borrow().link.as_ref()
    }
}

impl std::fmt::Debug for DomPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeRef {{ references: {:?} }}",
            self.get().map(|n| crate::utils::print_node(&n))
        )
    }
}

#[derive(PartialEq, Debug, Default, Clone)]
struct NodeRefInner {
    node: Option<Node>,
    link: Option<DomPosition>,
}

impl DomPosition {
    /// Get the wrapped Node reference if it exists
    pub fn get(&self) -> Option<Node> {
        let inner = self.0.borrow();
        inner.node.clone().or_else(|| inner.link.as_ref()?.get())
    }

    /// Try converting the node reference into another form
    pub fn cast<INTO: AsRef<Node> + From<JsValue>>(&self) -> Option<INTO> {
        let node = self.get();
        node.map(Into::into).map(INTO::from)
    }

    /// Place a Node in a reference for later use
    pub(crate) fn set(&self, node: Option<Node>) {
        let mut this = self.0.borrow_mut();
        this.node = node;
        this.link = None;
    }
}

#[cfg(feature = "csr")]
mod feat_csr {
    use super::*;

    impl DomPosition {
        /// Link a downstream `NodeRef`
        pub(crate) fn link(&self, node_ref: Self) {
            // Avoid circular references
            if self == &node_ref {
                return;
            }

            let mut this = self.0.borrow_mut();
            this.node = None;
            this.link = Some(node_ref);
        }

        /// Wrap an existing `Node` in a `NodeRef`
        pub(crate) fn new(node: Node) -> Self {
            let node_ref = DomPosition::default();
            node_ref.set(Some(node));
            node_ref
        }
    }
}

#[cfg(feature = "wasm_test")]
#[cfg(test)]
mod tests {
    use gloo_utils::document;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn self_linking_node_ref() {
        let node: Node = document().create_text_node("test node").into();
        let node_ref = DomPosition::new(node.clone());
        let node_ref_2 = DomPosition::new(node.clone());

        // Link to self
        node_ref.link(node_ref.clone());
        assert_eq!(node, node_ref.get().unwrap());

        // Create cycle of two node refs
        node_ref.link(node_ref_2.clone());
        node_ref_2.link(node_ref);
        assert_eq!(node, node_ref_2.get().unwrap());
    }
}
