//! This module contains fragments implementation.
use std::ops::{Deref, DerefMut};

use super::{Key, VNode};

#[derive(Clone, Copy, Debug, PartialEq)]
enum FullyKeyedState {
    KnownFullyKeyed,
    KnownMissingKeys,
    Unknown,
}

/// This struct represents a fragment of the Virtual DOM tree.
#[derive(Clone, Debug)]
pub struct VList {
    /// The list of child [VNode]s
    pub(crate) children: Vec<VNode>,

    /// All [VNode]s in the VList have keys
    fully_keyed: FullyKeyedState,

    pub key: Option<Key>,
}

impl PartialEq for VList {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children && self.key == other.key
    }
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

impl DerefMut for VList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.fully_keyed = FullyKeyedState::Unknown;
        &mut self.children
    }
}

impl VList {
    /// Creates a new empty [VList] instance.
    pub const fn new() -> Self {
        Self {
            children: Vec::new(),
            key: None,
            fully_keyed: FullyKeyedState::KnownFullyKeyed,
        }
    }

    /// Creates a new [VList] instance with children.
    pub fn with_children(children: Vec<VNode>, key: Option<Key>) -> Self {
        let mut vlist = VList {
            fully_keyed: FullyKeyedState::Unknown,
            children,
            key,
        };
        vlist.fully_keyed = if vlist.fully_keyed() {
            FullyKeyedState::KnownFullyKeyed
        } else {
            FullyKeyedState::KnownMissingKeys
        };
        vlist
    }

    /// Add [VNode] child.
    pub fn add_child(&mut self, child: VNode) {
        if self.fully_keyed == FullyKeyedState::KnownFullyKeyed && !child.has_key() {
            self.fully_keyed = FullyKeyedState::KnownMissingKeys;
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

    /// Recheck, if the all the children have keys.
    ///
    /// You can run this, after modifying the child list through the [DerefMut] implementation of
    /// [VList], to precompute an internally kept flag, which speeds up reconciliation later.
    pub fn recheck_fully_keyed(&mut self) {
        self.fully_keyed = if self.fully_keyed() {
            FullyKeyedState::KnownFullyKeyed
        } else {
            FullyKeyedState::KnownMissingKeys
        };
    }

    pub(crate) fn fully_keyed(&self) -> bool {
        match self.fully_keyed {
            FullyKeyedState::KnownFullyKeyed => true,
            FullyKeyedState::KnownMissingKeys => false,
            FullyKeyedState::Unknown => self.children.iter().all(|c| c.has_key()),
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
        assert_eq!(
            vlist.fully_keyed,
            FullyKeyedState::KnownFullyKeyed,
            "should start fully keyed"
        );
        // add a child that is keyed
        vlist.add_child(VNode::VTag({
            let mut tag = VTag::new("a");
            tag.key = Some(42u32.into());
            tag.into()
        }));
        assert_eq!(
            vlist.fully_keyed,
            FullyKeyedState::KnownFullyKeyed,
            "should still be fully keyed"
        );
        assert_eq!(vlist.len(), 1, "should contain 1 child");
        // now add a child that is not keyed
        vlist.add_child(VNode::VText(VText::new("lorem ipsum")));
        assert_eq!(
            vlist.fully_keyed,
            FullyKeyedState::KnownMissingKeys,
            "should not be fully keyed, text tags have no key"
        );
        let _: &mut [VNode] = &mut vlist; // Use deref mut
        assert_eq!(
            vlist.fully_keyed,
            FullyKeyedState::Unknown,
            "key state should be unknown, since it was potentially modified through children"
        );
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use futures::stream::{FuturesOrdered, StreamExt};

    use super::*;
    use crate::html::AnyScope;
    use crate::platform::io::{self, BufWriter};

    impl VList {
        pub(crate) async fn render_into_stream(
            &self,
            w: &mut BufWriter,
            parent_scope: &AnyScope,
            hydratable: bool,
        ) {
            match &self.children[..] {
                [] => {}
                [child] => {
                    child.render_into_stream(w, parent_scope, hydratable).await;
                }
                _ => {
                    let buf_capacity = w.capacity();

                    // Concurrently render all children.
                    let mut children: FuturesOrdered<_> = self
                        .children
                        .iter()
                        .map(|m| async move {
                            let (mut w, r) = io::buffer(buf_capacity);

                            m.render_into_stream(&mut w, parent_scope, hydratable).await;
                            drop(w);

                            r
                        })
                        .collect();

                    while let Some(mut r) = children.next().await {
                        while let Some(next_chunk) = r.next().await {
                            w.write(next_chunk.into());
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "ssr")]
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
