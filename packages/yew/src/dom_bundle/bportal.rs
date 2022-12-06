//! This module contains the bundle implementation of a portal [BPortal].

use web_sys::Element;

use super::{test_log, BNode, BSubtree};
use crate::dom_bundle::{Reconcilable, ReconcileTarget};
use crate::html::{AnyScope, NodeRef};
use crate::virtual_dom::{Key, VPortal};

/// The bundle implementation to [VPortal].
#[derive(Debug)]
pub struct BPortal {
    // The inner root
    inner_root: BSubtree,
    /// The element under which the content is inserted.
    host: Element,
    /// The next sibling after the inserted content
    inner_sibling: NodeRef,
    /// The inserted node
    node: Box<BNode>,
}

impl ReconcileTarget for BPortal {
    fn detach(self, _root: &BSubtree, _parent: &Element, _parent_to_detach: bool) {
        test_log!("Detaching portal from host",);
        self.node.detach(&self.inner_root, &self.host, false);
    }

    fn shift(&self, _next_parent: &Element, next_sibling: NodeRef) -> NodeRef {
        // portals have nothing in it's original place of DOM, we also do nothing.

        next_sibling
    }
}

impl Reconcilable for VPortal {
    type Bundle = BPortal;

    fn attach(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        host_next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let Self {
            host,
            inner_sibling,
            node,
        } = self;
        let inner_sibling = {
            let sib_ref = NodeRef::default();
            sib_ref.set(inner_sibling);
            sib_ref
        };
        let inner_root = root.create_subroot(parent.clone(), &host);
        let (_, inner) = node.attach(&inner_root, parent_scope, &host, inner_sibling.clone());
        (
            host_next_sibling,
            BPortal {
                inner_root,
                host,
                node: Box::new(inner),
                inner_sibling,
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
        match bundle {
            BNode::Portal(portal) => {
                self.reconcile(root, parent_scope, parent, next_sibling, portal)
            }
            _ => self.replace(root, parent_scope, parent, next_sibling, bundle),
        }
    }

    fn reconcile(
        self,
        _root: &BSubtree,
        parent_scope: &AnyScope,
        _parent: &Element,
        next_sibling: NodeRef,
        portal: &mut Self::Bundle,
    ) -> NodeRef {
        let Self {
            host,
            inner_sibling,
            node,
        } = self;

        let old_host = std::mem::replace(&mut portal.host, host);

        let should_shift = old_host != portal.host || portal.inner_sibling.get() != inner_sibling;
        portal.inner_sibling.set(inner_sibling);

        if should_shift {
            // Remount the inner node somewhere else instead of diffing
            // Move the node, but keep the state
            let inner_sibling = portal.inner_sibling.clone();
            portal.node.shift(&portal.host, inner_sibling);
        }
        node.reconcile_node(
            &portal.inner_root,
            parent_scope,
            &portal.host,
            portal.inner_sibling.clone(),
            &mut portal.node,
        );
        next_sibling
    }
}

impl BPortal {
    /// Get the key of the underlying portal
    pub fn key(&self) -> Option<&Key> {
        self.node.key()
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod layout_tests {
    extern crate self as yew;

    use gloo::utils::document;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    use web_sys::HtmlInputElement;
    use yew::virtual_dom::VPortal;

    use super::*;
    use crate::tests::layout_tests::{diff_layouts, TestLayout};
    use crate::virtual_dom::VNode;
    use crate::{create_portal, html};

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn diff() {
        let mut layouts = vec![];
        let first_target = gloo::utils::document().create_element("i").unwrap();
        let second_target = gloo::utils::document().create_element("o").unwrap();
        let target_with_child = gloo::utils::document().create_element("i").unwrap();
        let target_child = gloo::utils::document().create_element("s").unwrap();
        target_with_child.append_child(&target_child).unwrap();

        layouts.push(TestLayout {
            name: "Portal - first target",
            node: html! {
                <div>
                    {VNode::VRef(first_target.clone().into())}
                    {VNode::VRef(second_target.clone().into())}
                    {VNode::VPortal(VPortal::new(
                        html! { {"PORTAL"} },
                        first_target.clone(),
                    ))}
                    {"AFTER"}
                </div>
            },
            expected: "<div><i>PORTAL</i><o></o>AFTER</div>",
        });
        layouts.push(TestLayout {
            name: "Portal - second target",
            node: html! {
                <div>
                    {VNode::VRef(first_target.clone().into())}
                    {VNode::VRef(second_target.clone().into())}
                    {VNode::VPortal(VPortal::new(
                        html! { {"PORTAL"} },
                        second_target.clone(),
                    ))}
                    {"AFTER"}
                </div>
            },
            expected: "<div><i></i><o>PORTAL</o>AFTER</div>",
        });
        layouts.push(TestLayout {
            name: "Portal - update inner content",
            node: html! {
                <div>
                    {VNode::VRef(first_target.clone().into())}
                    {VNode::VRef(second_target.clone().into())}
                    {VNode::VPortal(VPortal::new(
                        html! { <> {"PORTAL"} <b /> </> },
                        second_target.clone(),
                    ))}
                    {"AFTER"}
                </div>
            },
            expected: "<div><i></i><o>PORTAL<b></b></o>AFTER</div>",
        });
        layouts.push(TestLayout {
            name: "Portal - replaced by text",
            node: html! {
                <div>
                    {VNode::VRef(first_target.clone().into())}
                    {VNode::VRef(second_target.clone().into())}
                    {"FOO"}
                    {"AFTER"}
                </div>
            },
            expected: "<div><i></i><o></o>FOOAFTER</div>",
        });
        layouts.push(TestLayout {
            name: "Portal - next sibling",
            node: html! {
                <div>
                    {VNode::VRef(target_with_child.clone().into())}
                    {VNode::VPortal(VPortal::new_before(
                        html! { {"PORTAL"} },
                        target_with_child.clone(),
                        Some(target_child.clone().into()),
                    ))}
                </div>
            },
            expected: "<div><i>PORTAL<s></s></i></div>",
        });

        diff_layouts(layouts)
    }

    fn setup_parent_with_portal() -> (BSubtree, AnyScope, Element, Element) {
        let scope = AnyScope::test();
        let parent = document().create_element("div").unwrap();
        let portal_host = document().create_element("div").unwrap();
        let root = BSubtree::create_root(&parent);

        let body = document().body().unwrap();
        body.append_child(&parent).unwrap();
        body.append_child(&portal_host).unwrap();

        (root, scope, parent, portal_host)
    }

    #[test]
    fn test_no_shift() {
        // Portals shouldn't shift (which e.g. causes internal inputs to unfocus) when sibling
        // doesn't change.
        let (root, scope, parent, portal_host) = setup_parent_with_portal();
        let input_ref = NodeRef::default();

        let portal = create_portal(
            html! { <input type="text" ref={&input_ref} /> },
            portal_host,
        );
        let (_, mut bundle) = portal
            .clone()
            .attach(&root, &scope, &parent, NodeRef::default());

        // Focus the input, then reconcile again
        let input_el = input_ref.cast::<HtmlInputElement>().unwrap();
        input_el.focus().unwrap();

        let _ = portal.reconcile_node(&root, &scope, &parent, NodeRef::default(), &mut bundle);

        let new_input_el = input_ref.cast::<HtmlInputElement>().unwrap();
        assert_eq!(input_el, new_input_el);
        assert_eq!(document().active_element(), Some(new_input_el.into()));
    }
}
