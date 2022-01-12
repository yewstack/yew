use super::{Key, VDiff, VNode};
use crate::html::{AnyScope, NodeRef};
use web_sys::{Element, Node};

/// This struct represents a suspendable DOM fragment.
#[derive(Clone, Debug, PartialEq)]
pub struct VSuspense {
    /// Child nodes.
    children: Box<VNode>,

    /// Fallback nodes when suspended.
    fallback: Box<VNode>,

    /// The element to attach to when children is not attached to DOM
    detached_parent: Option<Element>,

    /// Whether the current status is suspended.
    suspended: bool,

    /// The Key.
    pub(crate) key: Option<Key>,
}

impl VSuspense {
    pub(crate) fn new(
        children: VNode,
        fallback: VNode,
        detached_parent: Option<Element>,
        suspended: bool,
        key: Option<Key>,
    ) -> Self {
        Self {
            children: children.into(),
            fallback: fallback.into(),
            detached_parent,
            suspended,
            key,
        }
    }

    pub(crate) fn first_node(&self) -> Option<Node> {
        if self.suspended {
            self.fallback.first_node()
        } else {
            self.children.first_node()
        }
    }
}

impl VDiff for VSuspense {
    fn detach(&mut self, parent: &Element) {
        if self.suspended {
            self.fallback.detach(parent);
            if let Some(ref m) = self.detached_parent {
                self.children.detach(m);
            }
        } else {
            self.children.detach(parent);
        }
    }

    fn shift(&self, previous_parent: &Element, next_parent: &Element, next_sibling: NodeRef) {
        if self.suspended {
            self.fallback
                .shift(previous_parent, next_parent, next_sibling);
        } else {
            self.children
                .shift(previous_parent, next_parent, next_sibling);
        }
    }

    fn apply(
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: Option<VNode>,
    ) -> NodeRef {
        let detached_parent = self.detached_parent.as_ref().expect("no detached parent?");

        let (already_suspended, children_ancestor, fallback_ancestor) = match ancestor {
            Some(VNode::VSuspense(mut m)) => {
                // We only preserve the child state if they are the same suspense.
                if m.key != self.key || self.detached_parent != m.detached_parent {
                    m.detach(parent);

                    (false, None, None)
                } else {
                    (m.suspended, Some(*m.children), Some(*m.fallback))
                }
            }
            Some(mut m) => {
                m.detach(parent);
                (false, None, None)
            }
            None => (false, None, None),
        };

        // When it's suspended, we render children into an element that is detached from the dom
        // tree while rendering fallback UI into the original place where children resides in.
        match (self.suspended, already_suspended) {
            (true, true) => {
                self.children.apply(
                    parent_scope,
                    detached_parent,
                    NodeRef::default(),
                    children_ancestor,
                );

                self.fallback
                    .apply(parent_scope, parent, next_sibling, fallback_ancestor)
            }

            (false, false) => {
                self.children
                    .apply(parent_scope, parent, next_sibling, children_ancestor)
            }

            (true, false) => {
                children_ancestor.as_ref().unwrap().shift(
                    parent,
                    detached_parent,
                    NodeRef::default(),
                );

                self.children.apply(
                    parent_scope,
                    detached_parent,
                    NodeRef::default(),
                    children_ancestor,
                );

                // first render of fallback, ancestor needs to be None.
                self.fallback
                    .apply(parent_scope, parent, next_sibling, None)
            }

            (false, true) => {
                fallback_ancestor.unwrap().detach(parent);

                children_ancestor.as_ref().unwrap().shift(
                    detached_parent,
                    parent,
                    next_sibling.clone(),
                );
                self.children
                    .apply(parent_scope, parent, next_sibling, children_ancestor)
            }
        }
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use super::*;

    impl VSuspense {
        pub(crate) async fn render_to_string(&self, w: &mut String, parent_scope: &AnyScope) {
            // always render children on the server side.
            self.children.render_to_string(w, parent_scope).await;
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32"), feature = "ssr"))]
mod ssr_tests {
    use std::rc::Rc;
    use std::time::Duration;

    use tokio::task::{spawn_local, LocalSet};
    use tokio::test;
    use tokio::time::sleep;

    use crate::prelude::*;
    use crate::suspense::{Suspension, SuspensionResult};
    use crate::ServerRenderer;

    #[test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_suspense() {
        #[derive(PartialEq)]
        pub struct SleepState {
            s: Suspension,
        }

        impl SleepState {
            fn new() -> Self {
                let (s, handle) = Suspension::new();

                // we use tokio spawn local here.
                spawn_local(async move {
                    // we use tokio sleep here.
                    sleep(Duration::from_millis(50)).await;

                    handle.resume();
                });

                Self { s }
            }
        }

        impl Reducible for SleepState {
            type Action = ();

            fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
                Self::new().into()
            }
        }

        pub fn use_sleep() -> SuspensionResult<Rc<dyn Fn()>> {
            let sleep_state = use_reducer(SleepState::new);

            if sleep_state.s.resumed() {
                Ok(Rc::new(move || sleep_state.dispatch(())))
            } else {
                Err(sleep_state.s.clone())
            }
        }

        #[derive(PartialEq, Properties, Debug)]
        struct ChildProps {
            name: String,
        }

        #[function_component]
        fn Child(props: &ChildProps) -> HtmlResult {
            use_sleep()?;
            Ok(html! { <div>{"Hello, "}{&props.name}{"!"}</div> })
        }

        #[function_component]
        fn Comp() -> Html {
            let fallback = html! {"loading..."};

            html! {
                <Suspense {fallback}>
                    <Child name="Jane" />
                    <Child name="John" />
                    <Child name="Josh" />
                </Suspense>
            }
        }

        let local = LocalSet::new();

        let s = local
            .run_until(async move {
                let renderer = ServerRenderer::<Comp>::new();

                renderer.render().await
            })
            .await;

        assert_eq!(
            s,
            "<div>Hello, Jane!</div><div>Hello, John!</div><div>Hello, Josh!</div>"
        );
    }
}
