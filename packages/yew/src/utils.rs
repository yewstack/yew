//! This module contains useful utilities to get information about the current document.

use anyhow::{anyhow, Error};
use std::marker::PhantomData;
use web_sys::{Document, Window};
use yew::html::ChildrenRenderer;

/// Map IntoIterator<Item=Into<T>> to Iterator<Item=T>
pub fn into_node_iter<IT, T, R>(it: IT) -> impl Iterator<Item = R>
where
    IT: IntoIterator<Item = T>,
    T: Into<R>,
{
    it.into_iter().map(|n| n.into())
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
