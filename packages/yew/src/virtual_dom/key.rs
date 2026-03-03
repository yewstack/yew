//! This module contains the implementation yew's virtual nodes' keys.

use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::num::NonZeroU64;
use std::rc::Rc;

use crate::html::ImplicitClone;

fn hash_value<H: Hash + ?Sized>(value: &H) -> NonZeroU64 {
    use std::hash::DefaultHasher;

    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    NonZeroU64::new(hasher.finish()).unwrap_or(NonZeroU64::MIN)
}

/// Represents the (optional) key of Yew's virtual nodes.
///
/// Keys are cheap to clone (a single `u64` copy) and to compare (a single
/// integer comparison). Internally a key stores a hash of the value it was
/// created from, so no heap allocation is required for numeric types in release
/// builds.
///
/// In debug builds the original string representation is kept alongside the
/// hash, enabling better diagnostics.
///
/// # Type-aware hashing
///
/// Keys created from different types are **not** equal even when their string
/// representations coincide. For example `Key::from("1")` and `Key::from(1u64)`
/// are distinct.
#[derive(Clone, ImplicitClone)]
pub struct Key {
    hash: NonZeroU64,
    #[cfg(debug_assertions)]
    original: Rc<str>,
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        #[cfg(debug_assertions)]
        {
            self.original.fmt(f)
        }
        #[cfg(not(debug_assertions))]
        {
            write!(f, "#{}", self.hash)
        }
    }
}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        #[cfg(debug_assertions)]
        {
            write!(f, "Key({:?})", self.original)
        }
        #[cfg(not(debug_assertions))]
        {
            write!(f, "Key(#{})", self.hash)
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for Key {}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash.cmp(&other.hash)
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl From<Rc<str>> for Key {
    fn from(key: Rc<str>) -> Self {
        Self {
            hash: hash_value(&*key),
            #[cfg(debug_assertions)]
            original: key,
        }
    }
}

impl From<&'_ str> for Key {
    fn from(key: &'_ str) -> Self {
        Self {
            hash: hash_value(key),
            #[cfg(debug_assertions)]
            original: Rc::from(key),
        }
    }
}

impl From<String> for Key {
    fn from(key: String) -> Self {
        Self::from(key.as_str())
    }
}

macro_rules! key_impl_from_numeric {
    ($type:ty) => {
        impl From<$type> for Key {
            fn from(key: $type) -> Self {
                Self {
                    hash: hash_value(&key),
                    #[cfg(debug_assertions)]
                    original: Rc::from(key.to_string().as_str()),
                }
            }
        }
    };
}

key_impl_from_numeric!(char);
key_impl_from_numeric!(u8);
key_impl_from_numeric!(u16);
key_impl_from_numeric!(u32);
key_impl_from_numeric!(u64);
key_impl_from_numeric!(u128);
key_impl_from_numeric!(usize);
key_impl_from_numeric!(i8);
key_impl_from_numeric!(i16);
key_impl_from_numeric!(i32);
key_impl_from_numeric!(i64);
key_impl_from_numeric!(i128);
key_impl_from_numeric!(isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_str_equal() {
        assert_eq!(Key::from("hello"), Key::from("hello"));
    }

    #[test]
    fn different_str_not_equal() {
        assert_ne!(Key::from("hello"), Key::from("world"));
    }

    #[test]
    fn same_integer_equal() {
        assert_eq!(Key::from(42u64), Key::from(42u64));
    }

    #[test]
    fn different_integer_not_equal() {
        assert_ne!(Key::from(1u64), Key::from(2u64));
    }

    #[test]
    fn str_and_integer_not_equal() {
        assert_ne!(Key::from("0"), Key::from(0u64));
    }

    #[test]
    fn string_and_str_equal() {
        assert_eq!(Key::from("abc"), Key::from(String::from("abc")));
    }

    #[test]
    fn rc_str_and_str_equal() {
        assert_eq!(Key::from("abc"), Key::from(Rc::<str>::from("abc")));
    }

    #[test]
    fn option_key_niche_optimised() {
        assert_eq!(
            std::mem::size_of::<Option<Key>>(),
            std::mem::size_of::<Key>()
        );
    }
}

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
#[cfg(test)]
mod wasm_tests {
    use std::rc::Rc;

    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use crate::html;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn all_key_conversions() {
        let _ = html! {
            <key="string literal">
                <img key={"String".to_owned()} />
                <p key={Rc::<str>::from("rc")}></p>
                <key='a'>
                    <p key=11_usize></p>
                    <p key=12_u8></p>
                    <p key=13_u16></p>
                    <p key=14_u32></p>
                    <p key=15_u64></p>
                    <p key=16_u128></p>
                    <p key=21_isize></p>
                    <p key=22_i8></p>
                    <p key=23_i16></p>
                    <p key=24_i32></p>
                    <p key=25_i128></p>
                </>
            </>
        };
    }
}
