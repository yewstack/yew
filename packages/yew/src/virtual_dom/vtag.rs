//! This module contains the implementation of a virtual element node [VTag].

use std::borrow::Cow;
use std::cmp::PartialEq;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use web_sys::{HtmlInputElement as InputElement, HtmlTextAreaElement as TextAreaElement};

use super::{ApplyAttributeAs, AttrValue, Attributes, Key, Listener, Listeners, VList, VNode};
use crate::html::{IntoPropValue, NodeRef};

/// SVG namespace string used for creating svg elements
pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// Default namespace for html elements
pub const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

/// Value field corresponding to an [Element]'s `value` property
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Value<T>(Option<AttrValue>, PhantomData<T>);

impl<T> Default for Value<T> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl<T> Value<T> {
    /// Create a new value. The caller should take care that the value is valid for the element's
    /// `value` property
    fn new(value: Option<AttrValue>) -> Self {
        Value(value, PhantomData)
    }

    /// Set a new value. The caller should take care that the value is valid for the element's
    /// `value` property
    fn set(&mut self, value: Option<AttrValue>) {
        self.0 = value;
    }
}

impl<T> Deref for Value<T> {
    type Target = Option<AttrValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Fields specific to
/// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) [VTag](crate::virtual_dom::VTag)s
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub(crate) struct InputFields {
    /// Contains a value of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    pub(crate) value: Value<InputElement>,
    /// Represents `checked` attribute of
    /// [input](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-checked).
    /// It exists to override standard behavior of `checked` attribute, because
    /// in original HTML it sets `defaultChecked` value of `InputElement`, but for reactive
    /// frameworks it's more useful to control `checked` value of an `InputElement`.
    pub(crate) checked: Option<bool>,
}

impl Deref for InputFields {
    type Target = Value<InputElement>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for InputFields {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl InputFields {
    /// Crate new attributes for an [InputElement] element
    fn new(value: Option<AttrValue>, checked: Option<bool>) -> Self {
        Self {
            value: Value::new(value),
            checked,
        }
    }
}

/// [VTag] fields that are specific to different [VTag] kinds.
/// Decreases the memory footprint of [VTag] by avoiding impossible field and value combinations.
#[derive(Debug, Clone)]
pub(crate) enum VTagInner {
    /// Fields specific to
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
    /// [VTag]s
    Input(InputFields),
    /// Fields specific to
    /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
    /// [VTag]s
    Textarea {
        /// Contains a value of an
        /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
        value: Value<TextAreaElement>,
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
#[derive(Debug, Clone)]
pub struct VTag {
    /// [VTag] fields that are specific to different [VTag] kinds.
    pub(crate) inner: VTagInner,
    /// List of attached listeners.
    pub(crate) listeners: Listeners,
    /// A node reference used for DOM access in Component lifecycle methods
    pub node_ref: NodeRef,
    /// List of attributes.
    pub attributes: Attributes,
    pub key: Option<Key>,
}

impl VTag {
    /// Creates a new [VTag] instance with `tag` name (cannot be changed later in DOM).
    pub fn new(tag: impl Into<Cow<'static, str>>) -> Self {
        let tag: Cow<'static, str> = tag.into();
        Self::new_base(
            match &*tag.to_ascii_lowercase() {
                "input" => VTagInner::Input(Default::default()),
                "textarea" => VTagInner::Textarea {
                    value: Default::default(),
                },
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
        checked: Option<bool>,
        node_ref: NodeRef,
        key: Option<Key>,
        // at bottom for more readable macro-expanded coded
        attributes: Attributes,
        listeners: Listeners,
    ) -> Self {
        VTag::new_base(
            VTagInner::Input(InputFields::new(
                value,
                // In HTML node `checked` attribute sets `defaultChecked` parameter,
                // but we use own field to control real `checked` parameter
                checked,
            )),
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
            VTagInner::Textarea {
                value: Value::new(value),
            },
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
            attributes,
            listeners,
            node_ref,
            key,
        }
    }

    /// Returns tag of an [Element](web_sys::Element). In HTML tags are always uppercase.
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
            children.add_child(child)
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
                // This is mutable because the VList is not Sync
                static mut EMPTY: VList = VList::new();

                // SAFETY: The EMPTY value is always read-only
                unsafe { &EMPTY }
            }
        }
    }

    /// Returns a mutable reference to the children of this [VTag], if the node can have
    /// children
    pub fn children_mut(&mut self) -> Option<&mut VList> {
        match &mut self.inner {
            VTagInner::Other { children, .. } => Some(children),
            _ => None,
        }
    }

    /// Returns the children of this [VTag]
    pub fn into_children(self) -> VList {
        match self.inner {
            VTagInner::Other { children, .. } => children,
            _ => VList::new(),
        }
    }

    /// Returns the `value` of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) or
    /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
    pub fn value(&self) -> Option<&AttrValue> {
        match &self.inner {
            VTagInner::Input(f) => f.as_ref(),
            VTagInner::Textarea { value } => value.as_ref(),
            VTagInner::Other { .. } => None,
        }
    }

    /// Sets `value` for an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) or
    /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
    pub fn set_value(&mut self, value: impl IntoPropValue<Option<AttrValue>>) {
        match &mut self.inner {
            VTagInner::Input(f) => {
                f.set(value.into_prop_value());
            }
            VTagInner::Textarea { value: dst } => {
                dst.set(value.into_prop_value());
            }
            VTagInner::Other { .. } => (),
        }
    }

    /// Returns `checked` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// (Does not affect the value of the node's attribute).
    pub fn checked(&self) -> Option<bool> {
        match &self.inner {
            VTagInner::Input(f) => f.checked,
            _ => None,
        }
    }

    /// Sets `checked` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// (Does not affect the value of the node's attribute).
    pub fn set_checked(&mut self, value: bool) {
        if let VTagInner::Input(f) = &mut self.inner {
            f.checked = Some(value);
        }
    }

    /// Keeps the current value of the `checked` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// (Does not affect the value of the node's attribute).
    pub fn preserve_checked(&mut self) {
        if let VTagInner::Input(f) = &mut self.inner {
            f.checked = None;
        }
    }

    /// Adds a key-value pair to attributes
    ///
    /// Not every attribute works when it set as an attribute. We use workarounds for:
    /// `value` and `checked`.
    pub fn add_attribute(&mut self, key: &'static str, value: impl Into<AttrValue>) {
        self.attributes.get_mut_index_map().insert(
            AttrValue::Static(key),
            (value.into(), ApplyAttributeAs::Attribute),
        );
    }

    /// Set the given key as property on the element
    ///
    /// [`js_sys::Reflect`] is used for setting properties.
    pub fn add_property(&mut self, key: &'static str, value: impl Into<AttrValue>) {
        self.attributes.get_mut_index_map().insert(
            AttrValue::Static(key),
            (value.into(), ApplyAttributeAs::Property),
        );
    }

    /// Sets attributes to a virtual node.
    ///
    /// Not every attribute works when it set as an attribute. We use workarounds for:
    /// `value` and `checked`.
    pub fn set_attributes(&mut self, attrs: impl Into<Attributes>) {
        self.attributes = attrs.into();
    }

    #[doc(hidden)]
    pub fn __macro_push_attr(&mut self, key: &'static str, value: impl IntoPropValue<AttrValue>) {
        self.attributes.get_mut_index_map().insert(
            AttrValue::from(key),
            (value.into_prop_value(), ApplyAttributeAs::Property),
        );
    }

    /// Add event listener on the [VTag]'s  [Element](web_sys::Element).
    /// Returns `true` if the listener has been added, `false` otherwise.
    pub fn add_listener(&mut self, listener: Rc<dyn Listener>) -> bool {
        match &mut self.listeners {
            Listeners::None => {
                self.set_listeners([Some(listener)].into());
                true
            }
            Listeners::Pending(listeners) => {
                let mut listeners = mem::take(listeners).into_vec();
                listeners.push(Some(listener));

                self.set_listeners(listeners.into());
                true
            }
        }
    }

    /// Set event listeners on the [VTag]'s  [Element](web_sys::Element)
    pub fn set_listeners(&mut self, listeners: Box<[Option<Rc<dyn Listener>>]>) {
        self.listeners = Listeners::Pending(listeners);
    }
}

impl PartialEq for VTag {
    fn eq(&self, other: &VTag) -> bool {
        use VTagInner::*;

        (match (&self.inner, &other.inner) {
            (Input(l), Input(r)) => l == r,
            (Textarea { value: value_l }, Textarea { value: value_r }) => value_l == value_r,
            (Other { tag: tag_l, .. }, Other { tag: tag_r, .. }) => tag_l == tag_r,
            _ => false,
        }) && self.listeners.eq(&other.listeners)
            && self.attributes == other.attributes
            // Diff children last, as recursion is the most expensive
            && match (&self.inner, &other.inner) {
                (Other { children: ch_l, .. }, Other { children: ch_r, .. }) => ch_l == ch_r,
                _ => true,
            }
    }
}

#[cfg(feature = "ssr")]
mod feat_ssr {
    use std::fmt::Write;

    use super::*;
    use crate::html::AnyScope;
    use crate::platform::fmt::BufWriter;
    use crate::virtual_dom::VText;

    // Elements that cannot have any child elements.
    static VOID_ELEMENTS: &[&str; 14] = &[
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ];

    impl VTag {
        pub(crate) async fn render_into_stream(
            &self,
            w: &mut BufWriter,
            parent_scope: &AnyScope,
            hydratable: bool,
        ) {
            let _ = w.write_str("<");
            let _ = w.write_str(self.tag());

            let write_attr = |w: &mut BufWriter, name: &str, val: Option<&str>| {
                let _ = w.write_str(" ");
                let _ = w.write_str(name);

                if let Some(m) = val {
                    let _ = w.write_str("=\"");
                    let _ = w.write_str(&html_escape::encode_double_quoted_attribute(m));
                    let _ = w.write_str("\"");
                }
            };

            if let VTagInner::Input(_) = self.inner {
                if let Some(m) = self.value() {
                    write_attr(w, "value", Some(m));
                }

                // Setting is as an attribute sets the `defaultChecked` property. Only emit this
                // if it's explicitly set to checked.
                if self.checked() == Some(true) {
                    write_attr(w, "checked", None);
                }
            }

            for (k, v) in self.attributes.iter() {
                write_attr(w, k, Some(v));
            }

            let _ = w.write_str(">");

            match self.inner {
                VTagInner::Input(_) => {}
                VTagInner::Textarea { .. } => {
                    if let Some(m) = self.value() {
                        VText::new(m.to_owned())
                            .render_into_stream(w, parent_scope, hydratable)
                            .await;
                    }

                    let _ = w.write_str("</textarea>");
                }
                VTagInner::Other {
                    ref tag,
                    ref children,
                    ..
                } => {
                    if !VOID_ELEMENTS.contains(&tag.as_ref()) {
                        children
                            .render_into_stream(w, parent_scope, hydratable)
                            .await;

                        let _ = w.write_str("</");
                        let _ = w.write_str(tag);
                        let _ = w.write_str(">");
                    } else {
                        // We don't write children of void elements nor closing tags.
                        debug_assert!(children.is_empty(), "{} cannot have any children!", tag);
                    }
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "ssr")]
#[cfg(test)]
mod ssr_tests {
    use tokio::test;

    use crate::prelude::*;
    use crate::ServerRenderer;

    #[test]
    async fn test_simple_tag() {
        #[function_component]
        fn Comp() -> Html {
            html! { <div></div> }
        }

        let s = ServerRenderer::<Comp>::new()
            .hydratable(false)
            .render()
            .await;

        assert_eq!(s, "<div></div>");
    }

    #[test]
    async fn test_simple_tag_with_attr() {
        #[function_component]
        fn Comp() -> Html {
            html! { <div class="abc"></div> }
        }

        let s = ServerRenderer::<Comp>::new()
            .hydratable(false)
            .render()
            .await;

        assert_eq!(s, r#"<div class="abc"></div>"#);
    }

    #[test]
    async fn test_simple_tag_with_content() {
        #[function_component]
        fn Comp() -> Html {
            html! { <div>{"Hello!"}</div> }
        }

        let s = ServerRenderer::<Comp>::new()
            .hydratable(false)
            .render()
            .await;

        assert_eq!(s, r#"<div>Hello!</div>"#);
    }

    #[test]
    async fn test_simple_tag_with_nested_tag_and_input() {
        #[function_component]
        fn Comp() -> Html {
            html! { <div>{"Hello!"}<input value="abc" type="text" /></div> }
        }

        let s = ServerRenderer::<Comp>::new()
            .hydratable(false)
            .render()
            .await;

        assert_eq!(s, r#"<div>Hello!<input value="abc" type="text"></div>"#);
    }

    #[test]
    async fn test_textarea() {
        #[function_component]
        fn Comp() -> Html {
            html! { <textarea value="teststring" /> }
        }

        let s = ServerRenderer::<Comp>::new()
            .hydratable(false)
            .render()
            .await;

        assert_eq!(s, r#"<textarea>teststring</textarea>"#);
    }
}
