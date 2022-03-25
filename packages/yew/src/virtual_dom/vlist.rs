//! This module contains fragments implementation.
use super::{Key, VNode};
use std::ops::{Deref, DerefMut};

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

impl DerefMut for VList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Caller might change the keys of the VList or add unkeyed children.
        // Defensively assume they will.
        self.fully_keyed = false;

        &mut self.children
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

    /// Recheck, if the all the children have keys.
    ///
    /// Run this, after modifying the child list that contained only keyed children prior to the
    /// mutable dereference.
    pub fn recheck_fully_keyed(&mut self) {
        self.fully_keyed = self.children.iter().all(|ch| ch.has_key());
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use super::*;
    use crate::html::AnyScope;

    impl VList {
        pub(crate) async fn render_to_string(
            &self,
            w: &mut String,
            parent_scope: &AnyScope,
            hydratable: bool,
        ) {
            // Concurrently render all children.
            for fragment in futures::future::join_all(self.children.iter().map(|m| async move {
                let mut w = String::new();

                m.render_to_string(&mut w, parent_scope, hydratable).await;

                w
            }))
            .await
            {
                w.push_str(&fragment)
            }
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32"), feature = "ssr"))]
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
