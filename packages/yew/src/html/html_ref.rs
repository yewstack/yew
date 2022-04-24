use std::any::Any;
use std::cell::{Ref, RefCell};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::JsValue;
use web_sys::Node;

use super::{AnyScope, Scope};
use crate::BaseComponent;

/// Trait for types that can be referenced with a [`HtmlRef`].
pub trait ErasedStorage {
    /// The type that is actually stored in the ref. This indirection exists to limit code size of
    /// the associated retrieval and storage methods. If you don't care about these intricacies,
    /// using `type Erased = Self` is often a good choice, making the implementations of trait
    /// methods trivial.
    type Erased: 'static + fmt::Debug;
    /// Upcast self into the stored, erased, value.
    fn upcast(self) -> Self::Erased;
    /// Downcast a reference to the erased value into the original value type.
    ///
    /// # Panics
    ///
    /// This can panic when the erased value was derived from a different type than `Self`.
    fn downcast_ref(erased: &Self::Erased) -> &Self;
}

impl<T: BaseComponent> ErasedStorage for Scope<T> {
    type Erased = AnyScope;

    fn upcast(self) -> Self::Erased {
        self.into()
    }

    fn downcast_ref(erased: &Self::Erased) -> &Self {
        erased.try_downcast_ref::<T>().unwrap()
    }
}

impl ErasedStorage for Node {
    type Erased = Node;

    fn upcast(self) -> Node {
        self
    }

    fn downcast_ref(erased: &Self::Erased) -> &Self {
        erased
    }
}

/// Use this type as the [`BaseComponent::Reference`] type when a component can not be referenced.
#[derive(Debug, Clone)]
pub struct NoReference;

impl ErasedStorage for NoReference {
    type Erased = ();

    fn upcast(self) -> Self::Erased {}

    fn downcast_ref(_: &Self::Erased) -> &Self {
        static NO_REF: NoReference = NoReference;
        &NO_REF
    }
}

trait ErasedRef: fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}

impl dyn ErasedRef {
    fn downcast_inner<E: 'static>(&self) -> &RefState<E> {
        self.as_any()
            .downcast_ref()
            .expect("the correct inner ref-type")
    }

    fn downcast<T: ErasedStorage>(&self) -> &RefState<T::Erased> {
        self.downcast_inner()
    }
}

struct RefState<T> {
    binding: RefCell<Option<T>>,
}

impl<T> Default for RefState<T> {
    fn default() -> Self {
        Self {
            binding: Default::default(),
        }
    }
}

impl<T: 'static + fmt::Debug> fmt::Debug for RefState<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.binding.fmt(f)
    }
}

impl<T: 'static + fmt::Debug> ErasedRef for RefState<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn erased_eq(left: &Rc<dyn ErasedRef>, right: &Rc<dyn ErasedRef>) -> bool {
    let thin_left = Rc::as_ptr(left) as *const ();
    let thin_right = Rc::as_ptr(right) as *const ();
    std::ptr::eq(thin_left, thin_right)
}

fn get_erased_ref<E: 'static>(storage: &Rc<dyn ErasedRef>) -> Ref<'_, Option<E>> {
    storage.downcast_inner::<E>().binding.borrow()
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
///             log_ref: HtmlRef::default(),
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
pub struct HtmlRef<T: ErasedStorage> {
    inner: Rc<dyn ErasedRef>,
    _phantom: PhantomData<T>,
}

impl<T: ErasedStorage> std::fmt::Debug for HtmlRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HtmlRef").field("ref", &self.inner).finish()
    }
}

impl<T: ErasedStorage> Clone for HtmlRef<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T: ErasedStorage> Default for HtmlRef<T> {
    fn default() -> Self {
        let inner: Rc<RefState<T::Erased>> = Rc::default();
        Self {
            inner,
            _phantom: PhantomData,
        }
    }
}

impl<T: ErasedStorage> PartialEq for HtmlRef<T> {
    fn eq(&self, othr: &Self) -> bool {
        erased_eq(&self.inner, &othr.inner)
    }
}

