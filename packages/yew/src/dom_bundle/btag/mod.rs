//! This module contains the bundle implementation of a tag [BTag]

mod attributes;
mod listeners;

pub use listeners::set_event_bubbling;

use super::{insert_node, BList, BNode, DomBundle, Reconcilable};
use crate::html::AnyScope;
use crate::virtual_dom::vtag::{InputFields, VTagInner, Value, SVG_NAMESPACE};
use crate::virtual_dom::{Attributes, Key, VTag};
use crate::NodeRef;
use gloo::console;
use gloo_utils::document;
use listeners::ListenerRegistration;
use std::ops::DerefMut;
use std::{borrow::Cow, hint::unreachable_unchecked};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlTextAreaElement as TextAreaElement};

/// Applies contained changes to DOM [web_sys::Element]
trait Apply {
    /// [web_sys::Element] subtype to apply the changes to
    type Element;
    type Bundle;

    /// Apply contained values to [Element](Self::Element) with no ancestor
    fn apply(self, el: &Self::Element) -> Self::Bundle;

    /// Apply diff between [self] and `bundle` to [Element](Self::Element).
    fn apply_diff(self, el: &Self::Element, bundle: &mut Self::Bundle);
}

/// [BTag] fields that are specific to different [BTag] kinds.
/// Decreases the memory footprint of [BTag] by avoiding impossible field and value combinations.
#[derive(Debug)]
enum BTagInner {
    /// Fields specific to
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
    Input(InputFields),
    /// Fields specific to
    /// [TextArea](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea)
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
        child_bundle: BList,
    },
}

/// The bundle implementation to [VTag]
#[derive(Debug)]
pub struct BTag {
    /// [BTag] fields that are specific to different [BTag] kinds.
    inner: BTagInner,
    listeners: ListenerRegistration,
    attributes: Attributes,
    /// A reference to the DOM [`Element`].
    reference: Element,
    /// A node reference used for DOM access in Component lifecycle methods
    node_ref: NodeRef,
    key: Option<Key>,
}

impl DomBundle for BTag {
    fn detach(self, parent: &Element, parent_to_detach: bool) {
        self.listeners.unregister();

        let node = self.reference;
        // recursively remove its children
        if let BTagInner::Other { child_bundle, .. } = self.inner {
            // This tag will be removed, so there's no point to remove any child.
            child_bundle.detach(&node, true);
        }
        if !parent_to_detach {
            let result = parent.remove_child(&node);

            if result.is_err() {
                console::warn!("Node not found to remove VTag");
            }
        }
        // It could be that the ref was already reused when rendering another element.
        // Only unset the ref it still belongs to our node
        if self.node_ref.get().as_ref() == Some(&node) {
            self.node_ref.set(None);
        }
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) {
        next_parent
            .insert_before(&self.reference, next_sibling.get().as_ref())
            .unwrap();
    }
}

impl Reconcilable for VTag {
    type Bundle = BTag;

    fn attach(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let el = self.create_element(parent);
        let Self {
            listeners,
            attributes,
            node_ref,
            key,
            ..
        } = self;
        insert_node(&el, parent, next_sibling.get().as_ref());

        let attributes = attributes.apply(&el);
        let listeners = listeners.apply(&el);

        let inner = match self.inner {
            VTagInner::Input(f) => {
                let f = f.apply(el.unchecked_ref());
                BTagInner::Input(f)
            }
            VTagInner::Textarea { value } => {
                let value = value.apply(el.unchecked_ref());
                BTagInner::Textarea { value }
            }
            VTagInner::Other { children, tag } => {
                let (_, child_bundle) = children.attach(parent_scope, &el, NodeRef::default());
                BTagInner::Other { child_bundle, tag }
            }
        };
        node_ref.set(Some(el.clone().into()));
        (
            node_ref.clone(),
            BTag {
                inner,
                listeners,
                reference: el,
                attributes,
                key,
                node_ref,
            },
        )
    }

    fn reconcile_node(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        // This kind of branching patching routine reduces branch predictor misses and the need to
        // unpack the enums (including `Option`s) all the time, resulting in a more streamlined
        // patching flow
        match bundle {
            // If the ancestor is a tag of the same type, don't recreate, keep the
            // old tag and update its attributes and children.
            BNode::Tag(ex) if self.key == ex.key => {
                if match (&self.inner, &ex.inner) {
                    (VTagInner::Input(_), BTagInner::Input(_)) => true,
                    (VTagInner::Textarea { .. }, BTagInner::Textarea { .. }) => true,
                    (VTagInner::Other { tag: l, .. }, BTagInner::Other { tag: r, .. })
                        if l == r =>
                    {
                        true
                    }
                    _ => false,
                } {
                    return self.reconcile(parent_scope, parent, next_sibling, ex.deref_mut());
                }
            }
            _ => {}
        };
        self.replace(parent_scope, parent, next_sibling, bundle)
    }

