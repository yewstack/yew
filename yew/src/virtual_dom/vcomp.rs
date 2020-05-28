//! This module contains the implementation of a virtual component `VComp`.

use super::{Transformer, VDiff, VNode};
use crate::html::{AnyScope, Component, ComponentUpdate, NodeRef, Scope};
use crate::utils::document;
use cfg_if::cfg_if;
use std::any::TypeId;
use std::fmt;
use std::mem::swap;
use std::rc::Rc;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Element, INode, Node, TextNode};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::{Element, Node, Text as TextNode};
    }
}

/// The method generates an instance of a component.
type Generator = dyn Fn(GeneratorType) -> Mounted;

/// Components can be generated by mounting or by overwriting an old component.
enum GeneratorType {
    Mount(AnyScope, Element, TextNode),
    Overwrite(AnyScope),
}

/// A virtual component.
#[derive(Clone)]
pub struct VComp {
    type_id: TypeId,
    state: MountState,
    pub(crate) node_ref: NodeRef,
    pub(crate) key: Option<String>,
}

/// A virtual child component.
pub struct VChild<COMP: Component> {
    /// The component properties
    pub props: COMP::Properties,
    /// Reference to the mounted node
    node_ref: NodeRef,
    key: Option<String>,
}

impl<COMP: Component> Clone for VChild<COMP> {
    fn clone(&self) -> Self {
        VChild {
            props: self.props.clone(),
            node_ref: self.node_ref.clone(),
            key: self.key.clone(),
        }
    }
}

impl<COMP: Component> PartialEq for VChild<COMP>
where
    COMP::Properties: PartialEq,
{
    fn eq(&self, other: &VChild<COMP>) -> bool {
        self.props == other.props
    }
}

impl<COMP> VChild<COMP>
where
    COMP: Component,
{
    /// Creates a child component that can be accessed and modified by its parent.
    pub fn new(props: COMP::Properties, node_ref: NodeRef, key: Option<String>) -> Self {
        Self {
            props,
            node_ref,
            key,
        }
    }
}

impl<COMP> From<VChild<COMP>> for VComp
where
    COMP: Component,
{
    fn from(vchild: VChild<COMP>) -> Self {
        VComp::new::<COMP>(vchild.props, vchild.node_ref, vchild.key)
    }
}

#[derive(Clone)]
enum MountState {
    Unmounted(Unmounted),
    Mounted(Mounted),
    Mounting,
    Detached,
    Overwritten,
}

#[derive(Clone)]
struct Unmounted {
    generator: Rc<Generator>,
}

struct Mounted {
    node_ref: NodeRef,
    scope: AnyScope,
    destroyer: Box<dyn FnOnce()>,
}

impl Clone for Mounted {
    fn clone(&self) -> Self {
        panic!("Mounted components are not allowed to be cloned!")
    }
}

impl VComp {
    /// This method prepares a generator to make a new instance of the `Component`.
    pub fn new<COMP>(props: COMP::Properties, node_ref: NodeRef, key: Option<String>) -> Self
    where
        COMP: Component,
    {
        let node_ref_clone = node_ref.clone();
        let generator = move |generator_type: GeneratorType| -> Mounted {
            match generator_type {
                GeneratorType::Mount(parent_scope, element, dummy_node) => {
                    let scope: Scope<COMP> = Scope::new(Some(parent_scope));

                    let mut scope = scope.mount_in_place(
                        element,
                        Some(VNode::VRef(dummy_node.into())),
                        node_ref_clone.clone(),
                        props.clone(),
                    );

                    Mounted {
                        node_ref: node_ref_clone.clone(),
                        scope: scope.clone().into(),
                        destroyer: Box::new(move || scope.destroy()),
                    }
                }
                GeneratorType::Overwrite(any_scope) => {
                    let mut scope: Scope<COMP> = any_scope.downcast();
                    scope.update(
                        ComponentUpdate::Properties(props.clone(), node_ref_clone.clone()),
                        false,
                    );

                    Mounted {
                        node_ref: node_ref_clone.clone(),
                        scope: scope.clone().into(),
                        destroyer: Box::new(move || scope.destroy()),
                    }
                }
            }
        };

        VComp {
            type_id: TypeId::of::<COMP>(),
            state: MountState::Unmounted(Unmounted {
                generator: Rc::new(generator),
            }),
            node_ref,
            key,
        }
    }
}

