//! This module contains the implementation of a virtual element node `VTag`.

use std::fmt;
use std::borrow::Cow;
use std::collections::HashSet;
use std::cmp::PartialEq;
use stdweb::web::{INode, Node, IElement, Element, EventListenerHandle, document};
use stdweb::web::html_element::InputElement;
use stdweb::unstable::TryFrom;
use html::{ScopeEnv, Component};
use super::{Listener, Listeners, Classes, Attributes, Patch, Reform, VDiff, VNode};

/// A type for a virtual
/// [Element](https://developer.mozilla.org/en-US/docs/Web/API/Element)
/// representation.
pub struct VTag<CTX, COMP: Component<CTX>> {
    /// A tag of the element.
    tag: Cow<'static, str>,
    /// A reference to the `Element`.
    pub reference: Option<Element>,
    /// List of attached listeners.
    pub listeners: Listeners<CTX, COMP>,
    /// List of attributes.
    pub attributes: Attributes,
    /// The list of children nodes. Which also could have own children.
    pub childs: Vec<VNode<CTX, COMP>>,
    /// List of attached classes.
    pub classes: Classes,
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
    /// _Service field_. Keeps handler for attached listeners
    /// to have an opportunity to drop them later.
    captured: Vec<EventListenerHandle>,
}

impl<CTX, COMP: Component<CTX>> VTag<CTX, COMP> {
    /// Creates a new `VTag` instance with `tag` name (cannot be changed later in DOM).
    pub fn new<S: Into<Cow<'static, str>>>(tag: S) -> Self {
        VTag {
            tag: tag.into(),
            reference: None,
            classes: Classes::new(),
            attributes: Attributes::new(),
            listeners: Vec::new(),
            captured: Vec::new(),
            childs: Vec::new(),
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
    pub fn add_child(&mut self, child: VNode<CTX, COMP>) {
        self.childs.push(child);
    }

    /// Add classes to this virtual node. Actually it will set by
    /// [Element.classList.add](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList)
    /// call later.
    pub fn add_classes(&mut self, class: &str) {
        let class = class.trim();
        if !class.is_empty() {
            self.classes.insert(class.into());
        }
    }

    /// Sets `value` for an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    pub fn set_value<T: ToString>(&mut self, value: &T) {
        self.value = Some(value.to_string());
    }

    /// Sets `kind` property of an
    /// [InputElement](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input).
    /// Same as set `type` attribute.
    pub fn set_kind<T: ToString>(&mut self, value: T) {
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
    /// `class`, `type/kind`, `value` and `checked`.
    pub fn add_attribute<T: ToString>(&mut self, name: &str, value: T) {
        self.attributes.insert(name.to_owned(), value.to_string());
    }

    /// Adds new listener to the node.
    /// It's boxed because we want to keep it in a single list.
    /// Lates `Listener::attach` called to attach actual listener to a DOM node.
    pub fn add_listener(&mut self, listener: Box<Listener<CTX, COMP>>) {
        self.listeners.push(listener);
    }

    fn soakup_classes(&mut self, ancestor: &mut Option<Self>) -> Vec<Patch<String, ()>> {
        let mut changes = Vec::new();
        if let &mut Some(ref ancestor) = ancestor {
            let to_add = self.classes.difference(&ancestor.classes).map(|class| {
                Patch::Add(class.to_owned(), ())
            });
            changes.extend(to_add);
            let to_remove = ancestor.classes.difference(&self.classes).map(|class| {
                Patch::Remove(class.to_owned())
            });
            changes.extend(to_remove);
        } else {
            // Add everything
            let to_add = self.classes.iter().map(|class| Patch::Add(class.to_owned(), ()));
            changes.extend(to_add);
        }
        changes
    }

    fn soakup_attributes(&mut self, ancestor: &mut Option<Self>) -> Vec<Patch<String, String>> {
        let mut changes = Vec::new();
        if let &mut Some(ref mut ancestor) = ancestor {
            let left_keys = self.attributes.keys().collect::<HashSet<_>>();
            let right_keys = ancestor.attributes.keys().collect::<HashSet<_>>();
            let to_add = left_keys.difference(&right_keys).map(|key| {
                let value = self.attributes.get(*key).expect("attribute of vtag lost");
                Patch::Add(key.to_string(), value.to_string())
            });
            changes.extend(to_add);
            for key in left_keys.intersection(&right_keys) {
                let left_value = self.attributes.get(*key).expect("attribute of the left side lost");
                let right_value = ancestor.attributes.get(*key).expect("attribute of the right side lost");
                if left_value != right_value {
                    let mutator = Patch::Replace(key.to_string(), left_value.to_string());
                    changes.push(mutator);
                }
            }
            let to_remove = right_keys.difference(&left_keys).map(|key| {
                Patch::Remove(key.to_string())
            });
            changes.extend(to_remove);
        } else {
            for (key, value) in self.attributes.iter() {
                let mutator = Patch::Add(key.to_string(), value.to_string());
                changes.push(mutator);
            }
        }
        changes
    }

    fn soakup_kind(&mut self, ancestor: &mut Option<Self>) -> Option<Patch<String, ()>> {
        match (
            &self.kind,
            ancestor.as_mut().and_then(|anc| anc.kind.take()),
        ) {
            (&Some(ref left), Some(ref right)) => {
                if left != right {
                    Some(Patch::Replace(left.to_string(), ()))
                } else {
                    None
                }
            }
            (&Some(ref left), None) => Some(Patch::Add(left.to_string(), ())),
            (&None, Some(right)) => Some(Patch::Remove(right)),
            (&None, None) => None,
        }
    }

    fn soakup_value(&mut self, ancestor: &mut Option<Self>) -> Option<Patch<String, ()>> {
        match (
            &self.value,
            ancestor.as_mut().and_then(|anc| anc.value.take()),
        ) {
            (&Some(ref left), Some(ref right)) => {
                if left != right {
                    Some(Patch::Replace(left.to_string(), ()))
                } else {
                    None
                }
            }
            (&Some(ref left), None) => Some(Patch::Add(left.to_string(), ())),
            (&None, Some(right)) => Some(Patch::Remove(right)),
            (&None, None) => None,
        }
    }
}

impl<CTX: 'static, COMP: Component<CTX>> VDiff for VTag<CTX, COMP> {
    type Context = CTX;
    type Component = COMP;

