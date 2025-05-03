//! Submodule implementing the `Properties` trait for several integer types.

use crate::Properties;

/// Macro implementing the `Properties` trait for various integer types.
macro_rules! impl_properties_for_integers {
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

impl_properties_for_integers!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize, bool);
