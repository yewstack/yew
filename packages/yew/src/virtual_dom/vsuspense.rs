#[cfg(feature = "hydration")]
use std::collections::VecDeque;

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

    /// The fallback fragment when the suspense boundary is hydrating.
    #[cfg(feature = "hydration")]
    fallback_fragment: Option<VecDeque<Node>>,

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
            #[cfg(feature = "hydration")]
            fallback_fragment: None,
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
    fn detach(&mut self, parent: &Element, parent_to_detach: bool) {
        if self.suspended {
            #[cfg(feature = "hydration")]
            {
                if let Some(m) = self.fallback_fragment.take() {
                    if !parent_to_detach {
                        for node in m.into_iter() {
                            parent
                                .remove_child(&node)
                                .expect("failed to remove child element");
                        }
                    }
                } else {
                    self.fallback.detach(parent, parent_to_detach);
                }
            }

            #[cfg(not(feature = "hydration"))]
            self.fallback.detach(parent, parent_to_detach);

            if let Some(ref m) = self.detached_parent {
                self.children.detach(m, false);
            }
        } else {
            self.children.detach(parent, parent_to_detach);
        }
    }

    fn shift(&self, previous_parent: &Element, next_parent: &Element, next_sibling: NodeRef) {
        if self.suspended {
            #[cfg(feature = "hydration")]
            {
                use crate::virtual_dom::shift_fragment;
                if let Some(ref m) = self.fallback_fragment {
                    shift_fragment(m, previous_parent, next_parent, next_sibling);
                } else {
                    self.fallback
                        .shift(previous_parent, next_parent, next_sibling);
                }
            }

            #[cfg(not(feature = "hydration"))]
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

        let (already_suspended, children_ancestor, fallback_ancestor, fallback_fragment) =
            match ancestor {
                Some(VNode::VSuspense(mut m)) => {
                    // We only preserve the child state if they are the same suspense.
                    if m.key != self.key || self.detached_parent != m.detached_parent {
                        m.detach(parent, false);

                        (false, None, None, None)
                    } else {
                        (
                            m.suspended,
                            Some(*m.children),
                            Some(*m.fallback),
                            m.fallback_fragment,
                        )
                    }
                }
                Some(mut m) => {
                    m.detach(parent, false);
                    (false, None, None, None)
                }
                None => (false, None, None, None),
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

                #[cfg(feature = "hydration")]
                {
                    if fallback_fragment.is_none() {
                        self.fallback
                            .apply(parent_scope, parent, next_sibling, fallback_ancestor)
                    } else {
                        let node_ref = NodeRef::default();
                        node_ref.set(fallback_fragment.as_ref().and_then(|m| m.front().cloned()));

                        self.fallback_fragment = fallback_fragment;

                        node_ref
                    }
                }

                #[cfg(not(feature = "hydration"))]
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
                #[cfg(feature = "hydration")]
                {
                    if let Some(m) = fallback_fragment {
                        // We can simply remove the fallback fragments it's not connected to
                        // anything.
                        for node in m.into_iter() {
                            parent
                                .remove_child(&node)
                                .expect("failed to remove fragment node.");
                        }
                    } else {
                        fallback_ancestor.unwrap().detach(parent, false);
                    }
                }

                #[cfg(not(feature = "hydration"))]
                fallback_ancestor.unwrap().detach(parent, false);

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

#[cfg_attr(documenting, doc(cfg(feature = "hydration")))]
#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    use std::collections::VecDeque;

    use web_sys::Node;

    use crate::virtual_dom::{collect_between, trim_start_text_nodes, VHydrate};

    impl VHydrate for VSuspense {
        fn hydrate(
            &mut self,
            parent_scope: &AnyScope,
            parent: &Element,
            fragment: &mut VecDeque<Node>,
        ) -> NodeRef {
            let detached_parent = self.detached_parent.as_ref().expect("no detached parent?");

            // We start hydration with the VSuspense being suspended.
            // A subsequent render will resume the VSuspense if not needed to be suspended.
            self.suspended = true;

            let fallback_nodes = collect_between(fragment, parent, "suspense");

            let mut nodes = fallback_nodes
                .iter()
                .map(|m| m.clone_node_with_deep(true).expect("failed to clone node."))
                .collect::<VecDeque<_>>();

            for node in nodes.iter() {
                detached_parent.append_child(node).unwrap();
            }

            self.children
                .hydrate(parent_scope, detached_parent, &mut nodes);

            // We trim all text nodes before checking as it's likely these are whitespaces.
            trim_start_text_nodes(detached_parent, &mut nodes);

            assert!(nodes.is_empty(), "expected end of suspense, found node.");

            let first_node = fallback_nodes
                .front()
                .cloned()
                .map(NodeRef::new)
                .unwrap_or_else(NodeRef::default);

            self.fallback_fragment = Some(fallback_nodes);

            first_node
        }
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use super::*;

    impl VSuspense {
        pub(crate) async fn render_to_string(
            &self,
            w: &mut String,
            parent_scope: &AnyScope,
            hydratable: bool,
        ) {
            if hydratable {
                w.push_str("<!--yew-suspense-start-->");
            }
            // always render children on the server side.
            self.children
                .render_to_string(w, parent_scope, hydratable)
                .await;

            if hydratable {
                w.push_str("<!--yew-suspense-end-->");
            }
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

        #[hook]
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
                let mut renderer = ServerRenderer::<Comp>::new();
                renderer.set_hydratable(false);

                renderer.render().await
            })
            .await;

        assert_eq!(
            s,
            "<div>Hello, Jane!</div><div>Hello, John!</div><div>Hello, Josh!</div>"
        );
    }
}
