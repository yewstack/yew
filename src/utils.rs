//! This module contains useful utils to get information about the current document.

use failure::{err_msg, Error};
use stdweb::web::document;

use crate::virtual_dom::VNode;

/// Returns `host` for the current document. Useful to connect to a server that server the app.
pub fn host() -> Result<String, Error> {
    document()
        .location()
        .ok_or_else(|| err_msg("can't get location"))
        .and_then(|l| l.host().map_err(Error::from))
}

/// Specialty type necessary for helping flattening components returned from nested html macros.
#[derive(Debug)]
pub struct NodeSeq<T>(Vec<T>)
where
    T: Into<VNode>;

impl<T: Into<VNode>> From<T> for NodeSeq<T> {
    fn from(val: T) -> Self {
        NodeSeq(vec![val])
    }
}

impl<T: Into<VNode>> From<Vec<T>> for NodeSeq<T> {
    fn from(val: Vec<T>) -> Self {
        NodeSeq(val)
    }
}

impl<T: Into<VNode>> IntoIterator for NodeSeq<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
