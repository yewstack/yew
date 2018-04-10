//! This module contains the implementation of a virtual text node `VElement`.

use super::{Reform, VDiff, VNode};
use html::{Component, ScopeEnv};
use std::cmp::PartialEq;
use std::fmt;
use std::marker::PhantomData;
use stdweb::web::{document, INode, Node, Element};



// pub enum ElementTag {
//     /// The value of Attr.name
//     Attr,
//     /// "#cdata-section"
//     CDATASection,
//     /// "#comment"
//     Comment,
//     /// "#document"
//     Document,
//     /// "#document-fragment"
//     DocumentFragment,
//     /// The value of DocumentType.name
//     DocumentType,
//     /// The value of Element.tagName
//     Element,
//     /// The entity name
//     Entity,
//     /// The name of entity reference
//     EntityReference,
//     /// The notation name
//     Notation,
//     /// The value of ProcessingInstruction.target
//     ProcessingInstruction,
//     /// "#text"
//     Text,
// }

/// A type for a virtual
/// [Element](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)
/// represenation.
pub struct VElement<CTX, COMP: Component<CTX>> {
    /// Contains a `tagName` of the element
    pub tag: String,
    /// A reference to the `Element`.
    pub reference: Option<Element>,
    _ctx: PhantomData<CTX>,
    _comp: PhantomData<COMP>,
}

impl<CTX: 'static, COMP: Component<CTX>> VElement<CTX, COMP> {
    /// Creates new virtual text node with a content.
    pub fn new(tag: String) -> Self {
        VElement {
            tag: tag,
            reference: None,
            _ctx: PhantomData,
            _comp: PhantomData,
        }
    }

    /// Create from element.
    ///
    /// You must specify the correct tag.
    pub fn from_element(tag: String, element: Element) -> Self {
        VElement {
            tag: tag,
            reference: Some(element),
            _ctx: PhantomData,
            _comp: PhantomData,
        }

    }
}

impl<CTX: 'static, COMP: Component<CTX>> VDiff for VElement<CTX, COMP> {
    type Context = CTX;
    type Component = COMP;

    /// Remove VTag from parent.
    fn remove(self, parent: &Node) -> Option<Node> {
        let node = self.reference
            .expect("tried to remove not rendered VElement from DOM");
        let sibling = node.next_sibling();
        if let Err(_) = parent.remove_child(&node) {
            warn!("Node not found to remove VElement");
        }
        sibling
    }

    fn apply(
        &mut self,
        parent: &Node,
        _precursor: Option<&Node>,
        opposite: Option<VNode<Self::Context, Self::Component>>,
        _scope: ScopeEnv<Self::Context, Self::Component>,
    ) -> Option<Node> {
        let reform = {
            match opposite {
                // // If element matched this type
                // Some(VNode::VText(mut vtext)) => {
                //     self.reference = vtext.reference.take();
                //     if self.text != vtext.text {
                //         if let Some(ref element) = self.reference {
                //             element.set_node_value(Some(&self.text));
                //         }
                //     }
                //     Reform::Keep
                // }
                Some(vnode) => Reform::Before(vnode.remove(parent)),
                None => Reform::Before(None),
            }
        };
        // note: https://developer.mozilla.org/en-US/docs/Web/API/SVGElement
        // SVG element is a part of "Element"
        match reform {
            Reform::Keep => { unreachable!() }
            Reform::Before(node) => {
                let element = document().create_element(&self.tag).unwrap();
                if let Some(sibling) = node {
                    eprintln!("inserting before");
                    parent
                        .insert_before(&element, &sibling)
                        .expect("can't insert element before sibling");
                } else {
                    eprintln!("appending child");
                    parent.append_child(&element);
                }
                self.reference = Some(element);
            }
        }
        self.reference.as_ref().map(|t| t.as_node().to_owned())
    }
}

impl<CTX, COMP: Component<CTX>> fmt::Debug for VElement<CTX, COMP> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VElement {{ tag: {} }}", self.tag)
    }
}
