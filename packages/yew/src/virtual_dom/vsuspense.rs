#[cfg(feature = "hydration")]
use super::Fragment;
use super::{VDiff, VNode};
use crate::html::{AnyScope, NodeRef};
use web_sys::{Element, Node};

/// An enum to Respresent Fallback UI for a VSuspense.
#[derive(Clone, Debug, PartialEq)]
enum VSuspenseFallback {
    /// Suspense Fallback during Rendering
    Render { root_node: Box<VNode> },
    /// Suspense Fallback during Hydration
    #[cfg(feature = "hydration")]
    Hydration { fragment: Fragment },
}

/// This struct represents a suspendable DOM fragment.
#[derive(Clone, Debug, PartialEq)]
pub struct VSuspense {
    /// Child nodes.
    children: Box<VNode>,

    /// Fallback nodes when suspended.
    ///
    /// None if not suspended.
    fallback: Option<VSuspenseFallback>,

    detached_parent: Option<Element>,
}

impl VSuspense {
    pub(crate) fn new(
        children: VNode,
        fallback: Option<VNode>,
        detached_parent: Option<Element>,
    ) -> Self {
        Self {
            children: children.into(),
            fallback: fallback.map(|m| VSuspenseFallback::Render {
                root_node: m.into(),
            }),
            detached_parent,
        }
    }

    pub(crate) fn first_node(&self) -> Option<Node> {
        match self.fallback {
            Some(VSuspenseFallback::Render { ref root_node, .. }) => root_node.first_node(),

            #[cfg(feature = "hydration")]
            Some(VSuspenseFallback::Hydration { ref fragment, .. }) => fragment.front().cloned(),

            None => self.children.first_node(),
        }
    }
}

impl VDiff for VSuspense {
    fn detach(&mut self, parent: &Element, parent_to_detach: bool) {
        let detached_parent = self.detached_parent.as_ref().expect("no detached parent?");

        match self.fallback {
            Some(VSuspenseFallback::Render { ref mut root_node }) => {
                root_node.detach(parent, parent_to_detach);
                self.children.detach(detached_parent, true);
            }

            #[cfg(feature = "hydration")]
            Some(VSuspenseFallback::Hydration { ref fragment }) => {
                if !parent_to_detach {
                    for node in fragment.iter() {
                        parent
                            .remove_child(node)
                            .expect("failed to remove child element");
                    }
                }

                self.children.detach(detached_parent, true);
            }

            None => {
                self.children.detach(parent, parent_to_detach);
            }
        }
    }

