//! This module contains useful utilities to get information about the current document.

use anyhow::{anyhow, Error};
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use yew::html::ChildrenRenderer;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Document, Window};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::{Document, Window};
    }
}

/// Returns the current window. This function will panic if there is no available window.
pub fn window() -> Window {
    cfg_match! {
        feature = "std_web" => stdweb::web::window(),
        feature = "web_sys" => web_sys::window().expect("no window available"),
    }
}

/// Returns the current document.
pub fn document() -> Document {
    cfg_match! {
        feature = "std_web" => stdweb::web::document(),
        feature = "web_sys" => window().document().unwrap(),
    }
}

/// Returns the `host` for the current document. Useful for connecting to the server which serves
/// the app.
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

/// Returns the `origin` of the current window.
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

/// A special type necessary for flattening components returned from nested html macros.
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

impl<IN: Into<OUT>, OUT> From<ChildrenRenderer<IN>> for NodeSeq<IN, OUT> {
    fn from(val: ChildrenRenderer<IN>) -> Self {
        Self(
            val.into_iter().map(|x| x.into()).collect(),
            PhantomData::default(),
        )
    }
}

impl<IN, OUT> IntoIterator for NodeSeq<IN, OUT> {
    type Item = OUT;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Reference to either a &'static str or an owned String
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StringRef {
    /// Reference to a static string
    Static(&'static str),

    /// Owned String
    Owned(String),
}

impl StringRef {
    /// Returns the length of the contained string
    pub fn len(&self) -> usize {
        match self {
            Self::Static(s) => s.len(),
            Self::Owned(s) => s.len(),
        }
    }

    /// Returns if the contained string is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns mutable reference to contents
    pub fn to_mut(&mut self) -> &mut String {
        self.as_mut()
    }
}

impl AsRef<str> for StringRef {
    fn as_ref(&self) -> &str {
        match self {
            Self::Static(s) => s,
            Self::Owned(s) => &s,
        }
    }
}

impl std::ops::Deref for StringRef {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl AsMut<String> for StringRef {
    fn as_mut(&mut self) -> &mut String {
        if let Self::Static(s) = self {
            *self = Self::Owned(s.to_owned())
        }
        match self {
            Self::Static(_) => unreachable!(),
            Self::Owned(s) => s,
        }
    }
}

impl Into<String> for StringRef {
    fn into(self) -> String {
        match self {
            Self::Static(s) => s.into(),
            Self::Owned(s) => s,
        }
    }
}

impl From<&str> for StringRef {
    fn from(val: &str) -> Self {
        Self::from(val.to_owned())
    }
}

impl From<&&str> for StringRef {
    fn from(val: &&str) -> Self {
        Self::from(val.to_owned())
    }
}

impl From<String> for StringRef {
    fn from(val: String) -> Self {
        Self::Owned(val)
    }
}

impl From<&String> for StringRef {
    fn from(val: &String) -> Self {
        Self::from(val.clone())
    }
}

impl PartialEq for StringRef {
    fn eq(&self, other: &StringRef) -> bool {
        (self.as_ref() as &str) == other.as_ref()
    }
}

impl Eq for StringRef {}

impl PartialEq<str> for StringRef {
    fn eq(&self, other: &str) -> bool {
        self.as_ref() == other
    }
}

impl std::hash::Hash for StringRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write((self.as_ref() as &str).as_bytes())
    }
}

impl std::fmt::Display for StringRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
