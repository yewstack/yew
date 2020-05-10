use crate::BoxedVNodeProducer;
use yew::virtual_dom::VComp;
use yew::{Component, NodeRef};

use crate::ScopeHolder;

/// Creates virtual components.
pub struct VCompProducer<COMP: Component>(Box<dyn FnOnce(ScopeHolder<COMP>) -> VComp>);

impl<COMP: Component> VCompProducer<COMP> {
    pub fn new<CHILD: Component>(props: CHILD::Properties) -> Self {
        // TODO allow getting the noderef as a parameter somewhere.
        VCompProducer(Box::new(move |scope| {
            VComp::new::<CHILD>(props, NodeRef::default(), None)
        }))
    }
}

impl<COMP: Component> From<VCompProducer<COMP>> for BoxedVNodeProducer<COMP> {
    fn from(vcomp_prod: VCompProducer<COMP>) -> Self {
        BoxedVNodeProducer::wrap(move |scope| (vcomp_prod.0)(scope).into())
    }
}
