//! This module contains the implementation yew's virtual nodes' keys.

use std::fmt::{self, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

use crate::html::ImplicitClone;

/// Represents the (optional) key of Yew's virtual nodes.
///
/// Keys are cheap to clone.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Key {
    key: Rc<str>,
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.key.fmt(f)
    }
}

impl Deref for Key {
    type Target = str;

    fn deref(&self) -> &str {
        self.key.as_ref()
    }
}

impl From<Rc<str>> for Key {
    fn from(key: Rc<str>) -> Self {
        Self { key }
    }
}

impl From<&'_ str> for Key {
    fn from(key: &'_ str) -> Self {
        let key: Rc<str> = Rc::from(key);
        Self::from(key)
    }
}

impl ImplicitClone for Key {}

macro_rules! key_impl_from_to_string {
    ($type:ty) => {
        impl From<$type> for Key {
            fn from(key: $type) -> Self {
                Self::from(key.to_string().as_str())
            }
        }
    };
}

key_impl_from_to_string!(String);
key_impl_from_to_string!(char);
key_impl_from_to_string!(u8);
key_impl_from_to_string!(u16);
key_impl_from_to_string!(u32);
key_impl_from_to_string!(u64);
key_impl_from_to_string!(u128);
key_impl_from_to_string!(usize);
key_impl_from_to_string!(i8);
key_impl_from_to_string!(i16);
key_impl_from_to_string!(i32);
key_impl_from_to_string!(i64);
key_impl_from_to_string!(i128);
key_impl_from_to_string!(isize);

#[cfg(test)]
mod test {
    use std::rc::Rc;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use crate::html;

    #[cfg(target_arch = "wasm32")]
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
                    <p key=15_u128></p>
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
