//! This module contains the implementation of a virtual text node `VText`.

use super::{Reform, VDiff, VNode};
use crate::utils::document;
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use log::warn;
use std::cmp::PartialEq;
use std::fmt;
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
#[derive(Clone)]
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
        if parent.remove_child(&node).is_err() {
            warn!("Node not found to remove VText");
        }
        next_sibling
    }

    /// Renders virtual node over existing `TextNode`, but only if value of text had changed.
    fn apply(
        &mut self,
        parent: &Element,
        previous_sibling: Option<&Node>,
        ancestor: Option<VNode>,
    ) -> Option<Node> {
        assert!(
            self.reference.is_none(),
            "reference is ignored so must not be set"
        );
        let reform = {
            match ancestor {
                // If element matched this type
                Some(VNode::VText(mut vtext)) => {
                    self.reference = vtext.reference.take();
                    if self.text != vtext.text {
                        if let Some(ref element) = self.reference {
                            element.set_node_value(Some(&self.text));
                        }
                    }
                    Reform::Keep
                }
                Some(mut vnode) => Reform::Before(vnode.detach(parent)),
                None => Reform::Before(None),
            }
        };
        match reform {
            Reform::Keep => {}
            Reform::Before(next_sibling) => {
                let element = document().create_text_node(&self.text);
                if let Some(next_sibling) = next_sibling {
                    let next_sibling = &next_sibling;
                    #[cfg(feature = "web_sys")]
                    let next_sibling = Some(next_sibling);
                    parent
                        .insert_before(&element, next_sibling)
                        .expect("can't insert text before the next sibling");
                } else if let Some(next_sibling) = previous_sibling.and_then(|p| p.next_sibling()) {
                    let next_sibling = &next_sibling;
                    #[cfg(feature = "web_sys")]
                    let next_sibling = Some(next_sibling);
                    parent
                        .insert_before(&element, next_sibling)
                        .expect("can't insert text before next_sibling");
                } else {
                    #[cfg_attr(
                        feature = "std_web",
                        allow(clippy::let_unit_value, unused_variables)
                    )]
                    {
                        let result = parent.append_child(&element);
                        #[cfg(feature = "web_sys")]
                        result.expect("can't append node to parent");
                    }
                }
                self.reference = Some(element);
            }
        }
        self.reference.as_ref().map(|t| {
            let node = cfg_match! {
                feature = "std_web" => t.as_node(),
                feature = "web_sys" => t.deref().deref(),
            };
            node.to_owned()
        })
    }
}

impl fmt::Debug for VText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VText {{ text: {} }}", self.text)
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
