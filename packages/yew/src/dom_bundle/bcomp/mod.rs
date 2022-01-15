mod bcomp_impl;
mod lifecycle;
mod scope;

#[cfg(debug_assertions)]
pub(self) use bcomp_impl::log_event;

pub use bcomp_impl::{BComp, Mountable, PropsWrapper};
pub use scope::{AnyScope, Scope, Scoped, SendAsMessage};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom_bundle::{DomBundle, Reconcilable};
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
        let document = gloo_utils::document();
        let parent_scope: AnyScope = AnyScope::test();
        let parent_element = document.create_element("div").unwrap();

        let comp = html! { <Comp></Comp> };
        let (_, mut bundle) = comp.attach(&parent_scope, &parent_element, NodeRef::default());

        for _ in 0..10000 {
            let node = html! { <Comp></Comp> };
            node.reconcile(
                &parent_scope,
                &parent_element,
                NodeRef::default(),
                &mut bundle,
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
