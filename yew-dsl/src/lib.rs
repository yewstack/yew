//! yew_dsl provides an Rust-based syntax for creating DOM elements.
//! It provides five basic functions with which you should be able to create complex layouts
//! (these are `tag`, `comp`, `text`, `populated_list` and `list`).

pub use crate::vcomp::VCompProducer;
use crate::vlist::VListProducer;
pub use crate::vtag::VTagProducer;
pub use crate::vtext::VTextProducer;
use yew::virtual_dom::VNode;
use yew::Component;

mod vcomp;
mod vlist;
mod vtag;
mod vtext;

use std::cell::RefCell;
use std::rc::Rc;
use yew::html::Scope;

/// A `ScopeHolder` contains a reference to the scope of the parent component.
type ScopeHolder<PARENT> = Rc<RefCell<Option<Scope<PARENT>>>>;

/// `BoxedVNodeProducer` is a wrapper around a function which produces a `VNode`.
pub struct BoxedVNodeProducer<COMP: Component>(Box<dyn FnOnce(ScopeHolder<COMP>) -> VNode>);

impl<COMP: Component> BoxedVNodeProducer<COMP> {
    fn wrap(f: impl FnOnce(ScopeHolder<COMP>) -> VNode + 'static) -> Self {
        BoxedVNodeProducer(Box::new(f))
    }
    fn execute(self, scope: &ScopeHolder<COMP>) -> VNode {
        (self.0)(scope.clone())
    }
    pub fn build(self) -> VNode {
        let scope = ScopeHolder::default();
        self.execute(&scope)
    }
}

impl<COMP: Component> Into<VNode> for BoxedVNodeProducer<COMP> {
    fn into(self) -> VNode {
        self.build()
    }
}

/// Creates HTML tags (e.g. 'span', 'div', etc).
pub fn tag<COMP: Component>(tag: &'static str) -> VTagProducer<COMP> {
    VTagProducer::new(tag)
}

/// Creates child components.
pub fn comp<COMP: Component, CHILD: Component>(props: CHILD::Properties) -> VCompProducer<COMP> {
    VCompProducer::new::<CHILD>(props)
}

/// Creates text nodes.
pub fn text<COMP: Component, TEXT: Into<String> + 'static>(text: TEXT) -> VTextProducer {
    VTextProducer::new::<TEXT>(text)
}

/// Creates new lists populatated with the data supplied to the function.
pub fn populated_list<COMP: Component>(list: Vec<BoxedVNodeProducer<COMP>>) -> VListProducer<COMP> {
    VListProducer::populated_new(list)
}

/// Creates new (empty) lists.
pub fn list<COMP: Component>() -> VListProducer<COMP> {
    VListProducer::new()
}
