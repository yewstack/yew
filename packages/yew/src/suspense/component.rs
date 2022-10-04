use crate::html::{Children, Html, Properties};

/// Properties for [Suspense].
#[derive(Properties, PartialEq, Debug, Clone)]
pub struct SuspenseProps {
    /// The Children of the current Suspense Component.
    #[prop_or_default]
    pub children: Children,

    /// The Fallback UI of the current Suspense Component.
    #[prop_or_default]
    pub fallback: Html,
}

#[cfg(any(feature = "csr", feature = "ssr"))]
mod feat_csr_ssr {
    use super::*;
    use crate::html::{Children, Component, Context, Html, Scope};
    use crate::suspense::Suspension;
    #[cfg(feature = "hydration")]
    use crate::suspense::SuspensionHandle;
    use crate::virtual_dom::{VNode, VSuspense};
    use crate::{function_component, html};

    #[derive(Properties, PartialEq, Debug, Clone)]
    pub(crate) struct BaseSuspenseProps {
        pub children: Children,
        pub fallback: Option<Html>,
    }

    #[derive(Debug)]
    pub(crate) enum BaseSuspenseMsg {
        Suspend(Suspension),
        Resume(Suspension),
    }

    #[derive(Debug)]
    pub(crate) struct BaseSuspense {
        suspensions: Vec<Suspension>,
        #[cfg(feature = "hydration")]
        hydration_handle: Option<SuspensionHandle>,
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
            let children = html! {<>{children}</>};

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

        #[cfg(feature = "hydration")]
        fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
            if first_render {
                if let Some(m) = self.hydration_handle.take() {
                    m.resume();
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
    }

    /// Suspend rendering and show a fallback UI until the underlying task completes.
    #[function_component]
    pub fn Suspense(props: &SuspenseProps) -> Html {
        let SuspenseProps { children, fallback } = props.clone();

        let fallback = html! {
            <BaseSuspense fallback={None}>
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
    use crate::function_component;

    /// Suspend rendering and show a fallback UI until the underlying task completes.
    #[function_component]
    pub fn Suspense(_props: &SuspenseProps) -> Html {
        Html::default()
    }
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub use feat_no_csr_ssr::*;