    /// Get binded node.
    fn get_node(&self) -> Option<Node> {
        self.reference.as_ref().map(|elem| elem.as_node().to_owned())
    }

    /// Remove VTag from parent.
    fn remove(self, parent: &Node) {
        let node = self.reference.expect("tried to remove not rendered VTag from DOM");
        if let Err(_) = parent.remove_child(&node) {
            warn!("Node not found to remove VTag");
        }
    }

    /// Renders virtual tag over DOM `Element`, but it also compares this with an opposite `VTag`
    /// to compute what to patch in the actual DOM nodes.
    fn apply(&mut self,
             parent: &Node,
             precursor: Option<&Node>,
             opposite: Option<VNode<Self::Context, Self::Component>>,
             env: ScopeEnv<Self::Context, Self::Component>) -> Option<Node>
    {
        let (reform, mut opposite) = {
            match opposite {
                Some(VNode::VTag(mut vtag)) => {
                    // Copy reference from right to left (as is)
                    match vtag.reference.take() {
                        Some(element) => {
                            if self.tag == vtag.tag {
                                (Reform::Keep(element), Some(vtag))
                            } else {
                                (Reform::Replace(element.as_node().to_owned()), None)
                            }
                        }
                        None => {
                            (Reform::Append, None)
                        }
                    }
                }
                Some(vnode) => {
                    if let Some(wrong) = vnode.get_node() {
                        (Reform::Replace(wrong.as_node().to_owned()), None)
                    } else {
                        (Reform::Append, None)
                    }
                }
                None => {
                    (Reform::Append, None)
                }
            }
        };

        let mut element = {
            match reform {
                Reform::Keep(element) => {
                    element
                }
                Reform::Append => {
                    let element = document().create_element(&self.tag);
                    let precursor = precursor.and_then(|node| node.next_sibling());
                    if let Some(precursor) = precursor {
                        parent.insert_before(&element, &precursor);
                    } else {
                        parent.append_child(&element);
                    }
                    element
                }
                Reform::Replace(wrong) => {
                    let element = document().create_element(&self.tag);
                    parent.replace_child(&element, &wrong);
                    element
                }
            }
        };

        {
            // Update parameters
            let mut rights = {
                if let Some(ref mut right) = opposite {
                    right.childs.drain(..).map(Some).collect::<Vec<_>>()
                } else {
                    Vec::new()
                }
            };
            let subject = &mut element;

            let changes = self.soakup_classes(&mut opposite);
            for change in changes {
                let list = subject.class_list();
                match change {
                    Patch::Add(class, _) |
                    Patch::Replace(class, _) => {
                        list.add(&class);
                    }
                    Patch::Remove(class) => {
                        list.remove(&class);
                    }
                }
            }

            let changes = self.soakup_attributes(&mut opposite);
            for change in changes {
                match change {
                    Patch::Add(key, value) |
                    Patch::Replace(key, value) => {
                        set_attribute(&subject, &key, &value);
                    }
                    Patch::Remove(key) => {
                        remove_attribute(&subject, &key);
                    }
                }
            }

            // `input` element has extra parameters to control
            // I override behavior of attributes to make it more clear
            // and useful in templates. For example I interpret `checked`
            // attribute as `checked` parameter, not `defaultChecked` as browsers do
            if let Ok(input) = InputElement::try_from(subject.clone()) {
                if let Some(change) = self.soakup_kind(&mut opposite) {
                    match change {
                        Patch::Add(kind, _) |
                        Patch::Replace(kind, _) => {
                            input.set_kind(&kind);
                        }
                        Patch::Remove(_) => {
                            input.set_kind("");
                        }
                    }
                }

                if let Some(change) = self.soakup_value(&mut opposite) {
                    match change {
                        Patch::Add(kind, _) |
                        Patch::Replace(kind, _) => {
                            input.set_value(&kind);
                        }
                        Patch::Remove(_) => {
                            input.set_value("");
                        }
                    }
                }

                // IMPORTANT! This parameters have to be set every time
                // to prevent strange behaviour in browser when DOM changed
                set_checked(&input, self.checked);
            }

            // Every render it removes all listeners and attach it back later
            // TODO Compare references of handler to do listeners update better
            if let Some(mut opposite) = opposite {
                for handle in opposite.captured.drain(..) {
                    handle.remove();
                }
            }

            for mut listener in self.listeners.drain(..) {
                let handle = listener.attach(&subject, env.sender());
                self.captured.push(handle);
            }

            let mut lefts = self.childs.iter_mut().map(Some).collect::<Vec<_>>();
            // Process children
            let diff = lefts.len() as i32 - rights.len() as i32;
            if diff > 0 {
                for _ in 0..diff {
                    rights.push(None);
                }
            } else if diff < 0 {
                for _ in 0..-diff {
                    lefts.push(None);
                }
            }
            // Start with an empty precursor, because it put childs to itself
            let mut precursor = None;
            for pair in lefts.into_iter().zip(rights) {
                match pair {
                    (Some(left), right) => {
                        precursor = left.apply(subject.as_node(), precursor.as_ref(), right, env.clone());
                    }
                    (None, Some(right)) => {
                        right.remove(subject.as_node());
                    }
                    (None, None) => {
                        panic!("redundant iterations during diff");
                    }
                }
            }
        }
        self.reference = Some(element);
        self.reference.as_ref().map(|e| e.as_node().to_owned())
    }
}

