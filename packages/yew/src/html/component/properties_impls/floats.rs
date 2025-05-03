//! Submodule implementing the `Properties` trait for several float types.

use crate::Properties;

/// Macro implementing the `Properties` trait for various float types.
macro_rules! impl_properties_for_floats {
	($($t:ty),+) => {
		$(
			impl Properties for $t {
				type Builder = $t;

				fn builder() -> Self::Builder {
					Self::Builder::default()
				}
			}
		)+
	};
}

impl_properties_for_floats!(f32, f64);
