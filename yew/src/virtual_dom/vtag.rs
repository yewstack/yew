//! This module contains the implementation of a virtual element node `VTag`.

use super::{Attributes, Key, Listener, Listeners, Patch, Transformer, VDiff, VList, VNode};
use crate::html::{AnyScope, NodeRef};
use crate::utils::document;
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use log::warn;
use std::borrow::Cow;
use std::cmp::PartialEq;
use std::rc::Rc;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use crate::html::EventListener;
        #[allow(unused_imports)]
        use stdweb::{_js_impl, js};
        use stdweb::unstable::TryFrom;
        use stdweb::web::html_element::{InputElement, TextAreaElement};
        use stdweb::web::{Element, IElement, INode};
    } else if #[cfg(feature = "web_sys")] {
        use gloo::events::EventListener;
        use std::ops::Deref;
        use wasm_bindgen::JsCast;
        use web_sys::{
            Element, HtmlInputElement as InputElement, HtmlTextAreaElement as TextAreaElement, HtmlButtonElement
        };
    }
}

/// SVG namespace string used for creating svg elements
pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// Default namespace for html elements
pub const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

/// Used to improve performance of runtime element checks
#[derive(Clone, Copy, Debug, PartialEq)]
enum ElementType {
    Input,
    Textarea,
    Button,
    Other,
}

impl ElementType {
    fn from_tag(tag: &str) -> Self {
        match tag.to_ascii_lowercase().as_str() {
            "input" => Self::Input,
            "textarea" => Self::Textarea,
            "button" => Self::Button,
            _ => Self::Other,
        }
    }
}

/// A type for a virtual
/// [Element](https://developer.mozilla.org/en-US/docs/Web/API/Element)
/// representation.
#[derive(Debug)]
pub struct VTag {
    /// A tag of the element.
    tag: Cow<'static, str>,
    /// Type of element.
    element_type: ElementType,
    /// A reference to the DOM `Element`.
    pub reference: Option<Element>,
    /// List of attached listeners.
    pub listeners: Listeners,
    /// List of attributes.
    pub attributes: Attributes,
    /// List of children nodes
    pub children: VList,
    /// Contains a value of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    pub value: Option<String>,
    /// Contains
    /// [kind](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#Form_%3Cinput%3E_types)
    /// value of an `InputElement`.
    pub kind: Option<String>,
    /// Represents `checked` attribute of
    /// [input](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-checked).
    /// It exists to override standard behavior of `checked` attribute, because
    /// in original HTML it sets `defaultChecked` value of `InputElement`, but for reactive
    /// frameworks it's more useful to control `checked` value of an `InputElement`.
    pub checked: bool,
    /// A node reference used for DOM access in Component lifecycle methods
    pub node_ref: NodeRef,
    /// Keeps handler for attached listeners to have an opportunity to drop them later.
    captured: Vec<EventListener>,

    pub key: Option<Key>,
}

impl Clone for VTag {
    fn clone(&self) -> Self {
        VTag {
            tag: self.tag.clone(),
            element_type: self.element_type,
            reference: None,
            listeners: self.listeners.clone(),
            attributes: self.attributes.clone(),
            children: self.children.clone(),
            value: self.value.clone(),
            kind: self.kind.clone(),
            checked: self.checked,
            node_ref: self.node_ref.clone(),
            key: self.key.clone(),
            captured: Vec::new(),
        }
    }
}

