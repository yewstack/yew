//! Implements lazy fetching of components

// A simple wrapper is easy to implement. This module exists to support message passing and more
// involved logic

use std::future::Future;

use crate::html::Scope;
use crate::suspense::Suspension;
use crate::{BaseComponent, Context, HtmlResult};

// we might be able to erase the BaseComponent bound here, then monomorphize for some size savings
/// This struct is (mentally) the same as a `dyn BaseComponent` but without informing the linker
/// about this.
#[derive(Debug)]
#[repr(C)]
struct CompVTableImpl<C: BaseComponent> {
    create: fn(&Context<C>) -> C,
    update: fn(&mut C, &Context<C>, C::Message) -> bool,
    changed: fn(&mut C, &Context<C>, &C::Properties) -> bool,
    view: fn(&C, &Context<C>) -> HtmlResult,
    rendered: fn(&mut C, &Context<C>, bool),
    destroy: fn(&mut C, &Context<C>),
    prepare_state: fn(&C) -> Option<String>,
    //...
}
/// Component vtable for a component.
///
/// Return `LazyVTable::<YourComponent>::vtable()` from your implementation of
/// [`LazyComponent::fetch`] after resolving it.
pub struct LazyVTable<C: BaseComponent> {
    imp: &'static CompVTableImpl<C>,
}
impl<C: BaseComponent> std::fmt::Debug for LazyVTable<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazyVTable")
            .field("vtable", &"...")
            .finish()
    }
}
impl<C: BaseComponent> Clone for LazyVTable<C> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<C: BaseComponent> Copy for LazyVTable<C> {}

impl<C: BaseComponent> LazyVTable<C> {
    /// Returns the singleton vtable for a component.
    ///
    /// Return this from [`LazyComponent::fetch`] for your lazy component.
    pub fn vtable() -> LazyVTable<C> {
        LazyVTable {
            imp: &const {
                CompVTableImpl {
                    create: C::create,
                    update: C::update,
                    changed: C::changed,
                    view: C::view,
                    rendered: C::rendered,
                    destroy: C::destroy,
                    prepare_state: C::prepare_state,
                }
            },
        }
    }
}
/// Implement this trait to support lazily loading a component.
///
/// Used in conjunction with the [`Lazy`] component.
pub trait LazyComponent: 'static {
    /// The component that is lazily being fetched
    type Underlying: BaseComponent;
    /// Fetch the component's impl
    fn fetch() -> impl Future<Output = LazyVTable<Self::Underlying>> + Send;
}

#[derive(Debug)]
enum LazyState<C: BaseComponent> {
    Pending(Suspension),
    Created(LazyVTable<C>, C),
}
/// Wrapper for a lazily fetched component
///
/// This component suspends as long as the underlying component is still being fetched,
/// then behaves as the underlying component itself.
#[derive(Debug)]
pub struct Lazy<C: LazyComponent> {
    inner_scope: Scope<C::Underlying>,
    state: LazyState<C::Underlying>,
}

/// Message to send to a lazy component
#[derive(Debug)]
pub enum LazyMessage<C: BaseComponent> {
    /// Forward the message to the underlying component once that is fetched
    Forward(C::Message),
    #[doc(hidden)]
    FetchFinished(LazyVTable<C>, C),
}

impl<C: LazyComponent> BaseComponent for Lazy<C> {
    type Message = LazyMessage<C::Underlying>;
    type Properties = <C::Underlying as BaseComponent>::Properties;

