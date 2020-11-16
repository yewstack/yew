use crate::NodeRef;

cfg_if::cfg_if! {
    if #[cfg(feature = "std_web")] {
        mod std_web;
    } else if #[cfg(feature = "web_sys")] {
        mod web_sys;
    } else if #[cfg(feature = "static_render")] {
        mod smr;
        pub use smr::{Document, Element, Node, Renderer, Window, EventListener};
    }
}

pub trait DomBackend {
    type Element;
    type Node;
    type Document;
    type Window;

    fn get_document() -> Self::Document {
        todo!()
    }

    fn get_origin() -> String {
        format!("blah")
    }

    fn get_host() -> String {
        format!("blah")
    }

    // Element-related methods
    fn element_as_node(element: &Self::Element) -> Self::Node;
    fn element_last_child(element: &Self::Element) -> Option<Self::Element>;
    fn element_remove_child(element: &Self::Element, child: &Self::Element) -> Option<()>;
    fn cast_node_ref<INTO>(node_ref: &NodeRef) -> Option<INTO>;

    // Document-related methods
}
