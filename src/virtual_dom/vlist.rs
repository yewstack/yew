//! This module contains fragments implementation.
use stdweb::web::{INode, Node};
use stdweb::unstable::TryInto;
use html::{ScopeEnv, Component};
use super::{VDiff, VNode};

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

enum Action<T> {
    Keep(T),
    Append,
    Replace(Node),
}

impl<CTX: 'static, COMP: Component<CTX>> VDiff for VList<CTX, COMP> {
    type Context = CTX;
    type Component = COMP;

    fn get_node(&self) -> Option<Node> {
        self.reference.as_ref().map(|tnode| tnode.as_node().to_owned())
    }

    fn remove(self, parent: &Node) {
        let node = self.reference.expect("tried to remove not rendered VList from DOM");
        if let Err(_) = parent.remove_child(&node) {
            warn!("Node not found to remove VList fragment");
        }
    }

    fn apply(&mut self,
             parent: &Node,
             precursor: Option<&Node>,
             opposite: Option<VNode<Self::Context, Self::Component>>,
             env: ScopeEnv<Self::Context, Self::Component>) -> Option<&Node>
    {
        let (action, mut opposite) = {
            match opposite {
                Some(VNode::VList(mut vlist)) => {
                    match vlist.reference.take() {
                        Some(element) => {
                            (Action::Keep(element), Some(vlist))
                        }
                        None => {
                            (Action::Append, None)
                        }
                    }
                }
                Some(vnode) => {
                    if let Some(wrong) = vnode.get_node() {
                        (Action::Replace(wrong.as_node().to_owned()), None)
                    } else {
                        (Action::Append, None)
                    }
                }
                None => {
                    (Action::Append, None)
                }
            }
        };
        let element = {
            match action {
                Action::Keep(element) => {
                    element
                }
                Action::Append => {
                    let element = document_fragment();
                    parent.append_child(&element);
                    element
                }
                Action::Replace(wrong) => {
                    let element = document_fragment();
                    parent.replace_child(&element, &wrong);
                    element
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
        let mut precursor = precursor;
        for pair in lefts.into_iter().zip(rights) {
            match pair {
                (Some(left), right) => {
                    precursor = left.apply(&element, precursor, right, env.clone());
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
        self.reference.as_ref().map(|n| n.as_node())
    }
}

// TODO Move to stdweb
/// Borrowed from `src/webapi/node.rs` of `stdweb`.
fn document_fragment() -> Node {
    js!(
        return document.createDocumentFragment();
    ).try_into().expect("can't create a fragment for a VList")
}
