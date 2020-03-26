//! This module contains the implementation to get all the VNode structure.
//! it is useful for testing and debuging. For example we want to know what contain this VNode:
//! ```
//! let button = html! {
//!     <button
//!         onclick=link.callback(|_| Msg::Clicked)
//!         class="button standard medium regular test-button">
//!         <div id="parent">{"parent"}</div>
//!     </button>
//! };
//! ```
//!
//! Now we use VStructure to get the complete Vnode button and log the result:
//!
//! ```
//!   let vnode_button = VNodeStruct::new(button);
//!   
//!   console.log(&format!("{:#?}", vnode_button));
//! ```
//!
//! We will get this in the console:
//!
//! ```
//! VNodeStruct {
//! vtag: Some(
//!     VTagStruct {
//!         reference: None,
//!         attributes: {
//!             "id": "example",
//!         },
//!         classes: Classes {
//!             set: {},
//!         },
//!         value: None,
//!         kind: None,
//!         checked: false,
//!         node_ref: NodeRef(
//!             RefCell {
//!                 value: NodeRefInner {
//!                     node: None,
//!                     link: None,
//!                 },
//!             },
//!         ),
//!     },
//! ),
//! vlist: None,
//! vtext: None,
//! vcomp: None,
//! vref: None,
//! children: Some(
//!     [
//!         VNodeStruct {
//!             vtag: None,
//!             vlist: Some(
//!                 VList {
//!                     children: [
//!                         VText { text: example },
//!                     ],
//!                     elide_placeholder: true,
//!                 },
//!             ),
//!             vtext: None,
//!             vcomp: None,
//!             vref: None,
//!             children: Some(
//!                 [
//!                     VNodeStruct {
//!                         vtag: None,
//!                         vlist: None,
//!                         vtext: Some(
//!                             VText { text: example },
//!                         ),
//!                         vcomp: None,
//!                         vref: None,
//!                         children: None,
//!                     },
//!                 ],
//!             ),
//!         },
//!     ],
//! ),
//! }
//! ```

use super::{Attributes, Classes, VComp, VList, VNode, VTag, VText};
use crate::html::NodeRef;
use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Element, Node};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::{Element, Node};
    }
}

#[derive(Debug, PartialEq)]
struct VTagStruct {
    reference: Option<Element>,
    attributes: Attributes,
    classes: Classes,
    value: Option<String>,
    kind: Option<String>,
    checked: bool,
    node_ref: NodeRef,
}

impl From<VTag> for VTagStruct {
    fn from(vtag: VTag) -> Self {
        VTagStruct {
            reference: vtag.reference,
            attributes: vtag.attributes,
            classes: vtag.classes,
            value: vtag.value,
            kind: vtag.kind,
            checked: vtag.checked,
            node_ref: vtag.node_ref,
        }
    }
}

/// all the properties of VNodeStruct
#[derive(Debug, PartialEq)]
pub struct VNodeStruct {
    vtag: Option<VTagStruct>,
    vlist: Option<VList>,
    vtext: Option<VText>,
    vcomp: Option<VComp>,
    vref: Option<Node>,
    children: Option<Vec<VNodeStruct>>,
}

impl VNodeStruct {
    /// Get all the complete structure from VNode
    pub fn new(vnode: VNode) -> Self {
        match vnode {
            VNode::VTag(ref vtag) => VNodeStruct {
                vtag: Some(VTagStruct::from(*vtag.clone())),
                vlist: None,
                vtext: None,
                vcomp: None,
                vref: None,
                children: if !vtag.children.is_empty() {
                    Some(vec![VNodeStruct::new(VNode::VList(vtag.children.clone()))])
                } else {
                    None
                },
            },
            VNode::VText(ref vtext) => VNodeStruct {
                vtag: None,
                vlist: None,
                vtext: Some(VText {
                    text: vtext.text.clone(),
                    reference: vtext.reference.clone(),
                }),
                vcomp: None,
                vref: None,
                children: None,
            },
            VNode::VList(ref vlist) => VNodeStruct {
                vtag: None,
                vlist: Some(vlist.clone()),
                vtext: None,
                vcomp: None,
                vref: None,
                children: if !vlist.children.is_empty() {
                    Some(
                        vlist
                            .children
                            .clone()
                            .into_iter()
                            .map(VNodeStruct::new)
                            .collect(),
                    )
                } else {
                    None
                },
            },
            VNode::VComp(ref vcomp) => VNodeStruct {
                vtag: None,
                vlist: None,
                vtext: None,
                vcomp: Some(vcomp.clone()),
                vref: None,
                children: None,
            },
            VNode::VRef(ref vref) => VNodeStruct {
                vtag: None,
                vlist: None,
                vtext: None,
                vcomp: None,
                vref: Some(vref.clone()),
                children: None,
            },
        }
    }
}
