//! Implements lazy fetching of components

// A simple wrapper is easy to implement. This module exists to support message passing and more
// involved logic

use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

use crate::html::Scope;
use crate::scheduler::Shared;
use crate::suspense::Suspension;
use crate::virtual_dom::VComp;
use crate::{BaseComponent, Context};

type ScopeRef<C> = Shared<Option<Scope<C>>>;

#[derive(Debug)]
struct CompVTableImpl<C: BaseComponent> {
    /// The way to create a component from properties and a way to reference it later.
    /// It is important that we return a structure that already captures the vtable to
    /// the component's functionality (Mountable), so that the linker doesn't see that
    /// the main module references C's BaseComponent impl.
    wrap_html: fn(Rc<C::Properties>, ScopeRef<C>) -> VComp,
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
            .field("vtable", &(self.imp as *const _))
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
        fn wrap_html<C: BaseComponent>(
            props: Rc<C::Properties>,
            scope_ref: Shared<Option<Scope<C>>>,
        ) -> VComp {
            VComp::new_with_ref(props, scope_ref)
        }
        LazyVTable {
            imp: &const {
                CompVTableImpl {
                    wrap_html: wrap_html::<C>,
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

enum LazyState<C: BaseComponent> {
    Pending(Suspension),
    #[allow(unused)] // Only constructed with feature csr or ssr
    Created(LazyVTable<C>),
}

impl<C: BaseComponent> std::fmt::Debug for LazyState<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending(arg0) => f.debug_tuple("Pending").field(arg0).finish(),
            Self::Created(arg0) => f.debug_tuple("Created").field(arg0).finish(),
        }
    }
}

/// Wrapper for a lazily fetched component
///
/// This component suspends as long as the underlying component is still being fetched,
/// then behaves as the underlying component itself.
pub struct Lazy<C: LazyComponent> {
    inner_scope: ScopeRef<C::Underlying>,
    // messages sent to the component before the inner_scope is set are buffered
    message_buffer: RefCell<Vec<<C::Underlying as BaseComponent>::Message>>,
    state: Shared<LazyState<C::Underlying>>,
}

impl<C: LazyComponent> std::fmt::Debug for Lazy<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lazy")
            .field("inner_scope", &self.inner_scope)
            .field("message_buffer", &"...")
            .field("state", &self.state)
            .finish()
    }
}

impl<C: LazyComponent> BaseComponent for Lazy<C> {
    type Message = <C::Underlying as BaseComponent>::Message;
    type Properties = <C::Underlying as BaseComponent>::Properties;

    fn create(ctx: &Context<Self>) -> Self {
        let host_scope = ctx.link().clone();
        let state = Rc::<RefCell<_>>::new_cyclic(move |state| {
            let state = state.clone();
            let suspension = Suspension::from_future(async move {
                // Ignore error in case receiver was dropped
                let vtable = C::fetch().await;
                #[cfg(any(feature = "ssr", feature = "csr"))]
                if let Some(state) = state.upgrade() {
                    *state.borrow_mut() = LazyState::Created(vtable);
                    // force a re-render with this new state (without a message exchange)
                    host_scope.schedule_render();
                }
                let _ = (host_scope, state, vtable);
            });
            RefCell::new(LazyState::Pending(suspension))
        });
        Self {
            inner_scope: Rc::default(),
            message_buffer: RefCell::default(),
            state,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        if let Some(inner) = self.inner_scope.borrow().as_ref() {
            inner.send_message(msg);
        } else {
            self.message_buffer.borrow_mut().push(msg);
        }
        false
    }

    fn changed(&mut self, _: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> crate::HtmlResult {
        match &*self.state.borrow() {
            LazyState::Pending(suspension) => Err(suspension.clone().into()),
            LazyState::Created(lazy_vtable) => {
                let comp =
                    (lazy_vtable.imp.wrap_html)(ctx.rc_props().clone(), self.inner_scope.clone());
                Ok(comp.into())
            }
        }
    }

    fn rendered(&mut self, _: &Context<Self>, first_render: bool) {
        if first_render {
            let inner = self.inner_scope.borrow();
            let inner = inner.as_ref().expect("lazy component to have rendered");
            inner.send_message_batch(std::mem::take(&mut *self.message_buffer.borrow_mut()));
        } else {
            #[cfg(debug_assertions)]
            assert!(
                self.message_buffer.borrow().is_empty(),
                "no message in buffer after first render"
            );
        }
    }

    fn destroy(&mut self, _: &Context<Self>) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }
}

/// Make a component accessible as a lazily loaded component in a separate wasm module
#[doc(hidden)]
#[macro_export]
macro_rules! __declare_lazy_component {
    ($comp:ty as $lazy_name:ident in $module:ident) => {
        struct Proxy;
        impl $crate::lazy::LazyComponent for Proxy {
            type Underlying = $comp;

            async fn fetch() -> $crate::lazy::LazyVTable<Self::Underlying> {
                #[$crate::lazy::wasm_split::wasm_split($module, wasm_split_path = $crate::lazy::wasm_split)]
                fn split_fetch() -> $crate::lazy::LazyVTable<$comp> {
                    $crate::lazy::LazyVTable::<$comp>::vtable()
                }
                struct F(
                    ::std::option::Option<
                        ::std::pin::Pin<
                            ::std::boxed::Box<
                                dyn ::std::future::Future<Output = $crate::lazy::LazyVTable<$comp>>
                                    + ::std::marker::Send,
                            >,
                        >,
                    >,
                );
                impl Future for F {
                    type Output = $crate::lazy::LazyVTable<$comp>;

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
                static CACHE: $crate::lazy::LazyCell<$crate::lazy::LazyVTable<$comp>, F> =
                    $crate::lazy::LazyCell::new(F(None));
                *::std::pin::Pin::static_ref(&CACHE).await.get_ref()
            }
        }
        type $lazy_name = $crate::lazy::Lazy<Proxy>;
    };
}
#[doc(hidden)]
pub use ::async_once_cell::Lazy as LazyCell;
#[doc(hidden)]
pub use ::wasm_split_helpers as wasm_split;

pub use crate::__declare_lazy_component as declare_lazy_component;