impl Unmounted {
    /// Mount a virtual component using a generator.
    fn mount(self, parent_scope: AnyScope, parent: Element, dummy_node: TextNode) -> Mounted {
        (self.generator)(GeneratorType::Mount(parent_scope, parent, dummy_node))
    }

    /// Overwrite an existing virtual component using a generator.
    fn replace(self, old: Mounted) -> Mounted {
        (self.generator)(GeneratorType::Overwrite(old.scope))
    }
}

enum Reform {
    Keep(Mounted),
    Before(Option<Node>),
}

impl VDiff for VComp {
    fn detach(&mut self, _parent: &Element) -> Option<Node> {
        let mut replace_state = MountState::Detached;
        swap(&mut replace_state, &mut self.state);
        match replace_state {
            MountState::Mounted(this) => {
                (this.destroyer)();
                this.node_ref.get().and_then(|node| node.next_sibling())
            }
            _ => None,
        }
    }

    fn apply(
        &mut self,
        scope: &AnyScope,
        parent: &Element,
        previous_sibling: Option<&Node>,
        ancestor: Option<VNode>,
    ) -> Option<Node> {
        let mut replace_state = MountState::Mounting;
        swap(&mut replace_state, &mut self.state);
        if let MountState::Unmounted(this) = replace_state {
            let reform = match ancestor {
                Some(VNode::VComp(mut vcomp)) => {
                    // If the ancestor is a Component of the same type, don't replace, keep the
                    // old Component but update the properties.
                    if self.type_id == vcomp.type_id {
                        let mut replace_state = MountState::Overwritten;
                        swap(&mut replace_state, &mut vcomp.state);
                        match replace_state {
                            MountState::Mounted(mounted) => Reform::Keep(mounted),
                            _ => Reform::Before(None),
                        }
                    } else {
                        Reform::Before(vcomp.detach(parent))
                    }
                }
                Some(mut vnode) => Reform::Before(vnode.detach(parent)),
                None => Reform::Before(None),
            };

            let mounted = match reform {
                Reform::Keep(mounted) => {
                    // Send properties update when the component is already rendered.
                    this.replace(mounted)
                }
                Reform::Before(next_sibling) => {
                    let dummy_node = document().create_text_node("");
                    if let Some(next_sibling) = next_sibling {
                        let next_sibling = &next_sibling;
                        #[cfg(feature = "web_sys")]
                        let next_sibling = Some(next_sibling);
                        parent
                            .insert_before(&dummy_node, next_sibling)
                            .expect("can't insert dummy component node before next sibling");
                    } else if let Some(next_sibling) =
                        previous_sibling.and_then(|p| p.next_sibling())
                    {
                        let next_sibling = &next_sibling;
                        #[cfg(feature = "web_sys")]
                        let next_sibling = Some(next_sibling);
                        parent
                            .insert_before(&dummy_node, next_sibling)
                            .expect("can't insert dummy component node before next sibling");
                    } else {
                        #[cfg_attr(
                            feature = "std_web",
                            allow(clippy::let_unit_value, unused_variables)
                        )]
                        {
                            let result = parent.append_child(&dummy_node);
                            #[cfg(feature = "web_sys")]
                            result.expect("can't append node to parent");
                        }
                    }
                    this.mount(scope.clone(), parent.to_owned(), dummy_node)
                }
            };

            self.state = MountState::Mounted(mounted);
        }
        None
    }
}

impl<T> Transformer<T, T> for VComp {
    fn transform(from: T) -> T {
        from
    }
}

impl<'a, T> Transformer<&'a T, T> for VComp
where
    T: Clone,
{
    fn transform(from: &'a T) -> T {
        from.clone()
    }
}

impl<'a> Transformer<&'a str, String> for VComp {
    fn transform(from: &'a str) -> String {
        from.to_owned()
    }
}

impl<T> Transformer<T, Option<T>> for VComp {
    fn transform(from: T) -> Option<T> {
        Some(from)
    }
}

impl<'a, T> Transformer<&'a T, Option<T>> for VComp
where
    T: Clone,
{
    fn transform(from: &T) -> Option<T> {
        Some(from.clone())
    }
}

