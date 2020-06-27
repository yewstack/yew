//! This module contains the implementation of a virtual component (`VComp`).

use super::{Transformer, VDiff, VNode};
use crate::html::{AnyScope, Component, ComponentUpdate, NodeRef, Scope, Scoped};
use crate::utils::document;
use cfg_if::cfg_if;
use std::any::TypeId;
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::{Element, Node};
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::{Element, Node};
    }
}

cfg_if! {
    if #[cfg(feature = "sans_mount_render")] {
        use super::{Html, HtmlRenderError};
        use std::convert::TryFrom;
    }
}

/// A virtual component.
pub struct VComp {
    type_id: TypeId,
    scope: Option<Box<dyn Scoped>>,
    props: Option<Box<dyn Mountable>>,
    pub(crate) node_ref: NodeRef,
    pub(crate) key: Option<String>,
}

impl Clone for VComp {
    fn clone(&self) -> Self {
        if self.scope.is_some() {
            panic!("Mounted components are not allowed to be cloned!");
        }

        Self {
            type_id: self.type_id,
            scope: None,
            props: self.props.as_ref().map(|m| m.copy()),
            node_ref: self.node_ref.clone(),
            key: self.key.clone(),
        }
    }
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

#[cfg(feature = "sans_mount_render")]
impl TryFrom<VComp> for Html {
    type Error = HtmlRenderError;

    fn try_from(value: VComp) -> Result<Html, HtmlRenderError> {
        let html: String = match &value.scope {
            None => "".to_string(),
            Some(scope) => match scope.root_vnode() {
                None => "".to_string(),
                Some(root_vnode) => Html::try_from(root_vnode.clone())?.to_string(),
            },
        };
        Ok(Html::new(html))
    }
}

impl VComp {
    /// Creates a new `VComp` instance.
    pub fn new<COMP>(props: COMP::Properties, node_ref: NodeRef, key: Option<String>) -> Self
    where
        COMP: Component,
    {
        VComp {
            type_id: TypeId::of::<COMP>(),
            node_ref,
            props: Some(Box::new(PropsWrapper::<COMP>::new(props))),
            scope: None,
            key,
        }
    }

    #[allow(unused)]
    pub(crate) fn root_vnode(&self) -> Option<impl Deref<Target = VNode> + '_> {
        self.scope.as_ref().and_then(|scope| scope.root_vnode())
    }
}

trait Mountable {
    fn copy(&self) -> Box<dyn Mountable>;
    fn mount(
        self: Box<Self>,
        node_ref: NodeRef,
        parent_scope: &AnyScope,
        parent: Element,
        next_sibling: NodeRef,
    ) -> Box<dyn Scoped>;
    fn reuse(self: Box<Self>, scope: &dyn Scoped, next_sibling: NodeRef);
}

struct PropsWrapper<COMP: Component> {
    props: COMP::Properties,
}

impl<COMP: Component> PropsWrapper<COMP> {
    pub fn new(props: COMP::Properties) -> Self {
        Self { props }
    }
}

impl<COMP: Component> Mountable for PropsWrapper<COMP> {
    fn copy(&self) -> Box<dyn Mountable> {
        let wrapper: PropsWrapper<COMP> = PropsWrapper {
            props: self.props.clone(),
        };
        Box::new(wrapper)
    }

    fn mount(
        self: Box<Self>,
        node_ref: NodeRef,
        parent_scope: &AnyScope,
        parent: Element,
        next_sibling: NodeRef,
    ) -> Box<dyn Scoped> {
        let scope: Scope<COMP> = Scope::new(Some(parent_scope.clone()));
        let scope = scope.mount_in_place(
            parent,
            next_sibling,
            Some(VNode::VRef(node_ref.get().unwrap())),
            node_ref,
            self.props,
        );

        Box::new(scope)
    }

    fn reuse(self: Box<Self>, scope: &dyn Scoped, next_sibling: NodeRef) {
        let scope: Scope<COMP> = scope.to_any().downcast();
        scope.update(ComponentUpdate::Properties(self.props, next_sibling), false);
    }
}

impl VDiff for VComp {
    fn detach(&mut self, _parent: &Element) {
        self.scope.take().expect("VComp is not mounted").destroy();
    }

