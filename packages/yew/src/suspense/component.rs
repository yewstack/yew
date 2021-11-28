use crate::html::{Children, Component, Context, Html, Properties, Scope};
use crate::virtual_dom::{Key, VList, VNode, VSuspense};

use gloo_utils::document;
use web_sys::Element;

use super::Suspension;

#[derive(Properties, PartialEq, Debug)]
pub struct SuspenseProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub fallback: Children,

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
    detached_parent: Element,
}

impl Component for Suspense {
    type Properties = SuspenseProps;
    type Message = SuspenseMsg;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            suspensions: Vec::new(),
            detached_parent: document().create_element("div").unwrap(),
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
        let children_vnode = VNode::from(VList::with_children(
            ctx.props().children.clone().into_iter().collect(),
            None,
        ));
        let fallback_vnode = VNode::from(VList::with_children(
            ctx.props().fallback.clone().into_iter().collect(),
            None,
        ));

        let vsuspense = VSuspense::new(
            children_vnode,
            fallback_vnode,
            self.detached_parent.clone(),
            !self.suspensions.is_empty(),
            ctx.props().key.clone(),
        );

        Ok(VNode::from(vsuspense))
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
