use std::any::Any;
use std::cell::{Ref, RefCell};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use wasm_bindgen::JsValue;
use web_sys::Node;

use crate::BaseComponent;

/// Use this type as the [`BaseComponent::Reference`] type when a component can not be referenced.
#[derive(Debug, Clone)]
pub enum NoReference {}

/// Wrapped reference to another component for later use in lifecycle methods.
///
/// # Example
/// Send messages to a child component
/// ```
/// # use yew::prelude::*;
/// use yew::html::{BindableRef, Scope};
///
/// struct MessageHolder {
///     msg: String,
/// }
///
/// impl ComponentWithRef for MessageHolder {
///     type Message = String;
///     type Properties = ();
///     type Reference = Scope<Self>;
///
///     fn create(ctx: &Context<Self>, bindable_ref: BindableRef<Self::Reference>) -> Self {
///         bindable_ref.bind(ctx.link().clone());
///         Self {
///             msg: "waiting...".to_string(),
///         }
///     }
///
///     fn changed(&mut self, _ctx: &Context<Self>) -> bool {
///         true
///     }
///
///     fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
///         self.msg = message;
///         true
///     }
///
///     fn view(&self, _ctx: &Context<Self>) -> HtmlResult {
///         Ok(html! { <span>{&self.msg}</span> })
///     }
///
///     fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
///
///     fn destroy(&mut self, _ctx: &Context<Self>) {}
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
pub struct HtmlRef<T> {
    inner: Rc<RefCell<Option<T>>>,
}

impl<T> fmt::Debug for HtmlRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HtmlRef").finish_non_exhaustive()
    }
}

impl<T> Clone for HtmlRef<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Default for HtmlRef<T> {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(None)),
        }
    }
}

impl<T> PartialEq for HtmlRef<T> {
    fn eq(&self, othr: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &othr.inner)
    }
}

impl<T: 'static> HtmlRef<T> {
    /// Create a new, unbound HtmlRef
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the referenced value, if it is bound
    pub fn get_ref(&self) -> Option<impl '_ + Deref<Target = T>> {
        let inner = (*self.inner).borrow();
        inner.deref().as_ref()?;
        Some(Ref::map(inner, |t| t.as_ref().unwrap()))
    }

    /// Get the referenced value, if the HtmlRef is bound
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        Some(self.get_ref()?.clone())
    }

    pub(crate) fn to_erased(&self) -> ErasedHtmlRef {
        ErasedHtmlRef {
            place: self.inner.clone() as Rc<RefCell<dyn Any>>,
            setter: Rc::new(|next_value, inner| {
                *inner.downcast_mut::<Option<T>>().unwrap() =
                    next_value.downcast_mut::<Option<T>>().unwrap().take();
            }),
        }
    }
}

type Setter = dyn Fn(&mut dyn Any, &mut dyn Any);
/// Internal form of a `HtmlRef`, erasing the component type.
/// The type-id is currently not stored, so be careful that the contained scope always has
/// the correct component type.
#[derive(Clone)]
pub(crate) struct ErasedHtmlRef {
    place: Rc<RefCell<dyn Any>>,
    setter: Rc<Setter>,
}

impl fmt::Debug for ErasedHtmlRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErasedHtmlRef").finish_non_exhaustive()
    }
}

impl PartialEq for ErasedHtmlRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.place, &other.place)
    }
}

impl ErasedHtmlRef {
    // Create an ErasedHtmlRef where the user is not interested in the value
    pub(crate) fn unbound<T: 'static>() -> Self {
        HtmlRef::<T>::default().to_erased()
    }

    pub(crate) fn set_erased<T: 'static>(&self, next_erased: T) {
        self.debug_assert_internal_type::<T>();
        let mut place = Some(next_erased);
        self.set_erased_impl(&mut place as &mut dyn Any);
        debug_assert!(
            place.is_none(),
            "should have been consumed by internal setter"
        );
    }

    #[cfg(feature = "csr")]
    pub(crate) fn unset_erased<T: 'static>(&self) {
        self.debug_assert_internal_type::<T>();
        let mut place: Option<T> = None;
        self.set_erased_impl(&mut place as &mut dyn Any);
    }

    pub(crate) fn set_erased_impl(&self, next_erased: &mut dyn Any /* &mut Option<T> */) {
        let mut place = self.place.borrow_mut();
        (self.setter)(next_erased, &mut *place);
    }

    /// `self = next`, but also transfer the contained reference from self to next.
    #[cfg(feature = "csr")]
    pub fn morph_erased(&mut self, mut next: Self) {
        if self == &next {
            return;
        }
        std::mem::swap(self, &mut next);
        let mut self_place = self.place.borrow_mut();
        let mut next_place = next.place.borrow_mut();
        (self.setter)(&mut *next_place, &mut *self_place);
    }

    pub(crate) fn debug_assert_internal_type<T: 'static>(&self) {
        debug_assert!(
            (*self.place.borrow()).type_id() == std::any::TypeId::of::<Option<T>>(),
            "Assumed internal type was {}",
            std::any::type_name::<Option<T>>(),
        );
    }

    #[cfg(any(feature = "csr", feature = "ssr"))]
    pub(crate) fn debug_assert_bound<T: 'static>(&self) {
        #[cfg(debug_assertions)]
        assert!(
            (*self.place.borrow()).type_id() == std::any::TypeId::of::<Option<NoReference>>()
                || self.place.borrow().downcast_ref::<Option<T>>().is_some(),
            "Expected that the component ref is bound by the time it has rendered. Did you forget \
             to bind it to a child?"
        );
    }

    /// Get the underlying node from an erased HtmlRef.
    #[cfg(feature = "csr")]
    pub(crate) fn get_node(&self) -> Option<Node> {
        let node = self.place.borrow();
        (*node).downcast_ref::<Option<Node>>().unwrap().clone()
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

impl NodeRef {
    /// Try converting the node reference into another form
    pub fn cast<INTO: AsRef<Node> + From<JsValue>>(&self) -> Option<INTO> {
        let node = self.get();
        node.map(Into::into).map(INTO::from)
    }
}

/// A ref that can be bound to. See also [`Component::bind_ref`].
#[derive(Debug)]
pub struct BindableRef<T> {
    inner: ErasedHtmlRef,
    _phantom: PhantomData<T>,
}

impl<T: 'static> BindableRef<T> {
    pub(crate) fn for_ref(inner: &ErasedHtmlRef) -> Self {
        Self {
            inner: inner.clone(),
            _phantom: PhantomData,
        }
    }

    /// Bind a value to the reference
    pub fn bind(self, value: T) {
        self.inner.set_erased(value)
    }

    /// Prepare to forward the reference to a nested component or element
    pub fn forward(self) -> HtmlRef<T> {
        // A downcast Rc<RefCell<dyn Any>> --> Rc<RefCell<Option<T>>>
        self.inner.debug_assert_internal_type::<T>();
        // SAFETY: we debug assert that the type is the same. We revert the following unsizing
        // coercion:
        let _sanity_check_unsizes = |rc: Rc<RefCell<Option<T>>>| -> Rc<RefCell<dyn Any>> { rc };
        let inner = unsafe {
            let raw: *const RefCell<dyn Any> = Rc::into_raw(self.inner.place.clone());
            Rc::from_raw(raw as *const RefCell<Option<T>>)
        };
        HtmlRef { inner }
    }
}

#[doc(hidden)]
impl BindableRef<NoReference> {
    pub fn fake_bind(self) {
        let this = self.inner;
        this.debug_assert_internal_type::<NoReference>();
        // no value to bind though!
    }
}