    fn create(ctx: &Context<Self>) -> Self {
        #[cfg(not(any(feature = "ssr", feature = "csr")))]
        {
            let _ = ctx;
            todo!("Component shouldn't render without any rendering mode enabled");
        }
        #[allow(unreachable_code)]
        let inner_scope;
        #[cfg(any(feature = "ssr", feature = "csr"))]
        {
            inner_scope = Scope::new(Some(ctx.link().clone().into()));
        }
        let creation_ctx = ctx.narrow_scope(&inner_scope);

        let link = ctx.link().clone();
        let suspension = Suspension::from_future(async move {
            // Ignore error in case receiver was dropped
            let vtable = C::fetch().await;
            let comp = (vtable.imp.create)(&creation_ctx);
            link.send_message(LazyMessage::FetchFinished(vtable, comp));
        });
        Self {
            inner_scope,
            state: LazyState::Pending(suspension),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let msg = match msg {
            LazyMessage::FetchFinished(vtable, comp) => {
                self.state = LazyState::Created(vtable, comp);
                return true;
            }
            LazyMessage::Forward(msg) => msg,
        };
        match &mut self.state {
            LazyState::Pending(_) => {
                // has a queueing implementation. We don't rerender until the suspension resolves
                self.inner_scope.send_message(msg);
                false
            }
            LazyState::Created(vtable, comp) => {
                (vtable.imp.update)(comp, &ctx.narrow_scope(&self.inner_scope), msg)
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if let LazyState::Created(vtable, comp) = &mut self.state {
            (vtable.imp.changed)(comp, &ctx.narrow_scope(&self.inner_scope), old_props)
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> crate::HtmlResult {
        match &self.state {
            LazyState::Pending(suspension) => Err(suspension.clone().into()),
            LazyState::Created(vtable, comp) => {
                (vtable.imp.view)(comp, &ctx.narrow_scope(&self.inner_scope))
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if let LazyState::Created(vtable, comp) = &mut self.state {
            (vtable.imp.rendered)(comp, &ctx.narrow_scope(&self.inner_scope), first_render)
        } else {
            unreachable!("can't get rendered before fetching the vtable")
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        if let LazyState::Created(vtable, comp) = &mut self.state {
            (vtable.imp.destroy)(comp, &ctx.narrow_scope(&self.inner_scope))
        }
    }

    fn prepare_state(&self) -> Option<String> {
        if let LazyState::Created(vtable, comp) = &self.state {
            (vtable.imp.prepare_state)(comp)
        } else {
            None
        }
    }
}

/// Make a component accessible as a lazily loaded component in a separate wasm module
#[doc(hidden)]
#[macro_export]
macro_rules! __declare_lazy_component {
    ($comp:ty as $lazy_name:ident in $module:ident) => {
        struct Proxy;
        impl ::yew::lazy::LazyComponent for Proxy {
            type Underlying = $comp;

            async fn fetch() -> ::yew::lazy::LazyVTable<Self::Underlying> {
                #[$crate::lazy::wasm_split::wasm_split($module, wasm_split_path = $crate::lazy::wasm_split)]
                fn split_fetch() -> ::yew::lazy::LazyVTable<$comp> {
                    ::yew::lazy::LazyVTable::<$comp>::vtable()
                }
                struct F(
                    ::std::option::Option<
                        ::std::pin::Pin<
                            ::std::boxed::Box<
                                dyn ::std::future::Future<Output = ::yew::lazy::LazyVTable<$comp>>
                                    + ::std::marker::Send,
                            >,
                        >,
                    >,
                );
                impl Future for F {
                    type Output = ::yew::lazy::LazyVTable<$comp>;

                    fn poll(
                        mut self: ::std::pin::Pin<&mut Self>,
                        cx: &mut ::std::task::Context<'_>,
                    ) -> ::std::task::Poll<Self::Output> {
                        self.0
                            .get_or_insert_with(|| ::std::boxed::Box::pin(split_fetch()))
                            .as_mut()
                            .poll(cx)
                    }
                }
                static CACHE: ::yew::lazy::LazyCell<::yew::lazy::LazyVTable<$comp>, F> =
                    ::yew::lazy::LazyCell::new(F(None));
                *::std::pin::Pin::static_ref(&CACHE).await.get_ref()
            }
        }
        type $lazy_name = ::yew::lazy::Lazy<Proxy>;
    };
}
#[doc(hidden)]
pub use ::async_once_cell::Lazy as LazyCell;
#[doc(hidden)]
pub use ::wasm_split_helpers as wasm_split;

pub use crate::__declare_lazy_component as declare_lazy_component;
