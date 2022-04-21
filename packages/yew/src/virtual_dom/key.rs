//! This module contains the implementation yew's virtual nodes' keys.

use crate::html::ImplicitClone;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;

// Invariants that must hold for the key implementation to work since `usize`
// and `isize` are casted to `u128` and `i128`.
// TODO: Uncomment these when const assertions are available in MSRV.
// const _: () = assert!(std::mem::size_of::<usize>() <= std::mem::size_of::<u128>());
// const _: () = assert!(std::mem::size_of::<isize>() <= std::mem::size_of::<i128>());

// Some seeds / salts picked at random for types which should be treated
// uniformly but distinct from each other.
const UNSIGNED_SEED: u64 = 0x3a85f087a0fd37aa;
const SIGNED_SEED: u64 = 0x607899f4fe4b09fa;
const BYTES_SEED: u64 = 0x27422aa0116c109c;

#[cfg(feature = "twox-hash")]
mod imp {
    use std::hash::Hash;
    use twox_hash::xxh3::{Hash128, HasherExt};

    pub(super) type Repr = u128;

    #[inline]
    pub(super) fn from_hash<T>(seed: u64, value: T) -> Repr
    where
        T: Hash,
    {
        let mut hasher = Hash128::with_seed(seed);
        value.hash(&mut hasher);
        hasher.finish_ext()
    }
}

#[cfg(not(feature = "twox-hash"))]
mod imp {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    pub(super) type Repr = u64;

    #[inline]
    pub(super) fn from_hash<T>(seed: u64, value: T) -> Repr
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        value.hash(&mut hasher);
        hasher.finish()
    }
}

/// Represents the (optional) key of Yew's virtual nodes.
///
/// Keys are cheap to clone.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Key(self::imp::Repr);

// Default implementation for byte sequences.
impl From<&[u8]> for Key {
    #[inline]
    fn from(key: &[u8]) -> Self {
        Self(self::imp::from_hash(BYTES_SEED, key))
    }
}

impl From<Rc<[u8]>> for Key {
    #[inline]
    fn from(key: Rc<[u8]>) -> Self {
        Key::from(key.as_ref())
    }
}

impl From<Rc<str>> for Key {
    #[inline]
    fn from(key: Rc<str>) -> Self {
        Self::from(key.as_ref().as_bytes())
    }
}

impl From<Arc<[u8]>> for Key {
    #[inline]
    fn from(key: Arc<[u8]>) -> Self {
        Key::from(key.as_ref())
    }
}

impl From<Arc<str>> for Key {
    #[inline]
    fn from(key: Arc<str>) -> Self {
        Self::from(key.as_ref().as_bytes())
    }
}

impl From<&str> for Key {
    #[inline]
    fn from(key: &str) -> Self {
        Self::from(key.as_bytes())
    }
}

impl From<String> for Key {
    #[inline]
    fn from(key: String) -> Self {
        Self::from(key.as_bytes())
    }
}

impl<const N: usize> From<[u8; N]> for Key {
    #[inline]
    fn from(key: [u8; N]) -> Self {
        Self::from(key.as_ref())
    }
}

impl From<Vec<u8>> for Key {
    #[inline]
    fn from(key: Vec<u8>) -> Self {
        Self::from(key.as_slice())
    }
}

impl From<char> for Key {
    #[inline]
    fn from(key: char) -> Key {
        Self::from(key as u32)
    }
}

impl ImplicitClone for Key {}

impl From<u128> for Key {
    #[inline]
    fn from(key: u128) -> Self {
        Self(self::imp::from_hash(UNSIGNED_SEED, key))
    }
}

impl From<i128> for Key {
    #[inline]
    fn from(key: i128) -> Self {
        Self(self::imp::from_hash(SIGNED_SEED, key))
    }
}

macro_rules! unsigned {
    ($type:ty) => {
        impl From<$type> for Key {
            #[inline]
            fn from(key: $type) -> Self {
                Self::from(key as u128)
            }
        }
    };
}

macro_rules! signed {
    ($type:ty) => {
        impl From<$type> for Key {
            #[inline]
            fn from(key: $type) -> Self {
                Self::from(key as i128)
            }
        }
    };
}

unsigned!(u8);
unsigned!(u16);
unsigned!(u32);
unsigned!(u64);
unsigned!(usize);
signed!(i8);
signed!(i16);
signed!(i32);
signed!(i64);
signed!(isize);

#[cfg(test)]
mod test {
    use super::Key;
    use crate::html;
    use std::rc::Rc;

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn all_key_conversions() {
        html! {
            <key="string literal">
                <img key={"String".to_owned()} />
                <p key={vec![1, 2, 3, 4]}></p>
                <p key={Rc::<str>::from("rc")}></p>
                <p key={Rc::<[u8]>::from(&[1, 2, 3, 4][..])}></p>
                <p key={[1, 2, 3, 4]}></p>
                <key='a'>
                    <p key=11_usize></p>
                    <p key=12_u8></p>
                    <p key=13_u16></p>
                    <p key=14_u32></p>
                    <p key=15_u64></p>
                    <p key=15_u128></p>
                    <p key=21_isize></p>
                    <p key=22_i8></p>
                    <p key=23_i16></p>
                    <p key=24_i32></p>
                    <p key=25_i64></p>
                    <p key=26_i128></p>
                </>
            </>
        };
    }

    #[test]
    fn key_from_integer_equality() {
        macro_rules! test_eq {
            ($n:expr) => {
                assert_eq!(Key::from($n), Key::from($n as u8));
                assert_eq!(Key::from($n), Key::from($n as u16));
                assert_eq!(Key::from($n), Key::from($n as u32));
                assert_eq!(Key::from($n), Key::from($n as u64));
                assert_eq!(Key::from($n), Key::from($n as u128));
                assert_eq!(Key::from($n), Key::from($n as usize));
            };
        }

        macro_rules! test_ne {
            ($n:expr) => {
                assert_ne!(Key::from($n), Key::from($n as i8));
                assert_ne!(Key::from($n), Key::from($n as i16));
                assert_ne!(Key::from($n), Key::from($n as i32));
                assert_ne!(Key::from($n), Key::from($n as i64));
                assert_ne!(Key::from($n), Key::from($n as i128));
                assert_ne!(Key::from($n), Key::from($n as isize));

                // Distinct from byte arrays.
                assert_ne!(Key::from($n), Key::from($n.to_le_bytes()));
                assert_ne!(Key::from($n), Key::from($n.to_le_bytes()));
                assert_ne!(Key::from($n), Key::from($n.to_le_bytes()));
                assert_ne!(Key::from($n), Key::from($n.to_le_bytes()));
                assert_ne!(Key::from($n), Key::from($n.to_le_bytes()));
                assert_ne!(Key::from($n), Key::from($n.to_le_bytes()));
            };
        }

        test_eq!(1usize);
        test_eq!(1u8);
        test_eq!(1u16);
        test_eq!(1u32);
        test_eq!(1u64);
        test_eq!(1u128);

        test_ne!(1usize);
        test_ne!(1u8);
        test_ne!(1u16);
        test_ne!(1u32);
        test_ne!(1u64);
        test_ne!(1u128);
    }
}
