pub use crate::dsl::vcomp::VCompProducer;
use crate::dsl::vlist::VListProducer;
pub use crate::dsl::vtag::VTagProducer;
pub use crate::dsl::vtext::VTextProducer;
use yew::virtual_dom::vcomp::ScopeHolder;
use yew::virtual_dom::VNode;
use yew::Component;

mod vcomp;
mod vlist;
mod vtag;
mod vtext;

/// Wrapper around a function that produces a vnode.
pub struct BoxedVNodeProducer<COMP: Component>(Box<dyn FnOnce(ScopeHolder<COMP>) -> VNode<COMP>>);

impl<COMP: Component> BoxedVNodeProducer<COMP> {
    fn wrap(f: impl FnOnce(ScopeHolder<COMP>) -> VNode<COMP> + 'static) -> Self {
        BoxedVNodeProducer(Box::new(f))
    }
    fn execute(self, scope: &ScopeHolder<COMP>) -> VNode<COMP> {
        (self.0)(scope.clone())
    }
    pub fn build(self) -> VNode<COMP> {
        let scope = ScopeHolder::default();
        self.execute(&scope)
    }
}

impl<COMP: Component> Into<VNode<COMP>> for BoxedVNodeProducer<COMP> {
    fn into(self) -> VNode<COMP> {
        self.build()
    }
}

/// Creates a tag node.
pub fn tag<COMP: Component>(tag: &'static str) -> VTagProducer<COMP> {
    VTagProducer::new(tag)
}

/// Creates a component (Specified by the second type parameter).
pub fn comp<COMP: Component, CHILD: Component>(props: CHILD::Properties) -> VCompProducer<COMP> {
    VCompProducer::new::<CHILD>(props)
}

/// Creates a text node
pub fn text<COMP: Component, T: Into<String> + 'static>(text: T) -> VTextProducer<COMP> {
    VTextProducer::new::<T>(text)
}

/// Creates a new vlist, populated with the provided vnodes
pub fn populated_list<COMP: Component>(list: Vec<BoxedVNodeProducer<COMP>>) -> VListProducer<COMP> {
    VListProducer::populated_new(list)
}

/// Creates a new vlist
pub fn list<COMP: Component>() -> VListProducer<COMP> {
    VListProducer::new()
}
