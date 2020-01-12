//! This module contains useful utils to get information about the current document.

use failure::{err_msg, Error};
use std::marker::PhantomData;

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

/// Specialty type necessary for helping flattening components returned from nested html macros.
#[derive(Debug)]
pub struct NodeSeq<IN, OUT>(Vec<OUT>, PhantomData<IN>);

impl<IN: Into<OUT>, OUT> From<IN> for NodeSeq<IN, OUT> {
    fn from(val: IN) -> Self {
        Self(vec![val.into()], PhantomData::default())
    }
}

impl<IN: Into<OUT>, OUT> From<Vec<IN>> for NodeSeq<IN, OUT> {
    fn from(val: Vec<IN>) -> Self {
        Self(
            val.into_iter().map(|x| x.into()).collect(),
            PhantomData::default(),
        )
    }
}

impl<IN: Into<OUT>, OUT> IntoIterator for NodeSeq<IN, OUT> {
    type Item = OUT;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
