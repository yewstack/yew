//! This module contains fragments implementation.
use std::ops::{Deref, DerefMut};

use super::{Key, VNode};

/// This struct represents a fragment of the Virtual DOM tree.
#[derive(Clone, Debug, PartialEq)]
pub struct VList {
    /// The list of child [VNode]s
    pub(crate) children: Vec<VNode>,

    /// All [VNode]s in the VList have keys
    pub(crate) fully_keyed: bool,

    pub key: Option<Key>,
}

impl Default for VList {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for VList {
    type Target = Vec<VNode>;

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

/// Mutable children of a [VList].
///
/// This struct has a `DerefMut` implementations into [`Vec<VNode>`](std::vec::Vec).
/// Prefer to use immutable access, since this re-checks if all nodes have keys when dropped.
pub struct ChildrenMut<'a> {
    children: &'a mut Vec<VNode>,
    fully_keyed: &'a mut bool,
}

impl<'a> Deref for ChildrenMut<'a> {
    type Target = Vec<VNode>;

    fn deref(&self) -> &Self::Target {
        self.children
    }
}

impl<'a> DerefMut for ChildrenMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        *self.fully_keyed = false;
        self.children
    }
}

impl<'a> Drop for ChildrenMut<'a> {
    fn drop(&mut self) {
        if !*self.fully_keyed {
            // Caller might have changed the keys of the VList or add unkeyed children.
            *self.fully_keyed = self.children.iter().all(|ch| ch.has_key());
        }
    }
}

impl<'a> std::fmt::Debug for ChildrenMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ChildrenMut").field(&self.children).finish()
    }
}

impl VList {
    /// Creates a new empty [VList] instance.
    pub const fn new() -> Self {
        Self {
            children: Vec::new(),
            key: None,
            fully_keyed: true,
        }
    }

    /// Creates a new [VList] instance with children.
    pub fn with_children(children: Vec<VNode>, key: Option<Key>) -> Self {
        VList {
            fully_keyed: children.iter().all(|ch| ch.has_key()),
            children,
            key,
        }
    }

    /// Add [VNode] child.
    pub fn add_child(&mut self, child: VNode) {
        if self.fully_keyed && !child.has_key() {
            self.fully_keyed = false;
        }
        self.children.push(child);
    }

    /// Add multiple [VNode] children.
    pub fn add_children(&mut self, children: impl IntoIterator<Item = VNode>) {
        let it = children.into_iter();
        let bound = it.size_hint();
        self.children.reserve(bound.1.unwrap_or(bound.0));
        for ch in it {
            self.add_child(ch);
        }
    }

    /// Get a mutable list of children in this vlist.
    pub fn children_mut(&mut self) -> ChildrenMut<'_> {
        ChildrenMut {
            children: &mut self.children,
            fully_keyed: &mut self.fully_keyed,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::virtual_dom::{VTag, VText};

    #[test]
    fn mutably_change_children() {
        let mut vlist = VList::new();
        assert!(vlist.fully_keyed, "should start fully keyed");
        // add a child that is keyed
        let mut children = vlist.children_mut();
        children.push(VNode::VTag({
            let mut tag = VTag::new("a");
            tag.key = Some(42u32.into());
            Box::new(tag)
        }));
        drop(children);
        assert!(vlist.fully_keyed, "should still be fully keyed");
        assert_eq!(vlist.len(), 1, "should contain 1 child");
        // now add a child that is not keyed
        let mut children = vlist.children_mut();
        children.push(VNode::VText(VText::new("lorem ipsum")));
        drop(children);
        assert!(
            !vlist.fully_keyed,
            "should not be fully keyed, text tags have no key"
        );
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use futures::stream::{FuturesOrdered, StreamExt};

    use super::*;
    use crate::html::AnyScope;
    use crate::server_renderer::BufWriter;

    impl VList {
        pub(crate) async fn render_into_stream(
            &self,
            w: &mut BufWriter,
            parent_scope: &AnyScope,
            hydratable: bool,
        ) {
            if self.children.len() < 2 {
                match self.children.first() {
                    Some(m) => {
                        m.render_into_stream(w, parent_scope, hydratable).await;
                    }

                    None => {}
                }

                return;
            }

            let buf_capacity = w.capacity();

            // Concurrently render all children.
            let mut children: FuturesOrdered<_> = self
                .children
                .iter()
                .map(|m| async move {
                    let (mut w, rx) = BufWriter::with_capacity(buf_capacity);

                    m.render_into_stream(&mut w, parent_scope, hydratable).await;
                    drop(w);

                    rx
                })
                .collect();

            while let Some(mut rx) = children.next().await {
                while let Some(next_chunk) = rx.next().await {
                    w.write(next_chunk.into());
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod ssr_tests {
    use tokio::test;

    use crate::prelude::*;
    use crate::ServerRenderer;

    #[test]
    async fn test_text_back_to_back() {
        #[function_component]
        fn Comp() -> Html {
            let s = "world";

            html! { <div>{"Hello "}{s}{"!"}</div> }
        }

        let s = ServerRenderer::<Comp>::new()
            .hydratable(false)
            .render()
            .await;

        assert_eq!(s, "<div>Hello world!</div>");
    }

    #[test]
    async fn test_fragment() {
        #[derive(PartialEq, Properties, Debug)]
        struct ChildProps {
            name: String,
        }

        #[function_component]
        fn Child(props: &ChildProps) -> Html {
            html! { <div>{"Hello, "}{&props.name}{"!"}</div> }
        }

        #[function_component]
        fn Comp() -> Html {
            html! {
                <>
                    <Child name="Jane" />
                    <Child name="John" />
                    <Child name="Josh" />
                </>
            }
        }

        let s = ServerRenderer::<Comp>::new()
            .hydratable(false)
            .render()
            .await;

        assert_eq!(
            s,
            "<div>Hello, Jane!</div><div>Hello, John!</div><div>Hello, Josh!</div>"
        );
    }
}
