use yew::context::ContextHandle;
use yew::prelude::*;

use crate::history::Location;
use crate::navigator::Navigator;
use crate::routable::Routable;
use crate::router::{LocationContext, NavigatorContext};

/// A [`ContextHandle`] for [`add_location_listener`](RouterScopeExt::add_location_listener).
pub struct LocationHandle {
    _inner: ContextHandle<LocationContext>,
}

/// A [`ContextHandle`] for [`add_navigator_listener`](RouterScopeExt::add_navigator_listener).
pub struct NavigatorHandle {
    _inner: ContextHandle<NavigatorContext>,
}

/// An extension to [`Scope`](yew::html::Scope) that provides location information and navigator
/// access.
///
/// You can access them on `ctx.link()`
///
/// # Example
///
/// Below is an example of the implementation of the [`Link`](crate::components::Link) component.
///
/// ```
/// # use std::marker::PhantomData;
/// # use wasm_bindgen::UnwrapThrowExt;
/// # use yew::prelude::*;
/// # use yew_router::prelude::*;
/// # use yew_router::components::LinkProps;
/// #
/// # pub struct Link<R: Routable + 'static> {
/// #     _data: PhantomData<R>,
/// # }
/// #
/// # pub enum Msg {
/// #     OnClick,
/// # }
/// #
/// impl<R: Routable + 'static> Component for Link<R> {
///     type Message = Msg;
///     type Properties = LinkProps<R>;
///
///     fn create(_ctx: &Context<Self>) -> Self {
///         Self { _data: PhantomData }
///     }
///
///     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
///         match msg {
///             Msg::OnClick => {
///                 ctx.link()
///                     .navigator()
///                     .expect_throw("failed to get navigator.")
///                     .push(&ctx.props().to);
///                 false
///             }
///         }
///     }
///
///     fn view(&self, ctx: &Context<Self>) -> Html {
///         html! {
///             <a class={ctx.props().classes.clone()}
///                 href={ctx.props().to.to_path()}
///                 onclick={ctx.link().callback(|e: MouseEvent| {
///                     e.prevent_default();
///                     Msg::OnClick
///                 })}
///             >
///                 { ctx.props().children.clone() }
///             </a>
///         }
///     }
/// }
/// ```
pub trait RouterScopeExt {
    /// Returns current [`Navigator`].
    fn navigator(&self) -> Option<Navigator>;

    /// Returns current [`Location`].
    fn location(&self) -> Option<Location>;

    /// Returns current route.
    fn route<R>(&self) -> Option<R>
    where
        R: Routable + 'static;

    /// Adds a listener that gets notified when location changes.
    ///
    /// # Note
    ///
    /// [`LocationHandle`] works like a normal [`ContextHandle`] and it unregisters the callback
    /// when the handle is dropped. You need to keep the handle for as long as you need the
    /// callback.
    fn add_location_listener(&self, cb: Callback<Location>) -> Option<LocationHandle>;

    /// Adds a listener that gets notified when navigator changes.
    ///
    /// # Note
    ///
    /// [`NavigatorHandle`] works like a normal [`ContextHandle`] and it unregisters the callback
    /// when the handle is dropped. You need to keep the handle for as long as you need the
    /// callback.
    fn add_navigator_listener(&self, cb: Callback<Navigator>) -> Option<NavigatorHandle>;
}

impl<COMP: Component> RouterScopeExt for yew::html::Scope<COMP> {
    fn navigator(&self) -> Option<Navigator> {
        self.context::<NavigatorContext>(Callback::from(|_| {}))
            .map(|(m, _)| m.navigator())
    }

    fn location(&self) -> Option<Location> {
        self.context::<LocationContext>(Callback::from(|_| {}))
            .map(|(m, _)| m.location())
    }

    fn route<R>(&self) -> Option<R>
    where
        R: Routable + 'static,
    {
        let navigator = self.navigator()?;
        let location = self.location()?;

        let path = navigator.strip_basename(location.path().into());

        R::recognize(&path)
    }

    fn add_location_listener(&self, cb: Callback<Location>) -> Option<LocationHandle> {
        self.context::<LocationContext>(Callback::from(move |m: LocationContext| {
            cb.emit(m.location())
        }))
        .map(|(_, m)| LocationHandle { _inner: m })
    }

    fn add_navigator_listener(&self, cb: Callback<Navigator>) -> Option<NavigatorHandle> {
        self.context::<NavigatorContext>(Callback::from(move |m: NavigatorContext| {
            cb.emit(m.navigator())
        }))
        .map(|(_, m)| NavigatorHandle { _inner: m })
    }
}
