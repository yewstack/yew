//! This module contains fragments implementation.
use super::{VDiff, VNode, VText};
use html::{Component, Scope};
use stdweb::web::Node;

/// This struct represents a fragment of the Virtual DOM tree.
pub struct VList<COMP: Component> {
    /// The list of children nodes. Which also could have own children.
    pub childs: Vec<VNode<COMP>>,
}

impl<COMP: Component> VList<COMP> {
    /// Creates a new `VTag` instance with `tag` name (cannot be changed later in DOM).
    pub fn new() -> Self {
        VList { childs: Vec::new() }
    }

    /// Add `VNode` child.
    pub fn add_child(&mut self, child: VNode<COMP>) {
        self.childs.push(child);
    }
}

impl<COMP: Component> VDiff for VList<COMP> {
    type Component = COMP;

    fn detach(&mut self, parent: &Node) -> Option<Node> {
        let mut last_sibling = None;
        for mut child in self.childs.drain(..) {
            last_sibling = child.detach(parent);
        }
        last_sibling
    }

    fn apply(
        &mut self,
        parent: &Node,
        precursor: Option<&Node>,
        ancestor: Option<VNode<Self::Component>>,
        env: &Scope<Self::Component>,
    ) -> Option<Node> {
        // Reuse precursor, because fragment reuse parent
        let mut precursor = precursor.map(|node| node.to_owned());
        let mut rights = {
            match ancestor {
                // If element matched this type
                Some(VNode::VList(mut vlist)) => {
                    // Previously rendered items
                    vlist.childs.drain(..).map(Some).collect::<Vec<_>>()
                }
                Some(vnode) => {
                    // Use the current node as a single fragment list
                    // and let the `apply` of `VNode` to handle it.
                    vec![Some(vnode)]
                }
                None => Vec::new(),
            }
        };
        // Collect elements of an ancestor if exists or use an empty vec
        // TODO DRY?!
        if self.childs.is_empty() {
            // Fixes: https://github.com/DenisKolodin/yew/issues/294
            // Without a placeholder the next element becomes first
            // and corrupts the order of rendering
            // We use empty text element to stake out a place
            let placeholder = VText::new("".into());
            self.childs.push(placeholder.into());
        }
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
            match pair {
                (Some(left), right) => {
                    precursor = left.apply(parent, precursor.as_ref(), right, &env);
                }
                (None, Some(mut right)) => {
                    right.detach(parent);
                }
                (None, None) => {
                    panic!("redundant iterations during diff");
                }
            }
        }
        precursor
    }
}
