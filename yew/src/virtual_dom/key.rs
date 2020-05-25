//! This module contains the implementation yew's virtual nodes' keys.

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

impl<K: Into<Rc<String>>> From<K> for Key {
    fn from(key: K) -> Self {
        Key { key: key.into() }
    }
}
