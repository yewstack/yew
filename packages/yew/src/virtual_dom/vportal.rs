//! This module contains the implementation of a portal `VPortal`.

use super::{VDiff, VNode};
use crate::html::{AnyScope, NodeRef};
use web_sys::Element;

#[derive(Debug, Clone)]
pub struct VPortal {
    /// The element under which the content is inserted.
    pub host: Element,
    /// The next sibling after the inserted content
    pub next_sibling: NodeRef,
    /// The inserted node
    pub node: Box<VNode>,
    /// The next sibling after the portal. Set when rendered
    pub sibling_ref: NodeRef,
}

impl VDiff for VPortal {
    fn detach(&mut self, _: &Element) {
        self.node.detach(&self.host);
        self.sibling_ref.set(None);
    }

    fn apply(
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: Option<VNode>,
    ) -> NodeRef {
        let inner_ancestor = match ancestor {
            Some(VNode::VPortal(old_portal)) => {
                let VPortal {
                    host: old_host,
                    next_sibling: old_sibling,
                    mut node,
                    ..
                } = old_portal;
                if old_host != self.host {
                    // Remount the inner node somewhere else instead of diffing
                    node.detach(&old_host);
                    None
                } else if old_sibling != self.next_sibling {
                    // Move the node, but keep the state
                    node.move_before(&self.host, &self.next_sibling.get());
                    Some(*node)
                } else {
                    Some(*node)
                }
            }
            Some(mut node) => {
                node.detach(parent);
                None
            }
            None => None,
        };

        self.node.apply(
            parent_scope,
            &self.host,
            self.next_sibling.clone(),
            inner_ancestor,
        );
        self.sibling_ref = next_sibling.clone();
        next_sibling
    }
}

impl VPortal {
    pub fn create(host: Element, content: VNode) -> Self {
        Self {
            host,
            next_sibling: NodeRef::default(),
            node: Box::new(content),
            sibling_ref: NodeRef::default(),
        }
    }
}

#[cfg(test)]
mod layout_tests {
    extern crate self as yew;

    use crate::html;
    use crate::virtual_dom::layout_tests::{diff_layouts, TestLayout};
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

        layouts.push(TestLayout {
            name: "Portal - first target",
            node: html! {
                <div>
                    {VNode::VRef(first_target.clone().into())}
                    {VNode::VRef(second_target.clone().into())}
                    {VNode::VPortal(VPortal::create(
                        first_target.clone(),
                        html! { {"PORTAL"} }
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
                    {VNode::VPortal(VPortal::create(
                        second_target.clone(),
                        html! { {"PORTAL"} }
                    ))}
                    {"AFTER"}
                </div>
            },
            expected: "<div><i></i><o>PORTAL</o>AFTER</div>",
        });
        layouts.push(TestLayout {
            name: "Portal replaced by text",
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

        diff_layouts(layouts)
    }
}
