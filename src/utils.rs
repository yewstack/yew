//! This module contains useful utils to get information about the current document.

use failure::{err_msg, Error};
#[cfg(feature = "std_web")]
use stdweb::web::document;

/// Returns `host` for the current document. Useful to connect to a server that server the app.
pub fn host() -> Result<String, Error> {
    #[cfg(feature = "std_web")]
    {
        document()
            .location()
            .ok_or_else(|| err_msg("can't get location"))
            .and_then(|l| l.host().map_err(Error::from))
    }
    #[cfg(feature = "web_sys")]
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .location()
        .ok_or_else(|| err_msg("can't get location"))
        .and_then(|l| {
            l.host().map_err(|e| {
                err_msg(
                    e.as_string()
                        .unwrap_or_else(|| String::from("error not recoverable")),
                )
            })
        })
}
