//! This module contains the implementation of a virtual element node [VTag].

use super::{Apply, AttrValue, Attributes, Key, Listener, Listeners, VDiff, VList, VNode};
use crate::html::{AnyScope, IntoPropValue, NodeRef};
use crate::utils::document;
use gloo::console;
use std::borrow::Cow;
use std::cmp::PartialEq;
use std::hint::unreachable_unchecked;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Once;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement as InputElement, HtmlTextAreaElement as TextAreaElement};

/// SVG namespace string used for creating svg elements
pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// Default namespace for html elements
pub const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

// Value field corresponding to an [Element]'s `value` property
#[derive(Clone, Debug, Eq, PartialEq)]
struct Value<T: AccessValue>(Option<AttrValue>, PhantomData<T>);

impl<T: AccessValue> Default for Value<T> {
    fn default() -> Self {
        Value(None, PhantomData)
    }
}

impl<T: AccessValue> Apply for Value<T> {
    type Element = T;

    fn apply(&mut self, el: &Self::Element) {
        if let Some(v) = &self.0 {
            el.set_value(v);
        }
    }

    fn apply_diff(&mut self, el: &Self::Element, ancestor: Self) {
        match (&self.0, &ancestor.0) {
            (Some(new), Some(_)) => {
                // Refresh value from the DOM. It might have changed.
                if new != &el.value() {
                    el.set_value(new);
                }
            }
            (Some(new), None) => el.set_value(new),
            (None, Some(_)) => el.set_value(""),
            (None, None) => (),
        }
    }
}

/// Able to have its value read or set
trait AccessValue {
    fn value(&self) -> String;
    fn set_value(&self, v: &str);
}

macro_rules! impl_access_value {
    ($( $type:ty )*) => {
        $(
            impl AccessValue for $type {
                #[inline]
                fn value(&self) -> String {
                    <$type>::value(&self)
                }

                #[inline]
                fn set_value(&self, v: &str) {
                    <$type>::set_value(&self, v)
                }
            }
        )*
    };
}
impl_access_value! {InputElement TextAreaElement}

/// Fields specific to
/// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) [VTag]s
#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct InputFields {
    /// Contains a value of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    value: Value<InputElement>,

    /// Represents `checked` attribute of
    /// [input](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-checked).
    /// It exists to override standard behavior of `checked` attribute, because
    /// in original HTML it sets `defaultChecked` value of `InputElement`, but for reactive
    /// frameworks it's more useful to control `checked` value of an `InputElement`.
    checked: bool,
}

impl Apply for InputFields {
    type Element = InputElement;

    fn apply(&mut self, el: &Self::Element) {
        // IMPORTANT! This parameter has to be set every time
        // to prevent strange behaviour in the browser when the DOM changes
        el.set_checked(self.checked);

        self.value.apply(el);
    }

    fn apply_diff(&mut self, el: &Self::Element, ancestor: Self) {
        // IMPORTANT! This parameter has to be set every time
        // to prevent strange behaviour in the browser when the DOM changes
        el.set_checked(self.checked);

        self.value.apply_diff(el, ancestor.value);
    }
}

/// [VTag] fields that are specific to different [VTag] kinds.
/// Decreases the memory footprint of [VTag] by avoiding impossible field and value combinations.
#[derive(Debug, Clone)]
enum VTagInner {
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
#[derive(Debug)]
pub struct VTag {
    /// [VTag] fields that are specific to different [VTag] kinds.
    inner: VTagInner,

    /// List of attached listeners.
    listeners: Listeners,

    /// A reference to the DOM [`Element`].
    reference: Option<Element>,

    /// A node reference used for DOM access in Component lifecycle methods
    pub node_ref: NodeRef,

