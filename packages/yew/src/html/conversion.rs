use super::{Component, NodeRef, Scope};
use std::{borrow::Cow, rc::Rc};

/// Marker trait for types that the [`html!`] macro may clone implicitly.
pub trait ImplicitClone: Clone {}

// this is only implemented because there's no way to avoid cloning this value
impl ImplicitClone for Cow<'static, str> {}

impl<T: ImplicitClone> ImplicitClone for Option<T> {}
impl<T> ImplicitClone for Rc<T> {}

impl ImplicitClone for NodeRef {}
impl<Comp: Component> ImplicitClone for Scope<Comp> {}
// TODO there are still a few missing like AgentScope

/// TODO
pub trait IntoPropValue<T> {
    /// TODO
    fn into_prop_value(self) -> T;
}

impl<T> IntoPropValue<T> for T {
    fn into_prop_value(self) -> T {
        self
    }
}
impl<T> IntoPropValue<T> for &T
where
    T: ImplicitClone,
{
    fn into_prop_value(self) -> T {
        self.clone()
    }
}

impl<T> IntoPropValue<Option<T>> for T {
    fn into_prop_value(self) -> Option<T> {
        Some(self)
    }
}
impl<T> IntoPropValue<Option<T>> for &T
where
    T: ImplicitClone,
{
    fn into_prop_value(self) -> Option<T> {
        Some(self.clone())
    }
}

macro_rules! impl_into_prop {
    (|$value:ident: $from_ty:ty| -> $to_ty:ty { $conversion:expr }) => {
        // implement V -> T
        impl IntoPropValue<$to_ty> for $from_ty {
            fn into_prop_value(self) -> $to_ty {
                let $value = self;
                $conversion
            }
        }
        // implement V -> Option<T>
        impl IntoOptPropValue<$to_ty> for $from_ty {
            fn into_opt_prop_value(self) -> Option<$to_ty> {
                let $value = self;
                Some({ $conversion })
            }
        }
    };
}

// implemented with literals in mind
impl_into_prop!(|value: &'static str| -> String { value.to_owned() });

impl_into_prop!(|value: &'static str| -> Cow<'static, str> { Cow::Borrowed(value) });
impl_into_prop!(|value: String| -> Cow<'static, str> { Cow::Owned(value) });

/// A trait similar to `Into<T>` which allows conversion to an optional value of a `Properties`
/// struct.
pub trait IntoOptPropValue<T> {
    /// Convert `self` to an optional value of a `Properties` struct.
    fn into_opt_prop_value(self) -> Option<T>;
}
impl<T, V> IntoOptPropValue<V> for T
where
    T: IntoPropValue<Option<V>>,
{
    fn into_opt_prop_value(self) -> Option<V> {
        self.into_prop_value()
    }
}
