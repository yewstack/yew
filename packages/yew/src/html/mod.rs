//! The main html module which defines components, listeners, and class helpers.

mod classes;
mod component;
mod conversion;
mod error;
mod listener;

use std::cell::RefCell;
use std::rc::Rc;

pub use classes::*;
pub use component::*;
pub use conversion::*;
pub use error::*;
pub use listener::*;
use wasm_bindgen::JsValue;
use web_sys::{Element, Node};

use crate::sealed::Sealed;
use crate::virtual_dom::{VNode, VPortal};

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
/// # use yew::prelude::*;
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
}

#[cfg(feature = "csr")]
mod feat_csr {
    use super::*;

    impl NodeRef {
        /// Link a downstream `NodeRef`
        pub(crate) fn link(&self, node_ref: Self) {
            // Avoid circular references
            debug_assert!(
                self != &node_ref,
                "no circular references allowed! Report this as a bug in yew"
            );

            let mut this = self.0.borrow_mut();
            this.node = None;
            this.link = Some(node_ref);
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
    }
}

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    #[cfg(debug_assertions)]
    thread_local! {
        // A special marker element that should not be referenced
        static TRAP: Node = gloo::utils::document().create_element("div").unwrap().into();
    }

    impl NodeRef {
        // A new "placeholder" node ref that should not be accessed
        #[inline]
        pub(crate) fn new_debug_trapped() -> Self {
            #[cfg(debug_assertions)]
            {
                Self::new(TRAP.with(|trap| trap.clone()))
            }
            #[cfg(not(debug_assertions))]
            {
                Self::default()
            }
        }

        #[inline]
        pub(crate) fn debug_assert_not_trapped(&self) {
            #[cfg(debug_assertions)]
            TRAP.with(|trap| {
                assert!(
                    self.get().as_ref() != Some(trap),
                    "should not use a trapped node ref"
                )
            })
        }
    }
}

/// Render children into a DOM node that exists outside the hierarchy of the parent
/// component.
/// ## Relevant examples
/// - [Portals](https://github.com/yewstack/yew/tree/master/examples/portals)
pub fn create_portal(child: Html, host: Element) -> Html {
    VNode::VPortal(VPortal::new(child, host))
}