    fn reconcile(
        self,
        parent_scope: &AnyScope,
        _parent: &Element,
        _next_sibling: NodeRef,
        tag: &mut Self::Bundle,
    ) -> NodeRef {
        let el = &tag.reference;
        self.attributes.apply_diff(el, &mut tag.attributes);
        self.listeners.apply_diff(el, &mut tag.listeners);

        match (self.inner, &mut tag.inner) {
            (VTagInner::Input(new), BTagInner::Input(old)) => {
                new.apply_diff(el.unchecked_ref(), old);
            }
            (VTagInner::Textarea { value: new }, BTagInner::Textarea { value: old }) => {
                new.apply_diff(el.unchecked_ref(), old);
            }
            (
                VTagInner::Other { children: new, .. },
                BTagInner::Other {
                    child_bundle: old, ..
                },
            ) => {
                new.reconcile(parent_scope, el, NodeRef::default(), old);
            }
            // Can not happen, because we checked for tag equability above
            _ => unsafe { unreachable_unchecked() },
        }

        tag.key = self.key;

        if self.node_ref != tag.node_ref && tag.node_ref.get().as_ref() == Some(el) {
            tag.node_ref.set(None);
        }
        if self.node_ref != tag.node_ref {
            tag.node_ref = self.node_ref;
            tag.node_ref.set(Some(el.clone().into()));
        }

        tag.node_ref.clone()
    }
}

impl VTag {
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

impl BTag {
    /// Get the key of the underlying tag
    pub(super) fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }

    #[cfg(test)]
    fn reference(&self) -> &Element {
        &self.reference
    }

    #[cfg(test)]
    fn children(&self) -> &[BNode] {
        match &self.inner {
            BTagInner::Other { child_bundle, .. } => child_bundle,
            _ => &[],
        }
    }

