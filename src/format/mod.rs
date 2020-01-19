//! Utility module to convert data to types and back by
//! specific formats like: JSON, BSON, TOML, YAML, XML.
//!
//! All types here are lazy and it's necessary to
//! use `Into` and `From` traits to get (convert) the data.

use anyhow::Error;
use thiserror::Error as ThisError;

#[macro_use]
pub mod macros;

#[cfg(feature = "bincode")]
pub mod bincode;
#[cfg(feature = "cbor")]
pub mod cbor;
pub mod json;
#[cfg(feature = "msgpack")]
pub mod msgpack;
pub mod nothing;
#[cfg(feature = "toml")]
pub mod toml;
#[cfg(feature = "yaml")]
pub mod yaml;

#[cfg(feature = "bincode")]
pub use self::bincode::Bincode;
#[cfg(feature = "cbor")]
pub use self::cbor::Cbor;
pub use self::json::Json;
#[cfg(feature = "msgpack")]
pub use self::msgpack::MsgPack;
pub use self::nothing::Nothing;
#[cfg(feature = "toml")]
pub use self::toml::Toml;
#[cfg(feature = "yaml")]
pub use self::yaml::Yaml;

/// A representation of a value which can be stored and restored as a text.
///
/// Some formats are binary only and can't be serialized to or deserialized
/// from Text.  Attempting to do so will return an Err(FormatError).
pub type Text = Result<String, Error>;

/// A representation of a value which can be stored and restored as a binary.
pub type Binary = Result<Vec<u8>, Error>;

/// A helper which represents a specific format.
#[doc(hidden)]
pub type Format<T> = Result<T, Error>;

/// Represents formatting errors.
#[derive(Debug, ThisError)]
pub enum FormatError {
    /// Received text for a binary format, e.g. someone sending text
    /// on a WebSocket that is using a binary serialization format, like Cbor.
    #[error("received text for a binary format")]
    ReceivedTextForBinary,
    /// Trying to encode a binary format as text", e.g., trying to
    /// store a Cbor encoded value in a String.
    #[error("trying to encode a binary format as Text")]
    CantEncodeBinaryAsText,
}
