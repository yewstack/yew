//! Contains an implementation of empty serialization format (`Nothing`).

use super::{Binary, Text};
use failure::err_msg;

/// A representation of an empty data. Nothing stored. Nothing restored.
#[derive(Debug)]
pub struct Nothing;

impl Into<Text> for Nothing {
    fn into(self) -> Text {
        Err(err_msg("nothing"))
    }
}

impl From<Text> for Nothing {
    fn from(_: Text) -> Nothing {
        Nothing
    }
}

impl Into<Binary> for Nothing {
    fn into(self) -> Binary {
        Err(err_msg("nothing"))
    }
}

impl From<Binary> for Nothing {
    fn from(_: Binary) -> Nothing {
        Nothing
    }
}
