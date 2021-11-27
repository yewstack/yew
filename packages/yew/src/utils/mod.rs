//! This module contains useful utilities to get information about the current document.

use std::marker::PhantomData;
use yew::html::ChildrenRenderer;
use yew::html::RenderResult;

/// A special type necessary for flattening components returned from nested html macros.
#[derive(Debug)]
pub struct Node<I, O>(O, PhantomData<I>);

impl<I, O> Node<I, O> {
    /// Returns the wrapped value.
    pub fn into_value(self) -> O {
        self.0
    }
}

/// A special trait to convert to a `RenderResult`.
pub trait TryIntoNode<I, O> {
    /// Performs the conversion.
    fn try_into_node(self) -> RenderResult<Node<I, O>>;
}

impl<I, O> TryIntoNode<I, O> for I
where
    I: Into<O>,
{
    fn try_into_node(self) -> RenderResult<Node<I, O>> {
        Ok(Node(self.into(), PhantomData::default()))
    }
}

impl<I, O> TryIntoNode<I, O> for RenderResult<I>
where
    I: Into<O>,
{
    fn try_into_node(self) -> RenderResult<Node<I, O>> {
        Ok(Node(self?.into(), PhantomData::default()))
    }
}

/// A special type necessary for flattening components returned from nested html macros.
#[derive(Debug)]
pub struct NodeSeq<I, O>(Vec<O>, PhantomData<I>);

impl<I, O> IntoIterator for NodeSeq<I, O> {
    type Item = O;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// A special trait to convert to a `NodeSeq`.
pub trait TryIntoNodeSeq<I, O> {
    /// Performs the conversion.
    fn try_into_node_seq(self) -> RenderResult<NodeSeq<I, O>>;
}

impl<I, O> TryIntoNodeSeq<I, O> for I
where
    I: Into<O>,
{
    fn try_into_node_seq(self) -> RenderResult<NodeSeq<I, O>> {
        Ok(NodeSeq(vec![self.into()], PhantomData::default()))
    }
}

impl<I, O> TryIntoNodeSeq<I, O> for Vec<I>
where
    I: Into<O>,
{
    fn try_into_node_seq(self) -> RenderResult<NodeSeq<I, O>> {
        Ok(NodeSeq(
            self.into_iter().map(|x| x.into()).collect(),
            PhantomData::default(),
        ))
    }
}

impl<I, O> TryIntoNodeSeq<I, O> for ChildrenRenderer<I>
where
    I: Into<O>,
{
    fn try_into_node_seq(self) -> RenderResult<NodeSeq<I, O>> {
        Ok(NodeSeq(
            self.into_iter().map(|x| x.into()).collect(),
            PhantomData::default(),
        ))
    }
}

impl<I, O> TryIntoNodeSeq<I, O> for RenderResult<I>
where
    I: Into<O>,
{
    fn try_into_node_seq(self) -> RenderResult<NodeSeq<I, O>> {
        Ok(NodeSeq(vec![self?.into()], PhantomData::default()))
    }
}

impl<I, O> TryIntoNodeSeq<I, O> for RenderResult<Vec<I>>
where
    I: Into<O>,
{
    fn try_into_node_seq(self) -> RenderResult<NodeSeq<I, O>> {
        Ok(NodeSeq(
            self?.into_iter().map(|x| x.into()).collect(),
            PhantomData::default(),
        ))
    }
}

impl<I, O> TryIntoNodeSeq<I, O> for Vec<RenderResult<I>>
where
    I: Into<O>,
{
    fn try_into_node_seq(self) -> RenderResult<NodeSeq<I, O>> {
        let mut nodes = Vec::new();

        for node in self {
            nodes.push(node?.into());
        }

        Ok(NodeSeq(nodes, PhantomData::default()))
    }
}

impl<I, O> TryIntoNodeSeq<I, O> for RenderResult<ChildrenRenderer<I>>
where
    I: Into<O>,
{
    fn try_into_node_seq(self) -> RenderResult<NodeSeq<I, O>> {
        Ok(NodeSeq(
            self?.into_iter().map(|x| x.into()).collect(),
            PhantomData::default(),
        ))
    }
}

impl<I, O> TryIntoNodeSeq<I, O> for ChildrenRenderer<RenderResult<I>>
where
    I: Into<O>,
{
    fn try_into_node_seq(self) -> RenderResult<NodeSeq<I, O>> {
        let mut nodes = Vec::new();

        for node in self {
            nodes.push(node?.into());
        }

        Ok(NodeSeq(nodes, PhantomData::default()))
    }
}

impl<I, O> TryIntoNodeSeq<I, O> for RenderResult<ChildrenRenderer<RenderResult<I>>>
where
    I: Into<O>,
{
    fn try_into_node_seq(self) -> RenderResult<NodeSeq<I, O>> {
        let mut nodes = Vec::new();

        for node in self? {
            nodes.push(node?.into());
        }

        Ok(NodeSeq(nodes, PhantomData::default()))
    }
}

/// Hack to force type mismatch compile errors in yew-macro.
//
// TODO: replace with `compile_error!`, when `type_name_of_val` is stabilised (https://github.com/rust-lang/rust/issues/66359).
#[doc(hidden)]
pub fn __ensure_type<T>(_: T) {}

/// Print the [web_sys::Node]'s contents as a string for debugging purposes
pub fn print_node(n: &web_sys::Node) -> String {
    use wasm_bindgen::JsCast;

    match n.dyn_ref::<web_sys::Element>() {
        Some(el) => el.outer_html(),
        None => n.text_content().unwrap_or_default(),
    }
}
