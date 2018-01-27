//! This module contains fragments implementation.
use stdweb::web::Node;
use html::{ScopeEnv, Component};
use super::{VDiff, VNode};

/// This struct represents a fragment of the Virtual DOM tree.
pub struct VList<CTX, COMP: Component<CTX>> {
    /// The list of children nodes. Which also could have own children.
    pub childs: Vec<VNode<CTX, COMP>>,
}

impl<CTX, COMP: Component<CTX>> VList<CTX, COMP> {
    /// Creates a new `VTag` instance with `tag` name (cannot be changed later in DOM).
    pub fn new() -> Self {
        VList {
            childs: Vec::new(),
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
        None
    }

    fn remove(self, parent: &Node) {
        for child in self.childs {
            child.remove(parent);
        }
    }

    fn apply(&mut self,
             parent: &Node,
             precursor: Option<&Node>,
             mut opposite: Option<VNode<Self::Context, Self::Component>>,
             env: ScopeEnv<Self::Context, Self::Component>) -> Option<Node>
    {
        // Collect elements of an opposite if exists or use an empty vec
        // TODO DRY?!
        let mut rights = {
            if let Some(VNode::VList(ref mut right)) = opposite {
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
        // Reuse precursor, because fragment reuse parent
        let mut precursor = precursor.map(|node| node.to_owned());
        for pair in lefts.into_iter().zip(rights) {
            match pair {
                (Some(left), right) => {
                    precursor = left.apply(parent, precursor.as_ref(), right, env.clone());
                }
                (None, Some(right)) => {
                    right.remove(parent);
                }
                (None, None) => {
                    panic!("redundant iterations during diff");
                }
            }
        }
        precursor
    }
}
