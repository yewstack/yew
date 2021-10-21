use crate::history::*;
use crate::routable::Routable;
use crate::router::RouterState;

use yew::context::ContextHandle;
use yew::prelude::*;

/// A [`ContextHandle`] for [`add_history_listener`](RouterScopeExt::add_history_listener).
pub struct HistoryHandle {
    _inner: ContextHandle<RouterState>,
}

/// An extension to [`Scope`](yew::html::Scope) that provides session history information.
///
/// You can access on `ctx.link()`
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
/// # use yew_router::components::{LinkProps, Msg};
/// #
/// # pub struct Link<R: Routable + 'static> {
/// #     _data: PhantomData<R>,
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
///                     .history()
///                     .expect_throw("failed to read history")
///                     .push(ctx.props().to.clone());
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
    /// Returns current [`History`].
    fn history(&self) -> Option<AnyHistory>;

    /// Returns current [`Location`].
    fn location(&self) -> Option<AnyLocation>;

    /// Returns current route.
    fn route<R>(&self) -> Option<R>
    where
        R: Routable + 'static;

    /// Adds a listener that gets notified when history changes.
    ///
    /// # Note
    ///
    /// [`HistoryHandle`] works like a normal [`ContextHandle`] and it unregisters the callback
    /// when the handle is dropped. You need to keep the handle for as long as you need the
    /// callback.
    fn add_history_listener(&self, cb: Callback<AnyHistory>) -> Option<HistoryHandle>;
}

impl<COMP: Component> RouterScopeExt for yew::html::Scope<COMP> {
    fn history(&self) -> Option<AnyHistory> {
        self.context::<RouterState>(Callback::from(|_| {}))
            .map(|(m, _)| m.history())
    }

    fn location(&self) -> Option<AnyLocation> {
        self.history().map(|m| m.location())
    }

    fn route<R>(&self) -> Option<R>
    where
        R: Routable + 'static,
    {
        self.location()?.route()
    }

    fn add_history_listener(&self, cb: Callback<AnyHistory>) -> Option<HistoryHandle> {
        self.context::<RouterState>(Callback::from(move |m: RouterState| cb.emit(m.history())))
            .map(|(_, m)| HistoryHandle { _inner: m })
    }
}
