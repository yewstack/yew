//! Parser for yew-router's matcher syntax.
//! This syntax allows specifying if a route should produce an enum variant or struct,
//! and allows capturing sections from the route to be incorporated into its associated variant or struct.

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_qualifications
)]

mod core;
mod error;
pub mod parser;
pub use crate::core::FieldNamingScheme;
pub use error::{ParseError, PrettyParseError};
mod optimizer;
pub use optimizer::{convert_tokens, parse_str_and_optimize_tokens};
use std::collections::HashMap;

/// Alias of `HashMap<&'a str, String>` that represent strings captured from a route.
///
/// Captures contain keys corresponding to named match sections,
/// and values containing the content captured by those sections.
pub type Captures<'a> = HashMap<&'a str, String>;

/// Tokens used to determine how to match and capture sections from a URL.
#[derive(Debug, PartialEq, Clone)]
pub enum MatcherToken {
    /// Section-related tokens can be condensed into a match.
    Exact(String),
    /// Capture section.
    Capture(CaptureVariant),
    /// End token - if the string hasn't been consumed entirely, then the parse will fail.
    /// This is useful for being able to specify more general matchers for variants that would
    /// otherwise match above more specific variants.
    End,
}

/// Variants that indicate how part of a string should be captured.
#[derive(Debug, PartialEq, Clone)]
pub enum CaptureVariant {
    /// {}
    Unnamed,
    /// {*}
    ManyUnnamed,
    /// {5}
    NumberedUnnamed {
        /// Number of sections to match.
        sections: usize,
    },
    /// {name} - captures a section and adds it to the map with a given name.
    Named(String),
    /// {*:name} - captures over many sections and adds it to the map with a given name.
    ManyNamed(String),
    /// {2:name} - captures a fixed number of sections with a given name.
    NumberedNamed {
        /// Number of sections to match.
        sections: usize,
        /// The key to be entered in the `Matches` map.
        name: String,
    },
}
