use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::Element;

use crate::dom_bundle::bnode::BNode;
use crate::dom_bundle::traits::{Reconcilable, ReconcileTarget};
use crate::dom_bundle::utils::insert_node;
use crate::dom_bundle::BSubtree;
use crate::html::AnyScope;
use crate::virtual_dom::VRaw;
use crate::{AttrValue, NodeRef};

pub struct BRaw {
    pub html: AttrValue,
    reference: NodeRef,
}

impl BRaw {
    fn create_element(html: &str) -> Option<Element> {
        let div: JsValue = gloo::utils::document()
            .create_element("div")
            .unwrap_throw()
            .into();
        let div: web_sys::HtmlElement = div.into();
        let html = html.trim();
        div.set_inner_html(html);
        let children = div.children();
        return if children.length() == 0 {
            None
        } else if children.length() == 1 {
            children.get_with_index(0)
        } else {
            Some(div.into())
        };
    }

    fn detach_bundle(&self, parent: &Element) {
        if let Some(node) = self.reference.cast::<Element>() {
            parent
                .remove_child(&node)
                .expect("failed to remove braw node");
        }
    }
}

impl ReconcileTarget for BRaw {
    fn detach(self, _root: &BSubtree, parent: &Element, _parent_to_detach: bool) {
        self.detach_bundle(parent);
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) -> NodeRef {
        if let Some(node) = self.reference.cast::<Element>() {
            if let Some(parent) = node.parent_node() {
                parent.remove_child(&node).unwrap_throw();
            }

            next_parent
                .insert_before(&node, next_sibling.get().as_ref())
                .unwrap_throw();

            return NodeRef::new(node.into());
        }
        NodeRef::default()
    }
}

impl Reconcilable for VRaw {
    type Bundle = BRaw;

    fn attach(
        self,
        _root: &BSubtree,
        _parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {

        let element = BRaw::create_element(&self.html);
        let node_ref = NodeRef::default();

        if let Some(element) = element {
            insert_node(&element, parent, next_sibling.get().as_ref());
            node_ref.set(Some(element.into()));
        }
        (
            node_ref.clone(),
            BRaw {
                html: self.html,
                reference: node_ref,
            },
        )
    }

    fn reconcile_node(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        // we don't have a way to diff what's changed in the string so we remove the node if it's
        // present and reattach it
        if let BNode::Raw(raw) = bundle {
            raw.detach_bundle(parent)
        }
        let (node_ref, braw) = self.attach(root, parent_scope, parent, next_sibling);
        *bundle = braw.into();
        node_ref
    }

    fn reconcile(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut Self::Bundle,
    ) -> NodeRef {
        // we don't have a way to diff what's changed in the string so we remove the node and
        // reattach it
        bundle.detach_bundle(parent);
        let (node_ref, braw) = self.attach(root, parent_scope, parent, next_sibling);
        *bundle = braw;
        node_ref
    }
}
