//! Component properties module

pub use yew_macro::Properties;

/// Trait for building properties for a component
pub trait Properties: PartialEq {
    /// Builder that will be used to construct properties
    type Builder;

    /// Entrypoint for building properties
    fn builder() -> Self::Builder;
}

/// Trait finishing the builder and verifying all props were set
#[doc(hidden)]
pub trait Buildable {
    type Output;
    fn build(this: Self) -> Self::Output;
}

/// Dummy struct targeted by assertions that all props were set
#[doc(hidden)]
#[derive(Debug)]
pub struct AssertAllProps;

/// Builder for when a component has no properties
#[derive(Debug, PartialEq)]
#[doc(hidden)]
pub struct EmptyBuilder;

impl Properties for () {
    type Builder = EmptyBuilder;

    fn builder() -> Self::Builder {
        EmptyBuilder
    }
}

impl Buildable for EmptyBuilder {
    type Output = ();

    /// Build empty properties
    fn build(_: Self) {}
}
