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

use super::{Html, HtmlResult, IntoHtmlResult};

#[cfg(feature = "hydration")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum RenderMode {
    Hydration,
    Render,
    #[cfg(feature = "ssr")]
    Ssr,
}

/// The [`Component`]'s context. This contains component's [`Scope`] and props and
/// is passed to every lifecycle method.
#[derive(Debug)]
pub struct Context<COMP: BaseComponent> {
    scope: Scope<COMP>,
    props: Rc<COMP::Properties>,
    #[cfg(feature = "hydration")]
    creation_mode: RenderMode,

    #[cfg(feature = "hydration")]
    prepared_state: Option<String>,
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
        &self.props
    }

    #[cfg(feature = "hydration")]
    pub(crate) fn creation_mode(&self) -> RenderMode {
        self.creation_mode
    }

    /// The component's prepared state
    pub fn prepared_state(&self) -> Option<&str> {
        #[cfg(not(feature = "hydration"))]
        let state = None;

        #[cfg(feature = "hydration")]
        let state = self.prepared_state.as_deref();

        state
    }
}

/// The common base of both function components and struct components.
///
/// If you are taken here by doc links, you might be looking for [`Component`] or
/// [`#[function_component]`](crate::functional::function_component).
///
/// We provide a blanket implementation of this trait for every member that implements
/// [`Component`].
///
/// # Warning
///
/// This trait may be subject to heavy changes between versions and is not intended for direct
/// implementation.
///
/// You should used the [`Component`] trait or the
/// [`#[function_component]`](crate::functional::function_component) macro to define your
/// components.
pub trait BaseComponent: Sized + 'static {
    /// The Component's Message.
    type Message: 'static;

    /// The Component's Properties.
    type Properties: Properties;

    /// Creates a component.
    fn create(ctx: &Context<Self>) -> Self;

    /// Updates component's internal state.
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool;

    /// React to changes of component properties.
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool;

    /// Returns a component layout to be rendered.
    fn view(&self, ctx: &Context<Self>) -> HtmlResult;

    /// Notified after a layout is rendered.
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool);

    /// Notified before a component is destroyed.
    fn destroy(&mut self, ctx: &Context<Self>);

    /// Prepares the server-side state.
    fn prepare_state(&self) -> Option<String>;
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
    ///
    /// By default, this function will return true and thus make the component re-render.
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    /// Called when properties passed to the component change
    ///
    /// Returned bool indicates whether to render this Component after changed.
    ///
    /// By default, this function will return true and thus make the component re-render.
    #[allow(unused_variables)]
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
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

    /// Prepares the state during server side rendering.
    ///
    /// This state will be sent to the client side and is available via `ctx.prepared_state()`.
    ///
    /// This method is only called during server-side rendering after the component has been
    /// rendered.
    fn prepare_state(&self) -> Option<String> {
        None
    }

    /// Called right before a Component is unmounted.
    #[allow(unused_variables)]
    fn destroy(&mut self, ctx: &Context<Self>) {}
}

impl<T> BaseComponent for T
where
    T: Sized + Component + 'static,
{
    type Message = <T as Component>::Message;
    type Properties = <T as Component>::Properties;

    fn create(ctx: &Context<Self>) -> Self {
        Component::create(ctx)
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        Component::update(self, ctx, msg)
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        Component::changed(self, ctx, old_props)
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

    fn prepare_state(&self) -> Option<String> {
        Component::prepare_state(self)
    }
}

#[cfg(test)]
#[cfg(any(feature = "ssr", feature = "csr"))]
mod tests {
    use super::*;

    struct MyCustomComponent;

    impl Component for MyCustomComponent {
        type Message = ();
        type Properties = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self
        }

        fn view(&self, _ctx: &Context<Self>) -> Html {
            Default::default()
        }
    }

    #[test]
    fn make_sure_component_update_and_changed_rerender() {
        let mut comp = MyCustomComponent;
        let ctx = Context {
            scope: Scope::new(None),
            props: Rc::new(()),
            #[cfg(feature = "hydration")]
            creation_mode: crate::html::RenderMode::Hydration,
            #[cfg(feature = "hydration")]
            prepared_state: None,
        };
        assert!(Component::update(&mut comp, &ctx, ()));
        assert!(Component::changed(&mut comp, &ctx, &Rc::new(())));
    }
}
