#[macro_use]
mod macros;
mod events;

use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement};

use crate::Callback;
pub use events::*;

/// A trait to obtain a generic event target.
///
/// The methods in this trait are convenient helpers that use the [`JsCast`] trait internally
/// to do the conversion.
pub trait TypedTarget
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
    /// use yew::{prelude::*, web_sys::{Event, HtmlTextAreaElement}};
    /// # enum Msg {
    /// #   Value(String),
    /// # }
    /// # struct Comp {
    /// #    link: ComponentLink<Self>,
    /// # }
    /// # impl Component for Comp {
    /// # type Properties = ();
    /// # type Message = Msg;
    /// # fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    /// #   Self { link }
    /// # }
    /// # fn update(&mut self, _: Self::Message) -> ShouldRender { false }
    /// # fn change(&mut self, _: Self::Properties) -> ShouldRender { false }
    ///
    /// fn view(&self) -> Html {
    ///     html! {
    ///         <div
    ///             onchange={self.link.batch_callback(|e: Event| {
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
    /// consider using [`TypedTarget::target_unchecked_into<T>`]_
    #[inline]
    fn target_dyn_into<T>(&self) -> Option<T>
    where
        T: AsRef<HtmlElement> + JsCast,
    {
        self.as_ref()
            .target()
            .and_then(|target| target.dyn_into().ok())
    }

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
    /// use yew::{prelude::*, web_sys::{Event, HtmlInputElement}};
    /// # enum Msg {
    /// #   Value(String),
    /// # }
    /// # struct Comp {
    /// #    link: ComponentLink<Self>,
    /// # }
    /// # impl Component for Comp {
    /// # type Properties = ();
    /// # type Message = Msg;
    /// # fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    /// #   Self { link }
    /// # }
    /// # fn update(&mut self, _: Self::Message) -> ShouldRender { false }
    /// # fn change(&mut self, _: Self::Properties) -> ShouldRender { false }
    ///
    /// fn view(&self) -> Html {
    ///     html! {
    ///         <input type="text"
    ///             onchange={self.link.callback(|e: Event| {
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
    #[inline]
    fn target_unchecked_into<T>(&self) -> T
    where
        T: AsRef<HtmlElement> + JsCast,
    {
        self.as_ref().target().unwrap().unchecked_into()
    }
}

impl<E: AsRef<Event>> TypedTarget for E {}

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