    #[cfg(test)]
    fn tag(&self) -> &str {
        match &self.inner {
            BTagInner::Input { .. } => "input",
            BTagInner::Textarea { .. } => "textarea",
            BTagInner::Other { tag, .. } => tag.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom_bundle::{BNode, DomBundle, Reconcilable};
    use crate::html;
    use crate::html::AnyScope;
    use crate::virtual_dom::vtag::{HTML_NAMESPACE, SVG_NAMESPACE};
    use crate::virtual_dom::{AttrValue, VNode, VTag};
    use crate::{Html, NodeRef};
    use gloo_utils::document;
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

    fn assert_vtag(node: VNode) -> VTag {
        if let VNode::VTag(vtag) = node {
            return *vtag;
        }
        panic!("should be vtag");
    }

    fn assert_btag_ref(node: &BNode) -> &BTag {
        if let BNode::Tag(vtag) = node {
            return vtag;
        }
        panic!("should be btag");
    }

    fn assert_vtag_ref(node: &VNode) -> &VTag {
        if let VNode::VTag(vtag) = node {
            return vtag;
        }
        panic!("should be vtag");
    }

    fn assert_btag_mut(node: &mut BNode) -> &mut BTag {
        if let BNode::Tag(btag) = node {
            return btag;
        }
        panic!("should be btag");
    }

    fn assert_namespace(vtag: &BTag, namespace: &'static str) {
        assert_eq!(vtag.reference().namespace_uri().unwrap(), namespace);
    }

    #[test]
    fn supports_svg() {
        let document = web_sys::window().unwrap().document().unwrap();

        let scope = test_scope();
        let div_el = document.create_element("div").unwrap();
        let namespace = SVG_NAMESPACE;
        let namespace = Some(namespace);
        let svg_el = document.create_element_ns(namespace, "svg").unwrap();

        let g_node = html! { <g class="segment"></g> };
        let path_node = html! { <path></path> };
        let svg_node = html! { <svg>{path_node}</svg> };

        let svg_tag = assert_vtag(svg_node);
        let (_, svg_tag) = svg_tag.attach(&scope, &div_el, NodeRef::default());
        assert_namespace(&svg_tag, SVG_NAMESPACE);
        let path_tag = assert_btag_ref(svg_tag.children().get(0).unwrap());
        assert_namespace(path_tag, SVG_NAMESPACE);

        let g_tag = assert_vtag(g_node.clone());
        let (_, g_tag) = g_tag.attach(&scope, &div_el, NodeRef::default());
        assert_namespace(&g_tag, HTML_NAMESPACE);

        let g_tag = assert_vtag(g_node);
        let (_, g_tag) = g_tag.attach(&scope, &svg_el, NodeRef::default());
        assert_namespace(&g_tag, SVG_NAMESPACE);
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

        let elem = html! { <div></div> };
        let (_, mut elem) = Reconcilable::attach(elem, &scope, &parent, NodeRef::default());
        let vtag = assert_btag_mut(&mut elem);
        // test if the className has not been set
        assert!(!vtag.reference().has_attribute("class"));
    }

    fn test_set_class_name(gen_html: impl FnOnce() -> Html) {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        let elem = gen_html();
        let (_, mut elem) = Reconcilable::attach(elem, &scope, &parent, NodeRef::default());
        let vtag = assert_btag_mut(&mut elem);
        // test if the className has been set
        assert!(vtag.reference().has_attribute("class"));
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
        let elem = html! { <input value={expected} /> };
        let (_, mut elem) = Reconcilable::attach(elem, &scope, &parent, NodeRef::default());
        let vtag = assert_btag_ref(&elem);

        // User input
        let input_ref = &vtag.reference();
        let input = input_ref.dyn_ref::<InputElement>();
        input.unwrap().set_value("User input");

        let next_elem = html! { <input value={expected} /> };
        let elem_vtag = assert_vtag(next_elem);

        // Sync happens here
        elem_vtag.reconcile_node(&scope, &parent, NodeRef::default(), &mut elem);
        let vtag = assert_btag_ref(&elem);

        // Get new current value of the input element
        let input_ref = &vtag.reference();
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
        let elem = html! { <input /> };
        let (_, mut elem) = Reconcilable::attach(elem, &scope, &parent, NodeRef::default());
        let vtag = assert_btag_ref(&elem);

        // User input
        let input_ref = &vtag.reference();
        let input = input_ref.dyn_ref::<InputElement>();
        input.unwrap().set_value("User input");

        let next_elem = html! { <input /> };
        let elem_vtag = assert_vtag(next_elem);

        // Value should not be refreshed
        elem_vtag.reconcile_node(&scope, &parent, NodeRef::default(), &mut elem);
        let vtag = assert_btag_ref(&elem);

        // Get user value of the input element
        let input_ref = &vtag.reference();
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

        let elem = html! { <@{
            let mut builder = String::new();
            builder.push('a');
            builder
        }/> };

        let (_, mut elem) = Reconcilable::attach(elem, &scope, &parent, NodeRef::default());
        let vtag = assert_btag_mut(&mut elem);
        // make sure the new tag name is used internally
        assert_eq!(vtag.tag(), "a");

        // Element.tagName is always in the canonical upper-case form.
        assert_eq!(vtag.reference().tag_name(), "A");
    }

    #[test]
    fn dynamic_tags_handle_value_attribute() {
        let div_el = html! {
            <@{"div"} value="Hello"/>
        };
        let div_vtag = assert_vtag_ref(&div_el);
        assert!(div_vtag.value().is_none());
        let v: Option<&str> = div_vtag
            .attributes
            .iter()
            .find(|(k, _)| k == &"value")
            .map(|(_, v)| AsRef::as_ref(v));
        assert_eq!(v, Some("Hello"));

        let input_el = html! {
            <@{"input"} value="World"/>
        };
        let input_vtag = assert_vtag_ref(&input_el);
        assert_eq!(input_vtag.value(), Some(&AttrValue::Static("World")));
        assert!(!input_vtag.attributes.iter().any(|(k, _)| k == "value"));
    }

    #[test]
    fn dynamic_tags_handle_weird_capitalization() {
        let el = html! {
            <@{"tExTAREa"}/>
        };
        let vtag = assert_vtag_ref(&el);
        assert_eq!(vtag.tag(), "textarea");
    }

    #[test]
    fn reset_node_ref() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        let node_ref = NodeRef::default();
        let elem: VNode = html! { <div ref={node_ref.clone()}></div> };
        assert_vtag_ref(&elem);
        let (_, elem) = elem.attach(&scope, &parent, NodeRef::default());
        assert_eq!(node_ref.get(), parent.first_child());
        elem.detach(&parent, false);
        assert!(node_ref.get().is_none());
    }

    #[test]
    fn vtag_reuse_should_reset_ancestors_node_ref() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&parent).unwrap();

