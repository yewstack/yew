//! This module contains the bundle implementation of a virtual component [BComp].

use super::{insert_node, BNode, BundleRoot, DomBundle, Reconcilable};
use crate::html::{AnyScope, BaseComponent, Scope};
use crate::virtual_dom::{Key, VComp, VNode};
use crate::NodeRef;
#[cfg(feature = "ssr")]
use futures::channel::oneshot;
#[cfg(feature = "ssr")]
use futures::future::{FutureExt, LocalBoxFuture};
use gloo_utils::document;
use std::cell::Ref;
use std::{any::TypeId, borrow::Borrow};
use std::{fmt, rc::Rc};
use web_sys::{Element, Node};

/// A virtual component. Compare with [VComp].
pub struct BComp {
    type_id: TypeId,
    scope: Box<dyn Scoped>,
    node_ref: NodeRef,
    key: Option<Key>,
}

impl BComp {
    /// Get the key of the underlying component
    pub(super) fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }
}

impl fmt::Debug for BComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BComp {{ root: {:?} }}",
            self.scope.as_ref().render_state(),
        )
    }
}

impl DomBundle for BComp {
    fn detach(self, _root: &BundleRoot, _parent: &Element, parent_to_detach: bool) {
        self.scope.destroy_boxed(parent_to_detach);
    }

    fn shift(&self, next_root: &BundleRoot, next_parent: &Element, next_sibling: NodeRef) {
        self.scope
            .shift_node(next_root, next_parent.clone(), next_sibling);
    }
}

impl Reconcilable for VComp {
    type Bundle = BComp;

    fn attach(
        self,
        root: &BundleRoot,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let VComp {
            type_id,
            mountable,
            node_ref,
            key,
        } = self;

        let scope = mountable.mount(
            node_ref.clone(),
            root,
            parent_scope,
            parent.to_owned(),
            next_sibling,
        );

        (
            node_ref.clone(),
            BComp {
                type_id,
                node_ref,
                key,
                scope,
            },
        )
    }

    fn reconcile_node(
        self,
        root: &BundleRoot,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        bundle: &mut BNode,
    ) -> NodeRef {
        match bundle {
            // If the existing bundle is the same type, reuse it and update its properties
            BNode::Comp(ref mut bcomp)
                if self.type_id == bcomp.type_id && self.key == bcomp.key =>
            {
                self.reconcile(root, parent_scope, parent, next_sibling, bcomp)
            }
            _ => self.replace(root, parent_scope, parent, next_sibling, bundle),
        }
    }

    fn reconcile(
        self,
        _root: &BundleRoot,
        _parent_scope: &AnyScope,
        _parent: &Element,
        next_sibling: NodeRef,
        bcomp: &mut Self::Bundle,
    ) -> NodeRef {
        let VComp {
            mountable,
            node_ref,
            key,
            type_id: _,
        } = self;

        bcomp.key = key;
        let old_ref = std::mem::replace(&mut bcomp.node_ref, node_ref.clone());
        bcomp.node_ref.reuse(old_ref);
        mountable.reuse(node_ref.clone(), bcomp.scope.borrow(), next_sibling);
        node_ref
    }
}

pub trait Mountable {
    fn copy(&self) -> Box<dyn Mountable>;
    fn mount(
        self: Box<Self>,
        node_ref: NodeRef,
        root: &BundleRoot,
        parent_scope: &AnyScope,
        parent: Element,
        next_sibling: NodeRef,
    ) -> Box<dyn Scoped>;
    fn reuse(self: Box<Self>, node_ref: NodeRef, scope: &dyn Scoped, next_sibling: NodeRef);

    #[cfg(feature = "ssr")]
    fn render_to_string<'a>(
        &'a self,
        w: &'a mut String,
        parent_scope: &'a AnyScope,
    ) -> LocalBoxFuture<'a, ()>;
}

pub struct PropsWrapper<COMP: BaseComponent> {
    props: Rc<COMP::Properties>,
}

impl<COMP: BaseComponent> PropsWrapper<COMP> {
    pub fn new(props: Rc<COMP::Properties>) -> Self {
        Self { props }
    }
}

impl<COMP: BaseComponent> Mountable for PropsWrapper<COMP> {
    fn copy(&self) -> Box<dyn Mountable> {
        let wrapper: PropsWrapper<COMP> = PropsWrapper {
            props: Rc::clone(&self.props),
        };
        Box::new(wrapper)
    }

