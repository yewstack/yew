//! This module contains the implementation of a virtual element node [VTag].

use super::{Apply, AttrValue, Attributes, Key, Listener, Listeners, VDiff, VList, VNode};
use crate::html::{AnyScope, IntoPropValue, NodeRef};
use crate::utils::document;
use gloo::console;
use std::borrow::Cow;
use std::cmp::PartialEq;
use std::hint::unreachable_unchecked;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use web_sys::Element;

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

/// A type for a virtual
/// [Element](https://developer.mozilla.org/en-US/docs/Web/API/Element)
/// representation.
#[derive(Debug)]
pub struct VTag {
    /// [VTag] fields that are specific to different [VTag] kinds.
    /// A tag of the element.
    pub tag: Cow<'static, str>,

    /// List of child nodes
    pub children: VList,

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
            tag: self.tag.clone(),
            reference: None,
            listeners: self.listeners.clone(),
            attributes: self.attributes.clone(),
            node_ref: self.node_ref.clone(),
            key: self.key.clone(),
            children: self.children.clone(),
        }
    }
}

impl VTag {
    /// Creates a new [VTag] instance with `tag` name (cannot be changed later in DOM).
    pub fn new(tag: impl Into<Cow<'static, str>>) -> Self {
        let tag: Cow<'static, str> = tag.into();
        Self::new_base(
            tag,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }

    /// Constructs a [VTag] from [VTagInner] and fields common to all [VTag] kinds
    #[inline]
    #[allow(clippy::too_many_arguments)]
    #[doc(hidden)]
    pub fn new_base(
        tag: Cow<'static, str>,
        node_ref: NodeRef,
        key: Option<Key>,
        attributes: Attributes,
        listeners: Listeners,
        children: VList,
    ) -> Self {
        VTag {
            tag,
            children,
            reference: None,
            attributes,
            listeners,
            node_ref,
            key,
        }
    }

    /// Add [VNode] child.
    pub fn add_child(&mut self, child: VNode) {
        self.children.add_child(child);
    }

    /// Add multiple [VNode] children.
    pub fn add_children(&mut self, children: impl IntoIterator<Item = VNode>) {
        self.children.add_children(children)
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
        let tag = &*self.tag;
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
        self.children.detach(&node);
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
                    VNode::VTag(a) => self.key == a.key && self.tag == a.tag,
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

                if !self.children.is_empty() {
                    self.children
                        .apply(parent_scope, &el, NodeRef::default(), None);
                }
            }
            Some(ancestor) => {
                self.attributes.apply_diff(&el, ancestor.attributes);
                self.listeners.apply_diff(&el, ancestor.listeners);

                let new = &mut self.children;
                let mut old = ancestor.children;
                if !new.is_empty() {
                    new.apply(parent_scope, &el, NodeRef::default(), Some(old.into()));
                } else if !old.is_empty() {
                    old.detach(&el);
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
        self.tag == other.tag
            && self.listeners.eq(&other.listeners)
            && self.attributes == other.attributes
            // Diff children last, as recursion is the most expensive
            && self.children == other.children
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{html, Html};
    use wasm_bindgen::JsCast;
    use web_sys::HtmlInputElement as InputElement;

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
        let path_tag = assert_vtag(svg_tag.children.get(0).unwrap());
        assert_namespace(path_tag, SVG_NAMESPACE);

        let g_tag = assert_vtag_mut(&mut g_node);
        g_tag.apply(&scope, &div_el, NodeRef::default(), None);
        assert_namespace(g_tag, HTML_NAMESPACE);
        g_tag.reference = None;

        g_tag.apply(&scope, &svg_el, NodeRef::default(), None);
        assert_namespace(g_tag, SVG_NAMESPACE);
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
        assert_eq!(vtag.tag, "a");

        // Element.tagName is always in the canonical upper-case form.
        assert_eq!(vtag.reference.as_ref().unwrap().tag_name(), "A");
    }

    #[test]
    fn dynamic_tags_handle_weird_capitalization() {
        let mut el = html! {
            <@{"tExTAREa"}/>
        };
        let vtag = assert_vtag_mut(&mut el);
        assert_eq!(vtag.tag, "textarea");
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
