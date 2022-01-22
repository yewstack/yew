//! This module contains the items required for a statically typed VDOM

use std::collections::HashMap;
use std::rc::Rc;

use yew_macro::generate_element;

use crate::virtual_dom::{AttrValue, Key, Listener, VNode};
use crate::{Children, NodeRef};

generate_element! {
    button;
    props: {
        autofocus: AttrValue,
        disabled: AttrValue,
        form: AttrValue,
        formaction: AttrValue,
        formenctype: AttrValue,
        formmethod: AttrValue,
        formnovalidate: AttrValue,
        formtarget: AttrValue,
        name: AttrValue,
        type_: AttrValue,
        value: AttrValue,
        node_ref: NodeRef,
        key: Key,
        children: Children,
    }
}

/// Metadata of an HTML element
///
/// A [Component](crate::html::Component) is generated using this data for every element.
#[derive(Debug)]
pub struct ElementData {
    node_ref: NodeRef,
    attributes: HashMap<&'static str, AttrValue>,
    listeners: Vec<Option<Rc<dyn Listener>>>,
    key: Option<Key>,
    children: Vec<VNode>,
}
