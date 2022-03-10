use crate::html::{Children, Html, Properties};

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct SuspenseProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub fallback: Html,
}

#[cfg(any(feature = "render", feature = "ssr"))]
mod feat_render_ssr {
    use super::*;

    use web_sys::Element;

    use crate::html::{Children, Component, Context, Html, Scope};
    use crate::suspense::Suspension;
    use crate::virtual_dom::{VList, VNode, VSuspense};
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
        link: Scope<Self>,
        suspensions: Vec<Suspension>,
        detached_parent: Option<Element>,
    }

    impl Component for BaseSuspense {
        type Properties = BaseSuspenseProps;
        type Message = BaseSuspenseMsg;

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                link: _ctx.link().clone(),
                suspensions: Vec::new(),

                #[cfg(target_arch = "wasm32")]
                detached_parent: web_sys::window()
                    .and_then(|m| m.document())
                    .and_then(|m| m.create_element("div").ok()),

                #[cfg(not(target_arch = "wasm32"))]
                detached_parent: None,
            }
        }

        fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                Self::Message::Suspend(m) => {
                    assert!(
                        _ctx.props().fallback.is_some(),
                        "You cannot suspend from a component rendered as a fallback."
                    );

                    if m.resumed() {
                        return false;
                    }

                    m.listen(self.link.callback(Self::Message::Resume));

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

            match fallback {
                Some(fallback) => {
                    let children =
                        VNode::from(VList::with_children(children.into_iter().collect(), None));

                    let vsuspense = VSuspense::new(
                        children,
                        fallback,
                        self.detached_parent.clone(),
                        !self.suspensions.is_empty(),
                        // We don't need to key this as the key will be applied to the component.
                        None,
                    );

                    VNode::from(vsuspense)
                }
                None => html! {<>{children}</>},
            }
        }
    }

    impl BaseSuspense {
        pub(crate) fn suspend(&self, s: Suspension) {
            self.link.send_message(BaseSuspenseMsg::Suspend(s));
        }

        pub(crate) fn resume(&self, s: Suspension) {
            self.link.send_message(BaseSuspenseMsg::Resume(s));
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

#[cfg(any(feature = "render", feature = "ssr"))]
pub use feat_render_ssr::*;

#[cfg(not(any(feature = "ssr", feature = "render")))]
mod feat_no_render_ssr {
    use super::*;

    use crate::function_component;

    /// Suspend rendering and show a fallback UI until the underlying task completes.
    #[function_component]
    pub fn Suspense(_props: &SuspenseProps) -> Html {
        Html::default()
    }
}

#[cfg(not(any(feature = "ssr", feature = "render")))]
pub use feat_no_render_ssr::*;
