//! This module contains fragments implementation.
use super::{VDiff, VNode, VText};
use crate::html::AnyScope;
use cfg_if::cfg_if;
use std::collections::HashSet;
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
    pub key: Option<String>,
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
    pub fn new_with_children(children: Vec<VNode>, key: Option<String>) -> Self {
        VList { children, key }
    }

    /// Add `VNode` child.
    pub fn add_child(&mut self, child: VNode) {
        self.children.push(child);
    }
}

impl VDiff for VList {
    fn detach(&mut self, parent: &Element) {
        for mut child in self.children.drain(..) {
            child.detach(parent);
        }
    }

    fn apply(
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        mut next_sibling: Option<Node>,
        ancestor: Option<VNode>,
    ) -> Node {
        let ancestor_children = {
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

        if self.children.is_empty() {
            // Without a placeholder the next element becomes first
            // and corrupts the order of rendering
            // We use empty text element to stake out a place
            let placeholder = VText::new("".into());
            self.children.push(placeholder.into());
        }

        // Check for lefts to see if there are duplicates and show a warning.
        {
            let mut hash_set = HashSet::with_capacity(self.children.len());
            for l in self.children.iter() {
                if let Some(k) = l.key() {
                    if !hash_set.insert(k) {
                        log::error!("Duplicate key of {}", &k);
                    }
                }
            }

            // This warning should be removed in https://github.com/yewstack/yew/pull/1231
            if !hash_set.is_empty() {
                log::warn!("Keys currently have no effect");
            }
        }

        let ancestor_len = ancestor_children.len();
        let children_len = self.children.len();

        // Detach rights until length equals lefts.len()
        let mut rights = ancestor_children.into_iter().rev();
        let extra_rights = ancestor_len.saturating_sub(children_len);
        for _ in 0..extra_rights {
            rights.next().unwrap().detach(parent);
        }

        // Apply lefts until length equals rights.len()
        let mut lefts = self.children.iter_mut().rev();
        let extra_lefts = children_len.saturating_sub(ancestor_len);
        for _ in 0..extra_lefts {
            next_sibling = Some(lefts.next().unwrap().apply(
                parent_scope,
                parent,
                next_sibling,
                None,
            ));
        }

        // Diff children in reverse order to ensure next_sibling is updated properly
        for left in lefts {
            next_sibling = Some(left.apply(parent_scope, parent, next_sibling, rights.next()));
        }

        next_sibling.expect("VList should have at least one child")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

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

// stdweb doesn't have `inner_html` method
#[cfg(all(test, feature = "web_sys"))]
mod web_sys_tests {
    use super::{Element, Node};
    use crate::html::{AnyScope, Scope};
    use crate::prelude::*;
    use crate::virtual_dom::{VChild, VDiff, VList, VNode, VTag, VText};
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    struct Comp {
        props: CompProps,
    }

    #[derive(Properties, Clone)]
    struct CompProps {
        inner: Html,
    }

    impl Component for Comp {
        type Message = ();
        type Properties = CompProps;

        fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
            Comp { props }
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            unimplemented!();
        }

        fn change(&mut self, _: Self::Properties) -> ShouldRender {
            true
        }

        fn view(&self) -> Html {
            self.props.inner.clone()
        }
    }

    #[test]
    fn vlist_vdiff_apply_non_keyed_from_none_works_with_all_vnode_types_as_children() {
        let scheduler = yew::scheduler::scheduler();

        let vchild_with_tag: VChild<Comp> = VChild::new(
            CompProps {
                inner: html! { <p>{0}</p> },
            },
            NodeRef::default(),
            None,
        );

        let vchild_with_list: VChild<Comp> = VChild::new(
            CompProps {
                inner: html! { <>{"(list)"}</> },
            },
            NodeRef::default(),
            None,
        );

        let vchild_with_comp: VChild<Comp> = VChild::new(
            CompProps {
                inner: html! { <Comp inner=html!{ "COMP" } /> },
            },
            NodeRef::default(),
            None,
        );

        let element: Element = crate::utils::document().create_element("br").unwrap();
        let vchild_with_ref: VChild<Comp> = VChild::new(
            CompProps {
                inner: html! { <Comp inner=VNode::VRef(element.clone().into()) /> },
            },
            NodeRef::default(),
            None,
        );

        let vchild_empty: VChild<Comp> =
            VChild::new(CompProps { inner: html! {} }, NodeRef::default(), None);

        let vref_element: Element = crate::utils::document().create_element("i").unwrap();
        let vref_node: Node = vref_element.clone().into();
        let mut vtag = VTag::new("span");
        vtag.children.add_child(VTag::new("hr").into());
        let mut vlist = VList::new_with_children(
            vec![
                VNode::VText(VText::new("a".into())),
                VNode::VTag(Box::new(vtag)),
                VNode::VComp(vchild_with_ref.into()),
                VNode::VText(VText::new("c".into())),
                VNode::VComp(vchild_with_list.into()),
                VNode::VText(VText::new("d".into())),
                VNode::VComp(vchild_with_tag.into()),
                VNode::VComp(vchild_empty.into()),
                VNode::VList(VList::new_with_children(
                    vec![
                        VNode::VText(VText::new("foo".into())),
                        VNode::VComp(vchild_with_comp.into()),
                        VNode::VText(VText::new("bar".into())),
                    ],
                    None,
                )),
                VNode::VRef(vref_node),
            ],
            None,
        );

        let mut vlist_copy = vlist.clone();
        let parent_scope: AnyScope = Scope::<Comp>::new(None).into();
        let parent_element = crate::utils::document().create_element("div").unwrap();

        let scheduler_lock = scheduler.lock();
        vlist.apply(&parent_scope, &parent_element, None, None);
        drop(scheduler_lock);
        scheduler.start();

        assert_eq!(
            parent_element.inner_html(),
            "a<span><hr></span><br>c(list)d<p>0</p>fooCOMPbar<i></i>",
            "The VList didn't render properly."
        );

        let scheduler_lock = scheduler.lock();
        vlist_copy.apply(
            &parent_scope,
            &parent_element,
            None,
            Some(VNode::VList(vlist)),
        );
        drop(scheduler_lock);
        scheduler.start();

        assert_eq!(
            parent_element.inner_html(),
            "a<span><hr></span><br>c(list)d<p>0</p>fooCOMPbar<i></i>",
            "The VList didn't render properly."
        );
    }
}
