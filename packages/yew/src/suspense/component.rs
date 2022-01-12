use crate::html::{Children, Component, Context, Html, Properties, Scope};
use crate::virtual_dom::{Key, VList, VNode, VSuspense};

use web_sys::Element;

use super::Suspension;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct SuspenseProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub fallback: Html,

    #[prop_or_default]
    pub key: Option<Key>,
}

#[derive(Debug)]
pub enum SuspenseMsg {
    Suspend(Suspension),
    Resume(Suspension),
}

/// Suspend rendering and show a fallback UI until the underlying task completes.
#[derive(Debug)]
pub struct Suspense {
    link: Scope<Self>,
    suspensions: Vec<Suspension>,
    detached_parent: Option<Element>,
}

impl Component for Suspense {
    type Properties = SuspenseProps;
    type Message = SuspenseMsg;

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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Suspend(m) => {
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
        let SuspenseProps {
            children,
            fallback: fallback_vnode,
            key,
        } = (*ctx.props()).clone();

        let children_vnode =
            VNode::from(VList::with_children(children.into_iter().collect(), None));

        let vsuspense = VSuspense::new(
            children_vnode,
            fallback_vnode,
            self.detached_parent.clone(),
            !self.suspensions.is_empty(),
            key,
        );

        VNode::from(vsuspense)
    }
}

impl Suspense {
    pub(crate) fn suspend(&self, s: Suspension) {
        self.link.send_message(SuspenseMsg::Suspend(s));
    }

    pub(crate) fn resume(&self, s: Suspension) {
        self.link.send_message(SuspenseMsg::Resume(s));
    }
}
