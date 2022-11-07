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
    use std::cell::RefCell;

    use super::*;
    use crate::html::{Children, Html, Scope};
    use crate::suspense::Suspension;
    use crate::virtual_dom::{VNode, VSuspense};
    use crate::{
        function_component, html, use_reducer, ContextProvider, Reducible, UseReducerDispatcher,
    };

    #[derive(Default)]
    pub(crate) struct Suspensions {
        inner: RefCell<Vec<Suspension>>,
    }

    impl Reducible for Suspensions {
        type Action = BaseSuspenseMsg;

        fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
            {
                let mut inner = self.inner.borrow_mut();

                match action {
                    BaseSuspenseMsg::Resume(m) => {
                        inner.retain(|n| &m != n);
                    }
                    BaseSuspenseMsg::Suspend(m) => {
                        if m.resumed() {
                            drop(inner);
                            return self;
                        }
                        inner.push(m);
                    }
                }
            }

            self
        }
    }

    pub(crate) type DispatchSuspension = UseReducerDispatcher<Suspensions>;

    #[derive(Properties, PartialEq, Debug, Clone)]
    pub(crate) struct BaseSuspenseProps {
        pub children: Children,
        pub fallback: Option<Html>,
    }

    #[function_component]
    pub(crate) fn BaseSuspense(props: &BaseSuspenseProps) -> Html {
        let suspensions = use_reducer(Suspensions::default);

        let has_suspension = !suspensions.inner.borrow().is_empty();

        let BaseSuspenseProps { children, fallback } = props.clone();
        let dispatch_suspension = suspensions.dispatcher();

        let children = html! {
            <ContextProvider<DispatchSuspension> context={dispatch_suspension}>
                {children}
            </ContextProvider<DispatchSuspension>>
        };

        #[cfg(debug_assertions)]
        if has_suspension && fallback.is_none() {
            panic!("You cannot suspend from a fallback component.");
        }

        match fallback {
            Some(fallback) => {
                let vsuspense = VSuspense::new(
                    children,
                    fallback,
                    has_suspension,
                    // We don't need to key this as the key will be applied to the component.
                    None,
                );

                VNode::from(vsuspense)
            }
            None => children,
        }
    }

    pub(crate) fn resume_suspension(
        provider: &Scope<ContextProvider<DispatchSuspension>>,
        s: Suspension,
    ) {
        if let Some(provider) = provider.get_component() {
            let context = provider.get_context_value();
            context.dispatch(BaseSuspenseMsg::Resume(s));
        }
    }

    pub(crate) fn suspend_suspension(
        provider: &Scope<ContextProvider<DispatchSuspension>>,
        s: Suspension,
    ) {
        if let Some(provider) = provider.get_component() {
            let context = provider.get_context_value();
            context.dispatch(BaseSuspenseMsg::Suspend(s));
        }
    }

    #[derive(Debug)]
    pub(crate) enum BaseSuspenseMsg {
        Suspend(Suspension),
        Resume(Suspension),
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