impl<T: ErasedStorage> HtmlRef<T> {
    /// Create a new, unbound HtmlRef
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(any(feature = "csr", feature = "ssr"))]
    pub(crate) fn to_erased(&self) -> ErasedHtmlRef {
        Some(self.clone()).into()
    }

    /// Get the referenced value, if it is bound
    pub fn get_ref(&self) -> Option<impl '_ + Deref<Target = T>> {
        let erased_ref = get_erased_ref::<T::Erased>(&self.inner);
        erased_ref.as_ref()?; // TODO: use Ref::filter_map if that becomes stable
        Some(Ref::map(erased_ref, |erased| {
            T::downcast_ref(erased.as_ref().unwrap())
        }))
    }

    /// Get the referenced value, if the HtmlRef is bound
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        Some(self.get_ref()?.clone())
    }
}

impl NodeRef {
    /// Try converting the node reference into another form
    pub fn cast<INTO: AsRef<Node> + From<JsValue>>(&self) -> Option<INTO> {
        let node = self.get();
        node.map(Into::into).map(INTO::from)
    }
}

/// Internal form of a `HtmlRef`, erasing the component type.
/// The type-id is currently not stored, so be careful that the contained scope always has
/// the correct component type.
#[derive(Clone)]
pub(crate) struct ErasedHtmlRef(Rc<dyn ErasedRef>);

impl<T: ErasedStorage> From<Option<HtmlRef<T>>> for ErasedHtmlRef {
    fn from(user_ref: Option<HtmlRef<T>>) -> Self {
        Self(user_ref.unwrap_or_default().inner)
    }
}

impl std::fmt::Debug for ErasedHtmlRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErasedHtmlRef")
            .field("user_ref", &self.0)
            .finish()
    }
}

impl PartialEq for ErasedHtmlRef {
    fn eq(&self, other: &Self) -> bool {
        erased_eq(&self.0, &other.0)
    }
}

impl ErasedHtmlRef {
    /// Upcast an erased html ref. Using a wrong type will cause a panic later when accessing the
    /// value!
    #[cfg(feature = "csr")]
    pub(crate) fn to_unerased<T: ErasedStorage>(&self) -> HtmlRef<T> {
        HtmlRef {
            inner: self.0.clone(),
            _phantom: PhantomData,
        }
    }

    /// Place a Scope in a reference for later use
    pub(crate) fn set<T: ErasedStorage>(&self, next_ref: Option<T>) {
        let next_ref = next_ref.map(|r| r.upcast());
        let inner = self.0.downcast::<T>();
        let mut this = inner.binding.borrow_mut();
        *this = next_ref;
    }

    /// `self` should be bound. Then, behave like
    /// ```ignore
    /// let scope = self."take"().unwrap_or_else(get_scope);
    /// next.set(Some(scope));
    /// *self = next;
    /// ```
    /// but avoid to call `get_scope` when possible
    #[cfg(feature = "csr")]
    pub(crate) fn morph_into<T: ErasedStorage>(&mut self, next: Self) {
        if self == &next {
            return;
        }
        let old = std::mem::replace(&mut self.0, next.0);
        let old = old.downcast::<T>();
        let new = self.0.downcast::<T>();
        *new.binding.borrow_mut() = old.binding.borrow_mut().take();
    }
}

/// An alias for [`HtmlRef`], picking the correct referenced type for a
/// [`Component`](crate::Component)
pub type ComponentRef<COMP> = HtmlRef<<COMP as BaseComponent>::Reference>;

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
pub type NodeRef = HtmlRef<Node>;

/// A ref that can be bound to. See also [`Component::bind_ref`].
#[derive(Debug)]
pub struct BindableRef<T: ErasedStorage> {
    inner: ErasedHtmlRef,
    _phantom: PhantomData<T>,
}

impl<T: ErasedStorage> BindableRef<T> {
    #[cfg(feature = "csr")]
    pub(crate) fn for_ref(inner: &ErasedHtmlRef) -> Self {
        Self {
            inner: inner.clone(),
            _phantom: PhantomData,
        }
    }

    /// Bind a value to the reference
    pub fn bind(&mut self, value: T) {
        self.inner.set::<T>(Some(value))
    }
}
