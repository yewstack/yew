use super::{Component, NodeRef, Scope};
use crate::virtual_dom::AttrValue;
use std::rc::Rc;

pub use imut::ImplicitClone;

impl ImplicitClone for NodeRef {}
impl<Comp: Component> ImplicitClone for Scope<Comp> {}
// TODO there are still a few missing

/// A trait similar to `Into<T>` which allows conversion to a value of a `Properties` struct.
pub trait IntoPropValue<T> {
    /// Convert `self` to a value of a `Properties` struct.
    fn into_prop_value(self) -> T;
}

impl<T> IntoPropValue<T> for T {
    #[inline]
    fn into_prop_value(self) -> T {
        self
    }
}
impl<T> IntoPropValue<T> for &T
where
    T: ImplicitClone,
{
    #[inline]
    fn into_prop_value(self) -> T {
        self.clone()
    }
}

impl<T> IntoPropValue<Option<T>> for T {
    #[inline]
    fn into_prop_value(self) -> Option<T> {
        Some(self)
    }
}
impl<T> IntoPropValue<Option<T>> for &T
where
    T: ImplicitClone,
{
    #[inline]
    fn into_prop_value(self) -> Option<T> {
        Some(self.clone())
    }
}

macro_rules! impl_into_prop {
    (|$value:ident: $from_ty:ty| -> $to_ty:ty { $conversion:expr }) => {
        // implement V -> T
        impl IntoPropValue<$to_ty> for $from_ty {
            #[inline]
            fn into_prop_value(self) -> $to_ty {
                let $value = self;
                $conversion
            }
        }
        // implement V -> Option<T>
        impl IntoPropValue<Option<$to_ty>> for $from_ty {
            #[inline]
            fn into_prop_value(self) -> Option<$to_ty> {
                let $value = self;
                Some({ $conversion })
            }
        }
        // implement Option<V> -> Option<T>
        impl IntoPropValue<Option<$to_ty>> for Option<$from_ty> {
            #[inline]
            fn into_prop_value(self) -> Option<$to_ty> {
                self.map(IntoPropValue::into_prop_value)
            }
        }
    };
}

// implemented with literals in mind
impl_into_prop!(|value: &'static str| -> String { value.to_owned() });

impl_into_prop!(|value: &'static str| -> AttrValue { AttrValue::Static(value) });
impl_into_prop!(|value: String| -> AttrValue { AttrValue::Rc(Rc::from(value)) });
impl_into_prop!(|value: Rc<str>| -> AttrValue { AttrValue::Rc(value) });

impl<T: ImplicitClone + 'static> IntoPropValue<imut::IArray<T>> for &'static [T] {
    fn into_prop_value(self) -> imut::IArray<T> {
        imut::IArray::from(self)
    }
}

impl<T: ImplicitClone + 'static> IntoPropValue<imut::IArray<T>> for Vec<T> {
    fn into_prop_value(self) -> imut::IArray<T> {
        imut::IArray::from(self)
    }
}

impl<K: Eq + std::hash::Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static>
    IntoPropValue<imut::IMap<K, V>> for &'static [(K, V)]
{
    fn into_prop_value(self) -> imut::IMap<K, V> {
        imut::IMap::from(self)
    }
}

impl<K: Eq + std::hash::Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static>
    IntoPropValue<imut::IMap<K, V>> for indexmap::IndexMap<K, V>
{
    fn into_prop_value(self) -> imut::IMap<K, V> {
        imut::IMap::from(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str() {
        let _: String = "foo".into_prop_value();
        let _: Option<String> = "foo".into_prop_value();
        let _: AttrValue = "foo".into_prop_value();
        let _: Option<AttrValue> = "foo".into_prop_value();
        let _: Option<AttrValue> = Rc::<str>::from("foo").into_prop_value();
    }
}
