//! Component properties module

pub use yew_macro::Properties;

/// Trait for building properties for a component
pub trait Properties: PartialEq {
    /// Builder that will be used to construct properties
    type Builder;

    /// Entrypoint for building properties
    fn builder() -> Self::Builder;
}

#[doc(hidden)]
mod __macro {
    /// A marker trait to ensure that the builder has received a specific required prop.
    /// For each required impl in a property, we generate:
    /// - a struct with the name of the prop, which takes the place of `P`.
    /// - a token wrapper, `HasP<TokenTail>`, that records that the build state represented includes
    ///   the state in `TokenTail` + `P`. Such tokens are returned from the setter on the builder,
    ///   to verify the build state.
    /// - An `impl<T> HasP<T>: HasProp<P, _>` saying that a state represented by a token of
    ///   `HasP<_>` indeed verifies P has been set.
    /// - An `impl<Q> HasP<Tail>: HasProp<Q, _> where Tail: HasProp<Q>` saying that any props set
    ///   previously (represented by the tail) is still set after P has been set.
    /// - ^ the two impls would be overlapping, where it not for the `How` argument, which resolves
    ///   the conflict.
    pub trait HasProp<P, How> {}

    /// A marker trait to ensure that the builder has received all required props.
    /// For each struct deriving [`Properties`], an impl is generated, requiring `HasProp<p>` for
    /// all properties marked as required as a bound on the impl.
    ///
    /// [`Properties`]: super::Properties
    pub trait HasAllProps<P, How> {}

    /// Trait finishing the builder and verifying all props were set.
    /// The structure can be a bit surprising, and is related to how the proc macro reports errors
    /// - why have a prepare_build method? This captures the argument types, but now `How`, and
    ///   returns an internal type with a method that can be called without further qualification.
    ///   We need the additional types, to avoid collision with property names in the Builder. We
    ///   want to avoid qualification to persuade rust not to report the `finish_build` method name.
    /// - why have a AllPropsFor trait? We want the trait to be on the Token, not on a type
    ///   associated or derived from it, so that it shows up in errors directly instead of through
    ///   convoluted traces.
    pub trait Buildable<Token> {
        /// Property type being built
        type Output;
        /// Instead of `Token` directly, a wrapped token type is checked for trait impls in macro
        /// code. This avoids problems related to blanket impls.
        type WrappedToken;
        /// This method "captures" the builder and token type, but does not verify yet.
        fn prepare_build(builder: Self, _: &Token) -> PreBuild<Token, Self>
        where
            Self: Sized,
        {
            PreBuild {
                builder,
                _token: std::marker::PhantomData,
            }
        }
        /// Build the props from self. Expected to panic if not all props where set.
        fn build(this: Self) -> Self::Output;
    }
    /// Helper alias for a Builder, also capturing the prop Token recording the provided props.
    #[derive(Debug)]
    pub struct PreBuild<Token, B> {
        _token: std::marker::PhantomData<Token>,
        builder: B,
    }

    impl<Token, B: Buildable<Token>> PreBuild<Token, B> {
        /// This is the method that introduces the actual bound verifying all props where set.
        pub fn build<How>(self) -> B::Output
        where
            Token: AllPropsFor<B, How>,
        {
            B::build(self.builder)
        }
    }

    /// Trait to specify the requirement for Self to be a valid token signaling all props have been
    /// provided to the builder.
    pub trait AllPropsFor<Builder, How> {}

    impl<Token, Builder: Buildable<Token>, How> AllPropsFor<Builder, How> for Token where
        Builder::WrappedToken: HasAllProps<Builder::Output, How>
    {
    }

    /// Dummy struct targeted by assertions that all props were set
    #[derive(Debug)]
    pub struct AssertAllProps;

    /// Builder for when a component has no properties
    #[derive(Debug, PartialEq, Eq)]
    pub struct EmptyBuilder;

    impl super::Properties for () {
        type Builder = EmptyBuilder;

        fn builder() -> Self::Builder {
            EmptyBuilder
        }
    }

    impl<T> Buildable<T> for EmptyBuilder {
        type Output = ();
        type WrappedToken = ();

        /// Build empty properties
        fn build(_: Self) {}
    }

    impl<T> HasAllProps<(), T> for T {}
}

#[doc(hidden)]
pub use __macro::{AllPropsFor, AssertAllProps, Buildable, HasAllProps, HasProp};
