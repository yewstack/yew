//! Components wrapped with context including properties, state, and link

mod children;
#[cfg(any(feature = "csr", feature = "ssr"))]
mod lifecycle;
mod marker;
mod properties;
mod scope;

use std::rc::Rc;

pub use children::*;
pub use marker::*;
pub use properties::*;
#[cfg(feature = "csr")]
pub(crate) use scope::Scoped;
pub use scope::{AnyScope, Scope, SendAsMessage};

use super::{BindableRef, Html, HtmlResult, IntoHtmlResult, NoReference};

#[cfg(debug_assertions)]
#[cfg(any(feature = "csr", feature = "ssr"))]
mod feat_csr_ssr {
    use wasm_bindgen::prelude::wasm_bindgen;
    use wasm_bindgen::JsValue;

    thread_local! {
         static EVENT_HISTORY: std::cell::RefCell<std::collections::HashMap<usize, Vec<String>>>
            = Default::default();
    }

    /// Push [Component] event to lifecycle debugging registry
    pub(crate) fn log_event(comp_id: usize, event: impl ToString) {
        EVENT_HISTORY.with(|h| {
            h.borrow_mut()
                .entry(comp_id)
                .or_default()
                .push(event.to_string())
        });
    }

    /// Get [Component] event log from lifecycle debugging registry
    #[wasm_bindgen(js_name = "yewGetEventLog")]
    pub fn _get_event_log(comp_id: usize) -> Option<Vec<JsValue>> {
        EVENT_HISTORY.with(|h| {
            Some(
                h.borrow()
                    .get(&comp_id)?
                    .iter()
                    .map(|l| (*l).clone().into())
                    .collect(),
            )
        })
    }
}

#[cfg(feature = "hydration")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum RenderMode {
    Hydration,
    Render,
    #[cfg(feature = "ssr")]
    Ssr,
}

#[cfg(debug_assertions)]
#[cfg(any(feature = "csr", feature = "ssr"))]
pub(crate) use feat_csr_ssr::*;

/// The [`Component`]'s context. This contains component's [`Scope`] and and props and
/// is passed to every lifecycle method.
#[derive(Debug)]
pub struct Context<COMP: BaseComponent> {
    scope: Scope<COMP>,
    props: Rc<COMP::Properties>,
    #[cfg(feature = "hydration")]
    mode: RenderMode,
}

impl<COMP: BaseComponent> Context<COMP> {
    /// The component scope
    #[inline]
    pub fn link(&self) -> &Scope<COMP> {
        &self.scope
    }

    /// The component's props
    #[inline]
    pub fn props(&self) -> &COMP::Properties {
        &*self.props
    }

    #[cfg(feature = "hydration")]
    pub(crate) fn mode(&self) -> RenderMode {
        self.mode
    }
}

/// The common base of both function components and struct components.
///
/// If you are taken here by doc links, you might be looking for [`Component`] or
/// [`#[function_component]`](crate::functional::function_component).
///
/// We provide a blanket implementation of this trait for every member that implements
/// [`Component`] or [`ComponentWithRef`].
pub trait BaseComponent: Sized + 'static {
    /// The Component's Message.
    type Message: 'static;

    /// The Component's Properties.
    type Properties: Properties;

    /// The Component's Reference type.
    type Reference: 'static;

    /// Creates a component.
    fn create(ctx: &Context<Self>, bindable_ref: BindableRef<'_, Self::Reference>) -> Self;

    /// Updates component's internal state.
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool;

    /// React to changes of component properties.
    fn changed(&mut self, ctx: &Context<Self>) -> bool;

    /// Returns a component layout to be rendered.
    fn view(&self, ctx: &Context<Self>) -> HtmlResult;

    /// Notified after a layout is rendered.
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool);

    /// Notified before a component is destroyed.
    fn destroy(&mut self, ctx: &Context<Self>);
}

