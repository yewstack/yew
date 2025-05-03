//! Submodule implementing the `Properties` trait for the `String` type.

use crate::Properties;

impl Properties for String {
    type Builder = String;

    fn builder() -> Self::Builder {
        Self::Builder::new()
    }
}

impl Properties for char {
    type Builder = char;

    fn builder() -> Self::Builder {
        Self::Builder::default()
    }
}