    fn apply(
        &mut self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: Option<VNode>,
    ) -> NodeRef {
        let mountable = self.props.take().expect("VComp has already been mounted");

        if let Some(mut ancestor) = ancestor {
            if let VNode::VComp(ref mut vcomp) = &mut ancestor {
                // If the ancestor is the same type, reuse it and update its properties
                if self.type_id == vcomp.type_id {
                    self.node_ref.link(vcomp.node_ref.clone());
                    let scope = vcomp.scope.take().expect("VComp is not mounted");
                    mountable.reuse(scope.borrow(), next_sibling);
                    self.scope = Some(scope);
                    return vcomp.node_ref.clone();
                }
            }

            ancestor.detach(parent);
        }

        let placeholder: Node = document().create_text_node("").into();
        super::insert_node(&placeholder, parent, next_sibling.get());
        self.node_ref.set(Some(placeholder));
        let scope = mountable.mount(
            self.node_ref.clone(),
            parent_scope,
            parent.to_owned(),
            next_sibling,
        );
        self.scope = Some(scope);
        self.node_ref.clone()
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
    use super::*;
    use crate::macros::Properties;
    use crate::{html, Children, Component, ComponentLink, Html, NodeRef, ShouldRender};
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

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
    fn set_component_key() {
        let test_key = "test".to_string();
        let check_key = |vnode: VNode| {
            assert_eq!(vnode.key().as_ref(), Some(&test_key));
        };

        let props = Props {
            field_1: 1,
            field_2: 1,
        };
        let props_2 = props.clone();

        check_key(html! { <Comp key=test_key.clone() /> });
        check_key(html! { <Comp key=test_key.clone() field_1=1 /> });
        check_key(html! { <Comp field_1=1 key=test_key.clone() /> });
        check_key(html! { <Comp with props key=test_key.clone() /> });
        check_key(html! { <Comp key=test_key.clone() with props_2 /> });
    }

    #[test]
    fn set_component_node_ref() {
        let test_node: Node = document().create_text_node("test").into();
        let test_node_ref = NodeRef::new(test_node);
        let check_node_ref = |vnode: VNode| {
            assert_eq!(vnode.first_node(), test_node_ref.get().unwrap());
        };

        let props = Props {
            field_1: 1,
            field_2: 1,
        };
        let props_2 = props.clone();

        check_node_ref(html! { <Comp ref=test_node_ref.clone() /> });
        check_node_ref(html! { <Comp ref=test_node_ref.clone() field_1=1 /> });
        check_node_ref(html! { <Comp field_1=1 ref=test_node_ref.clone() /> });
        check_node_ref(html! { <Comp with props ref=test_node_ref.clone() /> });
        check_node_ref(html! { <Comp ref=test_node_ref.clone() with props_2 /> });
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

    #[derive(Clone, Properties)]
    pub struct ListProps {
        pub children: Children,
    }
    pub struct List(ListProps);
    impl Component for List {
        type Message = ();
        type Properties = ListProps;

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

    #[cfg(feature = "web_sys")]
    use super::{AnyScope, Element};

    #[cfg(feature = "web_sys")]
    fn setup_parent() -> (AnyScope, Element) {
        let scope = AnyScope {
            type_id: std::any::TypeId::of::<()>(),
            parent: None,
            state: std::rc::Rc::new(()),
        };
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        (scope, parent)
    }

    #[cfg(feature = "web_sys")]
    fn get_html(mut node: Html, scope: &AnyScope, parent: &Element) -> String {
        // clear parent
        parent.set_inner_html("");

        node.apply(&scope, &parent, NodeRef::default(), None);
        parent.inner_html()
    }

    #[test]
    #[cfg(feature = "web_sys")]
    fn all_ways_of_passing_children_work() {
        let (scope, parent) = setup_parent();

        let children: Vec<_> = vec!["a", "b", "c"]
            .drain(..)
            .map(|text| html! {<span>{ text }</span>})
            .collect();
        let children_renderer = Children::new(children.clone());
        let expected_html = "\
        <ul>\
            <li><span>a</span></li>\
            <li><span>b</span></li>\
            <li><span>c</span></li>\
        </ul>";

        let prop_method = html! {
            <List children=children_renderer.clone()/>
        };
        assert_eq!(get_html(prop_method, &scope, &parent), expected_html);

        let children_renderer_method = html! {
            <List>
                { children_renderer.clone() }
            </List>
        };
        assert_eq!(
            get_html(children_renderer_method, &scope, &parent),
            expected_html
        );

        let direct_method = html! {
            <List>
                { children.clone() }
            </List>
        };
        assert_eq!(get_html(direct_method, &scope, &parent), expected_html);

        let for_method = html! {
            <List>
                { for children }
            </List>
        };
        assert_eq!(get_html(for_method, &scope, &parent), expected_html);
    }
}

#[cfg(all(test, feature = "web_sys"))]
mod layout_tests {
    use crate::virtual_dom::layout_tests::{diff_layouts, TestLayout};
    use crate::{Children, Component, ComponentLink, Html, Properties, ShouldRender};
    use std::marker::PhantomData;

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    struct Comp<T> {
        _marker: PhantomData<T>,
        props: CompProps,
    }

    #[derive(Properties, Clone)]
    struct CompProps {
        #[prop_or_default]
        children: Children,
    }

    impl<T: 'static> Component for Comp<T> {
        type Message = ();
        type Properties = CompProps;

        fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
            Comp {
                _marker: PhantomData::default(),
                props,
            }
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            unimplemented!();
        }

        fn change(&mut self, props: Self::Properties) -> ShouldRender {
            self.props = props;
            true
        }

        fn view(&self) -> Html {
            html! {
                <>{ self.props.children.clone() }</>
            }
        }
    }

    struct A;
    struct B;

    #[test]
    fn diff() {
        let layout1 = TestLayout {
            node: html! {
                <Comp<A>>
                    <Comp<B>></Comp<B>>
                    {"C"}
                </Comp<A>>
            },
            expected: "C",
        };

        let layout2 = TestLayout {
            node: html! {
                <Comp<A>>
                    {"A"}
                </Comp<A>>
            },
            expected: "A",
        };

        let layout3 = TestLayout {
            node: html! {
                <Comp<B>>
                    <Comp<A>></Comp<A>>
                    {"B"}
                </Comp<B>>
            },
            expected: "B",
        };

        let layout4 = TestLayout {
            node: html! {
                <Comp<B>>
                    <Comp<A>>{"A"}</Comp<A>>
                    {"B"}
                </Comp<B>>
            },
            expected: "AB",
        };

        let layout5 = TestLayout {
            node: html! {
                <Comp<B>>
                    <>
                        <Comp<A>>
                            {"A"}
                        </Comp<A>>
                    </>
                    {"B"}
                </Comp<B>>
            },
            expected: "AB",
        };

        let layout6 = TestLayout {
            node: html! {
                <Comp<B>>
                    <>
                        <Comp<A>>
                            {"A"}
                        </Comp<A>>
                        {"B"}
                    </>
                    {"C"}
                </Comp<B>>
            },
            expected: "ABC",
        };

        let layout7 = TestLayout {
            node: html! {
                <Comp<B>>
                    <>
                        <Comp<A>>
                            {"A"}
                        </Comp<A>>
                        <Comp<A>>
                            {"B"}
                        </Comp<A>>
                    </>
                    {"C"}
                </Comp<B>>
            },
            expected: "ABC",
        };

        let layout8 = TestLayout {
            node: html! {
                <Comp<B>>
                    <>
                        <Comp<A>>
                            {"A"}
                        </Comp<A>>
                        <Comp<A>>
                            <Comp<A>>
                                {"B"}
                            </Comp<A>>
                        </Comp<A>>
                    </>
                    {"C"}
                </Comp<B>>
            },
            expected: "ABC",
        };

        let layout9 = TestLayout {
            node: html! {
                <Comp<B>>
                    <>
                        <>
                            {"A"}
                        </>
                        <Comp<A>>
                            <Comp<A>>
                                {"B"}
                            </Comp<A>>
                        </Comp<A>>
                    </>
                    {"C"}
                </Comp<B>>
            },
            expected: "ABC",
        };

        let layout10 = TestLayout {
            node: html! {
                <Comp<B>>
                    <>
                        <Comp<A>>
                            <Comp<A>>
                                {"A"}
                            </Comp<A>>
                        </Comp<A>>
                        <>
                            {"B"}
                        </>
                    </>
                    {"C"}
                </Comp<B>>
            },
            expected: "ABC",
        };

        let layout11 = TestLayout {
            node: html! {
                <Comp<B>>
                    <>
                        <>
                            <Comp<A>>
                                <Comp<A>>
                                    {"A"}
                                </Comp<A>>
                                {"B"}
                            </Comp<A>>
                        </>
                    </>
                    {"C"}
                </Comp<B>>
            },
            expected: "ABC",
        };

        let layout12 = TestLayout {
            node: html! {
                <Comp<B>>
                    <>
                        <Comp<A>></Comp<A>>
                        <>
                            <Comp<A>>
                                <>
                                    <Comp<A>>
                                        {"A"}
                                    </Comp<A>>
                                    <></>
                                    <Comp<A>>
                                        <Comp<A>></Comp<A>>
                                        <></>
                                        {"B"}
                                        <></>
                                        <Comp<A>></Comp<A>>
                                    </Comp<A>>
                                </>
                            </Comp<A>>
                            <></>
                        </>
                        <Comp<A>></Comp<A>>
                    </>
                    {"C"}
                    <Comp<A>></Comp<A>>
                    <></>
                </Comp<B>>
            },
            expected: "ABC",
        };

        diff_layouts(vec![
            layout1, layout2, layout3, layout4, layout5, layout6, layout7, layout8, layout9,
            layout10, layout11, layout12,
        ]);
    }
}