impl<CTX, COMP: Component<CTX>> fmt::Debug for VTag<CTX, COMP> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VTag {{ tag: {} }}", self.tag)
    }
}

/// `stdweb` doesn't have methods to work with attributes now.
/// this is workaround from: https://github.com/koute/stdweb/issues/16#issuecomment-325195854
fn set_attribute(element: &Element, name: &str, value: &str) {
    js!( @(no_return) @{element}.setAttribute( @{name}, @{value} ); );
}

/// Removes attribute from a element by name.
fn remove_attribute(element: &Element, name: &str) {
    js!( @(no_return) @{element}.removeAttribute( @{name} ); );
}

/// Set `checked` value for the `InputElement`.
fn set_checked(input: &InputElement, value: bool) {
    js!( @(no_return) @{input}.checked = @{value}; );
}

impl<CTX, COMP: Component<CTX>> PartialEq for VTag<CTX, COMP> {
    fn eq(&self, other: &VTag<CTX, COMP>) -> bool {
        if self.tag != other.tag {
            return false;
        }

        if self.value != other.value {
            return false;
        }

        if self.kind != other.kind {
            return false;
        }

        if self.checked != other.checked {
            return false;
        }

        if self.listeners.len() != other.listeners.len() {
            return false;
        }

        for i in 0..self.listeners.len() {
            let a = &self.listeners[i];
            let b = &other.listeners[i];

            if a.kind() != b.kind() {
                return false;
            }
        }

        if self.attributes != other.attributes {
            return false;
        }

        if self.classes != other.classes {
            return false;
        }

        if self.childs.len() != other.childs.len() {
            return false;
        }

        for i in 0..self.childs.len() {
            let a = &self.childs[i];
            let b = &other.childs[i];

            if a != b {
                return false;
            }
        }

        true
    }
}
