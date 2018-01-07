//! This module contains the implementation of abstract virtual node.

use std::fmt;
use std::cmp::PartialEq;
use stdweb::web::{INode, Node, Element, TextNode, document};
use virtual_dom::{VTag, VText, VComp};
use html::{AppSender, SharedContext};

/// Bind virtual element to a DOM reference.
pub enum VNode<MSG, CTX> {
    /// A bind between `VTag` and `Element`.
    VTag {
        /// A reference to the `Element`.
        reference: Option<Element>,
        /// A virtual tag node which was applied.
        vtag: VTag<MSG, CTX>,
    },
    /// A bind between `VText` and `TextNode`.
    VText {
        /// A reference to the `TextNode`.
        reference: Option<TextNode>,
        /// A virtual text node which was applied.
        vtext: VText,
    },
    VComp {
        reference: Option<Element>,
        vcomp: VComp<MSG, CTX>,
    },
}


impl<MSG, CTX> VNode<MSG, CTX> {
    fn remove<T: INode>(self, parent: &T) {
        let opt_ref: Option<Node> = {
            match self {
                VNode::VTag { reference, .. } => reference.map(Node::from),
                VNode::VText { reference, .. } => reference.map(Node::from),
                VNode::VComp { reference, .. } => reference.map(Node::from),
            }
        };
        if let Some(node) = opt_ref {
            if let Err(_) = parent.remove_child(&node) {
                warn!("Node not found to remove: {:?}", node);
            }
        }
    }

    /// Virtual rendering for the node. It uses parent node and existend children (virtual and DOM)
    /// to check the difference and apply patches to the actual DOM represenatation.
    pub fn apply<T: INode>(&mut self, parent: &T, last: Option<VNode<MSG, CTX>>, sender: AppSender<MSG, CTX>) {
        match *self {
            VNode::VTag {
                ref mut vtag,
                ref mut reference,
            } => {
                let left = vtag;
                let mut right = None;
                match last {
                    Some(VNode::VTag {
                             vtag,
                             reference: Some(element),
                         }) => {
                        // Copy reference from right to left (as is)
                        if left.tag() == vtag.tag() {
                            right = Some(vtag);
                            *reference = Some(element);
                        } else {
                            let wrong = element;
                            let element = document().create_element(left.tag());
                            parent.replace_child(&element, &wrong);
                            *reference = Some(element);
                        }
                    }
                    Some(VNode::VText { reference: Some(wrong), .. }) => {
                        let element = document().create_element(left.tag());
                        parent.replace_child(&element, &wrong);
                        *reference = Some(element);
                    }
                    Some(VNode::VComp { reference: Some(wrong), .. }) => {
                        let element = document().create_element(left.tag());
                        parent.replace_child(&element, &wrong);
                        *reference = Some(element);
                    }
                    Some(VNode::VTag { reference: None, .. }) |
                    Some(VNode::VText { reference: None, .. }) |
                    Some(VNode::VComp { reference: None, .. }) |
                    None => {
                        let element = document().create_element(left.tag());
                        parent.append_child(&element);
                        *reference = Some(element);
                    }
                }
                let element_mut = reference.as_mut().expect("vtag must be here");
                // Update parameters
                let mut rights = {
                    if let Some(ref mut right) = right {
                        right.childs.drain(..).map(Some).collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    }
                };
                left.render(element_mut, right, sender.clone());
                let mut lefts = left.childs.iter_mut().map(Some).collect::<Vec<_>>();
                // Process children
                let diff = lefts.len() as i32 - rights.len() as i32;
                if diff > 0 {
                    for _ in 0..diff {
                        rights.push(None);
                    }
                } else if diff < 0 {
                    for _ in 0..-diff {
                        lefts.push(None);
                    }
                }
                for pair in lefts.into_iter().zip(rights) {
                    match pair {
                        (Some(left), right) => {
                            left.apply(element_mut, right, sender.clone());
                        }
                        (None, Some(right)) => {
                            right.remove(element_mut);
                        }
                        (None, None) => {
                            panic!("redundant iterations during diff");
                        }
                    }
                }
                //vtag.apply(parent, reference, last, messages);
            }
            VNode::VText {
                ref mut vtext,
                ref mut reference,
            } => {
                let left = vtext;
                let mut right = None;
                match last {
                    Some(VNode::VText {
                             vtext,
                             reference: Some(element),
                         }) => {
                        right = Some(vtext);
                        *reference = Some(element);
                    }
                    Some(VNode::VTag { reference: Some(wrong), .. }) |
                    Some(VNode::VComp { reference: Some(wrong), .. }) => {
                        let element = document().create_text_node(&left.text);
                        parent.replace_child(&element, &wrong);
                        *reference = Some(element);
                    }
                    Some(VNode::VTag { reference: None, .. }) |
                    Some(VNode::VText { reference: None, .. }) |
                    Some(VNode::VComp { reference: None, .. }) |
                    None => {
                        let element = document().create_text_node(&left.text);
                        parent.append_child(&element);
                        *reference = Some(element);
                    }
                }
                let element_mut = reference.as_mut().expect("vtext must be here");
                left.render(element_mut, right);
            }
            VNode::VComp {
                ref mut vcomp,
                ref mut reference,
            } => {
                let left = vcomp;
                let mut right = None;
                match last {
                    Some(VNode::VComp {
                             vcomp,
                             reference: Some(element),
                         }) => {
                        right = Some(vcomp);
                        *reference = Some(element);
                    }
                    None => {
                        let element = document().create_element("div");
                        parent.append_child(&element);
                        left.mount(&element, sender.context());
                        *reference = Some(element);
                    }
                    _ => {
                        eprintln!("Diff not implemented for components");
                    }
                }
            }
        }
    }
}

impl<MSG, CTX> From<VText> for VNode<MSG, CTX> {
    fn from(vtext: VText) -> Self {
        VNode::VText {
            reference: None,
            vtext,
        }
    }
}

impl<MSG, CTX> From<VTag<MSG, CTX>> for VNode<MSG, CTX> {
    fn from(vtag: VTag<MSG, CTX>) -> Self {
        VNode::VTag {
            reference: None,
            vtag,
        }
    }
}

impl<MSG, CTX> From<VComp<MSG, CTX>> for VNode<MSG, CTX> {
    fn from(vcomp: VComp<MSG, CTX>) -> Self {
        VNode::VComp {
            reference: None,
            vcomp,
        }
    }
}

impl<MSG, CTX, T: ToString> From<T> for VNode<MSG, CTX> {
    fn from(value: T) -> Self {
        VNode::VText {
            reference: None,
            vtext: VText::new(value),
        }
    }
}

impl<MSG, CTX> fmt::Debug for VNode<MSG, CTX> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &VNode::VTag { ref vtag, .. } => vtag.fmt(f),
            &VNode::VText { ref vtext, .. } => vtext.fmt(f),
            &VNode::VComp { ref vcomp, .. } => "Component<>".fmt(f),
        }
    }
}

impl<MSG, CTX> PartialEq for VNode<MSG, CTX> {
    fn eq(&self, other: &VNode<MSG, CTX>) -> bool {
        match *self {
            VNode::VTag { vtag: ref vtag_a, .. } => {
                match *other {
                    VNode::VTag { vtag: ref vtag_b, .. } => {
                        vtag_a == vtag_b
                    },
                    _ => false
                }
            }
            VNode::VText { vtext: ref vtext_a, .. } => {
                match *other {
                    VNode::VText { vtext: ref vtext_b, .. } => {
                        vtext_a == vtext_b
                    },
                    _ => false
                }
            }
            VNode::VComp { .. } => {
                false
            }
        }
    }
}
