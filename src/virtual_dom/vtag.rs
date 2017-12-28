use std::fmt;
use std::borrow::Cow;
use std::collections::HashSet;
use stdweb::web::{IElement, Element, EventListenerHandle};
use stdweb::web::html_element::InputElement;
use stdweb::unstable::TryFrom;
use virtual_dom::{Messages, Listener, Listeners, Classes, Attributes, Patch, VNode};

pub struct VTag<MSG> {
    pub tag: Cow<'static, str>,
    pub listeners: Listeners<MSG>,
    pub captured: Vec<EventListenerHandle>,
    pub attributes: Attributes,
    pub childs: Vec<VNode<MSG>>,
    pub classes: Classes,
    pub value: Option<String>,
    pub kind: Option<String>,
    pub checked: bool,
}

impl<MSG> VTag<MSG> {
    pub fn new<S: Into<Cow<'static, str>>>(tag: S) -> Self {
        VTag {
            tag: tag.into(),
            classes: Classes::new(),
            attributes: Attributes::new(),
            listeners: Vec::new(),
            captured: Vec::new(),
            childs: Vec::new(),
            value: None,
            kind: None,
            /// In HTML node `checked` attribute sets `defaultChecked` parameter,
            /// but we use own field to control real `checked` parameter
            checked: false,
        }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn add_child(&mut self, child: VNode<MSG>) {
        self.childs.push(child);
    }

    pub fn add_classes(&mut self, class: &str) {
        let class = class.trim();
        if !class.is_empty() {
            self.classes.insert(class.into());
        }
    }

    pub fn set_value<T: ToString>(&mut self, value: &T) {
        self.value = Some(value.to_string());
    }

    pub fn set_kind<T: ToString>(&mut self, value: T) {
        self.kind = Some(value.to_string());
    }

    pub fn set_checked(&mut self, value: bool) {
        self.checked = value;
    }

    pub fn add_attribute<T: ToString>(&mut self, name: &str, value: T) {
        self.attributes.insert(name.to_owned(), value.to_string());
    }

    pub fn add_listener(&mut self, listener: Box<Listener<MSG>>) {
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
                let value = self.attributes.get(*key).unwrap();
                Patch::Add(key.to_string(), value.to_string())
            });
            changes.extend(to_add);
            for key in left_keys.intersection(&right_keys) {
                let left_value = self.attributes.get(*key).unwrap();
                let right_value = ancestor.attributes.get(*key).unwrap();
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

impl<MSG> VTag<MSG> {
    pub fn render(&mut self, subject: &Element, mut opposite: Option<Self>, messages: Messages<MSG>) {
        // TODO Replace self if tagName differs

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
                debug!("Removing handler...");
                handle.remove();
            }
        }

        for mut listener in self.listeners.drain(..) {
            debug!("Add listener...");
            let handle = listener.attach(&subject, messages.clone());
            self.captured.push(handle);
        }
    }
}

impl<MSG> fmt::Debug for VTag<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VTag {{ tag: {} }}", self.tag)
    }
}

// stdweb doesn't have methods to work with attributes
// this is workaround from: https://github.com/koute/stdweb/issues/16#issuecomment-325195854
fn set_attribute(element: &Element, name: &str, value: &str) {
    js!( @{element}.setAttribute( @{name}, @{value} ); );
}

fn remove_attribute(element: &Element, name: &str) {
    js!( @{element}.removeAttribute( @{name} ); );
}

fn set_checked(input: &InputElement, value: bool) {
    js!( @{input}.checked = @{value}; );
}
