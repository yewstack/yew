//! This module contains useful utils to get information about the current document.

use anyhow::{anyhow, Error};
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use std::marker::PhantomData;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Document, Window};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::{Document, Window};
    }
}

/// Returns current window.
pub fn window() -> Window {
    cfg_match! {
        feature = "std_web" => stdweb::web::window(),
        feature = "web_sys" => web_sys::window().expect("no window available"),
    }
}

/// Returns current document.
pub fn document() -> Document {
    cfg_match! {
        feature = "std_web" => stdweb::web::document(),
        feature = "web_sys" => window().document().unwrap(),
    }
}

/// Returns `host` for the current document. Useful to connect to a server that serves the app.
pub fn host() -> Result<String, Error> {
    let location = document()
        .location()
        .ok_or_else(|| anyhow!("can't get location"))?;

    #[cfg(feature = "std_web")]
    let host = location.host().map_err(Error::from)?;

    #[cfg(feature = "web_sys")]
    let host = location.host().map_err(|e| {
        anyhow!(e
            .as_string()
            .unwrap_or_else(|| String::from("error not recoverable")),)
    })?;

    Ok(host)
}

/// Returns `origin` for the current window.
pub fn origin() -> Result<String, Error> {
    let location = window().location();

    #[cfg(feature = "std_web")]
    let location = location.ok_or_else(|| anyhow!("can't get location"))?;

    #[cfg(feature = "std_web")]
    let origin = location.origin().map_err(Error::from)?;

    #[cfg(feature = "web_sys")]
    let origin = location.origin().map_err(|e| {
        anyhow!(e
            .as_string()
            .unwrap_or_else(|| String::from("error not recoverable")),)
    })?;

    Ok(origin)
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
