//! This module contains useful utils to get information about the current document.

use failure::{err_msg, Error};

#[cfg(feature = "std_web")]
/// Returns current document.
pub fn document() -> stdweb::web::Document {
    stdweb::web::document()
}

#[cfg(feature = "web_sys")]
/// Returns current document.
pub fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

/// Returns `host` for the current document. Useful to connect to a server that server the app.
pub fn host() -> Result<String, Error> {
    let location = document()
        .location()
        .ok_or_else(|| err_msg("can't get location"))?;

    #[cfg(feature = "std_web")]
    let host = location.host().map_err(Error::from)?;

    #[cfg(feature = "web_sys")]
    let host = location.host().map_err(|e| {
        err_msg(
            e.as_string()
                .unwrap_or_else(|| String::from("error not recoverable")),
        )
    })?;

    Ok(host)
}
