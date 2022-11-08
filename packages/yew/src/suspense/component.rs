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

#[cfg(feature = "csr")]
mod feat_csr {

    use super::*;
    use crate::context::ContextStore;
    use crate::html::{AnyScope, Scope};
    use crate::suspense::Suspension;
    use crate::ContextProvider;

    #[cfg(feature = "csr")]
    pub(crate) fn resume_suspension(
        provider: &Scope<ContextProvider<DispatchSuspension>>,
        s: Suspension,
    ) {
        if let Some(provider) =
            ContextStore::<DispatchSuspension>::get(&AnyScope::from(provider.clone()))
        {
            let context = provider.borrow().get_context_value();
            context.dispatch(SuspensionsAction::Resume(s));
        }
    }

    #[cfg(feature = "csr")]
    pub(crate) fn suspend_suspension(
        provider: &Scope<ContextProvider<DispatchSuspension>>,
        s: Suspension,
    ) {
        if let Some(provider) =
            ContextStore::<DispatchSuspension>::get(&AnyScope::from(provider.clone()))
        {
            let context = provider.borrow().get_context_value();
            context.dispatch(SuspensionsAction::Suspend(s));
        }
    }
}

#[cfg(feature = "csr")]
pub(crate) use feat_csr::*;

#[cfg(any(feature = "csr", feature = "ssr"))]
mod feat_csr_ssr {

    use std::cell::RefCell;

    use super::*;
    use crate::html::{Children, Html};
    use crate::suspense::Suspension;
    use crate::virtual_dom::{VNode, VSuspense};
    use crate::{
        function_component, html, use_reducer, ContextProvider, Reducible, UseReducerDispatcher,
    };

    pub(crate) type DispatchSuspension = UseReducerDispatcher<Suspensions>;

    #[derive(Properties, PartialEq, Debug, Clone)]
    pub(crate) struct BaseSuspenseProps {
        pub children: Children,
        pub fallback: Option<Html>,
    }

    #[derive(Debug)]
    pub(crate) enum SuspensionsAction {
        #[cfg(feature = "csr")]
        Suspend(Suspension),
        #[cfg(feature = "csr")]
        Resume(Suspension),
    }

    #[derive(Default)]
    pub(crate) struct Suspensions {
        inner: RefCell<Vec<Suspension>>,
    }

    impl Reducible for Suspensions {
        type Action = SuspensionsAction;

        fn reduce(self: std::rc::Rc<Self>, _action: Self::Action) -> std::rc::Rc<Self> {
            #[cfg(feature = "csr")]
            {
                let mut inner = self.inner.borrow_mut();

                match _action {
                    SuspensionsAction::Resume(m) => {
                        inner.retain(|n| &m != n);
                    }
                    SuspensionsAction::Suspend(m) => {
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