impl VTag {
    /// Creates a new `VTag` instance with `tag` name (cannot be changed later in DOM).
    pub fn new<S: Into<Cow<'static, str>>>(tag: S) -> Self {
        let tag: Cow<'static, str> = tag.into();
        let element_type = ElementType::from_tag(&tag);
        VTag {
            tag,
            element_type,
            reference: None,
            attributes: Attributes::new(),
            listeners: Vec::new(),
            captured: Vec::new(),
            children: VList::new(),
            node_ref: NodeRef::default(),
            key: None,
            value: None,
            kind: None,
            // In HTML node `checked` attribute sets `defaultChecked` parameter,
            // but we use own field to control real `checked` parameter
            checked: false,
        }
    }

    /// Returns tag of an `Element`. In HTML tags are always uppercase.
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// Add `VNode` child.
    pub fn add_child(&mut self, child: VNode) {
        self.children.add_child(child);
    }

    /// Add multiple `VNode` children.
    pub fn add_children(&mut self, children: impl IntoIterator<Item = VNode>) {
        self.children.add_children(children);
    }

    /// Sets `value` for an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    pub fn set_value<T: ToString>(&mut self, value: &T) {
        self.value = Some(value.to_string());
    }

    /// Sets `kind` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// Same as set `type` attribute.
    pub fn set_kind<T: ToString>(&mut self, value: &T) {
        self.kind = Some(value.to_string());
    }

    /// Sets `checked` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// (Not a value of node's attribute).
    pub fn set_checked(&mut self, value: bool) {
        self.checked = value;
    }

    /// Adds attribute to a virtual node. Not every attribute works when
    /// it set as attribute. We use workarounds for:
    /// `type/kind`, `value` and `checked`.
    ///
    /// If this virtual node has this attribute present, the value is replaced.
    pub fn add_attribute<T: ToString>(&mut self, name: &str, value: &T) {
        self.attributes.insert(name.to_owned(), value.to_string());
    }

    /// Sets a boolean attribute if `value` is true. Removes if `value` is false. The name
    /// of the attribute will be used as the value.
    ///
    /// Example: `<button disabled="disabled">`
    pub fn set_boolean_attribute(&mut self, name: &str, value: bool) {
        if value {
            self.attributes.insert(name.to_owned(), name.to_owned());
        } else {
            self.attributes.remove(name);
        }
    }

    /// Adds attributes to a virtual node. Not every attribute works when
    /// it set as attribute. We use workarounds for:
    /// `type/kind`, `value` and `checked`.
    pub fn add_attributes(&mut self, attrs: Vec<(String, String)>) {
        for (name, value) in attrs {
            self.attributes.insert(name, value);
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
    pub fn add_listeners(&mut self, listeners: Vec<Rc<dyn Listener>>) {
        for listener in listeners {
            self.listeners.push(listener);
        }
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
        // Don't refresh value if the element is not controlled
        if self.value.is_none() {
            return;
        }

        if let Some(element) = self.reference.as_ref() {
            if self.element_type == ElementType::Input {
                let input_el = cfg_match! {
                    feature = "std_web" => InputElement::try_from(element.clone()).ok(),
                    feature = "web_sys" => element.dyn_ref::<InputElement>(),
                };
                if let Some(input) = input_el {
                    let current_value = cfg_match! {
                        feature = "std_web" => input.raw_value(),
                        feature = "web_sys" => input.value(),
                    };
                    self.set_value(&current_value)
                }
            } else if self.element_type == ElementType::Textarea {
                let textarea_el = cfg_match! {
                    feature = "std_web" => TextAreaElement::try_from(element.clone()).ok(),
                    feature = "web_sys" => element.dyn_ref::<TextAreaElement>(),
                };
                if let Some(tae) = textarea_el {
                    let current_value = &tae.value();
                    self.set_value(&current_value)
                }
            }
        }
    }

    /// This handles patching of attributes when the keys are equal but
    /// the values are different.
    fn diff_attributes<'a>(
        &'a self,
        ancestor: &'a Option<Box<Self>>,
    ) -> impl Iterator<Item = Patch<&'a str, &'a str>> + 'a {
        // Only change what is necessary.
        let to_add_or_replace =
            self.attributes.iter().filter_map(move |(key, value)| {
                match ancestor
                    .as_ref()
                    .and_then(|ancestor| ancestor.attributes.get(&**key))
                {
                    None => Some(Patch::Add(&**key, &**value)),
                    Some(ancestor_value) if value != ancestor_value => {
                        Some(Patch::Replace(&**key, &**value))
                    }
                    _ => None,
                }
            });
        let to_remove = ancestor
            .iter()
            .flat_map(|ancestor| ancestor.attributes.keys())
            .filter(move |key| !self.attributes.contains_key(&**key))
            .map(|key| Patch::Remove(&**key));

        to_add_or_replace.chain(to_remove)
    }

    /// Similar to `diff_attributes` except there is only a single `kind`.
    fn diff_kind<'a>(&'a self, ancestor: &'a Option<Box<Self>>) -> Option<Patch<&'a str, ()>> {
        match (
            self.kind.as_ref(),
            ancestor.as_ref().and_then(|anc| anc.kind.as_ref()),
        ) {
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

    /// Almost identical in spirit to `diff_kind`
    fn diff_value<'a>(&'a self, ancestor: &'a Option<Box<Self>>) -> Option<Patch<&'a str, ()>> {
        match (
            self.value.as_ref(),
            ancestor.as_ref().and_then(|anc| anc.value.as_ref()),
        ) {
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

    fn apply_diffs(&mut self, ancestor: &Option<Box<Self>>) {
        let element = self.reference.as_ref().expect("element expected");

        // Update parameters
        let changes = self.diff_attributes(ancestor);

        // apply attribute patches including an optional "class"-attribute patch
        for change in changes {
            match change {
                Patch::Add(key, value) | Patch::Replace(key, value) => {
                    element
                        .set_attribute(&key, &value)
                        .expect("invalid attribute key");
                }
                Patch::Remove(key) => {
                    cfg_match! {
                        feature = "std_web" => element.remove_attribute(&key),
                        feature = "web_sys" => element.remove_attribute(&key).expect("could not remove attribute"),
                    };
                }
            }
        }

        // TODO: add std_web after https://github.com/koute/stdweb/issues/395 will be approved
        // Check this out: https://github.com/yewstack/yew/pull/1033/commits/4b4e958bb1ccac0524eb20f63f06ae394c20553d
        #[cfg(feature = "web_sys")]
        {
            if self.element_type == ElementType::Button {
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
        if self.element_type == ElementType::Input {
            if let Some(input) = {
                cfg_match! {
                    feature = "std_web" => InputElement::try_from(element.clone()).ok(),
                    feature = "web_sys" => element.dyn_ref::<InputElement>(),
                }
            } {
                if let Some(change) = self.diff_kind(ancestor) {
                    let kind = match change {
                        Patch::Add(kind, _) | Patch::Replace(kind, _) => kind,
                        Patch::Remove(_) => "",
                    };
                    cfg_match! {
                        feature = "std_web" => ({
                            //https://github.com/koute/stdweb/commit/3b85c941db00b8e3c942624afd50c5929085fb08
                            //input.set_kind(&kind);
                            let input = &input;
                            js! { @(no_return)
                                @{input}.type = @{kind};
                            }
                        }),
                        feature = "web_sys" => input.set_type(kind),
                    }
                }

                if let Some(change) = self.diff_value(ancestor) {
                    let raw_value = match change {
                        Patch::Add(kind, _) | Patch::Replace(kind, _) => kind,
                        Patch::Remove(_) => "",
                    };
                    cfg_match! {
                        feature = "std_web" => input.set_raw_value(raw_value),
                        feature = "web_sys" => input.set_value(raw_value),
                    };
                }

                // IMPORTANT! This parameter has to be set every time
                // to prevent strange behaviour in the browser when the DOM changes
                set_checked(&input, self.checked);
            }
        } else if self.element_type == ElementType::Textarea {
            if let Some(tae) = {
                cfg_match! {
                    feature = "std_web" => TextAreaElement::try_from(element.clone()).ok(),
                    feature = "web_sys" => element.dyn_ref::<TextAreaElement>(),
                }
            } {
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

    fn create_element(&self, parent: &Element) -> Element {
        if self.tag == "svg"
            || parent
                .namespace_uri()
                .map_or(false, |ns| ns == SVG_NAMESPACE)
        {
            let namespace = cfg_match! {
                feature = "std_web" => SVG_NAMESPACE,
                feature = "web_sys" => Some(SVG_NAMESPACE),
            };
            document()
                .create_element_ns(namespace, &self.tag)
                .expect("can't create namespaced element for vtag")
        } else {
            document()
                .create_element(&self.tag)
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
        self.children.detach(&node);
        if parent.remove_child(&node).is_err() {
            warn!("Node not found to remove VTag");
        }
    }

    /// Renders virtual tag over DOM `Element`, but it also compares this with an ancestor `VTag`
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
                VNode::VTag(vtag) if self.tag == vtag.tag && self.key == vtag.key => Some(vtag),
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

        self.apply_diffs(&ancestor_tag);
        self.recreate_listeners(&mut ancestor_tag);

        // Process children
        let element = self.reference.as_ref().expect("Reference should be set");
        if !self.children.is_empty() {
            self.children.apply(
                parent_scope,
                element,
                NodeRef::default(),
                ancestor_tag.map(|a| a.children.into()),
            );
        } else if let Some(mut ancestor_tag) = ancestor_tag {
            ancestor_tag.children.detach(element);
        }

        let node = cfg_match! {
            feature = "std_web" => element.as_node(),
            feature = "web_sys" => element.deref(),
        };
        self.node_ref.set(Some(node.clone()));
        self.node_ref.clone()
    }
}

/// Set `checked` value for the `InputElement`.
fn set_checked(input: &InputElement, value: bool) {
    cfg_match! {
        feature = "std_web" => js!( @(no_return) @{input}.checked = @{value}; ),
        feature = "web_sys" => input.set_checked(value),
    };
}

impl PartialEq for VTag {
    fn eq(&self, other: &VTag) -> bool {
        self.tag == other.tag
            && self.value == other.value
            && self.kind == other.kind
            && self.checked == other.checked
            && self.listeners.len() == other.listeners.len()
            && self
                .listeners
                .iter()
                .map(|l| l.kind())
                .eq(other.listeners.iter().map(|l| l.kind()))
            && self.attributes == other.attributes
            && self.children == other.children
    }
}

impl<T> Transformer<T, T> for VTag {
    fn transform(from: T) -> T {
        from
    }
}

impl<'a, T> Transformer<&'a T, T> for VTag
where
    T: Clone,
{
    fn transform(from: &'a T) -> T {
        from.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html;
    use std::any::TypeId;
    #[cfg(feature = "std_web")]
    use stdweb::web::{document, IElement};
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    fn test_scope() -> AnyScope {
        AnyScope {
            type_id: TypeId::of::<()>(),
            parent: None,
            state: Rc::new(()),
        }
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

    #[test]
    fn classes_from_local_variables() {
        let a = html! {
            <div class=("class-1", "class-2")></div>
        };

        let class_2 = "class-2";
        let b = html! {
            <div class=("class-1", class_2)></div>
        };

        let class_2_fmt = format!("class-{}", 2);
        let c = html! {
            <div class=("class-1", class_2_fmt)></div>
        };

        assert_eq!(a, b);
        assert_eq!(a, c);
    }

    /// Returns the class attribute as str reference, or "" if the attribute is not set.
    fn get_class_str(vtag: &VTag) -> &str {
        vtag.attributes
            .get("class")
            .map(AsRef::as_ref)
            .unwrap_or("")
    }

    /// Note: Compares to "" if the class attribute is not set.
    fn assert_class(vnode: VNode, class: &str) {
        if let VNode::VTag(ref vtag) = vnode {
            assert_eq!(get_class_str(vtag), class);
        } else {
            panic!("expected VTag");
        }
    }

    #[test]
    fn supports_multiple_non_unique_classes_tuple() {
        let a = html! {
            <div class=("class-1", "class-1 class-2")></div>
        };

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(!get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn supports_multiple_classes_string() {
        let a = html! {
            <div class="class-1 class-2   class-3"></div>
        };

        let b = html! {
            <div class="class-2 class-3 class-1"></div>
        };

        assert_ne!(a, b);

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn supports_multiple_classes_slice() {
        let classes = ["class-1", "class-2"];
        let a = html! {
            <div class=&classes[..]></div>
        };

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(!get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn supports_multiple_classes_one_value_slice() {
        let classes = ["class-1 class-2", "class-1"];
        let a = html! {
            <div class=&classes[..]></div>
        };

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(!get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn supports_multiple_classes_vec() {
        let mut classes = vec!["class-1"];
        classes.push("class-2");
        let a = html! {
            <div class=classes></div>
        };

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(!get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn supports_multiple_classes_one_value_vec() {
        let classes = vec!["class-1 class-2", "class-1"];
        let a = html! {
            <div class=classes></div>
        };

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(!get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn filter_empty_string_classes() {
        let a = html! { <div class=vec![""]></div> };
        let b = html! { <div class=("", "")></div> };
        let c = html! { <div class=""></div> };
        let d_arr = [""];
        let d = html! { <div class=&d_arr[..]></div> };

        if let VNode::VTag(vtag) = a {
            assert!(!vtag.attributes.contains_key("class"));
        } else {
            panic!("vtag expected");
        }

        if let VNode::VTag(vtag) = b {
            assert!(!vtag.attributes.contains_key("class"));
        } else {
            panic!("vtag expected");
        }

        if let VNode::VTag(vtag) = c {
            assert!(!vtag.attributes.contains_key("class"));
        } else {
            panic!("vtag expected");
        }

        if let VNode::VTag(vtag) = d {
            assert!(!vtag.attributes.contains_key("class"));
        } else {
            panic!("vtag expected");
        }
    }

    fn assert_vtag(node: &mut VNode) -> &mut VTag {
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
        #[cfg(feature = "std_web")]
        let document = document();
        #[cfg(feature = "web_sys")]
        let document = web_sys::window().unwrap().document().unwrap();

        let scope = test_scope();
        let div_el = document.create_element("div").unwrap();
        let namespace = SVG_NAMESPACE;
        #[cfg(feature = "web_sys")]
        let namespace = Some(namespace);
        let svg_el = document.create_element_ns(namespace, "svg").unwrap();

        let mut g_node = html! { <g class="segment"></g> };
        let path_node = html! { <path></path> };
        let mut svg_node = html! { <svg>{path_node}</svg> };

        let svg_tag = assert_vtag(&mut svg_node);
        svg_tag.apply(&scope, &div_el, NodeRef::default(), None);
        assert_namespace(svg_tag, SVG_NAMESPACE);
        let path_tag = assert_vtag(svg_tag.children.get_mut(0).unwrap());
        assert_namespace(path_tag, SVG_NAMESPACE);

        let g_tag = assert_vtag(&mut g_node);
        g_tag.apply(&scope, &div_el, NodeRef::default(), None);
        assert_namespace(g_tag, HTML_NAMESPACE);
        g_tag.reference = None;

        g_tag.apply(&scope, &svg_el, NodeRef::default(), None);
        assert_namespace(g_tag, SVG_NAMESPACE);
    }

    #[test]
    fn keeps_order_of_classes() {
        let a = html! {
            <div class="class-1 class-2   class-3",></div>
        };

        if let VNode::VTag(vtag) = a {
            assert_eq!(get_class_str(&vtag), "class-1 class-2 class-3");
        }
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
            assert!(vtag.attributes.contains_key("aria-controls"));
            assert_eq!(
                vtag.attributes.get("aria-controls"),
                Some(&"it-works".into())
            );
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn it_checks_mixed_closing_tags() {
        let a = html! { <div> <div/>      </div> };
        let b = html! { <div> <div></div> </div> };
        assert_eq!(a, b);
    }

    #[test]
    fn it_checks_misleading_gt() {
        html! { <div data-val=<u32 as Default>::default()></div> };
        html! { <div data-val=Box::<u32>::default()></div> };

        html! { <div><a data-val=<u32 as Default>::default() /> </div> };
        html! { <div><a data-val=Box::<u32>::default() /></div> };
    }

    #[test]
    fn it_does_not_set_empty_class_name() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        #[cfg(feature = "std_web")]
        document().body().unwrap().append_child(&parent);
        #[cfg(feature = "web_sys")]
        document().body().unwrap().append_child(&parent).unwrap();

        let mut elem = html! { <div class=""></div> };
        elem.apply(&scope, &parent, NodeRef::default(), None);
        let vtag = assert_vtag(&mut elem);
        // test if the className has not been set
        assert!(!vtag.reference.as_ref().unwrap().has_attribute("class"));
    }

    #[test]
    fn it_does_not_set_missing_class_name() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        #[cfg(feature = "std_web")]
        document().body().unwrap().append_child(&parent);
        #[cfg(feature = "web_sys")]
        document().body().unwrap().append_child(&parent).unwrap();

        let mut elem = html! { <div></div> };
        elem.apply(&scope, &parent, NodeRef::default(), None);
        let vtag = assert_vtag(&mut elem);
        // test if the className has not been set
        assert!(!vtag.reference.as_ref().unwrap().has_attribute("class"));
    }

    #[test]
    fn it_sets_class_name() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        #[cfg(feature = "std_web")]
        document().body().unwrap().append_child(&parent);
        #[cfg(feature = "web_sys")]
        document().body().unwrap().append_child(&parent).unwrap();

        let mut elem = html! { <div class="ferris the crab"></div> };
        elem.apply(&scope, &parent, NodeRef::default(), None);
        let vtag = assert_vtag(&mut elem);
        // test if the className has been set
        assert!(vtag.reference.as_ref().unwrap().has_attribute("class"));
    }

    #[test]
    fn tuple_different_types() {
        // check if tuples containing different types are compiling
        assert_class(
            html! { <div class=("class-1", "class-2".to_string(), vec!["class-3", "class-4"])></div> },
            "class-1 class-2 class-3 class-4",
        );
        assert_class(
            html! { <div class=("class-1", Some("class-2"), "class-3", Some("class-4".to_string()))></div> },
            "class-1 class-2 class-3 class-4",
        );
        // check different string references
        let str = "some-class";
        let string = str.to_string();
        let string_ref = &string;
        assert_class(html! { <p class=str /> }, "some-class");
        assert_class(html! { <p class=string.clone() /> }, "some-class");
        assert_class(html! { <p class=&Some(str) /> }, "some-class");
        assert_class(html! { <p class=string_ref /> }, "some-class");
        assert_class(html! { <p class=Some(str) /> }, "some-class");
        assert_class(html! { <p class=Some(string.clone()) /> }, "some-class");
        assert_class(html! { <p class=Some(string_ref) /> }, "some-class");
        assert_class(html! { <p class=&Some(string.clone()) /> }, "some-class");
        assert_class(html! { <p class=&Some(string_ref) /> }, "some-class");
        // check with None
        assert_class(html! { <p class=&Option::<&str>::None /> }, "");
        assert_class(html! { <p class=Option::<String>::None /> }, "");
        // check with variables
        let some: Option<&'static str> = Some("some");
        let none: Option<&'static str> = None;
        assert_class(html! { <p class=some /> }, "some");
        assert_class(html! { <p class=none /> }, "");
        // check with variables of different type
        let some: Option<bool> = Some(false);
        let none: Option<bool> = None;
        assert_class(html! { <p class=some.map(|i| i.to_string()) /> }, "false");
        assert_class(html! { <p class=none.map(|i| i.to_string()) /> }, "");
    }

    #[test]
    fn swap_order_of_classes() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        #[cfg(feature = "std_web")]
        document().body().unwrap().append_child(&parent);
        #[cfg(feature = "web_sys")]
        document().body().unwrap().append_child(&parent).unwrap();

        let mut elem = html! { <div class=("class-1", "class-2", "class-3")></div> };
        elem.apply(&scope, &parent, NodeRef::default(), None);

        let vtag = if let VNode::VTag(vtag) = elem {
            vtag
        } else {
            panic!("should be vtag")
        };

        let expected = "class-1 class-2 class-3";
        assert_eq!(get_class_str(&vtag), expected);
        assert_eq!(
            vtag.reference
                .as_ref()
                .unwrap()
                .get_attribute("class")
                .unwrap(),
            expected
        );

        let ancestor = vtag;
        let elem = html! { <div class=("class-3", "class-2", "class-1")></div> };
        let mut vtag = if let VNode::VTag(vtag) = elem {
            vtag
        } else {
            panic!("should be vtag")
        };
        vtag.apply(
            &scope,
            &parent,
            NodeRef::default(),
            Some(VNode::VTag(ancestor)),
        );

        let expected = "class-3 class-2 class-1";
        assert_eq!(get_class_str(&vtag), expected);
        assert_eq!(
            vtag.reference
                .as_ref()
                .unwrap()
                .get_attribute("class")
                .unwrap(),
            expected
        );
    }

    #[test]
    fn add_class_to_the_middle() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        #[cfg(feature = "std_web")]
        document().body().unwrap().append_child(&parent);
        #[cfg(feature = "web_sys")]
        document().body().unwrap().append_child(&parent).unwrap();

        let mut elem = html! { <div class=("class-1", "class-3")></div> };
        elem.apply(&scope, &parent, NodeRef::default(), None);

        let vtag = if let VNode::VTag(vtag) = elem {
            vtag
        } else {
            panic!("should be vtag")
        };

        let expected = "class-1 class-3";
        assert_eq!(get_class_str(&vtag), expected);
        assert_eq!(
            vtag.reference
                .as_ref()
                .unwrap()
                .get_attribute("class")
                .unwrap(),
            expected
        );

        let ancestor = vtag;
        let elem = html! { <div class=("class-1", "class-2", "class-3")></div> };
        let mut vtag = if let VNode::VTag(vtag) = elem {
            vtag
        } else {
            panic!("should be vtag")
        };
        vtag.apply(
            &scope,
            &parent,
            NodeRef::default(),
            Some(VNode::VTag(ancestor)),
        );

        let expected = "class-1 class-2 class-3";
        assert_eq!(get_class_str(&vtag), expected);
        assert_eq!(
            vtag.reference
                .as_ref()
                .unwrap()
                .get_attribute("class")
                .unwrap(),
            expected
        );
    }

    #[test]
    fn controlled_input_synced() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        #[cfg(feature = "std_web")]
        document().body().unwrap().append_child(&parent);
        #[cfg(feature = "web_sys")]
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
        let input = cfg_match! {
            feature = "std_web" => InputElement::try_from(input_ref.clone()).ok(),
            feature = "web_sys" => input_ref.dyn_ref::<InputElement>(),
        };
        cfg_match! {
            feature = "std_web" => input.unwrap().set_raw_value("User input"),
            feature = "web_sys" => input.unwrap().set_value("User input"),
        };

        let ancestor = vtag;
        let mut elem = html! { <input value=expected /> };
        let vtag = assert_vtag(&mut elem);

        // Sync happens here
        vtag.apply(
            &scope,
            &parent,
            NodeRef::default(),
            Some(VNode::VTag(ancestor)),
        );

        // Get new current value of the input element
        let input_ref = vtag.reference.as_ref().unwrap();
        let input = cfg_match! {
            feature = "std_web" => InputElement::try_from(input_ref.clone()).ok(),
            feature = "web_sys" => input_ref.dyn_ref::<InputElement>(),
        }
        .unwrap();

        let current_value = cfg_match! {
            feature = "std_web" => input.raw_value(),
            feature = "web_sys" => input.value(),
        };

        // check whether not changed virtual dom value has been set to the input element
        assert_eq!(current_value, expected);
    }

    #[test]
    fn uncontrolled_input_unsynced() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        #[cfg(feature = "std_web")]
        document().body().unwrap().append_child(&parent);
        #[cfg(feature = "web_sys")]
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
        let input = cfg_match! {
            feature = "std_web" => InputElement::try_from(input_ref.clone()).ok(),
            feature = "web_sys" => input_ref.dyn_ref::<InputElement>(),
        };
        cfg_match! {
            feature = "std_web" => input.unwrap().set_raw_value("User input"),
            feature = "web_sys" => input.unwrap().set_value("User input"),
        };

        let ancestor = vtag;
        let mut elem = html! { <input /> };
        let vtag = assert_vtag(&mut elem);

        // Value should not be refreshed
        vtag.apply(
            &scope,
            &parent,
            NodeRef::default(),
            Some(VNode::VTag(ancestor)),
        );

        // Get user value of the input element
        let input_ref = vtag.reference.as_ref().unwrap();
        let input = cfg_match! {
            feature = "std_web" => InputElement::try_from(input_ref.clone()).ok(),
            feature = "web_sys" => input_ref.dyn_ref::<InputElement>(),
        }
        .unwrap();

        let current_value = cfg_match! {
            feature = "std_web" => input.raw_value(),
            feature = "web_sys" => input.value(),
        };

        // check whether not changed virtual dom value has been set to the input element
        assert_eq!(current_value, "User input");
    }

    #[test]
    fn dynamic_tags_work() {
        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        #[cfg(feature = "std_web")]
        document().body().unwrap().append_child(&parent);
        #[cfg(feature = "web_sys")]
        document().body().unwrap().append_child(&parent).unwrap();

        let mut elem = html! { <@{
            let mut builder = String::new();
            builder.push_str("a");
            builder
        }/> };

        elem.apply(&scope, &parent, NodeRef::default(), None);
        let vtag = assert_vtag(&mut elem);
        // make sure the new tag name is used internally
        assert_eq!(vtag.tag, "a");

        #[cfg(feature = "web_sys")]
        // Element.tagName is always in the canonical upper-case form.
        assert_eq!(vtag.reference.as_ref().unwrap().tag_name(), "A");
    }

    #[test]
    fn dynamic_tags_handle_value_attribute() {
        let mut div_el = html! {
            <@{"div"} value="Hello"/>
        };
        let div_vtag = assert_vtag(&mut div_el);
        assert!(div_vtag.value.is_none());
        assert_eq!(
            div_vtag.attributes.get("value").map(String::as_str),
            Some("Hello")
        );

        let mut input_el = html! {
            <@{"input"} value="World"/>
        };
        let input_vtag = assert_vtag(&mut input_el);
        assert_eq!(input_vtag.value, Some("World".to_string()));
        assert!(!input_vtag.attributes.contains_key("value"));
    }

    #[test]
    fn dynamic_tags_handle_weird_capitalization() {
        let mut el = html! {
            <@{"tExTAREa"}/>
        };
        let vtag = assert_vtag(&mut el);
        assert_eq!(vtag.tag(), "textarea");
    }
}

#[cfg(all(test, feature = "web_sys"))]
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
