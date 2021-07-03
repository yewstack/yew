//! This module contains the implementation of a virtual element node [VTag].

use super::{
    AttrValue, Attributes, Key, Listener, Listeners, Patch, PositionalAttr, VDiff, VList, VNode,
};
use crate::html::{AnyScope, IntoPropValue, NodeRef};
use crate::utils::document;
use gloo::events::EventListener;
use log::warn;
use std::borrow::Cow;
use std::cmp::PartialEq;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Once;
use wasm_bindgen::JsCast;
use web_sys::{
    Element, HtmlButtonElement, HtmlInputElement as InputElement,
    HtmlTextAreaElement as TextAreaElement,
};

/// SVG namespace string used for creating svg elements
pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// Default namespace for html elements
pub const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

/// [VTag] fields that are specific to different [VTag] kinds.
/// Decreases the memory footprint of [VTag] by avoiding impossible field and value combinations.
#[derive(Debug, Clone)]
enum VTagInner {
    /// Fields specific to
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
    /// [VTag]s
    Input {
        /// Contains a value of an
        /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
        value: Option<AttrValue>,

        /// Contains
        /// [kind](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#Form_%3Cinput%3E_types)
        /// value of an `InputElement`.
        kind: Option<AttrValue>,

        /// Represents `checked` attribute of
        /// [input](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-checked).
        /// It exists to override standard behavior of `checked` attribute, because
        /// in original HTML it sets `defaultChecked` value of `InputElement`, but for reactive
        /// frameworks it's more useful to control `checked` value of an `InputElement`.
        checked: bool,
    },

    /// Fields specific to
    /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
    /// [VTag]s
    Textarea {
        /// Contains a value of an
        /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
        value: Option<AttrValue>,
    },

    /// Fields for all other kinds of [VTag]s
    Other {
        /// A tag of the element.
        tag: Cow<'static, str>,

        /// List of child nodes
        children: VList,
    },
}

/// A type for a virtual
/// [Element](https://developer.mozilla.org/en-US/docs/Web/API/Element)
/// representation.
#[derive(Debug)]
pub struct VTag {
    /// [VTag] fields that are specific to different [VTag] kinds.
    inner: VTagInner,

    /// A reference to the DOM `Element`.
    pub reference: Option<Element>,

    /// List of attached listeners.
    pub listeners: Listeners,

    /// List of attributes.
    pub attributes: Attributes,

    /// A node reference used for DOM access in Component lifecycle methods
    pub node_ref: NodeRef,

    /// Keeps handler for attached listeners to have an opportunity to drop them later.
    captured: Vec<EventListener>,

    pub key: Option<Key>,
}

impl Clone for VTag {
    fn clone(&self) -> Self {
        VTag {
            inner: self.inner.clone(),
            reference: None,
            listeners: self.listeners.clone(),
            attributes: self.attributes.clone(),
            node_ref: self.node_ref.clone(),
            key: self.key.clone(),
            captured: Vec::new(),
        }
    }
}

impl VTag {
    /// Creates a new [VTag] instance with `tag` name (cannot be changed later in DOM).
    pub fn new(tag: impl Into<Cow<'static, str>>) -> Self {
        let tag: Cow<'static, str> = tag.into();
        Self::new_base(
            match &*tag.to_ascii_lowercase() {
                "input" => VTagInner::Input {
                    value: None,
                    kind: None,
                    checked: false,
                },
                "textarea" => VTagInner::Textarea { value: None },
                _ => VTagInner::Other {
                    tag,
                    children: Default::default(),
                },
            },
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }

    /// Creates a new
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) [VTag]
    /// instance.
    ///
    /// Unlike [VTag::new()], this sets all the public fields of [VTag] in one call. This allows the
    /// compiler to inline property and child list construction in the `html!` macro. This enables
    /// higher instruction parallelism by reducing data dependency and avoids `memcpy` of Vtag
    /// fields.
    #[doc(hidden)]
    #[allow(clippy::too_many_arguments)]
    pub fn __new_input(
        value: Option<AttrValue>,
        kind: Option<Cow<'static, str>>,
        checked: bool,
        node_ref: NodeRef,
        key: Option<Key>,
        // at bottom for more readable macro-expanded coded
        attributes: Attributes,
        listeners: Listeners,
    ) -> Self {
        VTag::new_base(
            VTagInner::Input {
                value,
                kind,
                // In HTML node `checked` attribute sets `defaultChecked` parameter,
                // but we use own field to control real `checked` parameter
                checked,
            },
            node_ref,
            key,
            attributes,
            listeners,
        )
    }

