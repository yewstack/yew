//! This module contains fragments implementation.
use stdweb::web::{INode, Node};
use stdweb::unstable::TryInto;
use html::{ScopeEnv, Component};
use super::{VDiff, VNode, VText, VTag};

/// This struct represents a fragment of the Virtual DOM tree.
pub struct VList<CTX, COMP: Component<CTX>> {
    /// The list of children nodes. Which also could have own children.
    pub childs: Vec<VNode<CTX, COMP>>,
    /// A reference to the `Node`.
    pub reference: Option<Node>,
}

impl<CTX, COMP: Component<CTX>> VList<CTX, COMP> {
    /// Creates a new `VTag` instance with `tag` name (cannot be changed later in DOM).
    pub fn new() -> Self {
        VList {
            childs: Vec::new(),
            reference: None,
        }
    }

    /// Add `VNode` child.
    pub fn add_child(&mut self, child: VNode<CTX, COMP>) {
        self.childs.push(child);
    }
}

impl<CTX: 'static, COMP: Component<CTX>> VDiff for VList<CTX, COMP> {
    type Context = CTX;
    type Component = COMP;

    fn get_node(&self) -> Option<Node> {
        self.reference.as_ref().map(|tnode| tnode.as_node().to_owned())
    }

    fn remove<T: INode>(self, parent: &T) {
        let node = self.reference.expect("tried to remove not rendered VList from DOM");
        if let Err(_) = parent.remove_child(&node) {
            warn!("Node not found to remove VList fragment");
        }
    }

    fn apply<T: INode>(&mut self,
             parent: &T,
             opposite: Option<VNode<Self::Context, Self::Component>>,
             env: ScopeEnv<Self::Context, Self::Component>)
    {
        let (element, mut opposite) = {
            match opposite {
                Some(VNode::VList(mut vlist)) => {
                    match vlist.reference.take() {
                        Some(element) => {
                            (element, Some(vlist))
                        }
                        None => {
                            let element = document_fragment();
                            parent.append_child(&element);
                            (element, None)
                        }
                    }
                }
                Some(VNode::VTag(VTag { reference: Some(wrong), .. })) => {
                    let element = document_fragment();
                    parent.replace_child(&element, &wrong);
                    (element, None)
                }
                Some(VNode::VText(VText { reference: Some(wrong), .. })) => {
                    let element = document_fragment();
                    parent.replace_child(&element, &wrong);
                    (element, None)
                }
                Some(VNode::VComp(vcomp)) => {
                    if let Some(wrong) = vcomp.get_node() {
                        let element = document_fragment();
                        parent.replace_child(&element, &wrong);
                        (element, None)
                    } else {
                        let element = document_fragment();
                        parent.append_child(&element);
                        (element, None)
                    }
                }
                Some(VNode::VRef(wrong)) => {
                    let element = document_fragment();
                    parent.replace_child(&element, &wrong);
                    (element, None)
                }
                Some(VNode::VTag(VTag { reference: None, .. })) |
                Some(VNode::VText(VText { reference: None, .. })) |
                None => {
                    let element = document_fragment();
                    parent.append_child(&element);
                    (element, None)
                }
            }
        };
        // Collect elements of an opposite if exists or use an empty vec
        // TODO DRY?!
        let mut rights = {
            if let Some(ref mut right) = opposite {
                right.childs.drain(..).map(Some).collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        };
        let mut lefts = self.childs.iter_mut().map(Some).collect::<Vec<_>>();
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
            println!("Rendering!");
            match pair {
                (Some(left), right) => {
                    left.apply(&element, right, env.clone());
                }
                (None, Some(right)) => {
                    right.remove(&element);
                }
                (None, None) => {
                    panic!("redundant iterations during diff");
                }
            }
        }
        self.reference = Some(element);
    }
}

// TODO Move to stdweb
/// Borrowed from `src/webapi/node.rs` of `stdweb`.
fn document_fragment() -> Node {
    js!(
        return document.createDocumentFragment();
    ).try_into().expect("can't create a fragment for a VList")
}
