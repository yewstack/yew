use super::{Component, NodeRef, Scope};
use crate::{callback::Callback, virtual_dom::Key};
use std::{borrow::Cow, rc::Rc};

/// Marker trait for types that the [`html!`] macro may clone implicitly.
pub trait ImplicitClone: Clone {}

impl<T> ImplicitClone for Rc<T> {}
impl ImplicitClone for String {}
impl ImplicitClone for Cow<'static, str> {}

// TODO move these implementations to the type definitions
impl<T> ImplicitClone for Callback<T> {}
impl ImplicitClone for Key {}
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
impl<T> IntoPropValue<Option<T>> for T {
    fn into_prop_value(self) -> Option<T> {
        Some(self)
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
impl<T> IntoPropValue<Option<T>> for &T
where
    T: ImplicitClone,
{
    fn into_prop_value(self) -> Option<T> {
        Some(self.clone())
    }
}
impl<T> IntoPropValue<Option<T>> for Option<&T>
where
    T: ImplicitClone,
{
    fn into_prop_value(self) -> Option<T> {
        self.map(Clone::clone)
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
        impl IntoPropValue<Option<$to_ty>> for $from_ty {
            fn into_prop_value(self) -> Option<$to_ty> {
                let $value = self;
                Some({ $conversion })
            }
        }
        // implement Option<V> -> Option<T>
        impl IntoPropValue<Option<$to_ty>> for Option<$from_ty> {
            fn into_prop_value(self) -> Option<$to_ty> {
                #[allow(clippy::redundant_closure)]
                self.map(|$value| $conversion)
            }
        }
    };
}

// implemented for string literals
impl_into_prop!(|value: &'static str| -> String { value.to_owned() });

impl_into_prop!(|value: &'static str| -> Cow<'static, str> { Cow::Borrowed(value) });
impl_into_prop!(|value: String| -> Cow<'static, str> { Cow::Owned(value) });
// we allow this because `String` is `ImplicitClone`
impl_into_prop!(|value: &String| -> Cow<'static, str> { Cow::Owned(value.to_owned()) });

/// TODO
pub trait IntoOptPropValue<T> {
    /// TODO
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