    /// Creates a new
    /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) [VTag]
    /// instance.
    ///
    /// Unlike [VTag::new()], this sets all the public fields of [VTag] in one call. This allows the
    /// compiler to inline property and child list construction in the `html!` macro. This enables
    /// higher instruction parallelism by reducing data dependency and avoids `memcpy` of Vtag
    /// fields.
    #[doc(hidden)]
    #[allow(clippy::too_many_arguments)]
    pub fn __new_textarea(
        value: Option<AttrValue>,
        node_ref: NodeRef,
        key: Option<Key>,
        // at bottom for more readable macro-expanded coded
        attributes: Attributes,
        listeners: Listeners,
    ) -> Self {
        VTag::new_base(
            VTagInner::Textarea { value },
            node_ref,
            key,
            attributes,
            listeners,
        )
    }

    /// Creates a new [VTag] instance with `tag` name (cannot be changed later in DOM).
    ///
    /// Unlike [VTag::new()], this sets all the public fields of [VTag] in one call. This allows the
    /// compiler to inline property and child list construction in the `html!` macro. This enables
    /// higher instruction parallelism by reducing data dependency and avoids `memcpy` of Vtag
    /// fields.
    #[doc(hidden)]
    #[allow(clippy::too_many_arguments)]
    pub fn __new_other(
        tag: Cow<'static, str>,
        node_ref: NodeRef,
        key: Option<Key>,
        // at bottom for more readable macro-expanded coded
        attributes: Attributes,
        listeners: Listeners,
        children: VList,
    ) -> Self {
        VTag::new_base(
            VTagInner::Other { tag, children },
            node_ref,
            key,
            attributes,
            listeners,
        )
    }

    /// Constructs a [VTag] from [VTagInner] and fields common to all [VTag] kinds
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn new_base(
        inner: VTagInner,
        node_ref: NodeRef,
        key: Option<Key>,
        attributes: Attributes,
        listeners: Listeners,
    ) -> Self {
        VTag {
            inner,
            reference: None,
            attributes,
            listeners,
            captured: Vec::new(),
            node_ref,
            key,
        }
    }

    /// Returns tag of an [Element]. In HTML tags are always uppercase.
    pub fn tag(&self) -> &str {
        match &self.inner {
            VTagInner::Input { .. } => "input",
            VTagInner::Textarea { .. } => "textarea",
            VTagInner::Other { tag, .. } => tag.as_ref(),
        }
    }

    /// Add [VNode] child.
    pub fn add_child(&mut self, child: VNode) {
        if let VTagInner::Other { children, .. } = &mut self.inner {
            children.add_child(child);
        }
    }

    /// Add multiple [VNode] children.
    pub fn add_children(&mut self, children: impl IntoIterator<Item = VNode>) {
        if let VTagInner::Other { children: dst, .. } = &mut self.inner {
            dst.add_children(children)
        }
    }

    /// Returns a reference to the children of this [VTag]
    pub fn children(&self) -> &VList {
        match &self.inner {
            VTagInner::Other { children, .. } => children,
            _ => {
                static mut EMPTY: MaybeUninit<VList> = MaybeUninit::uninit();
                static ONCE: Once = Once::new();
                unsafe {
                    ONCE.call_once(|| {
                        EMPTY = MaybeUninit::new(VList::default());
                    });
                    &*EMPTY.as_ptr()
                }
            }
        }
    }

    /// Returns a mutable reference to the children of this [VTag], if the node can have
    // children
    pub fn children_mut(&mut self) -> Option<&mut VList> {
        match &mut self.inner {
            VTagInner::Other { children, .. } => Some(children),
            _ => None,
        }
    }

    /// Returns the `value` of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) or
    /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
    pub fn value(&self) -> &Option<AttrValue> {
        match &self.inner {
            VTagInner::Input { value, .. } | VTagInner::Textarea { value } => value,
            VTagInner::Other { .. } => &None,
        }
    }

    /// Sets `value` for an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) or
    /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
    pub fn set_value(&mut self, value: impl IntoPropValue<Option<AttrValue>>) {
        match &mut self.inner {
            VTagInner::Input { value: dst, .. } | VTagInner::Textarea { value: dst } => {
                *dst = value.into_prop_value()
            }
            VTagInner::Other { .. } => (),
        }
    }

    /// Returns `kind` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// Same as set `type` attribute.
    pub fn kind(&self) -> &Option<AttrValue> {
        match &self.inner {
            VTagInner::Input { kind, .. } => kind,
            _ => &None,
        }
    }

    /// Sets `kind` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// Same as set `type` attribute.
    pub fn set_kind(&mut self, value: impl IntoPropValue<Option<AttrValue>>) {
        if let VTagInner::Input { kind, .. } = &mut self.inner {
            *kind = value.into_prop_value();
        }
    }

    /// Returns `checked` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// (Not a value of node's attribute).
    pub fn checked(&mut self) -> bool {
        match &mut self.inner {
            VTagInner::Input { checked, .. } => *checked,
            _ => false,
        }
    }

    /// Sets `checked` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// (Not a value of node's attribute).
    pub fn set_checked(&mut self, value: bool) {
        if let VTagInner::Input { checked, .. } = &mut self.inner {
            *checked = value;
        }
    }

    /// Adds a key-value pair to attributes
    ///
    /// Not every attribute works when it set as an attribute. We use workarounds for:
    /// `type/kind`, `value` and `checked`.
    pub fn add_attribute(&mut self, key: &'static str, value: impl Into<AttrValue>) {
        self.attributes
            .get_mut_index_map()
            .insert(key, value.into());
    }

    /// Sets attributes to a virtual node.
    ///
    /// Not every attribute works when it set as an attribute. We use workarounds for:
    /// `type/kind`, `value` and `checked`.
    pub fn set_attributes(&mut self, attrs: impl Into<Attributes>) {
        self.attributes = attrs.into();
    }

    #[doc(hidden)]
    pub fn __macro_push_attr(&mut self, attr: PositionalAttr) {
        match &mut self.attributes {
            Attributes::Vec(attrs) => attrs.push(attr),
            _ => unreachable!("the macro always uses positional attributes"),
        }
    }

    /// Adds new listener to the node.
    /// It's boxed because we want to keep it in a single list.
    /// Later `Listener::attach` will attach an actual listener to a DOM node.
    pub fn add_listener(&mut self, listener: Rc<dyn Listener>) {
        self.listeners.push(listener);
    }

    /// Adds new listeners to the node.
    /// They are boxed because we want to keep them in a single list.
    /// Later `Listener::attach` will attach an actual listener to a DOM node.
    pub fn add_listeners(&mut self, listeners: Listeners) {
        self.listeners.extend(listeners);
    }

    /// Every render it removes all listeners and attach it back later
    /// TODO(#943): Compare references of handler to do listeners update better
    fn recreate_listeners(&mut self, ancestor: &mut Option<Box<Self>>) {
        if let Some(ancestor) = ancestor.as_mut() {
            ancestor.captured.clear();
        }

        let element = self.reference.clone().expect("element expected");

        for listener in self.listeners.drain(..) {
            let handle = listener.attach(&element);
            self.captured.push(handle);
        }
    }

    fn refresh_value(&mut self) {
        macro_rules! refresh {
            ($value:expr, $el_type:ty) => {
                // Don't refresh value if the element is not controlled
                if $value.is_none() {
                    return;
                }

                if let Some(element) = self.reference.as_ref() {
                    if let Some(el) = element.dyn_ref::<$el_type>() {
                        *$value = Some(Cow::Owned(el.value()));
                    }
                }
            };
        }

        match &mut self.inner {
            VTagInner::Input { value, .. } => {
                refresh!(value, InputElement);
            }
            VTagInner::Textarea { value, .. } => {
                refresh!(value, TextAreaElement);
            }
            _ => (),
        }
    }

    /// Compares new kind with ancestor and produces a patch to apply, if any
    fn diff_kind<'a>(&'a self, ancestor: &'a Option<Box<Self>>) -> Option<Patch<&'a str, ()>> {
        match (&self.inner, ancestor.as_ref().map(|a| &a.inner)) {
            (VTagInner::Input { kind: left, .. }, Some(VTagInner::Input { kind: right, .. })) => {
                match (left, right) {
                    (Some(ref left), Some(ref right)) => {
                        if left != right {
                            Some(Patch::Replace(&**left, ()))
                        } else {
                            None
                        }
                    }
                    (Some(ref left), None) => Some(Patch::Add(&**left, ())),
                    (None, Some(right)) => Some(Patch::Remove(&**right)),
                    (None, None) => None,
                }
            }
            _ => None,
        }
    }

    /// Compares new value with ancestor and produces a patch to apply, if any
    fn diff_value<'a>(&'a self, ancestor: &'a Option<Box<Self>>) -> Option<Patch<&'a str, ()>> {
        match (&self.inner, ancestor.as_ref().map(|a| &a.inner)) {
            (VTagInner::Input { value: left, .. }, Some(VTagInner::Input { value: right, .. })) => {
                match (left, right) {
                    (Some(ref left), Some(ref right)) => {
                        if left != right {
                            Some(Patch::Replace(&**left, ()))
                        } else {
                            None
                        }
                    }
                    (Some(ref left), None) => Some(Patch::Add(&**left, ())),
                    (None, Some(right)) => Some(Patch::Remove(&**right)),
                    (None, None) => None,
                }
            }
            _ => None,
        }
    }

    fn apply_diffs(&mut self, ancestor: &mut Option<Box<Self>>) {
        let changes = if let Some(old_attributes) = ancestor.as_mut().map(|a| &mut a.attributes) {
            Attributes::diff(&self.attributes, old_attributes)
        } else {
            self.attributes
                .iter()
                .map(|(k, v)| Patch::Add(k, v))
                .collect()
        };

        let element = self.reference.as_ref().expect("element expected");

        for change in changes {
            match change {
                Patch::Add(key, value) | Patch::Replace(key, value) => {
                    element
                        .set_attribute(&key, &value)
                        .expect("invalid attribute key");
                }
                Patch::Remove(key) => {
                    element
                        .remove_attribute(&key)
                        .expect("could not remove attribute");
                }
            }
        }

        match &self.inner {
            VTagInner::Other { tag, .. } => {
                if &*tag == "button" {
                    if let Some(button) = element.dyn_ref::<HtmlButtonElement>() {
                        if let Some(change) = self.diff_kind(ancestor) {
                            let kind = match change {
                                Patch::Add(kind, _) | Patch::Replace(kind, _) => kind,
                                Patch::Remove(_) => "",
                            };
                            button.set_type(kind);
                        }
                    }
                }
            }
            // `input` element has extra parameters to control
            // I override behavior of attributes to make it more clear
            // and useful in templates. For example I interpret `checked`
            // attribute as `checked` parameter, not `defaultChecked` as browsers do
            VTagInner::Input { checked, .. } => {
                if let Some(input) = element.dyn_ref::<InputElement>() {
                    if let Some(change) = self.diff_kind(ancestor) {
                        let kind = match change {
                            Patch::Add(kind, _) | Patch::Replace(kind, _) => kind,
                            Patch::Remove(_) => "",
                        };
                        input.set_type(kind)
                    }

                    if let Some(change) = self.diff_value(ancestor) {
                        let raw_value = match change {
                            Patch::Add(kind, _) | Patch::Replace(kind, _) => kind,
                            Patch::Remove(_) => "",
                        };
                        input.set_value(raw_value)
                    }

                    // IMPORTANT! This parameter has to be set every time
                    // to prevent strange behaviour in the browser when the DOM changes
                    input.set_checked(*checked);
                }
            }
            VTagInner::Textarea { .. } => {
                if let Some(tae) = { element.dyn_ref::<TextAreaElement>() } {
                    if let Some(change) = self.diff_value(ancestor) {
                        let value = match change {
                            Patch::Add(kind, _) | Patch::Replace(kind, _) => kind,
                            Patch::Remove(_) => "",
                        };
                        tae.set_value(value);
                    }
                }
            }
        }
    }

    fn create_element(&self, parent: &Element) -> Element {
        let tag = self.tag();
        if tag == "svg"
            || parent
                .namespace_uri()
                .map_or(false, |ns| ns == SVG_NAMESPACE)
        {
            let namespace = Some(SVG_NAMESPACE);
            document()
                .create_element_ns(namespace, tag)
                .expect("can't create namespaced element for vtag")
        } else {
            document()
                .create_element(tag)
                .expect("can't create element for vtag")
        }
    }
}

