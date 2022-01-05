//! This module contains the implementation of a portal `VPortal`.

use std::borrow::BorrowMut;

use super::VNode;
use crate::dom_bundle::{DomBundle, VDiff};
use crate::html::{AnyScope, NodeRef};
use web_sys::{Element, Node};

/// Log an operation during tests for debugging purposes
/// Set RUSTFLAGS="--cfg verbose_tests" environment variable to activate.
macro_rules! test_log {
    ($fmt:literal, $($arg:expr),* $(,)?) => {
        #[cfg(all(test, feature = "wasm_test", verbose_tests))]
        ::wasm_bindgen_test::console_log!(concat!("\t  ", $fmt), $($arg),*);
    };
}

#[derive(Debug, Clone)]
pub struct VPortal {
    /// The element under which the content is inserted.
    pub host: Element,
    /// The next sibling after the inserted content
    pub next_sibling: NodeRef,
    /// The inserted node
    pub node: Box<VNode>,
    /// The next sibling after the portal. Set when rendered
    sibling_ref: NodeRef,
}

impl DomBundle for VPortal {
    fn detach(self, _: &Element) {
        test_log!("Detaching portal from host{:?}", self.host.outer_html());
        self.node.detach(&self.host);
        test_log!("Detached portal from host{:?}", self.host.outer_html());
        self.sibling_ref.set(None);
    }

    fn shift(&self, _next_parent: &Element, _next_sibling: NodeRef) {
        // portals have nothing in it's original place of DOM, we also do nothing.
    }
}

impl VDiff for VPortal {
    type Bundle = VPortal;

    fn attach(
        mut self,
        parent_scope: &AnyScope,
        _: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let (_, inner) = self
            .node
            .attach(parent_scope, &self.host, self.next_sibling.clone());
        self.node = Box::new(inner);
        self.sibling_ref = next_sibling.clone();
        (next_sibling, self)
    }

    fn apply(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: &mut VNode,
    ) -> NodeRef {
        if let VNode::VPortal(portal) = ancestor {
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

impl VPortal {
    /// Creates a [VPortal] rendering `content` in the DOM hierarchy under `host`.
    pub fn new(content: VNode, host: Element) -> Self {
        Self {
            host,
            next_sibling: NodeRef::default(),
            node: Box::new(content),
            sibling_ref: NodeRef::default(),
        }
    }
    /// Creates a [VPortal] rendering `content` in the DOM hierarchy under `host`.
    /// If `next_sibling` is given, the content is inserted before that [Node].
    /// The parent of `next_sibling`, if given, must be `host`.
    pub fn new_before(content: VNode, host: Element, next_sibling: Option<Node>) -> Self {
        Self {
            host,
            next_sibling: {
                let sib_ref = NodeRef::default();
                sib_ref.set(next_sibling);
                sib_ref
            },
            node: Box::new(content),
            sibling_ref: NodeRef::default(),
        }
    }
    /// Returns the [Node] following this [VPortal], if this [VPortal]
    /// has already been mounted in the DOM.
    pub fn next_sibling(&self) -> Option<Node> {
        self.sibling_ref.get()
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
