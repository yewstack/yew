//! This module contains useful utils to get information about the current document.

use failure::{err_msg, Error};
use stdweb::web::document;
use crate::html::ShouldRender;

/// Returns `host` for the current document. Useful to connect to a server that server the app.
pub fn host() -> Result<String, Error> {
    document()
        .location()
        .ok_or_else(|| err_msg("can't get location"))
        .and_then(|l| l.host().map_err(Error::from))
}



/// Blanket trait to provide a convenience method for assigning props in `changed` or values in `update`
pub trait NeqAssign {
    /// If `self` and `new` aren't equal, assigns `new` to `self` and returns true, otherwise returns false.
    fn neq_assign(&mut self, new: Self) -> ShouldRender;
}

impl <T: PartialEq> NeqAssign for T {
    fn neq_assign(&mut self, new: T) -> ShouldRender {
        if self != &new {
            *self = new;
            true
        } else {
            false
        }
    }
}