    fn mount(
        self: Box<Self>,
        node_ref: NodeRef,
        root: &BundleRoot,
        parent_scope: &AnyScope,
        parent: Element,
        next_sibling: NodeRef,
    ) -> Box<dyn Scoped> {
        let scope: Scope<COMP> = Scope::new(Some(parent_scope.clone()));
        let initial_render_state =
            ComponentRenderState::new(root.clone(), parent, next_sibling, &node_ref);
        scope.mount_in_place(initial_render_state, node_ref, self.props);

        Box::new(scope)
    }

    fn reuse(self: Box<Self>, node_ref: NodeRef, scope: &dyn Scoped, next_sibling: NodeRef) {
        let scope: Scope<COMP> = scope.to_any().downcast::<COMP>();
        scope.reuse(self.props, node_ref, next_sibling);
    }

    #[cfg(feature = "ssr")]
    fn render_to_string<'a>(
        &'a self,
        w: &'a mut String,
        parent_scope: &'a AnyScope,
    ) -> LocalBoxFuture<'a, ()> {
        async move {
            let scope: Scope<COMP> = Scope::new(Some(parent_scope.clone()));
            scope.render_to_string(w, self.props.clone()).await;
        }
        .boxed_local()
    }
}

pub struct ComponentRenderState {
    hosting_root: BundleRoot,
    view_node: BNode,
    /// When a component has no parent, it means that it should not be rendered.
    parent: Option<Element>,
    next_sibling: NodeRef,

    #[cfg(feature = "ssr")]
    html_sender: Option<oneshot::Sender<VNode>>,
}

impl std::fmt::Debug for ComponentRenderState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.view_node.fmt(f)
    }
}

impl ComponentRenderState {
    /// Prepare a place in the DOM to hold the eventual [VNode] from rendering a component
    pub(crate) fn new(
        hosting_root: BundleRoot,
        parent: Element,
        next_sibling: NodeRef,
        node_ref: &NodeRef,
    ) -> Self {
        let placeholder = {
            let placeholder: Node = document().create_text_node("").into();
            insert_node(&placeholder, &parent, next_sibling.get().as_ref());
            node_ref.set(Some(placeholder.clone()));
            BNode::Ref(placeholder)
        };
        Self {
            hosting_root,
            view_node: placeholder,
            parent: Some(parent),
            next_sibling,
            #[cfg(feature = "ssr")]
            html_sender: None,
        }
    }
    /// Set up server-side rendering of a component
    #[cfg(feature = "ssr")]
    pub(crate) fn new_ssr(tx: oneshot::Sender<VNode>) -> Self {
        use super::blist::BList;

        Self {
            hosting_root: BundleRoot::create_ssr(),
            view_node: BNode::List(BList::new()),
            parent: None,
            next_sibling: NodeRef::default(),
            html_sender: Some(tx),
        }
    }
    /// Reuse the render state, asserting a new next_sibling
    pub(crate) fn reuse(&mut self, next_sibling: NodeRef) {
        self.next_sibling = next_sibling;
    }
    /// Shift the rendered content to a new DOM position
    pub(crate) fn shift(
        &mut self,
        next_root: &BundleRoot,
        new_parent: Element,
        next_sibling: NodeRef,
    ) {
        self.view_node
            .shift(next_root, &new_parent, next_sibling.clone());

        self.hosting_root = next_root.clone();
        self.parent = Some(new_parent);
        self.next_sibling = next_sibling;
    }
    /// Reconcile the rendered content with a new [VNode]
    pub(crate) fn reconcile(&mut self, view: VNode, scope: &AnyScope) -> NodeRef {
        if let Some(ref parent) = self.parent {
            let next_sibling = self.next_sibling.clone();

            view.reconcile_node(
                &self.hosting_root,
                scope,
                parent,
                next_sibling,
                &mut self.view_node,
            )
        } else {
            #[cfg(feature = "ssr")]
            if let Some(tx) = self.html_sender.take() {
                tx.send(view).unwrap();
            }
            NodeRef::default()
        }
    }
    /// Detach the rendered content from the DOM
    pub(crate) fn detach(self, parent_to_detach: bool) {
        if let Some(ref m) = self.parent {
            self.view_node
                .detach(&self.hosting_root, m, parent_to_detach);
        }
    }

    pub(crate) fn should_trigger_rendered(&self) -> bool {
        self.parent.is_some()
    }
}

