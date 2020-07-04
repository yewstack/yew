//! This module contains the implementation yew's virtual nodes' keys.

use std::ops::Deref;
use std::rc::Rc;

/// Represents the (optional) key of Yew's virtual nodes.
///
/// Keys are cheap to clone.
// TODO (#1263): Explain when keys are useful and add an example.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Key {
    key: Rc<String>,
}

impl core::fmt::Display for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", &self.key)
    }
}

impl core::fmt::Debug for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", &self.key)
    }
}

impl From<Rc<String>> for Key {
    fn from(key: Rc<String>) -> Self {
        Key { key }
    }
}

impl Deref for Key {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.key.as_ref()
    }
}

macro_rules! key_impl_from_to_string {
    ($type:ty) => {
        impl From<$type> for Key {
            fn from(key: $type) -> Self {
                Key {
                    key: Rc::new(key.to_string()),
                }
            }
        }
    };
}
key_impl_from_to_string!(&'static str);
key_impl_from_to_string!(String);
key_impl_from_to_string!(char);
key_impl_from_to_string!(usize);
key_impl_from_to_string!(u8);
key_impl_from_to_string!(u16);
key_impl_from_to_string!(u32);
key_impl_from_to_string!(u64);
key_impl_from_to_string!(u128);
key_impl_from_to_string!(i8);
key_impl_from_to_string!(i16);
key_impl_from_to_string!(i32);
key_impl_from_to_string!(i64);
key_impl_from_to_string!(i128);

#[cfg(test)]
mod test {
    use crate::html;
    use std::rc::Rc;

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn all_key_conversions() {
        let rc_key = Rc::new("rc".to_string());
        html! {
            <key="string literal">
                <img key="String".to_string() />
                <p key=rc_key></p>
                <key='a'>
                    <p key=11_usize></p>
                    <p key=12_u8></p>
                    <p key=13_u16></p>
                    <p key=14_u32></p>
                    <p key=15_u64></p>
                    <p key=15_u128></p>
                    <p key=22_i8></p>
                    <p key=23_i16></p>
                    <p key=24_i32></p>
                    <p key=25_i128></p>
                </>
            </>
        };
    }
}
