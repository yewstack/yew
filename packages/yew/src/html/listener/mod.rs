#[macro_use]
mod events;

pub use events::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget};

use crate::Callback;

/// Cast [Event] `e` into it's target `T`.
///
/// This function mainly exists to provide type inference in the [impl_action] macro to the compiler
/// and avoid some verbosity by not having to type the signature over and over in closure
/// definitions.
#[inline]
pub(crate) fn cast_event<T>(e: Event) -> T
where
    T: JsCast,
{
    e.unchecked_into()
}

/// A trait to obtain a generic event target.
///
/// The methods in this trait are convenient helpers that use the [`JsCast`] trait internally
/// to do the conversion.
pub trait TargetCast
where
    Self: AsRef<Event>,
{
    /// Performs a dynamic cast (checked at runtime) of this events target into the type `T`.
    ///
    /// This method can return [`None`] for two reasons:
    /// - The event's target was [`None`]
    /// - The event's target type did not match `T`
    ///
    /// # Example
    ///
    /// ```
    /// use web_sys::HtmlTextAreaElement;
    /// use yew::prelude::*;
    /// # enum Msg {
    /// #   Value(String),
    /// # }
    /// # struct Comp;
    /// # impl Component for Comp {
    /// # type Message = Msg;
    /// # type Properties = ();
    /// # fn create(ctx: &Context<Self>) -> Self {
    /// #   Self
    /// # }
    ///
    /// fn view(&self, ctx: &Context<Self>) -> Html {
    ///     html! {
    ///         <div
    ///             onchange={ctx.link().batch_callback(|e: Event| {
    ///                 if let Some(input) = e.target_dyn_into::<HtmlTextAreaElement>() {
    ///                     Some(Msg::Value(input.value()))
    ///                 } else {
    ///                     None
    ///                 }
    ///             })}
    ///         >
    ///             <textarea />
    ///             <input type="text" />
    ///         </div>
    ///     }
    /// }
    /// # }
    /// ```
    /// _Note: if you can apply the [`Callback`] directly onto an element which doesn't have a child
    /// consider using [`TargetCast::target_unchecked_into<T>`]_
    #[inline]
    fn target_dyn_into<T>(&self) -> Option<T>
    where
        T: AsRef<EventTarget> + JsCast,
    {
        self.as_ref()
            .target()
            .and_then(|target| target.dyn_into().ok())
    }

    #[inline]
    /// Performs a zero-cost unchecked cast of this events target into the type `T`.
    ///
    /// This method **does not check whether the event target is an instance of `T`**. If used
    /// incorrectly then this method may cause runtime exceptions in both Rust and JS, this should
    /// be used with caution.
    ///
    /// A common safe usage of this method is within a [`Callback`] that is applied directly to an
    /// element that has no children, thus `T` will be the type of the element the [`Callback`] is
    /// applied to.
    ///
    /// # Example
    ///
    /// ```
    /// use web_sys::HtmlInputElement;
    /// use yew::prelude::*;
    /// # enum Msg {
    /// #   Value(String),
    /// # }
    /// # struct Comp;
    /// # impl Component for Comp {
    /// # type Message = Msg;
    /// # type Properties = ();
    /// # fn create(ctx: &Context<Self>) -> Self {
    /// #   Self
    /// # }
    ///
    /// fn view(&self, ctx: &Context<Self>) -> Html {
    ///     html! {
    ///         <input type="text"
    ///             onchange={ctx.link().callback(|e: Event| {
    ///                 // Safe to use as callback is on an `input` element so this event can
    ///                 // only come from this input!
    ///                 let input: HtmlInputElement = e.target_unchecked_into();
    ///                 Msg::Value(input.value())
    ///             })}
    ///         />
    ///     }
    /// }
    /// # }
    /// ```
    fn target_unchecked_into<T>(&self) -> T
    where
        T: AsRef<EventTarget> + JsCast,
    {
        self.as_ref().target().unwrap().unchecked_into()
    }
}

impl<E: AsRef<Event>> TargetCast for E {}

/// A trait similar to `Into<T>` which allows conversion of a value into a [`Callback`].
/// This is used for event listeners.
pub trait IntoEventCallback<EVENT> {
    /// Convert `self` to `Option<Callback<EVENT>>`
    fn into_event_callback(self) -> Option<Callback<EVENT>>;
}

impl<EVENT> IntoEventCallback<EVENT> for Callback<EVENT> {
    fn into_event_callback(self) -> Option<Callback<EVENT>> {
        Some(self)
    }
}

impl<EVENT> IntoEventCallback<EVENT> for &Callback<EVENT> {
    fn into_event_callback(self) -> Option<Callback<EVENT>> {
        Some(self.clone())
    }
}

impl<EVENT> IntoEventCallback<EVENT> for Option<Callback<EVENT>> {
    fn into_event_callback(self) -> Option<Callback<EVENT>> {
        self
    }
}

impl<T, EVENT> IntoEventCallback<EVENT> for T
where
    T: Fn(EVENT) + 'static,
{
    fn into_event_callback(self) -> Option<Callback<EVENT>> {
        Some(Callback::from(self))
    }
}

impl<T, EVENT> IntoEventCallback<EVENT> for Option<T>
where
    T: Fn(EVENT) + 'static,
{
    fn into_event_callback(self) -> Option<Callback<EVENT>> {
        Some(Callback::from(self?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_into_event_callback_types() {
        let f = |_: usize| ();
        let cb = Callback::from(f);

        // Callbacks
        let _: Option<Callback<usize>> = cb.clone().into_event_callback();
        let _: Option<Callback<usize>> = (&cb).into_event_callback();
        let _: Option<Callback<usize>> = Some(cb).into_event_callback();

        // Fns
        let _: Option<Callback<usize>> = f.into_event_callback();
        let _: Option<Callback<usize>> = Some(f).into_event_callback();
    }
}
