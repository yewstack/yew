//! Components wrapped with context including properties, state, and link

mod children;
mod intrinsic;
#[cfg(feature = "csr")]
mod lifecycle;
mod marker;
mod properties;
mod scope;

use std::any::Any;
use std::fmt;
use std::rc::Rc;

pub use children::*;
pub(crate) use intrinsic::{ComponentIntriustic, Intrinsical};
pub use marker::*;
pub use properties::*;
pub use scope::Scope;

use crate::FunctionComponent;

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
pub struct Context {
    mountable: Rc<dyn Intrinsical>,
    scope: Scope,
    #[cfg(feature = "hydration")]
    creation_mode: RenderMode,

    #[cfg(feature = "hydration")]
    prepared_state: Option<String>,
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Context")
    }
}

impl Context {
    /// The component scope
    #[inline]
    pub fn link(&self) -> &Scope {
        &self.scope
    }

    /// The component's props
    #[inline]
    pub fn props(&self) -> &dyn Any {
        self.mountable.any_props()
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
    /// The Component's Properties.
    type Properties: Properties;

    /// Creates a component.
    fn create(ctx: &Context) -> FunctionComponent;
}
