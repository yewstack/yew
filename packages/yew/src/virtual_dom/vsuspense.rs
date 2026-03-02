use super::{Key, VNode};
use crate::html::ImplicitClone;

/// This struct represents a suspendable DOM fragment.
#[derive(Clone, ImplicitClone, Debug, PartialEq)]
pub struct VSuspense {
    /// Child nodes.
    pub(crate) children: VNode,
    /// Fallback nodes when suspended.
    pub(crate) fallback: VNode,
    /// Whether the current status is suspended.
    pub(crate) suspended: bool,
    /// The Key.
    pub(crate) key: Option<Key>,
}

impl VSuspense {
    pub fn new(children: VNode, fallback: VNode, suspended: bool, key: Option<Key>) -> Self {
        Self {
            children,
            fallback,
            suspended,
            key,
        }
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use std::fmt::Write;
    use std::rc::Rc;
    use std::task::Poll;

    use futures::stream::StreamExt;
    use futures::{pin_mut, poll, FutureExt};

    use super::*;
    use crate::feat_ssr::{DeferredSuspense, SsrContext, VTagKind};
    use crate::html::AnyScope;
    use crate::platform::fmt::{self, BufWriter};
    use crate::platform::pinned::oneshot;
    use crate::platform::spawn_local;
    use crate::virtual_dom::Collectable;

    impl VSuspense {
        pub(crate) async fn render_into_stream(
            &self,
            w: &mut BufWriter,
            parent_scope: &AnyScope,
            hydratable: bool,
            parent_vtag_kind: VTagKind,
            ctx: &Rc<SsrContext>,
        ) {
            let collectable = Collectable::Suspense;

            let (mut child_w, child_r) = fmt::buffer();

            let children = self.children.clone();
            let scope_clone = parent_scope.clone();
            let ctx_clone = ctx.clone();

            let mut child_fut = async move {
                children
                    .render_into_stream(
                        &mut child_w,
                        &scope_clone,
                        hydratable,
                        parent_vtag_kind,
                        &ctx_clone,
                    )
                    .await;
            }
            .boxed_local();

            match poll!(&mut child_fut) {
                Poll::Ready(()) => {
                    if hydratable {
                        collectable.write_open_tag(w);
                    }

                    pin_mut!(child_r);
                    while let Some(m) = child_r.next().await {
                        let _ = w.write_str(m.as_str());
                    }

                    if hydratable {
                        collectable.write_close_tag(w);
                    }
                }
                Poll::Pending => {
                    let boundary_id = ctx.next_boundary_id();

                    if hydratable {
                        collectable.write_open_tag(w);
                    }

                    let _ = write!(w, r#"<!--$?--><template id="B:{boundary_id}"></template>"#);

                    self.fallback
                        .render_into_stream(w, parent_scope, hydratable, parent_vtag_kind, ctx)
                        .await;

                    let _ = w.write_str("<!--/$-->");

                    if hydratable {
                        collectable.write_close_tag(w);
                    }

                    let (content_tx, content_rx) = oneshot::channel();

                    spawn_local(async move {
                        child_fut.await;

                        let mut content = String::new();
                        pin_mut!(child_r);
                        while let Some(m) = child_r.next().await {
                            content.push_str(m.as_str());
                        }

                        let _ = content_tx.send(content);
                    });

                    ctx.push_deferred(DeferredSuspense {
                        boundary_id,
                        content_rx,
                    });
                }
            }
        }
    }
}

#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
#[cfg(feature = "ssr")]
#[cfg(test)]
mod ssr_tests {
    use std::rc::Rc;
    use std::time::Duration;

    use tokio::task::{spawn_local, LocalSet};
    use tokio::test;

    use crate::feat_ssr::YEW_SWAP_SCRIPT;
    use crate::platform::time::sleep;
    use crate::prelude::*;
    use crate::suspense::{Suspension, SuspensionResult};
    use crate::ServerRenderer;

    #[cfg(not(target_os = "wasi"))]
    #[test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_suspense() {
        #[derive(PartialEq)]
        pub struct SleepState {
            s: Suspension,
        }

        impl SleepState {
            fn new() -> Self {
                let (s, handle) = Suspension::new();

                spawn_local(async move {
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

        #[component]
        fn Child(props: &ChildProps) -> HtmlResult {
            use_sleep()?;
            Ok(html! { <div>{"Hello, "}{&props.name}{"!"}</div> })
        }

        #[component]
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
                ServerRenderer::<Comp>::new()
                    .hydratable(false)
                    .render()
                    .await
            })
            .await;

        let expected = format!(
            concat!(
                r#"<!--$?--><template id="B:0"></template>"#,
                "loading...",
                "<!--/$-->",
                "{}",
                r#"<template id="S:0">"#,
                "<div>Hello, Jane!</div><div>Hello, John!</div><div>Hello, Josh!</div>",
                "</template>",
                r#"<script>$YC("B:0","S:0")</script>"#,
            ),
            YEW_SWAP_SCRIPT,
        );
        assert_eq!(s, expected);
    }

    #[cfg(not(target_os = "wasi"))]
    #[test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_suspense_immediate() {
        #[derive(PartialEq, Properties, Debug)]
        struct ChildProps {
            name: String,
        }

        #[component]
        fn Child(props: &ChildProps) -> Html {
            html! { <div>{"Hello, "}{&props.name}{"!"}</div> }
        }

        #[component]
        fn Comp() -> Html {
            let fallback = html! {"loading..."};

            html! {
                <Suspense {fallback}>
                    <Child name="Jane" />
                </Suspense>
            }
        }

        let local = LocalSet::new();

        let s = local
            .run_until(async move {
                ServerRenderer::<Comp>::new()
                    .hydratable(false)
                    .render()
                    .await
            })
            .await;

        assert_eq!(s, "<div>Hello, Jane!</div>");
    }
}