pub trait Scoped {
    fn to_any(&self) -> AnyScope;
    /// Get the render state if it hasn't already been destroyed
    fn render_state(&self) -> Option<Ref<'_, ComponentRenderState>>;
    /// Shift the node associated with this scope to a new place
    fn shift_node(&self, next_root: &BundleRoot, parent: Element, next_sibling: NodeRef);
    /// Process an event to destroy a component
    fn destroy(self, parent_to_detach: bool);
    fn destroy_boxed(self: Box<Self>, parent_to_detach: bool);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom_bundle::{DomBundle, Reconcilable};
    use crate::scheduler;
    use crate::{
        html,
        virtual_dom::{Key, VChild, VNode},
        Children, Component, Context, Html, NodeRef, Properties,
    };
    use gloo_utils::document;
    use std::ops::Deref;
    use web_sys::Element;
    use web_sys::Node;

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

        fn create(_: &Context<Self>) -> Self {
            Comp
        }

        fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
            unimplemented!();
        }

        fn view(&self, _ctx: &Context<Self>) -> Html {
            html! { <div/> }
        }
    }

    #[test]
    fn update_loop() {
        let (root, scope, parent) = setup_parent();

        let comp = html! { <Comp></Comp> };
        let (_, mut bundle) = comp.attach(&root, &scope, &parent, NodeRef::default());
        scheduler::start_now();

        for _ in 0..10000 {
            let node = html! { <Comp></Comp> };
            node.reconcile_node(&root, &scope, &parent, NodeRef::default(), &mut bundle);
            scheduler::start_now();
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
            <Comp ..props />
        };
    }

    #[test]
    fn set_component_key() {
        let test_key: Key = "test".to_string().into();
        let check_key = |vnode: VNode| {
            assert_eq!(vnode.key(), Some(&test_key));
        };

        let props = Props {
            field_1: 1,
            field_2: 1,
        };
        let props_2 = props.clone();

        check_key(html! { <Comp key={test_key.clone()} /> });
        check_key(html! { <Comp key={test_key.clone()} field_1=1 /> });
        check_key(html! { <Comp field_1=1 key={test_key.clone()} /> });
        check_key(html! { <Comp key={test_key.clone()} ..props /> });
        check_key(html! { <Comp key={test_key.clone()} ..props_2 /> });
    }

    #[test]
    fn set_component_node_ref() {
        let test_node: Node = document().create_text_node("test").into();
        let test_node_ref = NodeRef::new(test_node);
        let check_node_ref = |vnode: VNode| {
            let vcomp = match vnode {
                VNode::VComp(vcomp) => vcomp,
                _ => unreachable!("should be a vcomp"),
            };
            assert_eq!(vcomp.node_ref, test_node_ref);
        };

        let props = Props {
            field_1: 1,
            field_2: 1,
        };
        let props_2 = props.clone();

        check_node_ref(html! { <Comp ref={test_node_ref.clone()} /> });
        check_node_ref(html! { <Comp ref={test_node_ref.clone()} field_1=1 /> });
        check_node_ref(html! { <Comp field_1=1 ref={test_node_ref.clone()} /> });
        check_node_ref(html! { <Comp ref={test_node_ref.clone()} ..props /> });
        check_node_ref(html! { <Comp ref={test_node_ref.clone()} ..props_2 /> });
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

    #[derive(Clone, Properties, PartialEq)]
    pub struct ListProps {
        pub children: Children,
    }
    pub struct List;
    impl Component for List {
        type Message = ();
        type Properties = ListProps;

        fn create(_: &Context<Self>) -> Self {
            Self
        }
        fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
            unimplemented!();
        }
        fn changed(&mut self, _ctx: &Context<Self>) -> bool {
            unimplemented!();
        }
        fn view(&self, ctx: &Context<Self>) -> Html {
            let item_iter = ctx
                .props()
                .children
                .iter()
                .map(|item| html! {<li>{ item }</li>});
            html! {
                <ul>{ for item_iter }</ul>
            }
        }
    }

    fn setup_parent() -> (BundleRoot, AnyScope, Element) {
        let scope = AnyScope::test();
        let parent = document().create_element("div").unwrap();
        let root = BundleRoot::create_root(&parent);

        document().body().unwrap().append_child(&parent).unwrap();

        (root, scope, parent)
    }

    fn get_html(node: Html, root: &BundleRoot, scope: &AnyScope, parent: &Element) -> String {
        // clear parent
        parent.set_inner_html("");

        node.attach(root, scope, parent, NodeRef::default());
        scheduler::start_now();
        parent.inner_html()
    }

    #[test]
    fn all_ways_of_passing_children_work() {
        let (root, scope, parent) = setup_parent();

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
            <List children={children_renderer.clone()} />
        };
        assert_eq!(get_html(prop_method, &root, &scope, &parent), expected_html);

        let children_renderer_method = html! {
            <List>
                { children_renderer }
            </List>
        };
        assert_eq!(
            get_html(children_renderer_method, &root, &scope, &parent),
            expected_html
        );

        let direct_method = html! {
            <List>
                { children.clone() }
            </List>
        };
        assert_eq!(
            get_html(direct_method, &root, &scope, &parent),
            expected_html
        );

        let for_method = html! {
            <List>
                { for children }
            </List>
        };
        assert_eq!(get_html(for_method, &root, &scope, &parent), expected_html);
    }

    #[test]
    fn reset_node_ref() {
        let (root, scope, parent) = setup_parent();

        let node_ref = NodeRef::default();
        let elem = html! { <Comp ref={node_ref.clone()}></Comp> };
        let (_, elem) = elem.attach(&root, &scope, &parent, NodeRef::default());
        scheduler::start_now();
        let parent_node = parent.deref();
        assert_eq!(node_ref.get(), parent_node.first_child());
        elem.detach(&root, &parent, false);
        scheduler::start_now();
        assert!(node_ref.get().is_none());
    }
}