    /// List of attributes.
    pub attributes: Attributes,

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
        }
    }
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
        checked: bool,
        node_ref: NodeRef,
        key: Option<Key>,
        // at bottom for more readable macro-expanded coded
        attributes: Attributes,
        listeners: Listeners,
    ) -> Self {
        VTag::new_base(
            VTagInner::Input(InputFields {
                value: Value(value, PhantomData),
                // In HTML node `checked` attribute sets `defaultChecked` parameter,
                // but we use own field to control real `checked` parameter
                checked,
            }),
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
                value: Value(value, PhantomData),
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
            reference: None,
            attributes,
            listeners,
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
    pub fn value(&self) -> Option<&AttrValue> {
        match &self.inner {
            VTagInner::Input(f) => f.value.0.as_ref(),
            VTagInner::Textarea { value } => value.0.as_ref(),
            VTagInner::Other { .. } => None,
        }
    }

    /// Sets `value` for an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) or
    /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
    pub fn set_value(&mut self, value: impl IntoPropValue<Option<AttrValue>>) {
        match &mut self.inner {
            VTagInner::Input(f) => {
                f.value.0 = value.into_prop_value();
            }
            VTagInner::Textarea { value: dst } => {
                dst.0 = value.into_prop_value();
            }
            VTagInner::Other { .. } => (),
        }
    }

    /// Returns `checked` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// (Not a value of node's attribute).
    pub fn checked(&mut self) -> bool {
        match &mut self.inner {
            VTagInner::Input(f) => f.checked,
            _ => false,
        }
    }

    /// Sets `checked` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// (Not a value of node's attribute).
    pub fn set_checked(&mut self, value: bool) {
        if let VTagInner::Input(f) = &mut self.inner {
            f.checked = value;
        }
    }

    /// Returns reference to the [Element] associated with this [VTag], if this [VTag] has already
    /// been mounted in the DOM
    pub fn reference(&self) -> Option<&Element> {
        self.reference.as_ref()
    }

    /// Adds a key-value pair to attributes
    ///
    /// Not every attribute works when it set as an attribute. We use workarounds for:
    /// `value` and `checked`.
    pub fn add_attribute(&mut self, key: &'static str, value: impl Into<AttrValue>) {
        self.attributes
            .get_mut_index_map()
            .insert(key, value.into());
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
        self.attributes
            .get_mut_index_map()
            .insert(key, value.into_prop_value());
    }

    /// Set event listeners on the [VTag]'s  [Element]
    pub fn set_listener(&mut self, listeners: Box<[Option<Rc<dyn Listener>>]>) {
        self.listeners = Listeners::Pending(listeners);
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

        self.listeners.unregister();

        // recursively remove its children
        if let VTagInner::Other { children, .. } = &mut self.inner {
            children.detach(&node);
        }
        if parent.remove_child(&node).is_err() {
            console::warn!("Node not found to remove VTag");
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
        // This kind of branching patching routine reduces branch predictor misses and the need to
        // unpack the enums (including `Option`s) all the time, resulting in a more streamlined
        // patching flow
        let (ancestor_tag, el) = match ancestor {
            Some(mut ancestor) => {
                // If the ancestor is a tag of the same type, don't recreate, keep the
                // old tag and update its attributes and children.
                if match &ancestor {
                    VNode::VTag(a) => {
                        self.key == a.key
                            && match (&self.inner, &a.inner) {
                                (VTagInner::Input(_), VTagInner::Input(_))
                                | (VTagInner::Textarea { .. }, VTagInner::Textarea { .. }) => true,
                                (
                                    VTagInner::Other { tag: l, .. },
                                    VTagInner::Other { tag: r, .. },
                                ) => l == r,
                                _ => false,
                            }
                    }
                    _ => false,
                } {
                    match ancestor {
                        VNode::VTag(mut a) => {
                            // Preserve the reference that already exists
                            let el = a.reference.take().unwrap();
                            a.node_ref.set(None);
                            (Some(a), el)
                        }
                        _ => unsafe { unreachable_unchecked() },
                    }
                } else {
                    let el = self.create_element(parent);
                    super::insert_node(&el, parent, ancestor.first_node().as_ref());
                    ancestor.detach(parent);
                    (None, el)
                }
            }
            None => (None, {
                let el = self.create_element(parent);
                super::insert_node(&el, parent, next_sibling.get().as_ref());
                el
            }),
        };

        match ancestor_tag {
            None => {
                self.attributes.apply(&el);
                self.listeners.apply(&el);

                match &mut self.inner {
                    VTagInner::Input(f) => {
                        f.apply(el.unchecked_ref());
                    }
                    VTagInner::Textarea { value } => {
                        value.apply(el.unchecked_ref());
                    }
                    VTagInner::Other { children, .. } => {
                        if !children.is_empty() {
                            children.apply(parent_scope, &el, NodeRef::default(), None);
                        }
                    }
                }
            }
            Some(ancestor) => {
                self.attributes.apply_diff(&el, ancestor.attributes);
                self.listeners.apply_diff(&el, ancestor.listeners);

                match (&mut self.inner, ancestor.inner) {
                    (VTagInner::Input(new), VTagInner::Input(old)) => {
                        new.apply_diff(el.unchecked_ref(), old);
                    }
                    (VTagInner::Textarea { value: new }, VTagInner::Textarea { value: old }) => {
                        new.apply_diff(el.unchecked_ref(), old);
                    }
                    (
                        VTagInner::Other { children: new, .. },
                        VTagInner::Other {
                            children: mut old, ..
                        },
                    ) => {
                        if !new.is_empty() {
                            new.apply(parent_scope, &el, NodeRef::default(), Some(old.into()));
                        } else if !old.is_empty() {
                            old.detach(&el);
                        }
                    }
                    // Can not happen, because we checked for tag equability above
                    _ => unsafe { unreachable_unchecked() },
                }
            }
        };

        self.node_ref.set(Some(el.deref().clone()));
        self.reference = el.into();
        self.node_ref.clone()
    }
}

impl PartialEq for VTag {
    fn eq(&self, other: &VTag) -> bool {
        use VTagInner::*;

        (match (&self.inner, &other.inner) {
            (
                 Input(l),
                Input (r),
            ) => l == r,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{html, Html};

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
    fn it_compares_attributes_static() {
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
    fn it_compares_attributes_dynamic() {
        let a = html! {
            <div a={"test".to_owned()}></div>
        };

        let b = html! {
            <div a={"test".to_owned()}></div>
        };

        let c = html! {
            <div a={"fail".to_owned()}></div>
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
    fn it_compares_classes_static() {
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
            <div class={format!("fail{}", "")}></div>
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
    }

    #[test]
    fn it_compares_classes_dynamic() {
        let a = html! {
            <div class={"test".to_owned()}></div>
        };

        let b = html! {
            <div class={"test".to_owned()}></div>
        };

        let c = html! {
            <div class={"fail".to_owned()}></div>
        };

        let d = html! {
            <div class={format!("fail{}", "")}></div>
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
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
        VDiff::apply(&mut elem, &scope, &parent, NodeRef::default(), None);
        let vtag = assert_vtag_mut(&mut elem);
        // test if the className has not been set
        assert!(!vtag.reference.as_ref().unwrap().has_attribute("class"));
    }

    fn test_set_class_name(gen_html: impl FnOnce() -> Html) {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        let mut elem = gen_html();
        VDiff::apply(&mut elem, &scope, &parent, NodeRef::default(), None);
        let vtag = assert_vtag_mut(&mut elem);
        // test if the className has been set
        assert!(vtag.reference.as_ref().unwrap().has_attribute("class"));
    }

    #[test]
    fn it_sets_class_name_static() {
        test_set_class_name(|| html! { <div class="ferris the crab"></div> });
    }

    #[test]
    fn it_sets_class_name_dynamic() {
        test_set_class_name(|| html! { <div class={"ferris the crab".to_owned()}></div> });
    }

    #[test]
    fn controlled_input_synced() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        let expected = "not_changed_value";

        // Initial state
        let mut elem = html! { <input value={expected} /> };
        VDiff::apply(&mut elem, &scope, &parent, NodeRef::default(), None);
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
        let mut elem = html! { <input value={expected} /> };
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
        VDiff::apply(&mut elem, &scope, &parent, NodeRef::default(), None);
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

        // Need to remove the element to clean up the dirty state of the DOM. Failing this causes
        // event listener tests to fail.
        parent.remove();
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

        VDiff::apply(&mut elem, &scope, &parent, NodeRef::default(), None);
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
        assert_eq!(input_vtag.value(), Some(&Cow::Borrowed("World")));
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
        let mut elem: VNode = html! { <div ref={node_ref.clone()}></div> };
        assert_vtag_mut(&mut elem);
        elem.apply(&scope, &parent, NodeRef::default(), None);
        let parent_node = parent.deref();
        assert_eq!(node_ref.get(), parent_node.first_child());
        elem.detach(&parent);
        assert!(node_ref.get().is_none());
    }

    #[test]
    fn vtag_reuse_should_reset_ancestors_node_ref() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&parent).unwrap();

        let node_ref_a = NodeRef::default();
        let mut elem_a = html! { <div id="a" ref={node_ref_a.clone()} /> };
        elem_a.apply(&scope, &parent, NodeRef::default(), None);

        // save the Node to check later that it has been reused.
        let node_a = node_ref_a.get().unwrap();

        let node_ref_b = NodeRef::default();
        let mut elem_b = html! { <div id="b" ref={node_ref_b.clone()} /> };
        elem_b.apply(&scope, &parent, NodeRef::default(), Some(elem_a));

        let node_b = node_ref_b.get().unwrap();

        assert_eq!(node_a, node_b, "VTag should have reused the element");
        assert!(
            node_ref_a.get().is_none(),
            "node_ref_a should have been reset when the element was reused."
        );
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
