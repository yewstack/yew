//! This module contains the implementation of a virtual text node `VText`.

use std::fmt;
use std::cmp::PartialEq;
use std::marker::PhantomData;
use stdweb::web::{INode, Node, TextNode, document};
use virtual_dom::{VTag, VNode, VList};
use html::{ScopeEnv, Component};
use super::VDiff;

/// A type for a virtual
/// [TextNode](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)
/// represenation.
pub struct VText<CTX, COMP: Component<CTX>> {
    /// Contains a text of the node.
    pub text: String,
    /// A reference to the `TextNode`.
    pub reference: Option<TextNode>,
    _ctx: PhantomData<CTX>,
    _comp: PhantomData<COMP>,
}

impl<CTX: 'static, COMP: Component<CTX>> VText<CTX, COMP> {
    /// Creates new virtual text node with a content.
    pub fn new(text: String) -> Self {
        VText {
            text,
            reference: None,
            _ctx: PhantomData,
            _comp: PhantomData,
        }
    }
}

impl<CTX: 'static, COMP: Component<CTX>> VDiff for VText<CTX, COMP> {
    type Context = CTX;
    type Component = COMP;

    /// Get binded node.
    fn get_node(&self) -> Option<Node> {
        self.reference.as_ref().map(|tnode| tnode.as_node().to_owned())
    }

    /// Remove VTag from parent.
    fn remove(self, parent: &Node) {
        let node = self.reference.expect("tried to remove not rendered VText from DOM");
        if let Err(_) = parent.remove_child(&node) {
            warn!("Node not found to remove VText");
        }
    }

    /// Renders virtual node over existent `TextNode`, but
    /// only if value of text had changed.
     /// Parameter `precursor` is necesssary for `VTag` and `VList` which
     /// has children and renders them.
    fn apply(&mut self,
             parent: &Node,
             _: Option<&Node>,
             opposite: Option<VNode<Self::Context, Self::Component>>,
             _: ScopeEnv<Self::Context, Self::Component>)
    {
        match opposite {
            // If element matched this type
            Some(VNode::VText(VText { text, reference: Some(element), .. })) => {
                if self.text != text {
                    element.set_node_value(Some(&self.text));
                }
                self.reference = Some(element);
            }
            // If element exists, but have a wrong type
            Some(VNode::VTag(VTag { reference: Some(wrong), .. })) => {
                let element = document().create_text_node(&self.text);
                parent.replace_child(&element, &wrong);
                self.reference = Some(element);
            }
            Some(VNode::VList(VList { reference: Some(wrong), .. })) => {
                let element = document().create_text_node(&self.text);
                parent.replace_child(&element, &wrong);
                self.reference = Some(element);
            }
            Some(VNode::VComp(vcomp)) => {
                if let Some(wrong) = vcomp.get_node() {
                    let element = document().create_text_node(&self.text);
                    parent.replace_child(&element, &wrong);
                    self.reference = Some(element);
                } else {
                    let element = document().create_text_node(&self.text);
                    parent.append_child(&element);
                    self.reference = Some(element);
                }
            }
            Some(VNode::VRef(node)) => {
                let element = document().create_text_node(&self.text);
                parent.replace_child(&element, &node);
                self.reference = Some(element);
            }
            // If element not exists
            Some(VNode::VTag(VTag { reference: None, .. })) |
            Some(VNode::VText(VText { reference: None, .. })) |
            Some(VNode::VList(VList { reference: None, .. })) |
            None => {
                let element = document().create_text_node(&self.text);
                parent.append_child(&element);
                self.reference = Some(element);
            }
        }
    }
}

impl<CTX, COMP: Component<CTX>> fmt::Debug for VText<CTX, COMP> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VText {{ text: {} }}", self.text)
    }
}

impl<CTX, COMP: Component<CTX>> PartialEq for VText<CTX, COMP> {
    fn eq(&self, other: &VText<CTX, COMP>) -> bool {
        return self.text == other.text;
    }
}
