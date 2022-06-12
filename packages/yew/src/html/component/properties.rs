//! Component properties module

pub use yew_macro::Properties;

/// Trait for building properties for a component
pub trait Properties: PartialEq {
    /// Builder that will be used to construct properties
    type Builder;

    /// Entrypoint for building properties
    fn builder() -> Self::Builder;
}

/// A marker trait to ensure that the builder has received a specific required prop
#[doc(hidden)]
pub trait HasProp<P, How> {}

/// A marker trait to ensure that the builder has received all required props
#[doc(hidden)]
pub trait HasAllProps<P, How> {}

/// Trait finishing the builder and verifying all props were set
#[doc(hidden)]
pub trait Buildable<Token: ?Sized> {
    type Output;
    type WrappedTok;
    fn pre_build(builder: Self, _: &Token) -> PreBuild<Token, Self>
    where
        Self: Sized,
    {
        PreBuild {
            builder,
            _token: std::marker::PhantomData,
        }
    }
    fn build(this: Self) -> Self::Output;
}
#[doc(hidden)]
#[derive(Debug)]
pub struct PreBuild<Token: ?Sized, B> {
    _token: std::marker::PhantomData<Token>,
    builder: B,
}

impl<Token, B: Buildable<Token>> PreBuild<Token, B> {
    #[doc(hidden)]
    pub fn build<How>(self) -> B::Output
    where
        Token: AllPropsFrom<Token, B, How>,
    {
        B::build(self.builder)
    }
}

#[doc(hidden)]
pub trait AllPropsFrom<Token, B: Buildable<Token>, How> {}

impl<Token, B, How> AllPropsFrom<Token, B, How> for Token
where
    B: Buildable<Token>,
    B::WrappedTok: HasAllProps<B::Output, How>,
{
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

impl<T> Buildable<T> for EmptyBuilder {
    type Output = ();
    type WrappedTok = ();

    /// Build empty properties
    fn build(_: Self) {}
}

impl<T> HasAllProps<(), T> for T {}