        let node_ref_a = NodeRef::default();
        let elem_a = html! { <div id="a" ref={node_ref_a.clone()} /> };
        let (_, mut elem) = elem_a.attach(&scope, &parent, NodeRef::default());

        // save the Node to check later that it has been reused.
        let node_a = node_ref_a.get().unwrap();

        let node_ref_b = NodeRef::default();
        let elem_b = html! { <div id="b" ref={node_ref_b.clone()} /> };
        elem_b.reconcile_node(&scope, &parent, NodeRef::default(), &mut elem);

        let node_b = node_ref_b.get().unwrap();

        assert_eq!(node_a, node_b, "VTag should have reused the element");
        assert!(
            node_ref_a.get().is_none(),
            "node_ref_a should have been reset when the element was reused."
        );
    }

    #[test]
    fn vtag_should_not_touch_newly_bound_refs() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&parent).unwrap();

        let test_ref = NodeRef::default();
        let before = html! {
            <>
                <div ref={&test_ref} id="before" />
            </>
        };
        let after = html! {
            <>
                <h6 />
                <div ref={&test_ref} id="after" />
            </>
        };
        // The point of this diff is to first render the "after" div and then detach the "before" div,
        // while both should be bound to the same node ref

        let (_, mut elem) = before.attach(&scope, &parent, NodeRef::default());
        after.reconcile_node(&scope, &parent, NodeRef::default(), &mut elem);

        assert_eq!(
            test_ref
                .get()
                .unwrap()
                .dyn_ref::<web_sys::Element>()
                .unwrap()
                .outer_html(),
            "<div id=\"after\"></div>"
        );
    }
}

#[cfg(test)]
mod layout_tests {
    extern crate self as yew;

    use crate::html;
    use crate::tests::layout_tests::{diff_layouts, TestLayout};

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

#[cfg(test)]
mod tests_without_browser {
    use crate::html;

    #[test]
    fn html_if_bool() {
        assert_eq!(
            html! {
                if true {
                    <div class="foo" />
                }
            },
            html! { <div class="foo" /> },
        );
        assert_eq!(
            html! {
                if false {
                    <div class="foo" />
                } else {
                    <div class="bar" />
                }
            },
            html! {
                <div class="bar" />
            },
        );
        assert_eq!(
            html! {
                if false {
                    <div class="foo" />
                }
            },
            html! {},
        );

        // non-root tests
        assert_eq!(
            html! {
                <div>
                    if true {
                        <div class="foo" />
                    }
                </div>
            },
            html! {
                <div>
                    <div class="foo" />
                </div>
            },
        );
        assert_eq!(
            html! {
                <div>
                    if false {
                        <div class="foo" />
                    } else {
                        <div class="bar" />
                    }
                </div>
            },
            html! {
                <div>
                    <div class="bar" />
                </div>
            },
        );
        assert_eq!(
            html! {
                <div>
                    if false {
                        <div class="foo" />
                    }
                </div>
            },
            html! {
                <div>
                    <></>
                </div>
            },
        );
    }

    #[test]
    fn html_if_option() {
        let option_foo = Some("foo");
        let none: Option<&'static str> = None;
        assert_eq!(
            html! {
                if let Some(class) = option_foo {
                    <div class={class} />
                }
            },
            html! { <div class="foo" /> },
        );
        assert_eq!(
            html! {
                if let Some(class) = none {
                    <div class={class} />
                } else {
                    <div class="bar" />
                }
            },
            html! { <div class="bar" /> },
        );
        assert_eq!(
            html! {
                if let Some(class) = none {
                    <div class={class} />
                }
            },
            html! {},
        );

        // non-root tests
        assert_eq!(
            html! {
                <div>
                    if let Some(class) = option_foo {
                        <div class={class} />
                    }
                </div>
            },
            html! { <div><div class="foo" /></div> },
        );
        assert_eq!(
            html! {
                <div>
                    if let Some(class) = none {
                        <div class={class} />
                    } else {
                        <div class="bar" />
                    }
                </div>
            },
            html! { <div><div class="bar" /></div> },
        );
        assert_eq!(
            html! {
                <div>
                    if let Some(class) = none {
                        <div class={class} />
                    }
                </div>
            },
            html! { <div><></></div> },
        );
    }
}
