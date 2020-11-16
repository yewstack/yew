pub mod mock;
mod smr;
use crate::Component;
pub use smr::*;

// use gloo::events::EventListener;
// use web_sys::{Element, Node};

/// Mounts a component, establishes the virtual dom, and updates when props change
struct StaticRenderer<COMP: Component> {
    component: std::marker::PhantomData<COMP>,
    // components: indexmap::IndexMap<>
}

use crate::Properties;
#[derive(Properties, Clone)]
struct EmptyProps {}

impl<COMP: Component> StaticRenderer<COMP> {
    fn mount_with_props(props: COMP::Properties) -> Self {
        Self {
            component: std::marker::PhantomData {},
        }
    }

    fn render(&self) -> String {
        format!("blah")
    }
}
