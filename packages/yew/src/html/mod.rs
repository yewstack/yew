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
        self.0.as_ptr() == other.0.as_ptr()
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
}

impl NodeRef {
    /// Get the wrapped Node reference if it exists
    pub fn get(&self) -> Option<Node> {
        let inner = self.0.borrow();
        inner.node.clone()
    }

    /// Try converting the node reference into another form
    pub fn cast<INTO: AsRef<Node> + From<JsValue>>(&self) -> Option<INTO> {
        let node = self.get();
        node.map(Into::into).map(INTO::from)
    }
}

#[derive(Clone)]
pub(crate) struct RetargetableDomPosition {
    target: Rc<RefCell<DomPosition>>,
}

#[derive(Clone)]
enum DomPositionVariant {
    Node(Option<Node>),
    Chained(RetargetableDomPosition),
}

#[derive(Clone)]
pub(crate) struct DomPosition {
    variant: DomPositionVariant,
}

impl std::fmt::Debug for DomPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DomPosition {{ next_sibling: {:?} }}",
            self.get().map(|n| crate::utils::print_node(&n))
        )
    }
}

impl std::fmt::Debug for RetargetableDomPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", *self.target.borrow())
    }
}

impl DomPosition {
    pub(crate) fn get(&self) -> Option<Node> {
        // we use an iterative approach to traverse a possible long chain for references
        // see for example issue #3043 why a recursive call is impossible for large lists in vdom

        // TODO: there could be some data structure that performs better here. E.g. a balanced tree
        // with parent pointers come to mind, but they are a bit fiddly to implement in rust
        let node = match &self.variant {
            DomPositionVariant::Node(ref n) => n.clone(),
            DomPositionVariant::Chained(ref chain) => {
                let mut this = chain.target.clone();
                loop {
                    //                          v------- borrow lives for this match expression
                    let next_this = match &this.borrow().variant {
                        DomPositionVariant::Node(ref n) => break n.clone(),
                        // We clone an Rc here temporarily, so that we don't have to consume stack
                        // space. The alternative would be to keep the
                        // `Ref<'_, DomPosition>` above in some temporary buffer
                        DomPositionVariant::Chained(ref chain) => chain.target.clone(),
                    };
                    this = next_this;
                }
            }
        };

        #[cfg(feature = "csr")]
        #[cfg(debug_assertions)]
        feat_csr::TRAP.with(|trap| {
            assert!(
                node.as_ref() != Some(trap),
                "should not use a trapped node ref"
            )
        });
        node
    }
}

#[cfg(feature = "csr")]
mod feat_csr {
    use super::*;

    impl NodeRef {
        pub(crate) fn set(&self, new_ref: Option<Node>) {
            let mut inner = self.0.borrow_mut();
            inner.node = new_ref;
        }
    }

    impl DomPosition {
        pub(crate) fn new(node: Node) -> Self {
            Self::create(Some(node))
        }

        pub(crate) fn create(node: Option<Node>) -> Self {
            Self {
                variant: DomPositionVariant::Node(node),
            }
        }

        pub(crate) fn at_end() -> Self {
            Self {
                variant: DomPositionVariant::Node(None),
            }
        }

        // A new "placeholder" node ref that should not be accessed
        #[inline]
        pub(crate) fn new_debug_trapped() -> Self {
            #[cfg(debug_assertions)]
            {
                Self::new(TRAP.with(|trap| trap.clone()))
            }
            #[cfg(not(debug_assertions))]
            {
                Self::at_end()
            }
        }
    }

    #[cfg(debug_assertions)]
    thread_local! {
        // A special marker element that should not be referenced
        pub(super) static TRAP: Node = gloo::utils::document().create_element("div").unwrap().into();
    }

    impl RetargetableDomPosition {
        pub(crate) fn new(initial_position: DomPosition) -> Self {
            Self {
                target: Rc::new(RefCell::new(initial_position)),
            }
        }

        pub(crate) fn new_debug_trapped() -> Self {
            Self::new(DomPosition::new_debug_trapped())
        }

        pub(crate) fn retarget(&self, next_position: DomPosition) {
            *self.target.borrow_mut() = next_position;
        }

        pub(crate) fn as_position(&self) -> DomPosition {
            DomPosition {
                variant: DomPositionVariant::Chained(self.clone()),
            }
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
