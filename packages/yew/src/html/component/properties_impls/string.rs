//! Submodule implementing the `Properties` trait for the `String` type.

use crate::Properties;

impl Properties for String {
    type Builder = String;

    fn builder() -> Self::Builder {
		Self::Builder::new()
    }
}