/// A struct [`Component`] that additionally can have a `ref` attribute.
///
/// We provide a blanket implementation of this trait for every member that implements
/// [`Component`].
pub trait ComponentWithRef: Sized + 'static {
    /// The Component's Message.
    type Message: 'static;

    /// The Component's Properties.
    type Properties: Properties;

    /// The Component's Reference type.
    type Reference: 'static;

    /// Creates a component.
    fn create(ctx: &Context<Self>, bindable_ref: BindableRef<'_, Self::Reference>) -> Self;

    /// Updates component's internal state.
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    /// React to changes of component properties.
    #[allow(unused_variables)]
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    /// Returns a component layout to be rendered.
    fn view(&self, ctx: &Context<Self>) -> HtmlResult;

    /// Notified after a layout is rendered.
    #[allow(unused_variables)]
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    /// Notified before a component is destroyed.
    #[allow(unused_variables)]
    fn destroy(&mut self, ctx: &Context<Self>) {}
}

/// Components are the basic building blocks of the UI in a Yew app. Each Component
/// chooses how to display itself using received props and self-managed state.
/// Components can be dynamic and interactive by declaring messages that are
/// triggered and handled asynchronously. This async update mechanism is inspired by
/// Elm and the actor model used in the Actix framework.
pub trait Component: Sized + 'static {
    /// Messages are used to make Components dynamic and interactive. Simple
    /// Component's can declare their Message type to be `()`. Complex Component's
    /// commonly use an enum to declare multiple Message types.
    type Message: 'static;

    /// The Component's properties.
    ///
    /// When the parent of a Component is re-rendered, it will either be re-created or
    /// receive new properties in the context passed to the `changed` lifecycle method.
    type Properties: Properties;

    /// Called when component is created.
    fn create(ctx: &Context<Self>) -> Self;

    /// Called when a new message is sent to the component via its scope.
    ///
    /// Components handle messages in their `update` method and commonly use this method
    /// to update their state and (optionally) re-render themselves.
    ///
    /// Returned bool indicates whether to render this Component after update.
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    /// Called when properties passed to the component change
    ///
    /// Returned bool indicates whether to render this Component after changed.
    #[allow(unused_variables)]
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    /// Components define their visual layout using a JSX-style syntax through the use of the
    /// `html!` procedural macro. The full guide to using the macro can be found in [Yew's
    /// documentation](https://yew.rs/concepts/html).
    ///
    /// Note that `view()` calls do not always follow a render request from `update()` or
    /// `changed()`. Yew may optimize some calls out to reduce virtual DOM tree generation overhead.
    /// The `create()` call is always followed by a call to `view()`.
    fn view(&self, ctx: &Context<Self>) -> Html;

    /// The `rendered` method is called after each time a Component is rendered but
    /// before the browser updates the page.
    ///
    /// Note that `rendered()` calls do not always follow a render request from `update()` or
    /// `changed()`. Yew may optimize some calls out to reduce virtual DOM tree generation overhead.
    /// The `create()` call is always followed by a call to `view()` and later `rendered()`.
    #[allow(unused_variables)]
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    /// Called right before a Component is unmounted.
    #[allow(unused_variables)]
    fn destroy(&mut self, ctx: &Context<Self>) {}
}

impl<T> ComponentWithRef for T
where
    T: Sized + Component + 'static,
{
    type Message = <T as Component>::Message;
    type Properties = <T as Component>::Properties;
    type Reference = NoReference;

    fn create(ctx: &Context<Self>, bindable_ref: BindableRef<'_, NoReference>) -> Self {
        bindable_ref.fake_bind();
        Component::create(ctx)
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        Component::update(self, ctx, msg)
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        Component::changed(self, ctx)
    }

    fn view(&self, ctx: &Context<Self>) -> HtmlResult {
        Component::view(self, ctx).into_html_result()
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        Component::rendered(self, ctx, first_render)
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        Component::destroy(self, ctx)
    }
}

impl<T> BaseComponent for T
where
    T: Sized + ComponentWithRef + 'static,
{
    type Message = <T as ComponentWithRef>::Message;
    type Properties = <T as ComponentWithRef>::Properties;
    type Reference = <T as ComponentWithRef>::Reference;

    fn create(ctx: &Context<Self>, bindable_ref: BindableRef<'_, Self::Reference>) -> Self {
        ComponentWithRef::create(ctx, bindable_ref)
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        ComponentWithRef::update(self, ctx, msg)
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        ComponentWithRef::changed(self, ctx)
    }

    fn view(&self, ctx: &Context<Self>) -> HtmlResult {
        ComponentWithRef::view(self, ctx).into_html_result()
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        ComponentWithRef::rendered(self, ctx, first_render)
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        ComponentWithRef::destroy(self, ctx)
    }
}
