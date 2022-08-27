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
        let div = gloo::utils::document().create_element("div").unwrap();
        let html = html.trim();
        div.set_inner_html(html);
        let children = div.children();
        return if children.length() == 0 {
            None
        } else if children.length() == 1 {
            children.get_with_index(0)
        } else {
            tracing::debug!(
                "HTML with more than one root node was passed as raw node. It will be wrapped in \
                 a <div>"
            );
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
                parent.remove_child(&node).unwrap();
            }

            next_parent
                .insert_before(&node, next_sibling.get().as_ref())
                .unwrap();

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
#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    use super::*;
    use crate::virtual_dom::VNode;
    use crate::dom_bundle::utils::setup_parent;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn braw_works_one_node() {
        let (root, scope, parent) = setup_parent();

        const HTML: &str ="<span>text</span>";
        let elem = VNode::from_raw_html(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, NodeRef::default());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), HTML)
    }

    #[test]
    fn braw_works_no_node() {
        let (root, scope, parent) = setup_parent();

        const HTML: &str ="";
        let elem = VNode::from_raw_html(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, NodeRef::default());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), HTML)
    }

    #[test]
    fn braw_works_one_node_nested() {
        let (root, scope, parent) = setup_parent();

        const HTML: &str =r#"<p>one <a href="https://yew.rs">link</a> more paragraph</p>"#;
        let elem = VNode::from_raw_html(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, NodeRef::default());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), HTML)
    }
    #[test]
    fn braw_works_multi_top_nodes() {
        let (root, scope, parent) = setup_parent();

        const HTML: &str =r#"<p>paragraph</p><a href="https://yew.rs">link</a>"#;
        let elem = VNode::from_raw_html(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, NodeRef::default());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), format!("<div>{}</div>", HTML))
    }

    fn assert_braw(node: &mut BNode) -> &mut BRaw {
        if let BNode::Raw(braw) = node {
            return braw;
        }
        panic!("should be braw");
    }
}
