use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

use super::{BNode, BSubtree, DomSlot, Reconcilable, ReconcileTarget};
use crate::html::AnyScope;
use crate::virtual_dom::VRaw;
use crate::AttrValue;

#[derive(Debug)]
pub struct BRaw {
    reference: Option<Node>,
    children_count: usize,
    html: AttrValue,
}

impl BRaw {
    fn create_elements(html: &str) -> Vec<Node> {
        let div = gloo::utils::document().create_element("div").unwrap();
        div.set_inner_html(html);
        let children = div.child_nodes();
        let children = js_sys::Array::from(&children);
        let children = children.to_vec();
        children
            .into_iter()
            .map(|it| it.unchecked_into())
            .collect::<Vec<_>>()
    }

    fn detach_bundle(&self, parent: &Element) {
        let mut next_node = self.reference.clone();
        for _ in 0..self.children_count {
            if let Some(node) = next_node {
                next_node = node.next_sibling();
                parent.remove_child(&node).unwrap();
            }
        }
    }

    fn position(&self, next_slot: DomSlot) -> DomSlot {
        self.reference
            .as_ref()
            .map(|n| DomSlot::at(n.clone()))
            .unwrap_or(next_slot)
    }
}

impl ReconcileTarget for BRaw {
    fn detach(self, _root: &BSubtree, parent: &Element, _parent_to_detach: bool) {
        self.detach_bundle(parent);
    }

    fn shift(&self, next_parent: &Element, slot: DomSlot) -> DomSlot {
        let mut next_node = self.reference.clone();
        for _ in 0..self.children_count {
            if let Some(node) = next_node {
                next_node = node.next_sibling();
                slot.insert(next_parent, &node);
            }
        }
        self.position(slot)
    }
}

impl Reconcilable for VRaw {
    type Bundle = BRaw;

    fn attach(
        self,
        _root: &BSubtree,
        _parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
    ) -> (DomSlot, Self::Bundle) {
        let elements = BRaw::create_elements(&self.html);
        let count = elements.len();
        let mut iter = elements.into_iter();
        let reference = iter.next();
        if let Some(ref first) = reference {
            slot.insert(parent, first);
            for ref child in iter {
                slot.insert(parent, child);
            }
        }
        let this = BRaw {
            reference,
            children_count: count,
            html: self.html,
        };
        (this.position(slot), this)
    }

    fn reconcile_node(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
        bundle: &mut BNode,
    ) -> DomSlot {
        match bundle {
            BNode::Raw(raw) if raw.html == self.html => raw.position(slot),
            BNode::Raw(raw) => self.reconcile(root, parent_scope, parent, slot, raw),
            _ => self.replace(root, parent_scope, parent, slot, bundle),
        }
    }