    fn shift(&self, previous_parent: &Element, next_parent: &Element, next_sibling: NodeRef) {
        match self.fallback {
            Some(VSuspenseFallback::Render { ref root_node }) => {
                root_node.shift(previous_parent, next_parent, next_sibling);
            }

            #[cfg(feature = "hydration")]
            Some(VSuspenseFallback::Hydration { ref fragment }) => {
                fragment.shift(previous_parent, next_parent, next_sibling)
            }

            None => {
                self.children
                    .shift(previous_parent, next_parent, next_sibling);
            }
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

        let (children_ancestor, fallback_ancestor) = match ancestor {
            Some(VNode::VSuspense(mut m)) => {
                // We only preserve the child state if they are the same suspense.
                if self.detached_parent != m.detached_parent {
                    m.detach(parent, false);

                    (None, None)
                } else {
                    (Some(*m.children), m.fallback)
                }
            }
            Some(mut m) => {
                m.detach(parent, false);
                (None, None)
            }
            None => (None, None),
        };

        // When it's suspended, we render children into an element that is detached from the dom
        // tree while rendering fallback UI into the original place where children resides in.
        match (self.fallback.as_mut(), fallback_ancestor) {
            // Currently Suspended, Continue to be Suspended.
            (Some(fallback), Some(fallback_ancestor)) => {
                match (fallback, fallback_ancestor) {
                    (
                        VSuspenseFallback::Render {
                            root_node: ref mut fallback,
                        },
                        VSuspenseFallback::Render {
                            root_node: fallback_ancestor,
                        },
                    ) => {
                        self.children.apply(
                            parent_scope,
                            detached_parent,
                            NodeRef::default(),
                            children_ancestor,
                        );
                        fallback.apply(parent_scope, parent, next_sibling, Some(*fallback_ancestor))
                    }

                    // current fallback cannot be Hydration.
                    #[cfg(feature = "hydration")]
                    (VSuspenseFallback::Hydration { .. }, VSuspenseFallback::Render { .. }) => {
                        panic!("invalid suspense state!")
                    }

                    #[cfg(feature = "hydration")]
                    (_, VSuspenseFallback::Hydration { fragment }) => {
                        self.children.apply(
                            parent_scope,
                            detached_parent,
                            NodeRef::default(),
                            children_ancestor,
                        );

                        let node_ref = NodeRef::default();
                        node_ref.set(fragment.front().cloned());

                        self.fallback = Some(VSuspenseFallback::Hydration { fragment });

                        node_ref
                    }
                }
            }

            // Currently not Suspended, Continue to be not Suspended.
            (None, None) => {
                self.children
                    .apply(parent_scope, parent, next_sibling, children_ancestor)
            }

            // The children is about to be suspended.
            (Some(fallback), None) => {
                match fallback {
                    VSuspenseFallback::Render {
                        root_node: ref mut fallback,
                    } => {
                        if let Some(ref m) = children_ancestor {
                            m.shift(parent, detached_parent, NodeRef::default());
                        }

                        self.children.apply(
                            parent_scope,
                            detached_parent,
                            NodeRef::default(),
                            children_ancestor,
                        );

                        // first render of fallback, ancestor needs to be None.
                        fallback.apply(parent_scope, parent, next_sibling, None)
                    }

                    // current fallback cannot be Hydration.
                    #[cfg(feature = "hydration")]
                    VSuspenseFallback::Hydration { .. } => {
                        panic!("invalid suspense state!")
                    }
                }
            }

            // The children is about to be resumed.
            (None, Some(fallback_ancestor)) => {
                match fallback_ancestor {
                    VSuspenseFallback::Render {
                        root_node: mut fallback_ancestor,
                    } => {
                        fallback_ancestor.detach(parent, false);

                        if let Some(ref m) = children_ancestor {
                            m.shift(detached_parent, parent, next_sibling.clone());
                        }

                        self.children
                            .apply(parent_scope, parent, next_sibling, children_ancestor)
                    }

                    #[cfg(feature = "hydration")]
                    VSuspenseFallback::Hydration { fragment } => {
                        // We can simply remove the fallback fragments it's not connected to
                        // anything.
                        for node in fragment.iter() {
                            parent
                                .remove_child(node)
                                .expect("failed to remove fragment node.");
                        }

                        if let Some(ref m) = children_ancestor {
                            m.shift(detached_parent, parent, next_sibling.clone());
                        }

                        self.children
                            .apply(parent_scope, parent, next_sibling, children_ancestor)
                    }
                }
            }
        }
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "hydration")))]
#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    use crate::virtual_dom::{Fragment, VHydrate};

    impl VHydrate for VSuspense {
        fn hydrate(
            &mut self,
            parent_scope: &AnyScope,
            parent: &Element,
            fragment: &mut Fragment,
        ) -> NodeRef {
            let detached_parent = self.detached_parent.as_ref().expect("no detached parent?");

            // We start hydration with the VSuspense being suspended.
            // A subsequent render will resume the VSuspense if not needed to be suspended.

            let fallback_nodes =
                Fragment::collect_between(fragment, parent, "<?", "</?", ">", "suspense");

            let mut nodes = fallback_nodes.deep_clone();

            for node in nodes.iter() {
                detached_parent.append_child(node).unwrap();
            }

            self.children
                .hydrate(parent_scope, detached_parent, &mut nodes);

            // We trim all text nodes before checking as it's likely these are whitespaces.
            nodes.trim_start_text_nodes(detached_parent);

            assert!(nodes.is_empty(), "expected end of suspense, found node.");

            let first_node = fallback_nodes
                .front()
                .cloned()
                .map(NodeRef::new)
                .unwrap_or_else(NodeRef::default);

            self.fallback = Some(VSuspenseFallback::Hydration {
                fragment: fallback_nodes,
            });

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
                w.push_str("<!--<?>-->");
            }
            // always render children on the server side.
            self.children
                .render_to_string(w, parent_scope, hydratable)
                .await;

            if hydratable {
                w.push_str("<!--</?>-->");
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
