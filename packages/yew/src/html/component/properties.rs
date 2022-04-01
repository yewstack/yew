//! Component properties module

use crate::html::HtmlRef;
pub use yew_macro::Properties;

mod sealed {
    use super::*;

    /// Trait to limit `ref_` to `HtmlRef<_>`.
    pub trait RefProp {}

    impl<T> RefProp for HtmlRef<T> {}
    impl RefProp for () {}
}

use sealed::RefProp;

/// Trait for building properties for a component
pub trait Properties: PartialEq {
    /// Builder that will be used to construct properties
    type Builder;
    /// The component reference type.
    type Ref: RefProp;

    /// Entrypoint for building properties
    fn builder() -> Self::Builder;

    /// Returns ref type.
    fn ref_(&self) -> &Self::Ref;
}

/// Builder for when a component has no properties
#[derive(Debug, PartialEq)]
#[doc(hidden)]
pub struct EmptyBuilder;

impl Properties for () {
    type Builder = EmptyBuilder;
    type Ref = ();

    fn builder() -> Self::Builder {
        EmptyBuilder
    }

    fn ref_(&self) -> &Self::Ref {
        &()
    }
}

impl EmptyBuilder {
    /// Build empty properties
    pub fn build(self) {}
}