    fn reconcile(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
        bundle: &mut Self::Bundle,
    ) -> DomSlot {
        if self.html != bundle.html {
            // we don't have a way to diff what's changed in the string so we remove the node and
            // reattach it
            bundle.detach_bundle(parent);
            let (node_ref, braw) = self.attach(root, parent_scope, parent, slot);
            *bundle = braw;
            node_ref
        } else {
            bundle.position(slot)
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    use gloo::utils::document;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use super::*;
    use crate::dom_bundle::utils::{setup_parent, setup_parent_and_sibling, SIBLING_CONTENT};
    use crate::virtual_dom::VNode;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn braw_works_one_node() {
        let (root, scope, parent) = setup_parent();

        const HTML: &str = "<span>text</span>";
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, DomSlot::at_end());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), HTML)
    }

    #[test]
    fn braw_works_no_node() {
        let (root, scope, parent) = setup_parent();

        const HTML: &str = "";
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, DomSlot::at_end());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), HTML)
    }

    #[test]
    fn braw_works_one_node_nested() {
        let (root, scope, parent) = setup_parent();

        const HTML: &str =
            r#"<p>one <a href="https://yew.rs">link</a> more paragraph</p><div>here</div>"#;
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, DomSlot::at_end());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), HTML)
    }
    #[test]
    fn braw_works_multi_top_nodes() {
        let (root, scope, parent) = setup_parent();

        const HTML: &str = r#"<p>paragraph</p><a href="https://yew.rs">link</a>"#;
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, DomSlot::at_end());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), HTML)
    }

    #[test]
    fn braw_detach_works_multi_node() {
        let (root, scope, parent) = setup_parent();

        const HTML: &str = r#"<p>paragraph</p><a href="https://yew.rs">link</a>"#;
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, DomSlot::at_end());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), HTML);
        elem.detach(&root, &parent, false);
        assert_eq!(parent.inner_html(), "");
    }

    #[test]
    fn braw_detach_works_single_node() {
        let (root, scope, parent) = setup_parent();

        const HTML: &str = r#"<p>paragraph</p>"#;
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, DomSlot::at_end());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), HTML);
        elem.detach(&root, &parent, false);
        assert_eq!(parent.inner_html(), "");
    }

    #[test]
    fn braw_detach_works_empty() {
        let (root, scope, parent) = setup_parent();

        const HTML: &str = "";
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, DomSlot::at_end());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), HTML);
        elem.detach(&root, &parent, false);
        assert_eq!(parent.inner_html(), "");
    }

    #[test]
    fn braw_works_one_node_sibling_attached() {
        let (root, scope, parent, sibling) = setup_parent_and_sibling();

        const HTML: &str = "<span>text</span>";
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, sibling);
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), format!("{}{}", HTML, SIBLING_CONTENT));
    }

    #[test]
    fn braw_works_no_node_sibling_attached() {
        let (root, scope, parent, sibling) = setup_parent_and_sibling();

        const HTML: &str = "";
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, sibling);
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), format!("{}{}", HTML, SIBLING_CONTENT));
    }

    #[test]
    fn braw_works_one_node_nested_sibling_attached() {
        let (root, scope, parent, sibling) = setup_parent_and_sibling();

        const HTML: &str =
            r#"<p>one <a href="https://yew.rs">link</a> more paragraph</p><div>here</div>"#;
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, sibling);
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), format!("{}{}", HTML, SIBLING_CONTENT));
    }
    #[test]
    fn braw_works_multi_top_nodes_sibling_attached() {
        let (root, scope, parent, sibling) = setup_parent_and_sibling();

        const HTML: &str = r#"<p>paragraph</p><a href="https://yew.rs">link</a>"#;
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, sibling);
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), format!("{}{}", HTML, SIBLING_CONTENT));
    }

    #[test]
    fn braw_detach_works_multi_node_sibling_attached() {
        let (root, scope, parent, sibling) = setup_parent_and_sibling();

        const HTML: &str = r#"<p>paragraph</p><a href="https://yew.rs">link</a>"#;
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, sibling);
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), format!("{}{}", HTML, SIBLING_CONTENT));
        elem.detach(&root, &parent, false);
        assert_eq!(parent.inner_html(), format!("{}", SIBLING_CONTENT))
    }

    #[test]
    fn braw_detach_works_single_node_sibling_attached() {
        let (root, scope, parent, sibling) = setup_parent_and_sibling();

        const HTML: &str = r#"<p>paragraph</p>"#;
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, sibling);
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), format!("{}{}", HTML, SIBLING_CONTENT));
        elem.detach(&root, &parent, false);
        assert_eq!(parent.inner_html(), format!("{}", SIBLING_CONTENT))
    }

    #[test]
    fn braw_detach_works_empty_sibling_attached() {
        let (root, scope, parent, sibling) = setup_parent_and_sibling();

        const HTML: &str = "";
        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, sibling);
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), format!("{}{}", HTML, SIBLING_CONTENT));
        elem.detach(&root, &parent, false);
        assert_eq!(parent.inner_html(), format!("{}", SIBLING_CONTENT))
    }

    #[test]
    fn braw_shift_works() {
        let (root, scope, parent) = setup_parent();
        const HTML: &str = r#"<p>paragraph</p>"#;

        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, DomSlot::at_end());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), HTML);

        let new_parent = document().create_element("section").unwrap();
        document().body().unwrap().append_child(&parent).unwrap();

        elem.shift(&new_parent, DomSlot::at_end());

        assert_eq!(new_parent.inner_html(), HTML);
        assert_eq!(parent.inner_html(), "");
    }

    #[test]
    fn braw_shift_with_sibling_works() {
        let (root, scope, parent, sibling) = setup_parent_and_sibling();
        const HTML: &str = r#"<p>paragraph</p>"#;

        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, sibling);
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), format!("{}{}", HTML, SIBLING_CONTENT));

        let new_parent = document().create_element("section").unwrap();
        document().body().unwrap().append_child(&parent).unwrap();

        let new_sibling = document().create_text_node(SIBLING_CONTENT);
        new_parent.append_child(&new_sibling).unwrap();
        let new_sibling_ref = DomSlot::at(new_sibling.into());

        elem.shift(&new_parent, new_sibling_ref);

        assert_eq!(parent.inner_html(), SIBLING_CONTENT);

        assert_eq!(
            new_parent.inner_html(),
            format!("{}{}", HTML, SIBLING_CONTENT)
        );
    }

    #[test]
    fn braw_shift_works_multi_node() {
        let (root, scope, parent) = setup_parent();
        const HTML: &str = r#"<p>paragraph</p><a href="https://yew.rs">link</a>"#;

        let elem = VNode::from_html_unchecked(HTML.into());
        let (_, mut elem) = elem.attach(&root, &scope, &parent, DomSlot::at_end());
        assert_braw(&mut elem);
        assert_eq!(parent.inner_html(), HTML);

        let new_parent = document().create_element("section").unwrap();
        document().body().unwrap().append_child(&parent).unwrap();

        elem.shift(&new_parent, DomSlot::at_end());

        assert_eq!(parent.inner_html(), "");
        assert_eq!(new_parent.inner_html(), HTML);
    }

    fn assert_braw(node: &mut BNode) -> &mut BRaw {
        if let BNode::Raw(braw) = node {
            return braw;
        }
        panic!("should be braw");
    }
}