impl VDiff for VTag {
    /// Remove VTag from parent.
    fn detach(&mut self, parent: &Element) {
        let node = self
            .reference
            .take()
            .expect("tried to remove not rendered VTag from DOM");

        // recursively remove its children
        if let VTagInner::Other { children, .. } = &mut self.inner {
            children.detach(&node);
        }
        if parent.remove_child(&node).is_err() {
            warn!("Node not found to remove VTag");
        }
        self.node_ref.set(None);
    }

    /// Renders virtual tag over DOM [Element], but it also compares this with an ancestor [VTag]
    /// to compute what to patch in the actual DOM nodes.
    fn apply(
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: Option<VNode>,
    ) -> NodeRef {
        let mut ancestor_tag = ancestor.and_then(|mut ancestor| {
            match ancestor {
                // If the ancestor is a tag of the same type, don't recreate, keep the
                // old tag and update its attributes and children.
                VNode::VTag(vtag) if self.tag() == vtag.tag() && self.key == vtag.key => Some(vtag),
                _ => {
                    let element = self.create_element(parent);
                    super::insert_node(&element, parent, Some(ancestor.first_node()));
                    self.reference = Some(element);
                    ancestor.detach(parent);
                    None
                }
            }
        });

        if let Some(ref mut ancestor_tag) = &mut ancestor_tag {
            // Refresh the current value to later compare it against the desired value
            // since it may have been changed since we last set it.
            ancestor_tag.refresh_value();
            // Preserve the reference that already exists.
            self.reference = ancestor_tag.reference.take();
        } else if self.reference.is_none() {
            let element = self.create_element(parent);
            super::insert_node(&element, parent, next_sibling.get());
            self.reference = Some(element);
        }

        self.apply_diffs(&mut ancestor_tag);
        self.recreate_listeners(&mut ancestor_tag);

        // Process children
        let element = self.reference.as_ref().expect("Reference should be set");
        if let VTagInner::Other { children, .. } = &mut self.inner {
            if !children.is_empty() {
                children.apply(
                    parent_scope,
                    element,
                    NodeRef::default(),
                    match ancestor_tag.map(|a| a.inner) {
                        Some(VTagInner::Other { children, .. }) => Some(children.into()),
                        _ => None,
                    },
                );
            } else if let Some(VTagInner::Other { children, .. }) =
                ancestor_tag.as_mut().map(|a| &mut a.inner)
            {
                children.detach(element);
            }
        }

        let node = element.deref();
        self.node_ref.set(Some(node.clone()));
        self.node_ref.clone()
    }
}

