pub mod layout_tests;

use super::Reconcilable;

use crate::virtual_dom::VNode;
use crate::{dom_bundle::BNode, html::AnyScope, NodeRef};
use web_sys::Element;

impl VNode {
    fn reconcile_sequentially(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut Option<BNode>,
    ) -> NodeRef {
        match bundle {
            None => {
                let (self_ref, node) = self.attach(parent_scope, parent, next_sibling);
                *bundle = Some(node);
                self_ref
            }
            Some(bundle) => self.reconcile_node(parent_scope, parent, next_sibling, bundle),
        }
    }
}
