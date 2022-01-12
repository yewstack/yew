//! This module contains the items required for a statically typed VDOM

use crate::events::*;
use crate::html::*;
use crate::virtual_dom::*;
use std::collections::HashMap;
use std::rc::Rc;
use yew_macro::{generate_element, global};

global! {
    attrs {
        autocapitalize: AttrValue,
        contextmenu: AttrValue,
        contenteditable: AttrValue,
        slot: AttrValue,
        spellcheck: AttrValue,
        class: AttrValue,
        title: AttrValue,
        itemprop: AttrValue,
        accesskey: AttrValue,
        lang: AttrValue,
        id: AttrValue,
        translate: AttrValue,
        draggable: AttrValue,
        style: AttrValue,
        dir: AttrValue,
        tabindex: AttrValue,
        hidden: AttrValue,
    }
    listeners {
        click: MouseEvent
    }
}

generate_element! {
    button;
    extends: GlobalAttributes;
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

// I can't fucking come up with a better name.
/// Props which are extended
pub trait PropsExtend {
    /// Provides [`ElementData`] of extender
    fn into_data(self) -> ElementData;
}

/// Metadata of an HTML element
///
/// A [Component](crate::html::Component) is generated using this data for every element.
///
/// # Note
///
/// While it says "Element" in the name, usage with an element necessarily required.
/// `#[properties(extends = Type)]` uses this type to provide information about the extended type.
#[derive(Debug)]
pub struct ElementData {
    node_ref: NodeRef,
    attributes: HashMap<&'static str, AttrValue>,
    listeners: Vec<Option<Rc<dyn Listener>>>,
    key: Option<Key>,
    children: Vec<VNode>,
}