#[cfg(test)]
mod layout_tests {
    extern crate self as yew;

    use crate::html;
    use crate::tests::layout_tests::{diff_layouts, TestLayout};
    use crate::{Children, Component, Context, Html, Properties};
    use std::marker::PhantomData;

    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    struct Comp<T> {
        _marker: PhantomData<T>,
    }

    #[derive(Properties, Clone, PartialEq)]
    struct CompProps {
        #[prop_or_default]
        children: Children,
    }

    impl<T: 'static> Component for Comp<T> {
        type Message = ();
        type Properties = CompProps;

        fn create(_: &Context<Self>) -> Self {
            Comp {
                _marker: PhantomData::default(),
            }
        }

        fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
            unimplemented!();
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <>{ ctx.props().children.clone() }</>
            }
        }
    }

    struct A;
    struct B;

    #[test]
    fn diff() {
        let layout1 = TestLayout {
            name: "1",
            node: html! {
                <Comp<A>>
                    <Comp<B>></Comp<B>>
                    {"C"}
                </Comp<A>>
            },
            expected: "C",
        };

        let layout2 = TestLayout {
            name: "2",
            node: html! {
                <Comp<A>>
                    {"A"}
                </Comp<A>>
            },
            expected: "A",
        };

        let layout3 = TestLayout {
            name: "3",
            node: html! {
                <Comp<B>>
                    <Comp<A>></Comp<A>>
                    {"B"}
                </Comp<B>>
            },
            expected: "B",
        };

        let layout4 = TestLayout {
            name: "4",
            node: html! {
                <Comp<B>>
                    <Comp<A>>{"A"}</Comp<A>>
                    {"B"}
                </Comp<B>>
            },
            expected: "AB",
        };

        let layout5 = TestLayout {
            name: "5",
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
            name: "6",
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
            name: "7",
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
            name: "8",
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
            name: "9",
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
            name: "10",
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
            name: "11",
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
            name: "12",
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

    #[test]
    fn component_with_children() {
        #[derive(Properties, PartialEq)]
        struct Props {
            children: Children,
        }

        struct ComponentWithChildren;

        impl Component for ComponentWithChildren {
            type Message = ();
            type Properties = Props;

            fn create(_ctx: &Context<Self>) -> Self {
                Self
            }

            fn view(&self, ctx: &Context<Self>) -> Html {
                html! {
                  <ul>
                    { for ctx.props().children.iter().map(|child| html! { <li>{ child }</li> }) }
                  </ul>
                }
            }
        }

        let layout = TestLayout {
            name: "13",
            node: html! {
                <ComponentWithChildren>
                    if true {
                        <span>{ "hello" }</span>
                        <span>{ "world" }</span>
                    }  else {
                        <span>{ "goodbye" }</span>
                        <span>{ "world" }</span>
                    }
                </ComponentWithChildren>
            },
            expected: "<ul><li><span>hello</span><span>world</span></li></ul>",
        };

        diff_layouts(vec![layout]);
    }
}
