//! This module contains the bundle implementation of a portal `BPortal`.

use super::test_log;
use super::BNode;
use crate::dom_bundle::{DomBundle, VDiff};
use crate::html::{AnyScope, NodeRef};
use crate::virtual_dom::Key;
use crate::virtual_dom::VPortal;
use std::borrow::BorrowMut;
use web_sys::Element;

/// The bundle implementation to [VPortal].
#[derive(Debug)]
pub struct BPortal {
    /// The element under which the content is inserted.
    host: Element,
    /// The next sibling after the inserted content
    next_sibling: NodeRef,
    /// The inserted node
    node: Box<BNode>,
}

impl DomBundle for BPortal {
    fn detach(self, _: &Element) {
        test_log!("Detaching portal from host{:?}", self.host.outer_html());
        self.node.detach(&self.host);
        test_log!("Detached portal from host{:?}", self.host.outer_html());
    }

    fn shift(&self, _next_parent: &Element, _next_sibling: NodeRef) {
        // portals have nothing in it's original place of DOM, we also do nothing.
    }
}

impl VDiff for VPortal {
    type Bundle = BPortal;

    fn attach(
        self,
        parent_scope: &AnyScope,
        _parent: &Element,
        host_next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let VPortal {
            host,
            next_sibling,
            node,
        } = self;
        let (_, inner) = node.attach(parent_scope, &host, next_sibling.clone());
        (
            host_next_sibling,
            BPortal {
                host,
                node: Box::new(inner),
                next_sibling,
            },
        )
    }

    fn apply(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: &mut BNode,
    ) -> NodeRef {
        if let BNode::BPortal(portal) = ancestor {
            let old_host = std::mem::replace(&mut portal.host, self.host);
            let old_sibling = std::mem::replace(&mut portal.next_sibling, self.next_sibling);
            let node = &mut portal.node;
            if old_host != portal.host || old_sibling != portal.next_sibling {
                // Remount the inner node somewhere else instead of diffing
                // Move the node, but keep the state
                node.move_before(&portal.host, &portal.next_sibling.get());
            }
            let inner_ancestor = node.borrow_mut();
            self.node
                .apply(parent_scope, parent, next_sibling.clone(), inner_ancestor);
            return next_sibling;
        }

        let (_, self_) = self.attach(parent_scope, parent, next_sibling.clone());
        ancestor.replace(parent, self_.into());
        next_sibling
    }
}

impl BPortal {
    pub(crate) fn key(&self) -> Option<&Key> {
        self.node.key()
    }
}

#[cfg(test)]
mod layout_tests {
    extern crate self as yew;

    use crate::html;
    use crate::tests::layout_tests::{diff_layouts, TestLayout};
    use crate::virtual_dom::VNode;
    use yew::virtual_dom::VPortal;

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn diff() {
        let mut layouts = vec![];
        let first_target = gloo_utils::document().create_element("i").unwrap();
        let second_target = gloo_utils::document().create_element("o").unwrap();
        let target_with_child = gloo_utils::document().create_element("i").unwrap();
        let target_child = gloo_utils::document().create_element("s").unwrap();
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
}
