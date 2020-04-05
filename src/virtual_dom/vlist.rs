//! This module contains fragments implementation.
use super::{VDiff, VNode, VText};
use cfg_if::cfg_if;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Element, Node};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::{Element, Node};
    }
}

/// This struct represents a fragment of the Virtual DOM tree.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct VList {
    /// The list of children nodes.
    pub children: Vec<VNode>,
    /// Never use a placeholder element if set to true.
    elide_placeholder: bool,
}

impl Deref for VList {
    type Target = Vec<VNode>;

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl DerefMut for VList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

impl VList {
    /// Creates a new empty `VList` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `VList` instance with children.
    pub fn new_with_children(children: Vec<VNode>) -> Self {
        VList {
            children,
            elide_placeholder: false,
        }
    }

    /// Creates a new empty `VList` instance which does not need a placeholder node.
    pub(crate) fn new_without_placeholder() -> Self {
        VList {
            children: Vec::new(),
            elide_placeholder: true,
        }
    }

    /// Add `VNode` child.
    pub fn add_child(&mut self, child: VNode) {
        self.children.push(child);
    }

    pub fn key(&self) -> Option<String> {
        let mut key = String::with_capacity(150);
        const START_STRING: &str = "#vlist_";
        key.push_str(START_STRING);
        for n in &self.children{
            if let Some(child_key) = &n.key() {
                key = key + child_key;
            }
        }
        if START_STRING == &key {
            None
        } else {
            Some(key)
        }
    }
}

impl VDiff for VList {
    fn detach(&mut self, parent: &Element) -> Option<Node> {
        let mut next_sibling = None;
        for mut child in self.children.drain(..) {
            next_sibling = child.detach(parent);
        }
        next_sibling
    }

    fn apply(
        &mut self,
        parent: &Element,
        previous_sibling: Option<&Node>,
        ancestor: Option<VNode>,
    ) -> Option<Node> {
        // Reuse previous_sibling, because fragment reuse parent
        let mut previous_sibling = previous_sibling.cloned();
        let mut rights = {
            match ancestor {
                // If element matched this type
                Some(VNode::VList(vlist)) => {
                    // Previously rendered items
                    vlist.children
                }
                Some(vnode) => {
                    // Use the current node as a single fragment list
                    // and let the `apply` of `VNode` to handle it.
                    vec![vnode]
                }
                None => Vec::new(),
            }
        };

        if self.children.is_empty() && !self.elide_placeholder {
            // Without a placeholder the next element becomes first
            // and corrupts the order of rendering
            // We use empty text element to stake out a place
            let placeholder = VText::new("".into());
            self.children.push(placeholder.into());
        }

        // Process children
        let mut lefts = self.children.iter_mut();
        let mut rights_nokeys = Vec::with_capacity(rights.len());
        let mut rights_lookup = HashMap::with_capacity(rights.len());
        for r in rights.drain(..) {
            if let Some(key) = r.key() {
                if rights_lookup.contains_key(&key) {
                    log::error!("Duplicate key of {}", &key);
                } else {
                    rights_lookup.insert(key.clone(), r);
                }
            } else {
                rights_nokeys.push(r);
            }
        }
        let mut rights = rights_nokeys.drain(..);
        loop {
            match lefts.next() {
                Some(left) => {
                    if let Some(key) = &left.key() {
                        let right = rights_lookup.remove(key);
                        match right {
                            Some(right) => {
                                previous_sibling =
                                    left.apply(parent, previous_sibling.as_ref(), Some(right));
                            }
                            None => {
                                previous_sibling =
                                    left.apply(parent, previous_sibling.as_ref(), None);
                            }
                        }
                    } else {
                        match rights.next() {
                            Some(right) => {
                                previous_sibling =
                                    left.apply(parent, previous_sibling.as_ref(), Some(right));
                            }
                            None => {
                                previous_sibling = left.apply(parent, previous_sibling.as_ref(), None);
                            }
                        }
                    }
                }
                None => break,
            }
        }
        loop {
            if let Some(ref mut right) = rights.next() {
                right.detach(parent);
            } else {
                break;
            }
        }
        for right in rights_lookup.values_mut() {
            right.detach(parent);
        }
        previous_sibling
    }
}

#[cfg(test)]
mod tests {
    use crate::{html, Component, ComponentLink, Html, ShouldRender};
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    struct Comp;

    impl Component for Comp {
        type Message = ();
        type Properties = ();

        fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
            Comp
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            unimplemented!();
        }

        fn change(&mut self, _: Self::Properties) -> ShouldRender {
            unimplemented!();
        }

        fn view(&self) -> Html {
            unimplemented!();
        }
    }

    #[test]
    fn check_fragments() {
        let fragment = html! {
            <>
            </>
        };
        html! {
            <div>
                { fragment }
            </div>
        };
    }
}
