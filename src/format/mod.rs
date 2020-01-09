//! Utility module to convert data to types and back by
//! specific formats like: JSON, BSON, TOML, YAML, XML.
//!
//! All types here are lazy and it's necessary to
//! use `Into` and `From` traits to get (convert) the data.

use cfg_if::cfg_if;
use failure::Error;

#[macro_use]
pub mod macros;

pub mod json;
pub mod nothing;

pub use self::json::Json;
pub use self::nothing::Nothing;

cfg_if! {
    if #[cfg(feature = "bincode")] {
        pub mod bincode;
        pub use self::bincode::Bincode;
    } else if #[cfg(feature = "cbor")] {
        pub mod cbor;
        pub use self::cbor::Cbor;
    } else if #[cfg(feature = "cbor_packed")] {
        pub mod cbor_packed;
        pub use self::cbor_packed::CborPacked;
    } else if #[cfg(feature = "msgpack")] {
        pub mod msgpack;
        pub use self::msgpack::MsgPack;
    } else if #[cfg(feature = "toml")] {
        pub mod toml;
        pub use self::toml::Toml;
    } else if #[cfg(feature = "yaml")] {
        pub mod yaml;
        pub use self::yaml::Yaml;
    }
}

/// A representation of a value which can be stored and restored as a text.
pub type Text = Result<String, Error>;

/// A representation of a value which can be stored and restored as a binary.
pub type Binary = Result<Vec<u8>, Error>;

/// A helper which represents a specific format.
#[doc(hidden)]
pub type Format<T> = Result<T, Error>;