impl<'a> Transformer<&'a str, Option<String>> for VComp {
    fn transform(from: &'a str) -> Option<String> {
        Some(from.to_owned())
    }
}

impl<'a> Transformer<Option<&'a str>, Option<String>> for VComp {
    fn transform(from: Option<&'a str>) -> Option<String> {
        from.map(|s| s.to_owned())
    }
}

impl PartialEq for VComp {
    fn eq(&self, other: &VComp) -> bool {
        self.type_id == other.type_id
    }
}

impl fmt::Debug for VComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("VComp")
    }
}

impl<COMP: Component> fmt::Debug for VChild<COMP> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("VChild<_>")
    }
}

#[cfg(test)]
mod tests {
    use super::{AnyScope, VChild, VDiff};
    use crate::macros::Properties;
    use crate::utils::document;
    use crate::{html, Children, Component, ComponentLink, Html, NodeRef, ShouldRender};
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    fn test_scope() -> AnyScope {
        AnyScope {
            type_id: std::any::TypeId::of::<()>(),
            parent: None,
            state: std::rc::Rc::new(()),
        }
    }

    struct Comp;

    #[derive(Clone, PartialEq, Properties)]
    struct Props {
        #[prop_or_default]
        field_1: u32,
        #[prop_or_default]
        field_2: u32,
    }

    impl Component for Comp {
        type Message = ();
        type Properties = Props;

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
    fn set_properties_to_component() {
        html! {
            <Comp />
        };

        html! {
            <Comp field_1=1 />
        };

        html! {
            <Comp field_2=2 />
        };

        html! {
            <Comp field_1=1 field_2=2 />
        };

        let props = Props {
            field_1: 1,
            field_2: 1,
        };

        html! {
            <Comp with props />
        };
    }

    #[test]
    fn vchild_partialeq() {
        let vchild1: VChild<Comp> = VChild::new(
            Props {
                field_1: 1,
                field_2: 1,
            },
            NodeRef::default(),
            None,
        );

        let vchild2: VChild<Comp> = VChild::new(
            Props {
                field_1: 1,
                field_2: 1,
            },
            NodeRef::default(),
            None,
        );

        let vchild3: VChild<Comp> = VChild::new(
            Props {
                field_1: 2,
                field_2: 2,
            },
            NodeRef::default(),
            None,
        );

        assert_eq!(vchild1, vchild2);
        assert_ne!(vchild1, vchild3);
        assert_ne!(vchild2, vchild3);
    }

    #[test]
    fn iterators_produce_same_results() {
        #[derive(Clone, Properties)]
        struct Props {
            children: Children,
        }
        struct List(Props);
        impl Component for List {
            type Message = ();
            type Properties = Props;

            fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
                Self(props)
            }

            fn update(&mut self, _: Self::Message) -> ShouldRender {
                unimplemented!();
            }

            fn change(&mut self, _: Self::Properties) -> ShouldRender {
                unimplemented!();
            }

            fn view(&self) -> Html {
                let item_iter = self.0.children.iter().map(|item| html! {<li>{ item }</li>});
                html! {
                    <ul>{ for item_iter }</ul>
                }
            }
        }
        let children: Vec<_> = vec!["a", "b", "c"]
            .drain(..)
            .map(|text| html! {<span>{ text }</span>})
            .collect();

        let scope = test_scope();
        let parent = document().create_element("div").unwrap();

        #[cfg(feature = "std_web")]
        document().body().unwrap().append_child(&parent);
        #[cfg(feature = "web_sys")]
        document().body().unwrap().append_child(&parent).unwrap();

        let mut list_method = html! {
            <List>
                { children.clone() }
            </List>
        };
        list_method.apply(&scope, &parent, None, None);
        let list_html = parent.inner_html();
        assert_eq!(
            list_html,
            "<ul>\
                <li><span>a</span></li>\
                <li><span>b</span></li>\
                <li><span>c</span></li>\
            </ul>"
        );

        parent.set_inner_html("");

        let mut for_method = html! {
            <List>
                { for children }
            </List>
        };
        for_method.apply(&scope, &parent, None, None);
        let for_html = parent.inner_html();

        assert_eq!(list_html, for_html)
    }
}
