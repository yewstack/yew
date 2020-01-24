//! Contains three macros for wrapping serde format.  Collectively they
//! allow you to define your own text and binary wrappers.

/// This macro is used for a format that can be encoded as Text.  It
/// is used in conjunction with a type definition for a tuple struct
/// with one (publically accessible) element of a generic type.  Since
/// any type that can be encoded as Text can also be encoded as Binary,
/// it should be used with the binary_format macro.
///
/// ## Example
///
/// ```rust
/// use yew::{binary_format, text_format};
///
/// pub struct Json<T>(pub T);
///
/// text_format!(Json based on serde_json);
/// binary_format!(Json based on serde_json);
/// ```
#[macro_export]
macro_rules! text_format {
    ($type:ident based on $format:ident) => {
        impl<'a, T> Into<$crate::format::Text> for $type<&'a T>
        where
            T: ::serde::Serialize,
        {
            fn into(self) -> $crate::format::Text {
                $format::to_string(&self.0).map_err(::anyhow::Error::from)
            }
        }

        impl<T> From<$crate::format::Text> for $type<Result<T, ::anyhow::Error>>
        where
            T: for<'de> ::serde::Deserialize<'de>,
        {
            fn from(value: $crate::format::Text) -> Self {
                match value {
                    Ok(data) => $type($format::from_str(&data).map_err(::anyhow::Error::from)),
                    Err(reason) => $type(Err(reason)),
                }
            }
        }
    };
}

/// This macro is used for a format that can be encoded as Binary.  It
/// is used in conjunction with a type definition for a tuple struct
/// with one (publicly accessible) element of a generic type.  Not
/// all types that can be encoded as Binary can be encoded as Text.
/// As such, this macro should be paired with the text_format macro
/// where such an encoding works (e.g., JSON), or with the
/// text_format_is_an_error macro for binary-only formats (e.g.,
/// MsgPack).
///
/// # Rely on serde's `to_vec` and `from_vec`
/// The simplest form of this macro relegates all the work to serde's
/// `to_vec` function for serialization and serde's `from_vec` for
/// deseriaization.
///
/// ## Examples
///
/// ### Binary that is also Text
///
/// ```rust
/// use yew::{binary_format, text_format};
///
/// pub struct Json<T>(pub T);
///
/// text_format!(Json based on serde_json);
/// binary_format!(Json based on serde_json);
/// ```
///
/// ### Binary only
/// ```rust
/// # mod to_make_rustdoc_happy {
///   use rmp_serde;
///   use yew::{binary_format, text_format_is_an_error};
///
///   pub struct MsgPack<T>(pub T);
///
///   binary_format!(MsgPack based on rmp_serde);
///   text_format_is_an_error!(MsgPack);
/// # }
/// ```
///
/// # Supply serialization and deserialization functions
///
/// In addition to the "based on" form of this macro demonstrated above,
/// you can use the three parameter second form to supply
/// non-standard (i.e., alternatives to serde's `to_vec` and/or `from_slice`)
/// helpers as the second and third parameters.
///
/// ## Example
/// ```rust
/// # mod to_make_rustdoc_happy {
///   use bincode;
///   use yew::{binary_format, text_format_is_an_error};
///
///   pub struct Bincode<T>(pub T);
///
///   binary_format!(Bincode, bincode::serialize, bincode::deserialize);
///   text_format_is_an_error!(Bincode);
/// # }
/// ```
#[macro_export]
macro_rules! binary_format {
    ($type:ident based on $format:ident) => {
        binary_format!($type, $format::to_vec, $format::from_slice);
    };
    ($type:ident, $into:path, $from:path) => {
        impl<'a, T> Into<$crate::format::Binary> for $type<&'a T>
        where
            T: ::serde::Serialize,
        {
            fn into(self) -> $crate::format::Binary {
                $into(&self.0).map_err(::anyhow::Error::from)
            }
        }

        impl<T> From<$crate::format::Binary> for $type<Result<T, ::anyhow::Error>>
        where
            T: for<'de> ::serde::Deserialize<'de>,
        {
            fn from(value: $crate::format::Binary) -> Self {
                match value {
                    Ok(data) => $type($from(&data).map_err(::anyhow::Error::from)),
                    Err(reason) => $type(Err(reason)),
                }
            }
        }
    };
}

/// This macro is used for a format that can be encoded as Binary but
/// can't be encoded as Text.  It is used in conjunction with a type
/// definition for a tuple struct with one (publically accessible)
/// element of a generic type.  This macro should be paired with the
/// binary_format macro that defines the binary-only format.
///
/// ## Example
/// ```rust
/// # mod to_make_rustdoc_happy {
///   use rmp_serde;
///   use yew::{binary_format, text_format_is_an_error};
///
///   pub struct MsgPack<T>(pub T);
///
///   binary_format!(MsgPack based on rmp_serde);
///   text_format_is_an_error!(MsgPack);
/// # }
/// ```
#[macro_export]
#[cfg(any(feature = "bincode", feature = "cbor", feature = "msgpack"))]
macro_rules! text_format_is_an_error {
    ($type:ident) => {
        use $crate::{format::FormatError, text_format};

        fn to_string<T>(_value: T) -> Result<String, ::anyhow::Error> {
            Err(FormatError::CantEncodeBinaryAsText.into())
        }

        fn from_str<T>(_s: &str) -> Result<T, ::anyhow::Error> {
            Err(FormatError::ReceivedTextForBinary.into())
        }

        text_format!($type based on self);
    }
}
