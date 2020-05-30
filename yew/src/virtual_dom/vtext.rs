//! This module contains the implementation of a virtual text node `VText`.

use super::{Reform, VDiff, VNode};
use crate::html::AnyScope;
use crate::utils::document;
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use std::cmp::PartialEq;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Element, INode, Node, TextNode};
    } else if #[cfg(feature = "web_sys")] {
        use std::ops::Deref;
        use web_sys::{Element, Node, Text as TextNode};
    }
}

/// A type for a virtual
/// [`TextNode`](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
/// representation.
#[derive(Clone, Debug)]
pub struct VText {
    /// Contains a text of the node.
    pub text: String,
    /// A reference to the `TextNode`.
    pub reference: Option<TextNode>,
}

impl VText {
    /// Creates new virtual text node with a content.
    pub fn new(text: String) -> Self {
        VText {
            text,
            reference: None,
        }
    }
}

impl VDiff for VText {
    /// Remove VText from parent.
    fn detach(&mut self, parent: &Element) -> Option<Node> {
        let node = self
            .reference
            .take()
            .expect("tried to remove not rendered VText from DOM");

        let next_sibling = node.next_sibling();
        parent
            .remove_child(&node)
            .expect("tried to remove not rendered VText from DOM");
        next_sibling
    }

    /// Renders virtual node over existing `TextNode`, but only if value of text
    /// has changed.
    fn apply(
        &mut self,
        _parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: Option<Node>,
        ancestor: Option<VNode>,
    ) -> Option<Node> {
        assert!(
            self.reference.is_none(),
            "reference is ignored so must not be set"
        );

        // Determine what to do from the type of the ancestor, i.e. the current
        // element in the DOM that this node will replace.
        let reform = match ancestor {
            // If the ancestor is of the same type than this node.
            Some(VNode::VText(mut vtext)) => {
                self.reference = vtext.reference.take();
                if self.text != vtext.text {
                    if let Some(ref element) = self.reference {
                        element.set_node_value(Some(&self.text));
                    }
                }
                Reform::Keep
            }
            // If there is an ancestor, but of another type, remove it from
            // the DOM and insert this at its position.
            Some(mut vnode) => Reform::Before(vnode.detach(parent)),
            // Otherwise there was no element.
            None => Reform::Before(next_sibling),
        };

        match reform {
            Reform::Keep => {
                // Nothing to do, we recycled the ancestor.
            }
            Reform::Before(next_sibling) => {
                // Create a new text DOM element.
                let element = document().create_text_node(&self.text);
                super::insert_node(&element, parent, next_sibling);

                // Finally, we store the newly created element.
                self.reference = Some(element);
            }
        }

        let text_node: TextNode = self
            .reference
            .as_ref()
            .cloned()
            .expect("there must be a reference");
        let node = cfg_match! {
            feature = "std_web" => text_node.as_node(),
            feature = "web_sys" => text_node.deref().deref(),
        };
        Some(node.to_owned())
    }
}

impl PartialEq for VText {
    fn eq(&self, other: &VText) -> bool {
        self.text == other.text
    }
}

#[cfg(test)]
mod test {
    use crate::{html, Component, ComponentLink, Html, ShouldRender};
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    struct Comp;

    impl Component for Comp {
        type Message = ();
        type Properties = ();

        fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
            Comp
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            unimplemented!();
        }

        fn change(&mut self, _: Self::Properties) -> ShouldRender {
            unimplemented!();
        }

        fn view(&self) -> Html {
            unimplemented!();
        }
    }

    #[test]
    fn text_as_root() {
        html! {
            "Text Node As Root"
        };

        html! {
            { "Text Node As Root" }
        };
    }
}
