use crate::html;
use crate::html::{Children, Component, Context, Html, Properties, Scope};
use crate::virtual_dom::{VList, VNode, VSuspense};

use web_sys::Element;

use super::Suspension;

#[derive(Properties, PartialEq, Debug, Clone)]
pub(crate) struct BaseSuspenseProps {
    pub children: Children,

    pub fallback: Html,

    pub suspendible: bool,
}

#[derive(Debug)]
pub(crate) enum BaseSuspenseMsg {
    Suspend(Suspension),
    Resume(Suspension),
}

/// The Implementation of Suspense Component.
#[derive(Debug)]
pub(crate) struct BaseSuspense {
    link: Scope<Self>,
    suspensions: Vec<Suspension>,
    detached_parent: Option<Element>,
}

impl Component for BaseSuspense {
    type Properties = BaseSuspenseProps;
    type Message = BaseSuspenseMsg;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            suspensions: Vec::new(),
            #[cfg(target_arch = "wasm32")]
            detached_parent: web_sys::window()
                .and_then(|m| m.document())
                .and_then(|m| m.create_element("div").ok()),

            #[cfg(not(target_arch = "wasm32"))]
            detached_parent: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Suspend(m) => {
                assert!(
                    ctx.props().suspendible,
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
        let BaseSuspenseProps {
            children, fallback, ..
        } = (*ctx.props()).clone();

        if ctx.props().suspendible {
            let children = VNode::from(VList::with_children(children.into_iter().collect(), None));
            let fallback = (!self.suspensions.is_empty()).then(|| fallback);

            let vsuspense = VSuspense::new(children, fallback, self.detached_parent.clone());
            VNode::from(vsuspense)
        } else {
            html! {<>{children}</>}
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

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct SuspenseProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub fallback: Html,
}

/// Suspend rendering and show a fallback UI until the underlying task completes.
#[derive(Debug)]
pub struct Suspense {}

impl Component for Suspense {
    type Properties = SuspenseProps;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let SuspenseProps { children, fallback } = ctx.props().clone();

        let fallback = html! {
            <BaseSuspense fallback={Html::default()} suspendible={false}>
                {fallback}
            </BaseSuspense>
        };

        html! {
            <BaseSuspense {fallback} suspendible={true}>
                {children}
            </BaseSuspense>
        }
    }
}