impl PartialEq for VTag {
    fn eq(&self, other: &VTag) -> bool {
        use VTagInner::*;

        (match (&self.inner, &other.inner) {
            (
                Input {
                    value: value_l,
                    kind: kind_l,
                    checked: checked_l,
                },
                Input {
                    value: value_r,
                    kind: kind_r,
                    checked: checked_r,
                },
            ) => value_l == value_r && kind_l == kind_r && checked_l == checked_r,
            (Textarea { value: value_l }, Textarea { value: value_r }) => value_l == value_r,
            (Other { tag: tag_l, .. }, Other { tag: tag_r, .. }) => tag_l == tag_r,
            _ => false,
        }) && self.listeners.len() == other.listeners.len()
            && self
                .listeners
                .iter()
                .map(|l| l.kind())
                .eq(other.listeners.iter().map(|l| l.kind()))
            && self.attributes == other.attributes
            // Diff children last, as recursion is the most expensive
            && match (&self.inner, &other.inner) {
                (Other { children: ch_l, .. }, Other { children: ch_r, .. }) => ch_l == ch_r,
                _ => true,
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html;

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    fn test_scope() -> AnyScope {
        AnyScope::test()
    }

    #[test]
    fn it_compares_tags() {
        let a = html! {
            <div></div>
        };

        let b = html! {
            <div></div>
        };

        let c = html! {
            <p></p>
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn it_compares_text() {
        let a = html! {
            <div>{ "correct" }</div>
        };

        let b = html! {
            <div>{ "correct" }</div>
        };

        let c = html! {
            <div>{ "incorrect" }</div>
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn it_compares_attributes() {
        let a = html! {
            <div a="test"></div>
        };

        let b = html! {
            <div a="test"></div>
        };

        let c = html! {
            <div a="fail"></div>
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn it_compares_children() {
        let a = html! {
            <div>
                <p></p>
            </div>
        };

        let b = html! {
            <div>
                <p></p>
            </div>
        };

        let c = html! {
            <div>
                <span></span>
            </div>
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn it_compares_classes() {
        let a = html! {
            <div class="test"></div>
        };

        let b = html! {
            <div class="test"></div>
        };

        let c = html! {
            <div class="fail"></div>
        };

        let d = html! {
            <div class=format!("fail{}", "")></div>
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(c, d);
    }

    fn assert_vtag(node: &VNode) -> &VTag {
        if let VNode::VTag(vtag) = node {
            return vtag;
        }
        panic!("should be vtag");
    }

    fn assert_vtag_mut(node: &mut VNode) -> &mut VTag {
        if let VNode::VTag(vtag) = node {
            return vtag;
        }
        panic!("should be vtag");
    }

    fn assert_namespace(vtag: &VTag, namespace: &'static str) {
        assert_eq!(
            vtag.reference.as_ref().unwrap().namespace_uri().unwrap(),
            namespace
        );
    }

    #[test]
    fn supports_svg() {
        let document = web_sys::window().unwrap().document().unwrap();

        let scope = test_scope();
        let div_el = document.create_element("div").unwrap();
        let namespace = SVG_NAMESPACE;
        let namespace = Some(namespace);
        let svg_el = document.create_element_ns(namespace, "svg").unwrap();

        let mut g_node = html! { <g class="segment"></g> };
        let path_node = html! { <path></path> };
        let mut svg_node = html! { <svg>{path_node}</svg> };

        let svg_tag = assert_vtag_mut(&mut svg_node);
        svg_tag.apply(&scope, &div_el, NodeRef::default(), None);
        assert_namespace(svg_tag, SVG_NAMESPACE);
        let path_tag = assert_vtag(svg_tag.children().get(0).unwrap());
        assert_namespace(path_tag, SVG_NAMESPACE);

        let g_tag = assert_vtag_mut(&mut g_node);
        g_tag.apply(&scope, &div_el, NodeRef::default(), None);
        assert_namespace(g_tag, HTML_NAMESPACE);
        g_tag.reference = None;

        g_tag.apply(&scope, &svg_el, NodeRef::default(), None);
        assert_namespace(g_tag, SVG_NAMESPACE);
    }

    #[test]
    fn it_compares_values() {
        let a = html! {
            <input value="test"/>
        };

        let b = html! {
            <input value="test"/>
        };

        let c = html! {
            <input value="fail"/>
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn it_compares_kinds() {
        let a = html! {
            <input type="text"/>
        };

        let b = html! {
            <input type="text"/>
        };

        let c = html! {
            <input type="hidden"/>
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn it_compares_checked() {
        let a = html! {
            <input type="checkbox" checked=false />
        };

        let b = html! {
            <input type="checkbox" checked=false />
        };

        let c = html! {
            <input type="checkbox" checked=true />
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn it_allows_aria_attributes() {
        let a = html! {
            <p aria-controls="it-works">
                <a class="btn btn-primary"
                   data-toggle="collapse"
                   href="#collapseExample"
                   role="button"
                   aria-expanded="false"
                   aria-controls="collapseExample">
                    { "Link with href" }
                </a>
                <button class="btn btn-primary"
                        type="button"
                        data-toggle="collapse"
                        data-target="#collapseExample"
                        aria-expanded="false"
                        aria-controls="collapseExample">
                    { "Button with data-target" }
                </button>
                <div own-attribute-with-multiple-parts="works" />
            </p>
        };
        if let VNode::VTag(vtag) = a {
            assert_eq!(
                vtag.attributes
                    .iter()
                    .find(|(k, _)| k == &"aria-controls")
                    .map(|(_, v)| v),
                Some("it-works")
            );
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn it_does_not_set_missing_class_name() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        let mut elem = html! { <div></div> };
        elem.apply(&scope, &parent, NodeRef::default(), None);
        let vtag = assert_vtag_mut(&mut elem);
        // test if the className has not been set
        assert!(!vtag.reference.as_ref().unwrap().has_attribute("class"));
    }

    #[test]
    fn it_sets_class_name() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        let mut elem = html! { <div class="ferris the crab"></div> };
        elem.apply(&scope, &parent, NodeRef::default(), None);
        let vtag = assert_vtag_mut(&mut elem);
        // test if the className has been set
        assert!(vtag.reference.as_ref().unwrap().has_attribute("class"));
    }

    #[test]
    fn controlled_input_synced() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        let expected = "not_changed_value";

        // Initial state
        let mut elem = html! { <input value=expected /> };
        elem.apply(&scope, &parent, NodeRef::default(), None);
        let vtag = if let VNode::VTag(vtag) = elem {
            vtag
        } else {
            panic!("should be vtag")
        };

        // User input
        let input_ref = vtag.reference.as_ref().unwrap();
        let input = input_ref.dyn_ref::<InputElement>();
        input.unwrap().set_value("User input");

        let ancestor = vtag;
        let mut elem = html! { <input value=expected /> };
        let vtag = assert_vtag_mut(&mut elem);

        // Sync happens here
        vtag.apply(
            &scope,
            &parent,
            NodeRef::default(),
            Some(VNode::VTag(ancestor)),
        );

        // Get new current value of the input element
        let input_ref = vtag.reference.as_ref().unwrap();
        let input = input_ref.dyn_ref::<InputElement>().unwrap();

        let current_value = input.value();

        // check whether not changed virtual dom value has been set to the input element
        assert_eq!(current_value, expected);
    }

    #[test]
    fn uncontrolled_input_unsynced() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        // Initial state
        let mut elem = html! { <input /> };
        elem.apply(&scope, &parent, NodeRef::default(), None);
        let vtag = if let VNode::VTag(vtag) = elem {
            vtag
        } else {
            panic!("should be vtag")
        };

        // User input
        let input_ref = vtag.reference.as_ref().unwrap();
        let input = input_ref.dyn_ref::<InputElement>();
        input.unwrap().set_value("User input");

        let ancestor = vtag;
        let mut elem = html! { <input /> };
        let vtag = assert_vtag_mut(&mut elem);

        // Value should not be refreshed
        vtag.apply(
            &scope,
            &parent,
            NodeRef::default(),
            Some(VNode::VTag(ancestor)),
        );

        // Get user value of the input element
        let input_ref = vtag.reference.as_ref().unwrap();
        let input = input_ref.dyn_ref::<InputElement>().unwrap();

        let current_value = input.value();

        // check whether not changed virtual dom value has been set to the input element
        assert_eq!(current_value, "User input");
    }

    #[test]
    fn dynamic_tags_work() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        let mut elem = html! { <@{
            let mut builder = String::new();
            builder.push('a');
            builder
        }/> };

        elem.apply(&scope, &parent, NodeRef::default(), None);
        let vtag = assert_vtag_mut(&mut elem);
        // make sure the new tag name is used internally
        assert_eq!(vtag.tag(), "a");

        // Element.tagName is always in the canonical upper-case form.
        assert_eq!(vtag.reference.as_ref().unwrap().tag_name(), "A");
    }

    #[test]
    fn dynamic_tags_handle_value_attribute() {
        let mut div_el = html! {
            <@{"div"} value="Hello"/>
        };
        let div_vtag = assert_vtag_mut(&mut div_el);
        assert!(div_vtag.value().is_none());
        let v: Option<&str> = div_vtag
            .attributes
            .iter()
            .find(|(k, _)| k == &"value")
            .map(|(_, v)| AsRef::as_ref(v));
        assert_eq!(v, Some("Hello"));

        let mut input_el = html! {
            <@{"input"} value="World"/>
        };
        let input_vtag = assert_vtag_mut(&mut input_el);
        assert_eq!(input_vtag.value(), &Some(Cow::Borrowed("World")));
        assert!(!input_vtag.attributes.iter().any(|(k, _)| k == "value"));
    }

    #[test]
    fn dynamic_tags_handle_weird_capitalization() {
        let mut el = html! {
            <@{"tExTAREa"}/>
        };
        let vtag = assert_vtag_mut(&mut el);
        assert_eq!(vtag.tag(), "textarea");
    }

    #[test]
    fn reset_node_ref() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        let node_ref = NodeRef::default();
        let mut elem: VNode = html! { <div ref=node_ref.clone()></div> };
        assert_vtag_mut(&mut elem);
        elem.apply(&scope, &parent, NodeRef::default(), None);
        let parent_node = parent.deref();
        assert_eq!(node_ref.get(), parent_node.first_child());
        elem.detach(&parent);
        assert!(node_ref.get().is_none());
    }

    /// Returns the class attribute as str reference, or "" if the attribute is not set.
    fn get_class_str(vtag: &VTag) -> &str {
        vtag.attributes
            .iter()
            .find(|(k, _)| k == &"class")
            .map(|(_, v)| AsRef::as_ref(v))
            .unwrap_or("")
    }

    #[test]
    fn old_class_syntax_is_still_supported() {
        let a_classes = "class-1 class-2".to_string();
        #[allow(deprecated)]
        let a = html! {
            <div class=("class-1", a_classes)></div>
        };

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(!get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }
}

#[cfg(test)]
mod layout_tests {
    extern crate self as yew;

    use crate::html;
    use crate::virtual_dom::layout_tests::{diff_layouts, TestLayout};

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn diff() {
        let layout1 = TestLayout {
            name: "1",
            node: html! {
                <ul>
                    <li>
                        {"a"}
                    </li>
                    <li>
                        {"b"}
                    </li>
                </ul>
            },
            expected: "<ul><li>a</li><li>b</li></ul>",
        };

        let layout2 = TestLayout {
            name: "2",
            node: html! {
                <ul>
                    <li>
                        {"a"}
                    </li>
                    <li>
                        {"b"}
                    </li>
                    <li>
                        {"d"}
                    </li>
                </ul>
            },
            expected: "<ul><li>a</li><li>b</li><li>d</li></ul>",
        };

        let layout3 = TestLayout {
            name: "3",
            node: html! {
                <ul>
                    <li>
                        {"a"}
                    </li>
                    <li>
                        {"b"}
                    </li>
                    <li>
                        {"c"}
                    </li>
                    <li>
                        {"d"}
                    </li>
                </ul>
            },
            expected: "<ul><li>a</li><li>b</li><li>c</li><li>d</li></ul>",
        };

        let layout4 = TestLayout {
            name: "4",
            node: html! {
                <ul>
                    <li>
                        <>
                            {"a"}
                        </>
                    </li>
                    <li>
                        {"b"}
                        <li>
                            {"c"}
                        </li>
                        <li>
                            {"d"}
                        </li>
                    </li>
                </ul>
            },
            expected: "<ul><li>a</li><li>b<li>c</li><li>d</li></li></ul>",
        };

        diff_layouts(vec![layout1, layout2, layout3, layout4]);
    }
}
