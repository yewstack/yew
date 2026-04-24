use crate::html::{Html, Properties};

/// Properties for [Suspense].
#[derive(Properties, PartialEq, Debug, Clone)]
pub struct SuspenseProps {
    /// The Children of the current Suspense Component.
    #[prop_or_default]
    pub children: Html,

    /// The Fallback UI of the current Suspense Component.
    #[prop_or_default]
    pub fallback: Html,
}

#[cfg(any(feature = "csr", feature = "ssr"))]
mod feat_csr_ssr {
    #[cfg(feature = "csr")]
    use std::cell::RefCell;

    use super::*;
    #[cfg(feature = "csr")]
    use crate::html::PendingRendered;
    use crate::html::{Component, Context, Html, Scope};
    use crate::suspense::Suspension;
    #[cfg(feature = "hydration")]
    use crate::suspense::SuspensionHandle;
    use crate::virtual_dom::{VNode, VSuspense};
    use crate::{component, html};

    #[derive(Properties, PartialEq, Debug, Clone)]
    pub(crate) struct BaseSuspenseProps {
        pub children: Html,
        #[prop_or(None)]
        pub fallback: Option<Html>,
    }

    #[derive(Debug)]
    pub(crate) enum BaseSuspenseMsg {
        Suspend(Suspension),
        Resume(Suspension),
    }

    pub(crate) struct BaseSuspense {
        suspensions: Vec<Suspension>,
        #[cfg(feature = "hydration")]
        hydration_handle: Option<SuspensionHandle>,
        /// Rendered runners for child components that resumed while this
        /// Suspense was still suspended (because of other pending siblings).
        /// Drained in `rendered` once the Suspense fully un-suspends, so
        /// effects fire only after children's DOM has been shifted into the
        /// live tree.
        ///
        /// A small `Vec` is used over a map; the expected population is the
        /// number of suspending direct descendants resumed in one boundary
        /// transition, typically just a handful.
        #[cfg(feature = "csr")]
        pending_rendered: RefCell<Vec<(usize, PendingRendered)>>,
    }

    impl std::fmt::Debug for BaseSuspense {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BaseSuspense")
                .field("suspensions", &self.suspensions)
                .finish()
        }
    }

    impl Component for BaseSuspense {
        type Message = BaseSuspenseMsg;
        type Properties = BaseSuspenseProps;

        fn create(_ctx: &Context<Self>) -> Self {
            #[cfg(not(feature = "hydration"))]
            let suspensions = Vec::new();

            // We create a suspension to block suspense until its rendered method is notified.
            #[cfg(feature = "hydration")]
            let (suspensions, hydration_handle) = {
                use crate::callback::Callback;
                use crate::html::RenderMode;

                match _ctx.creation_mode() {
                    RenderMode::Hydration => {
                        let link = _ctx.link().clone();
                        let (s, handle) = Suspension::new();
                        s.listen(Callback::from(move |s| {
                            link.send_message(BaseSuspenseMsg::Resume(s));
                        }));
                        (vec![s], Some(handle))
                    }
                    _ => (Vec::new(), None),
                }
            };

            Self {
                suspensions,
                #[cfg(feature = "hydration")]
                hydration_handle,
                #[cfg(feature = "csr")]
                pending_rendered: RefCell::new(Vec::new()),
            }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                Self::Message::Suspend(m) => {
                    assert!(
                        ctx.props().fallback.is_some(),
                        "You cannot suspend from a component rendered as a fallback."
                    );

                    if m.resumed() {
                        return false;
                    }

                    // If a suspension already exists, ignore it.
                    if self.suspensions.iter().any(|n| n == &m) {
                        return false;
                    }

                    self.suspensions.push(m);

                    true
                }
                Self::Message::Resume(ref m) => {
                    let suspensions_len = self.suspensions.len();
                    self.suspensions.retain(|n| m != n);

                    suspensions_len != self.suspensions.len()
                }
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let BaseSuspenseProps { children, fallback } = (*ctx.props()).clone();
            let children = VNode::VList(::std::rc::Rc::new(
                crate::virtual_dom::VList::with_children(vec![children], None),
            ));

            match fallback {
                Some(fallback) => {
                    let vsuspense = VSuspense::new(
                        children,
                        fallback,
                        !self.suspensions.is_empty(),
                        // We don't need to key this as the key will be applied to the component.
                        None,
                    );

                    VNode::from(vsuspense)
                }
                None => children,
            }
        }

        fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
            #[cfg(not(feature = "hydration"))]
            let _ = first_render;
            #[cfg(feature = "hydration")]
            if first_render {
                if let Some(m) = self.hydration_handle.take() {
                    m.resume();
                }
            }
            // Fire deferred rendered callbacks for children that resumed while
            // we were still suspended. Only safe now that we're un-suspended:
            // the last reconcile shifted their DOM into the live tree.
            #[cfg(feature = "csr")]
            if self.suspensions.is_empty() {
                let pending = std::mem::take(&mut *self.pending_rendered.borrow_mut());
                for (comp_id, p) in pending {
                    p.schedule(comp_id);
                }
            }
        }
    }

    impl BaseSuspense {
        pub(crate) fn suspend(scope: &Scope<Self>, s: Suspension) {
            scope.send_message(BaseSuspenseMsg::Suspend(s));
        }

        pub(crate) fn resume(scope: &Scope<Self>, s: Suspension) {
            scope.send_message(BaseSuspenseMsg::Resume(s));
        }

        /// Queue a child component's `rendered` lifecycle to be scheduled once
        /// this Suspense fully un-suspends and its reconcile has shifted the
        /// child's DOM into the live tree. If the child already has a pending
        /// entry (e.g. it re-committed between suspensions), the two are
        /// merged so `first_render=true` is not lost.
        #[cfg(feature = "csr")]
        pub(crate) fn defer_rendered(
            scope: &Scope<Self>,
            comp_id: usize,
            pending: PendingRendered,
        ) {
            let Some(comp) = scope.get_component() else {
                return;
            };
            let mut q = comp.pending_rendered.borrow_mut();
            if let Some(slot) = q.iter_mut().find(|(id, _)| *id == comp_id) {
                slot.1.absorb(pending);
            } else {
                q.push((comp_id, pending));
            }
        }
    }

    /// Suspend rendering and show a fallback UI until the underlying task completes.
    #[component]
    pub fn Suspense(props: &SuspenseProps) -> Html {
        let SuspenseProps { children, fallback } = props.clone();

        let fallback = html! {
            <BaseSuspense>
                {fallback}
            </BaseSuspense>
        };

        html! {
            <BaseSuspense {fallback}>
                {children}
            </BaseSuspense>
        }
    }
}

#[cfg(any(feature = "csr", feature = "ssr"))]
pub use feat_csr_ssr::*;

#[cfg(not(any(feature = "ssr", feature = "csr")))]
mod feat_no_csr_ssr {
    use super::*;
    use crate::component;

    /// Suspend rendering and show a fallback UI until the underlying task completes.
    #[component]
    pub fn Suspense(_props: &SuspenseProps) -> Html {
        Html::default()
    }
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub use feat_no_csr_ssr::*;
