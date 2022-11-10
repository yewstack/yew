//! Components wrapped with context including properties, state, and link

mod children;
mod intrinsic;
#[cfg(feature = "csr")]
mod lifecycle;
mod marker;
mod properties;
mod scope;

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

pub use crate::functional::Component;
