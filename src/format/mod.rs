//! Utility module to convert data to types and back by
//! specific formats like: JSON, BSON, TOML, YAML, XML.
//!
//! All types here are lazy and it's necessary to
//! use `Into` and `From` traits to get (convert) the data.

use failure::Error;

pub mod nothing;
pub mod json;
#[cfg(feature = "toml")]
pub mod toml;

pub use self::nothing::Nothing;
pub use self::json::Json;
#[cfg(feature = "toml")]
pub use self::toml::Toml;

/// A representation of a value which can be stored and restored as a text.
pub type Text = Result<String, Error>;

/// A representation of a value which can be stored and restored as a binary.
pub type Binary = Result<Vec<u8>, Error>;

/// A helper which represents a specific format.
#[doc(hidden)]
pub type Format<T> = Result<T, Error>;
