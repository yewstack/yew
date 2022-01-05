//! This module contains the bundle implementation of a virtual component `BComp`.

use super::{BNode, DomBundle, VDiff};
use crate::{
    html::{AnyScope, BaseComponent, Scope, Scoped},
    virtual_dom::{Key, VComp},
    NodeRef,
};
use std::{any::TypeId, borrow::Borrow, ops::Deref};
use std::{fmt, rc::Rc};
use web_sys::Element;

thread_local! {
    #[cfg(debug_assertions)]
     static EVENT_HISTORY: std::cell::RefCell<std::collections::HashMap<u64, Vec<String>>>
        = Default::default();
}

/// Push [VComp] event to lifecycle debugging registry
#[cfg(debug_assertions)]
pub(crate) fn log_event(vcomp_id: u64, event: impl ToString) {
    EVENT_HISTORY.with(|h| {
        h.borrow_mut()
            .entry(vcomp_id)
            .or_default()
            .push(event.to_string())
    });
}

/// Get [VComp] event log from lifecycle debugging registry
#[cfg(debug_assertions)]
#[allow(dead_code)]
pub(crate) fn get_event_log(vcomp_id: u64) -> Vec<String> {
    EVENT_HISTORY.with(|h| {
        h.borrow()
            .get(&vcomp_id)
            .map(|l| (*l).clone())
            .unwrap_or_default()
    })
}

/// A virtual component.
pub struct BComp {
    type_id: TypeId,
    scope: Box<dyn Scoped>,
    node_ref: NodeRef,
    key: Option<Key>,
}

impl BComp {
    pub(crate) fn root_bnode(&self) -> Option<impl Deref<Target = BNode> + '_> {
        self.scope.root_bnode()
    }
    pub(crate) fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }
}

impl fmt::Debug for BComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BComp {{ root: {:?} }}", self.root_bnode().as_deref())
    }
}

pub(crate) trait Mountable {
    fn copy(&self) -> Box<dyn Mountable>;
    fn mount(
        self: Box<Self>,
        node_ref: NodeRef,
        parent_scope: &AnyScope,
        parent: Element,
        next_sibling: NodeRef,
        #[cfg(debug_assertions)] id: u64,
    ) -> Box<dyn Scoped>;
    fn reuse(self: Box<Self>, node_ref: NodeRef, scope: &dyn Scoped, next_sibling: NodeRef);
}

pub(crate) struct PropsWrapper<COMP: BaseComponent> {
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
        parent_scope: &AnyScope,
        parent: Element,
        next_sibling: NodeRef,
        #[cfg(debug_assertions)] id: u64,
    ) -> Box<dyn Scoped> {
        let scope: Scope<COMP> = Scope::new(
            Some(parent_scope.clone()),
            #[cfg(debug_assertions)]
            id,
        );
        scope.mount_in_place(parent, next_sibling, node_ref, self.props);

        Box::new(scope)
    }

    fn reuse(self: Box<Self>, node_ref: NodeRef, scope: &dyn Scoped, next_sibling: NodeRef) {
        let scope: Scope<COMP> = scope.to_any().downcast();
        scope.reuse(self.props, node_ref, next_sibling);
    }
}

impl DomBundle for BComp {
    fn detach(mut self, _parent: &Element) {
        self.scope.destroy();
    }

    fn shift(&self, next_parent: &Element, next_sibling: NodeRef) {
        self.scope.shift_node(next_parent.clone(), next_sibling);
    }
}

impl VDiff for VComp {
    type Bundle = BComp;

    fn attach(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
    ) -> (NodeRef, Self::Bundle) {
        let VComp {
            type_id,
            props,
            node_ref,
            key,
        } = self;

        let scope = props.mount(
            node_ref.clone(),
            parent_scope,
            parent.to_owned(),
            next_sibling,
            #[cfg(debug_assertions)]
            {
                thread_local! {
                    static ID_COUNTER: std::cell::RefCell<u64> = Default::default();
                }

                ID_COUNTER.with(|c| {
                    let c = &mut *c.borrow_mut();
                    *c += 1;
                    *c
                })
            },
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

    fn apply(
        self,
        parent_scope: &AnyScope,
        parent: &Element,
        next_sibling: NodeRef,
        ancestor: &mut BNode,
    ) -> NodeRef {
        let bcomp = match ancestor {
            // If the ancestor is the same type, reuse it and update its properties
            BNode::BComp(ref mut bcomp)
                if self.type_id == bcomp.type_id && self.key == bcomp.key =>
            {
                bcomp
            }
            _ => {
                let (node_ref, self_) = self.attach(parent_scope, parent, next_sibling);
                ancestor.replace(parent, self_.into());
                return node_ref;
            }
        };
        let VComp {
            props,
            node_ref,
            key,
            type_id: _,
        } = self;
        bcomp.key = key;
        let old_ref = std::mem::replace(&mut bcomp.node_ref, node_ref.clone());
        bcomp.node_ref.reuse(old_ref);
        props.reuse(node_ref.clone(), bcomp.scope.borrow(), next_sibling);
        node_ref
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        html,
        virtual_dom::{VChild, VNode},
        Children, Component, Context, Html, NodeRef, Properties,
    };
    use gloo_utils::document;
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
        let document = gloo_utils::document();
        let parent_scope: AnyScope = AnyScope::test();
        let parent_element = document.create_element("div").unwrap();

        let ancestor = html! { <Comp></Comp> };
        let (_, mut comp) = ancestor.attach(&parent_scope, &parent_element, NodeRef::default());

        for _ in 0..10000 {
            let node = html! { <Comp></Comp> };
            node.apply(
                &parent_scope,
                &parent_element,
                NodeRef::default(),
                &mut comp,
            );
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

    use super::{AnyScope, Element};

    fn setup_parent() -> (AnyScope, Element) {
        let scope = AnyScope::test();
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        (scope, parent)
    }

    fn get_html(node: Html, scope: &AnyScope, parent: &Element) -> String {
        // clear parent
        parent.set_inner_html("");

        node.attach(scope, parent, NodeRef::default());
        parent.inner_html()
    }

    #[test]
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
            <List children={children_renderer.clone()} />
        };
        assert_eq!(get_html(prop_method, &scope, &parent), expected_html);

        let children_renderer_method = html! {
            <List>
                { children_renderer }
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

    #[test]
    fn reset_node_ref() {
        let scope = AnyScope::test();
        let parent = document().create_element("div").unwrap();

        document().body().unwrap().append_child(&parent).unwrap();

        let node_ref = NodeRef::default();
        let elem = html! { <Comp ref={node_ref.clone()}></Comp> };
        let (_, elem) = elem.attach(&scope, &parent, NodeRef::default());
        let parent_node = parent.deref();
        assert_eq!(node_ref.get(), parent_node.first_child());
        elem.detach(&parent);
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
