//! Components wrapped with context including properties, state, and link

mod children;
mod intrinsic;
#[cfg(feature = "csr")]
mod lifecycle;
mod marker;
mod properties;
mod scope;

use std::fmt;
use std::rc::Rc;

pub use children::*;
pub(crate) use intrinsic::{ComponentIntrinsic, Intrinsical};
pub use marker::*;
pub use properties::*;
pub use scope::Scope;

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
pub(crate) struct Context {
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

    pub fn intrisic(&self) -> &dyn Intrinsical {
        self.mountable.as_ref()
    }

    #[cfg(feature = "hydration")]
    pub(crate) fn creation_mode(&self) -> RenderMode {
        self.creation_mode
    }

    /// The component's prepared state
    #[cfg(feature = "hydration")]
    pub fn prepared_state(&self) -> Option<&str> {
        #[cfg(not(feature = "hydration"))]
        let state = None;

        #[cfg(feature = "hydration")]
        let state = self.prepared_state.as_deref();

        state
    }
}

pub use crate::functional::Component as BaseComponent;
