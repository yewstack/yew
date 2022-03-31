//! The main html module which defines components, listeners, and class helpers.

mod classes;
mod component;
mod conversion;
mod error;
mod listener;

use std::cell::{Ref, RefCell};
use std::marker::PhantomData;
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

    /// Place a Node in a reference for later use
    pub(crate) fn set(&self, node: Option<Node>) {
        let mut this = self.0.borrow_mut();
        this.node = node;
        this.link = None;
    }
}

/// Wrapped reference to another component for later use in lifecycle methods.
///
/// # Example
/// Send messages to a child component
/// ```
/// # use yew::prelude::*;
///
/// struct MessageHolder {
///     msg: String,
/// }
///
/// impl Component for MessageHolder {
///     type Message = String;
///     type Properties = ();
///
///     fn create(_ctx: &Context<Self>) -> Self {
///         Self {
///             msg: "waiting...".to_string(),
///         }
///     }
///
///     fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
///         self.msg = message;
///         true
///     }
///
///     fn view(&self, _ctx: &Context<Self>) -> Html {
///         html! { <span>{&self.msg}</span> }
///     }
/// }
///
/// pub struct Controller {
///     log_ref: ComponentRef<MessageHolder>,
/// }
///
/// impl Component for Controller {
///     type Message = ();
///     type Properties = ();
///
///     fn create(_ctx: &Context<Self>) -> Self {
///         Self {
///             log_ref: ComponentRef::default(),
///         }
///     }
///
///     fn view(&self, _ctx: &Context<Self>) -> Html {
///         let onclick = {
///             let log_ref = self.log_ref.clone();
///             Callback::from(move |_| {
///                 log_ref
///                     .get()
///                     .expect("a message holder")
///                     .send_message("example message".to_string())
///             })
///         };
///         html! {
///             <>
///                 <MessageHolder ref={&self.log_ref} />
///                 <button {onclick}>{"Send example message"}</button>
///             </>
///         }
///     }
/// }
/// ```
/// ## Relevant examples
/// - [`nested_list`](https://github.com/yewstack/yew/tree/master/examples/nested_list)
pub struct ComponentRef<COMP: BaseComponent>(Rc<RefCell<CompRefInner>>, PhantomData<COMP>);

impl<COMP: BaseComponent> std::fmt::Debug for ComponentRef<COMP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ComponentAnyRef {{ scope: {:?} }}", self.get_scope())
    }
}

impl<COMP: BaseComponent> Clone for ComponentRef<COMP> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<COMP: BaseComponent> Default for ComponentRef<COMP> {
    fn default() -> Self {
        Self(Rc::default(), PhantomData)
    }
}

impl<COMP: BaseComponent> PartialEq for ComponentRef<COMP> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<COMP: BaseComponent> ComponentRef<COMP> {
    fn get_scope(&self) -> Ref<'_, Option<AnyScope>> {
        Ref::map(self.0.borrow(), |s| &s.scope)
    }
}

impl<COMP: Component> ComponentRef<COMP> {
    /// Create a new, unbound component ref
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the scope of the referenced node, if it exists
    pub fn get(&self) -> Option<Scope<COMP>> {
        Some(self.get_scope().as_ref()?.downcast::<COMP>())
    }
}

/// Internal form of a `ComponentRef`, erasing the component type.
/// The type-id is currently not stored, so be careful that the contained scope always has
/// the correct component type.
#[derive(Default, Clone)]
pub(crate) struct ErasedComponentRef(Option<Rc<RefCell<CompRefInner>>>);

impl<COMP: BaseComponent> From<Option<ComponentRef<COMP>>> for ErasedComponentRef {
    fn from(user_ref: Option<ComponentRef<COMP>>) -> Self {
        match user_ref {
            Some(user_ref) => Self(Some(user_ref.0)),
            None => Self(None),
        }
    }
}

impl std::fmt::Debug for ErasedComponentRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref inner) = self.0 {
            write!(f, "ComponentAnyRef {{ scope: {:?} }}", inner.borrow().scope)
        } else {
            write!(f, "ComponentAnyRef(unbound)")
        }
    }
}

impl PartialEq for ErasedComponentRef {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (None, None) => true,
            (Some(ref l), Some(ref r)) => Rc::ptr_eq(l, r),
            _ => false,
        }
    }
}

#[derive(Default)]
struct CompRefInner {
    scope: Option<AnyScope>,
}

#[cfg(feature = "csr")]
mod feat_csr {
    use super::*;

    impl NodeRef {
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

        /// Wrap an existing `Node` in a `NodeRef`
        pub(crate) fn new(node: Node) -> Self {
            let node_ref = NodeRef::default();
            node_ref.set(Some(node));
            node_ref
        }
    }

    impl ErasedComponentRef {
        /// Place a Scope in a reference for later use
        pub(crate) fn set(&self, scope: Option<AnyScope>) {
            if let Some(ref inner) = self.0 {
                let mut this = inner.borrow_mut();
                this.scope = scope;
            }
        }

        /// `self` should be bound. Then, behave like
        /// ```ignore
        /// let scope = self."take"().unwrap_or_else(get_scope);
        /// next.set(Some(scope));
        /// *self = next;
        /// ```
        /// but avoid to call `get_scope` when possible
        pub(crate) fn morph_into(&mut self, next: Self, get_scope: impl FnOnce() -> AnyScope) {
            if self == &next {
                return;
            }
            let old = std::mem::replace(&mut self.0, next.0);
            let old_scope = old.and_then(|old| {
                // debug_assert!(old.borrow().scope == Some(get_scope()));
                old.borrow_mut().scope.take()
            });
            let new = match self.0 {
                Some(ref inner) => inner,
                None => return,
            };
            new.borrow_mut().scope = Some(old_scope.unwrap_or_else(get_scope));
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

#[cfg(feature = "wasm_test")]
#[cfg(test)]
mod tests {
    use gloo_utils::document;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use super::*;

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
