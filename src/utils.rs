//! This module contains useful utils to get information about the current document.

use failure::{err_msg, Error};
use stdweb::web::document;

/// Returns `host` for the current document. Useful to connect to a server that server the app.
pub fn host() -> Result<String, Error> {
    document()
        .location()
        .ok_or_else(|| err_msg("can't get location"))
        .and_then(|l| l.host().map_err(Error::from))
}
