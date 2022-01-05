//! The main html module which defines components, listeners, and class helpers.

mod classes;
mod component;
mod conversion;
mod error;
mod listener;

pub use classes::*;
pub use component::*;
pub use conversion::*;
pub use error::*;
pub use listener::*;

use crate::sealed::Sealed;
use crate::virtual_dom::{VNode, VPortal};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{Element, Node};

/// A type which expected as a result of `view` function implementation.
pub type Html = VNode;

/// An enhanced type of `Html` returned in suspendible function components.
pub type HtmlResult = RenderResult<Html>;

impl Sealed for HtmlResult {}
impl Sealed for Html {}

/// A trait to translate into a [`HtmlResult`].
pub trait IntoHtmlResult: Sealed {
    /// Performs the conversion.
    fn into_html_result(self) -> HtmlResult;
}

impl IntoHtmlResult for HtmlResult {
    #[inline(always)]
    fn into_html_result(self) -> HtmlResult {
        self
    }
}
impl IntoHtmlResult for Html {
    #[inline(always)]
    fn into_html_result(self) -> HtmlResult {
        Ok(self)
    }
}

/// Wrapped Node reference for later use in Component lifecycle methods.
///
/// # Example
/// Focus an `<input>` element on mount.
/// ```
/// use web_sys::HtmlInputElement;
///# use yew::prelude::*;
///
/// pub struct Input {
///     node_ref: NodeRef,
/// }
///
/// impl Component for Input {
///     type Message = ();
///     type Properties = ();
///
///     fn create(_ctx: &Context<Self>) -> Self {
///         Input {
///             node_ref: NodeRef::default(),
///         }
///     }
///
///     fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
///         if first_render {
///             if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
///                 input.focus();
///             }
///         }
///     }
///
///     fn view(&self, _ctx: &Context<Self>) -> Html {
///         html! {
///             <input ref={self.node_ref.clone()} type="text" />
///         }
///     }
/// }
/// ```
/// ## Relevant examples
/// - [Node Refs](https://github.com/yewstack/yew/tree/master/examples/node_refs)
#[derive(Default, Clone)]
pub struct NodeRef(Rc<RefCell<NodeRefInner>>);

impl PartialEq for NodeRef {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr() == other.0.as_ptr() || Some(self) == other.0.borrow().link.as_ref()
    }
}

impl std::fmt::Debug for NodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeRef {{ references: {:?} }}",
            self.get().map(|n| crate::utils::print_node(&n))
        )
    }
}

#[derive(PartialEq, Debug, Default, Clone)]
struct NodeRefInner {
    node: Option<Node>,
    link: Option<NodeRef>,
}

impl NodeRef {
    /// Get the wrapped Node reference if it exists
    pub fn get(&self) -> Option<Node> {
        let inner = self.0.borrow();
        inner.node.clone().or_else(|| inner.link.as_ref()?.get())
    }

    /// Try converting the node reference into another form
    pub fn cast<INTO: AsRef<Node> + From<JsValue>>(&self) -> Option<INTO> {
        let node = self.get();
        node.map(Into::into).map(INTO::from)
    }

    /// Wrap an existing `Node` in a `NodeRef`
    pub(crate) fn new(node: Node) -> Self {
        let node_ref = NodeRef::default();
        node_ref.set(Some(node));
        node_ref
    }

    /// Place a Node in a reference for later use
    pub(crate) fn set(&self, node: Option<Node>) {
        let mut this = self.0.borrow_mut();
        this.node = node;
        this.link = None;
    }

    /// Link a downstream `NodeRef`
    pub(crate) fn link(&self, node_ref: Self) {
        // Avoid circular references
        if self == &node_ref {
            return;
        }

        let mut this = self.0.borrow_mut();
        this.node = None;
        this.link = Some(node_ref);
    }

    /// Reuse an existing `NodeRef`
    pub(crate) fn reuse(&self, node_ref: Self) {
        // Avoid circular references
        if self == &node_ref {
            return;
        }

        let mut this = self.0.borrow_mut();
        let existing = node_ref.0.borrow();
        this.node = existing.node.clone();
        this.link = existing.link.clone();
    }
}

/// Render children into a DOM node that exists outside the hierarchy of the parent
/// component.
/// ## Relevant examples
/// - [Portals](https://github.com/yewstack/yew/tree/master/examples/portals)
pub fn create_portal(child: Html, host: Element) -> Html {
    VNode::VPortal(VPortal::new(child, host))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn self_linking_node_ref() {
        let node: Node = document().create_text_node("test node").into();
        let node_ref = NodeRef::new(node.clone());
        let node_ref_2 = NodeRef::new(node.clone());

        // Link to self
        node_ref.link(node_ref.clone());
        assert_eq!(node, node_ref.get().unwrap());

        // Create cycle of two node refs
        node_ref.link(node_ref_2.clone());
        node_ref_2.link(node_ref);
        assert_eq!(node, node_ref_2.get().unwrap());
    }
}
