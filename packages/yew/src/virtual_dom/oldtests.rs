mod vcomp {

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{
            html, Children, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender,
        };
        use cfg_match::cfg_match;

        #[cfg(feature = "std_web")]
        use stdweb::web::INode;

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
                true
            }

            fn view(&self) -> Html {
                html! { <div/> }
            }
        }

        #[test]
        fn update_loop() {
            let document = crate::utils::document();
            let parent_scope: AnyScope = crate::html::Scope::<Comp>::new(None).into();
            let parent_element = document.create_element("div").unwrap();

            let mut ancestor = html! { <Comp></Comp> };
            ancestor.apply(&parent_scope, &parent_element, NodeRef::default(), None);

            for _ in 0..10000 {
                let mut node = html! { <Comp></Comp> };
                node.apply(
                    &parent_scope,
                    &parent_element,
                    NodeRef::default(),
                    Some(ancestor),
                );
                ancestor = node;
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
            let test_key: Key = "test".to_string().into();
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
            let scope = AnyScope {
                type_id: std::any::TypeId::of::<()>(),
                parent: None,
                state: std::rc::Rc::new(()),
            };
            let parent = document().create_element("div").unwrap();

            #[cfg(feature = "std_web")]
            document().body().unwrap().append_child(&parent);
            #[cfg(feature = "web_sys")]
            document().body().unwrap().append_child(&parent).unwrap();

            let node_ref = NodeRef::default();
            let mut elem: VNode = html! { <Comp ref=node_ref.clone()></Comp> };
            elem.apply(&scope, &parent, NodeRef::default(), None);
            let parent_node = cfg_match! {
                feature = "std_web" => parent.as_node(),
                feature = "web_sys" => parent.deref(),
                feature = "static_render" => parent.as_node(),
            };
            assert_eq!(node_ref.get(), parent_node.first_child());
            elem.detach(&parent);
            assert!(node_ref.get().is_none());
        }
    }

    #[cfg(all(test, feature = "web_sys"))]
    mod layout_tests {
        extern crate self as yew;

        use crate::html;
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
    }
}

#[cfg(all(test, feature = "web_sys", feature = "wasm_bench"))]
mod benchmarks {
    use super::{Attributes, PositionalAttr};
    use std::borrow::Cow;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    fn create_pos_attrs() -> Vec<PositionalAttr> {
        vec![
            PositionalAttr::new("oh", Cow::Borrowed("danny")),
            PositionalAttr::new("boy", Cow::Borrowed("the")),
            PositionalAttr::new("pipes", Cow::Borrowed("the")),
            PositionalAttr::new("are", Cow::Borrowed("calling")),
            PositionalAttr::new("from", Cow::Borrowed("glen")),
            PositionalAttr::new("to", Cow::Borrowed("glen")),
            PositionalAttr::new("and", Cow::Borrowed("down")),
            PositionalAttr::new("the", Cow::Borrowed("mountain")),
            PositionalAttr::new("side", Cow::Borrowed("")),
        ]
    }

    fn run_benchmarks(name: &str, new: Vec<PositionalAttr>, old: Vec<PositionalAttr>) {
        let new_vec = Attributes::from(new);
        let old_vec = Attributes::from(old);

        let mut new_map = new_vec.clone();
        let _ = new_map.get_mut_index_map();
        let mut old_map = old_vec.clone();
        let _ = old_map.get_mut_index_map();

        const TIME_LIMIT: f64 = 2.0;

        let vv = easybench_wasm::bench_env_limit(TIME_LIMIT, (&new_vec, &old_vec), |(new, old)| {
            format!("{:?}", Attributes::diff(&new, &old))
        });
        let mm = easybench_wasm::bench_env_limit(TIME_LIMIT, (&new_map, &old_map), |(new, old)| {
            format!("{:?}", Attributes::diff(&new, &old))
        });

        let vm = easybench_wasm::bench_env_limit(TIME_LIMIT, (&new_vec, &old_map), |(new, old)| {
            format!("{:?}", Attributes::diff(&new, &old))
        });
        let mv = easybench_wasm::bench_env_limit(TIME_LIMIT, (&new_map, &old_vec), |(new, old)| {
            format!("{:?}", Attributes::diff(&new, &old))
        });

        wasm_bindgen_test::console_log!(
            "{}:\n\tvec-vec: {}\n\tmap-map: {}\n\tvec-map: {}\n\tmap-vec: {}",
            name,
            vv,
            mm,
            vm,
            mv
        );
    }

    #[wasm_bindgen_test]
    fn bench_diff_attributes_equal() {
        let old = create_pos_attrs();
        let new = old.clone();

        run_benchmarks("equal", new, old);
    }

    #[wasm_bindgen_test]
    fn bench_diff_attributes_length_end() {
        let old = create_pos_attrs();
        let mut new = old.clone();
        new.push(PositionalAttr::new("hidden", Cow::Borrowed("hidden")));

        run_benchmarks("added to end", new.clone(), old.clone());
        run_benchmarks("removed from end", old, new);
    }
    #[wasm_bindgen_test]
    fn bench_diff_attributes_length_start() {
        let old = create_pos_attrs();
        let mut new = old.clone();
        new.insert(0, PositionalAttr::new("hidden", Cow::Borrowed("hidden")));

        run_benchmarks("added to start", new.clone(), old.clone());
        run_benchmarks("removed from start", old, new);
    }

    #[wasm_bindgen_test]
    fn bench_diff_attributes_reorder() {
        let old = create_pos_attrs();
        let new = old.clone().into_iter().rev().collect();

        run_benchmarks("reordered", new, old);
    }

    #[wasm_bindgen_test]
    fn bench_diff_attributes_change_first() {
        let old = create_pos_attrs();
        let mut new = old.clone();
        new[0].1 = Some(Cow::Borrowed("changed"));

        run_benchmarks("changed first", new, old);
    }

    #[wasm_bindgen_test]
    fn bench_diff_attributes_change_middle() {
        let old = create_pos_attrs();
        let mut new = old.clone();
        new[old.len() / 2].1 = Some(Cow::Borrowed("changed"));

        run_benchmarks("changed middle", new, old);
    }

    #[wasm_bindgen_test]
    fn bench_diff_attributes_change_last() {
        let old = create_pos_attrs();
        let mut new = old.clone();
        new[old.len() - 1].1 = Some(Cow::Borrowed("changed"));

        run_benchmarks("changed last", new, old);
    }
}

mod vlist {

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
                    <>
                        {"a"}
                        {"b"}
                        <>
                            {"c"}
                            {"d"}
                        </>
                        {"e"}
                    </>
                },
                expected: "abcde",
            };

            let layout2 = TestLayout {
                name: "2",
                node: html! {
                    <>
                        {"a"}
                        {"b"}
                        <></>
                        {"e"}
                        {"f"}
                    </>
                },
                expected: "abef",
            };

            let layout3 = TestLayout {
                name: "3",
                node: html! {
                    <>
                        {"a"}
                        <></>
                        {"b"}
                        {"e"}
                    </>
                },
                expected: "abe",
            };

            let layout4 = TestLayout {
                name: "4",
                node: html! {
                    <>
                        {"a"}
                        <>
                            {"c"}
                            {"d"}
                        </>
                        {"b"}
                        {"e"}
                    </>
                },
                expected: "acdbe",
            };

            diff_layouts(vec![layout1, layout2, layout3, layout4]);
        }
    }

    #[cfg(all(test, feature = "web_sys"))]
    mod layout_tests_keys {
        extern crate self as yew;

        use crate::html;
        use crate::virtual_dom::layout_tests::{diff_layouts, TestLayout};
        use crate::virtual_dom::VNode;
        use crate::{Children, Component, ComponentLink, Html, Properties, ShouldRender};
        use web_sys::Node;

        #[cfg(feature = "wasm_test")]
        use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

        #[cfg(feature = "wasm_test")]
        wasm_bindgen_test_configure!(run_in_browser);

        struct Comp {
            id: usize,
            panic_if_changes: bool,
        }

        #[derive(Properties, Clone)]
        struct CountingCompProps {
            id: usize,
            #[prop_or(false)]
            can_change: bool,
        }

        impl Component for Comp {
            type Message = ();
            type Properties = CountingCompProps;

            fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
                Comp {
                    id: props.id,
                    panic_if_changes: props.can_change,
                }
            }

            fn change(&mut self, props: Self::Properties) -> ShouldRender {
                #[cfg(feature = "wasm_test")]
                wasm_bindgen_test::console_log!("Comp changed: {} -> {}", self.id, props.id);
                let changed = self.id != props.id;
                if self.panic_if_changes && changed {
                    panic!(
                        "VComp changed but should not have: {} -> {}.",
                        self.id, props.id
                    );
                }
                self.id = props.id;
                changed
            }

            fn update(&mut self, _: Self::Message) -> ShouldRender {
                unimplemented!();
            }

            fn view(&self) -> Html {
                html! { <p>{ self.id }</p> }
            }
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

            fn change(&mut self, mut props: Self::Properties) -> ShouldRender {
                std::mem::swap(&mut self.0, &mut props);
                self.0.children != props.children
            }

            fn view(&self) -> Html {
                html! { <>{ for self.0.children.iter() }</> }
            }
        }

        #[test]
        fn diff() {
            let mut layouts = vec![];

            let vref_node: Node = crate::utils::document().create_element("i").unwrap().into();
            layouts.push(TestLayout {
                name: "All VNode types as children",
                node: html! {
                    <>
                        {"a"}
                        <span key="vtag"></span>
                        {"c"}
                        {"d"}
                        <Comp id=0 key="vchild" />
                        <key="vlist">
                            {"foo"}
                            {"bar"}
                        </>
                        {VNode::VRef(vref_node)}
                    </>
                },
                expected: "a<span></span>cd<p>0</p>foobar<i></i>",
            });

            layouts.extend(vec![
                TestLayout {
                    name: "Inserting into VList first child - before",
                    node: html! {
                        <>
                            <key="VList">
                                <i key="i"></i>
                            </>
                            <p key="p"></p>
                        </>
                    },
                    expected: "<i></i><p></p>",
                },
                TestLayout {
                    name: "Inserting into VList first child - after",
                    node: html! {
                        <>
                            <key="VList">
                                <i key="i"></i>
                                <e key="e"></e>
                            </>
                            <p key="p"></p>
                        </>
                    },
                    expected: "<i></i><e></e><p></p>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "No matches - before",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <e key="e"></e>
                        </>
                    },
                    expected: "<i></i><e></e>",
                },
                TestLayout {
                    name: "No matches - after",
                    node: html! {
                        <>
                            <a key="a"></a>
                            <p key="p"></p>
                        </>
                    },
                    expected: "<a></a><p></p>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Append - before",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <e key="e"></e>
                        </>
                    },
                    expected: "<i></i><e></e>",
                },
                TestLayout {
                    name: "Append - after",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <e key="e"></e>
                            <p key="p"></p>
                        </>
                    },
                    expected: "<i></i><e></e><p></p>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Prepend - before",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <e key="e"></e>
                        </>
                    },
                    expected: "<i></i><e></e>",
                },
                TestLayout {
                    name: "Prepend - after",
                    node: html! {
                        <>
                            <p key="p"></p>
                            <i key="i"></i>
                            <e key="e"></e>
                        </>
                    },
                    expected: "<p></p><i></i><e></e>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Delete first - before",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <e key="e"></e>
                            <p key="p"></p>
                        </>
                    },
                    expected: "<i></i><e></e><p></p>",
                },
                TestLayout {
                    name: "Delete first - after",
                    node: html! {
                        <>
                            <e key="e"></e>
                            <p key="p"></p>
                        </>
                    },
                    expected: "<e></e><p></p>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Delete last - before",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <e key="e"></e>
                            <p key="p"></p>
                        </>
                    },
                    expected: "<i></i><e></e><p></p>",
                },
                TestLayout {
                    name: "Delete last - after",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <e key="e"></e>
                        </>
                    },
                    expected: "<i></i><e></e>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Delete last and change node type - before",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <e key="e"></e>
                            <p key="p"></p>
                        </>
                    },
                    expected: "<i></i><e></e><p></p>",
                },
                TestLayout {
                    name: "Delete last - after",
                    node: html! {
                        <>
                            <List key="i"><i/></List>
                            <List key="e"><e/></List>
                            <List key="a"><a/></List>
                        </>
                    },
                    expected: "<i></i><e></e><a></a>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Delete middle - before",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <e key="e"></e>
                            <p key="p"></p>
                            <a key="a"></a>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><a></a>",
                },
                TestLayout {
                    name: "Delete middle - after",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <e key="e2"></e>
                            <p key="p2"></p>
                            <a key="a"></a>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><a></a>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Delete middle and change node type - before",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <e key="e"></e>
                            <p key="p"></p>
                            <a key="a"></a>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><a></a>",
                },
                TestLayout {
                    name: "Delete middle and change node type- after",
                    node: html! {
                        <>
                            <List key="i2"><i/></List>
                            <e key="e"></e>
                            <List key="p"><p/></List>
                            <List key="a2"><a/></List>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><a></a>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Reverse - before",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <e key="e"></e>
                            <p key="p"></p>
                            <u key="u"></u>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><u></u>",
                },
                TestLayout {
                    name: "Reverse - after",
                    node: html! {
                        <>
                            <u key="u"></u>
                            <p key="p"></p>
                            <e key="e"></e>
                            <i key="i"></i>
                        </>
                    },
                    expected: "<u></u><p></p><e></e><i></i>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Reverse and change node type - before",
                    node: html! {
                        <>
                            <i key="i"></i>
                            <key="i1"></>
                            <key="i2"></>
                            <key="i3"></>
                            <e key="e"></e>
                            <key="yo">
                                <p key="p"></p>
                            </>
                            <u key="u"></u>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><u></u>",
                },
                TestLayout {
                    name: "Reverse and change node type - after",
                    node: html! {
                        <>
                            <List key="u"><u/></List>
                            <List key="p"><p/></List>
                            <List key="e"><e/></List>
                            <List key="i"><i/></List>
                        </>
                    },
                    expected: "<u></u><p></p><e></e><i></i>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Swap 1&2 - before",
                    node: html! {
                        <>
                            <i key="1"></i>
                            <e key="2"></e>
                            <p key="3"></p>
                            <a key="4"></a>
                            <u key="5"></u>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><a></a><u></u>",
                },
                TestLayout {
                    name: "Swap 1&2 - after",
                    node: html! {
                        <>
                            <e key="2"></e>
                            <i key="1"></i>
                            <p key="3"></p>
                            <a key="4"></a>
                            <u key="5"></u>
                        </>
                    },
                    expected: "<e></e><i></i><p></p><a></a><u></u>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Swap 1&2 and change node type - before",
                    node: html! {
                        <>
                            <i key="1"></i>
                            <e key="2"></e>
                            <p key="3"></p>
                            <a key="4"></a>
                            <u key="5"></u>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><a></a><u></u>",
                },
                TestLayout {
                    name: "Swap 1&2 and change node type - after",
                    node: html! {
                        <>
                            <List key="2"><e/></List>
                            <List key="1"><i/></List>
                            <List key="3"><p/></List>
                            <List key="4"><a/></List>
                            <List key="5"><u/></List>
                        </>
                    },
                    expected: "<e></e><i></i><p></p><a></a><u></u>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "test - before",
                    node: html! {
                        <>
                            <key="1">
                                <e key="e"></e>
                                <p key="p"></p>
                                <a key="a"></a>
                                <u key="u"></u>
                            </>
                            <key="2">
                                <e key="e"></e>
                                <p key="p"></p>
                                <a key="a"></a>
                                <u key="u"></u>
                            </>
                        </>
                    },
                    expected: "<e></e><p></p><a></a><u></u><e></e><p></p><a></a><u></u>",
                },
                TestLayout {
                    name: "Swap 4&5 - after",
                    node: html! {
                        <>
                            <e key="1"></e>
                            <key="2">
                                <p key="p"></p>
                                <i key="i"></i>
                            </>
                        </>
                    },
                    expected: "<e></e><p></p><i></i>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Swap 4&5 - before",
                    node: html! {
                        <>
                            <i key="1"></i>
                            <e key="2"></e>
                            <p key="3"></p>
                            <a key="4"></a>
                            <u key="5"></u>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><a></a><u></u>",
                },
                TestLayout {
                    name: "Swap 4&5 - after",
                    node: html! {
                        <>
                            <i key="1"></i>
                            <e key="2"></e>
                            <p key="3"></p>
                            <u key="5"></u>
                            <a key="4"></a>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><u></u><a></a>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Swap 1&5 - before",
                    node: html! {
                        <>
                            <i key="1"></i>
                            <e key="2"></e>
                            <p key="3"></p>
                            <a key="4"></a>
                            <u key="5"></u>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><a></a><u></u>",
                },
                TestLayout {
                    name: "Swap 1&5 - after",
                    node: html! {
                        <>
                            <u key="5"></u>
                            <e key="2"></e>
                            <p key="3"></p>
                            <a key="4"></a>
                            <i key="1"></i>
                        </>
                    },
                    expected: "<u></u><e></e><p></p><a></a><i></i>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Move 2 after 4 - before",
                    node: html! {
                        <>
                            <i key="1"></i>
                            <e key="2"></e>
                            <p key="3"></p>
                            <a key="4"></a>
                            <u key="5"></u>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><a></a><u></u>",
                },
                TestLayout {
                    name: "Move 2 after 4 - after",
                    node: html! {
                        <>
                            <i key="1"></i>
                            <p key="3"></p>
                            <a key="4"></a>
                            <e key="2"></e>
                            <u key="5"></u>
                        </>
                    },
                    expected: "<i></i><p></p><a></a><e></e><u></u>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Swap lists - before",
                    node: html! {
                        <>
                            <key="1">
                                <i></i>
                                <e></e>
                            </>
                            <key="2">
                                <a></a>
                                <u></u>
                            </>
                        </>
                    },
                    expected: "<i></i><e></e><a></a><u></u>",
                },
                TestLayout {
                    name: "Swap lists - after",
                    node: html! {
                        <>
                            <key="2">
                                <a></a>
                                <u></u>
                            </>
                            <key="1">
                                <i></i>
                                <e></e>
                            </>
                        </>
                    },
                    expected: "<a></a><u></u><i></i><e></e>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Swap lists with in-between - before",
                    node: html! {
                        <>
                            <key="1">
                                <i></i>
                                <e></e>
                            </>
                            <p key="between"></p>
                            <key="2">
                                <a></a>
                                <u></u>
                            </>
                        </>
                    },
                    expected: "<i></i><e></e><p></p><a></a><u></u>",
                },
                TestLayout {
                    name: "Swap lists with in-between - after",
                    node: html! {
                        <>
                            <key="2">
                                <a></a>
                                <u></u>
                            </>
                            <p key="between"></p>
                            <key="1">
                                <i></i>
                                <e></e>
                            </>
                        </>
                    },
                    expected: "<a></a><u></u><p></p><i></i><e></e>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Insert VComp front - before",
                    node: html! {
                        <>
                            <u key=1></u>
                            <a key=2></a>
                        </>
                    },
                    expected: "<u></u><a></a>",
                },
                TestLayout {
                    name: "Insert VComp front - after",
                    node: html! {
                        <>
                            <Comp id=0 key="comp"/>
                            <u key=1></u>
                            <a key=2></a>
                        </>
                    },
                    expected: "<p>0</p><u></u><a></a>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Insert VComp middle - before",
                    node: html! {
                        <>
                            <u key=1></u>
                            <a key=2></a>
                        </>
                    },
                    expected: "<u></u><a></a>",
                },
                TestLayout {
                    name: "Insert VComp middle - after",
                    node: html! {
                        <>
                            <u key=1></u>
                            <Comp id=0 key="comp"/>
                            <a key=2></a>
                        </>
                    },
                    expected: "<u></u><p>0</p><a></a>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Insert VComp back - before",
                    node: html! {
                        <>
                            <u key=1></u>
                            <a key=2></a>
                        </>
                    },
                    expected: "<u></u><a></a>",
                },
                TestLayout {
                    name: "Insert VComp back - after",
                    node: html! {
                        <>
                            <u key=1></u>
                            <a key=2></a>
                            <Comp id=0 key="comp"/>
                        </>
                    },
                    expected: "<u></u><a></a><p>0</p>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Reverse VComp children - before",
                    node: html! {
                        <>
                            <Comp id=1 key="comp-1"/>
                            <Comp id=2 key="comp-2"/>
                            <Comp id=3 key="comp-3"/>
                        </>
                    },
                    expected: "<p>1</p><p>2</p><p>3</p>",
                },
                TestLayout {
                    name: "Reverse VComp children - after",
                    node: html! {
                        <>
                            <Comp id=3 key="comp-3"/>
                            <Comp id=2 key="comp-2"/>
                            <Comp id=1 key="comp-1"/>
                        </>
                    },
                    expected: "<p>3</p><p>2</p><p>1</p>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Reverse VComp children with children - before",
                    node: html! {
                        <>
                            <List key="comp-1"><p>{"11"}</p><p>{"12"}</p></List>
                            <List key="comp-2"><p>{"21"}</p><p>{"22"}</p></List>
                            <List key="comp-3"><p>{"31"}</p><p>{"32"}</p></List>
                        </>
                    },
                    expected: "<p>11</p><p>12</p><p>21</p><p>22</p><p>31</p><p>32</p>",
                },
                TestLayout {
                    name: "Reverse VComp children with children - after",
                    node: html! {
                        <>
                            <List key="comp-3"><p>{"31"}</p><p>{"32"}</p></List>
                            <List key="comp-2"><p>{"21"}</p><p>{"22"}</p></List>
                            <List key="comp-1"><p>{"11"}</p><p>{"12"}</p></List>
                        </>
                    },
                    expected: "<p>31</p><p>32</p><p>21</p><p>22</p><p>11</p><p>12</p>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Complex component update - before",
                    node: html! {
                        <List>
                            <Comp id=1 key="comp-1"/>
                            <Comp id=2 key="comp-2"/>
                        </List>
                    },
                    expected: "<p>1</p><p>2</p>",
                },
                TestLayout {
                    name: "Complex component update - after",
                    node: html! {
                        <List>
                            <List key="comp-1">
                                <Comp id=1 />
                            </List>
                            <List key="comp-2">
                                <p>{"2"}</p>
                            </List>
                        </List>
                    },
                    expected: "<p>1</p><p>2</p>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Reorder VComp children with children - before",
                    node: html! {
                        <>
                            <List key="comp-1"><p>{"1"}</p></List>
                            <List key="comp-3"><p>{"3"}</p></List>
                            <List key="comp-5"><p>{"5"}</p></List>
                            <List key="comp-2"><p>{"2"}</p></List>
                            <List key="comp-4"><p>{"4"}</p></List>
                            <List key="comp-6"><p>{"6"}</p></List>
                        </>
                    },
                    expected: "<p>1</p><p>3</p><p>5</p><p>2</p><p>4</p><p>6</p>",
                },
                TestLayout {
                    name: "Reorder VComp children with children - after",
                    node: html! {
                        <>
                            <Comp id=6 key="comp-6"/>
                            <Comp id=5 key="comp-5"/>
                            <Comp id=4 key="comp-4"/>
                            <Comp id=3 key="comp-3"/>
                            <Comp id=2 key="comp-2"/>
                            <Comp id=1 key="comp-1"/>
                        </>
                    },
                    expected: "<p>6</p><p>5</p><p>4</p><p>3</p><p>2</p><p>1</p>",
                },
            ]);

            layouts.extend(vec![
                TestLayout {
                    name: "Replace and reorder components - before",
                    node: html! {
                        <List>
                            <List key="comp-1"><p>{"1"}</p></List>
                            <List key="comp-2"><p>{"2"}</p></List>
                            <List key="comp-3"><p>{"3"}</p></List>
                        </List>
                    },
                    expected: "<p>1</p><p>2</p><p>3</p>",
                },
                TestLayout {
                    name: "Replace and reorder components - after",
                    node: html! {
                        <List>
                            <Comp id=3 key="comp-3" />
                            <Comp id=2 key="comp-2" />
                            <Comp id=1 key="comp-1" />
                        </List>
                    },
                    expected: "<p>3</p><p>2</p><p>1</p>",
                },
            ]);

            diff_layouts(layouts);
        }
    }
}

mod vnode {
    #[cfg(all(test, feature = "web_sys"))]
    mod layout_tests {
        use super::*;
        use crate::virtual_dom::layout_tests::{diff_layouts, TestLayout};

        #[cfg(feature = "wasm_test")]
        use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

        #[cfg(feature = "wasm_test")]
        wasm_bindgen_test_configure!(run_in_browser);

        #[test]
        fn diff() {
            let document = crate::utils::document();
            let vref_node_1 = VNode::VRef(document.create_element("i").unwrap().into());
            let vref_node_2 = VNode::VRef(document.create_element("b").unwrap().into());

            let layout1 = TestLayout {
                name: "1",
                node: vref_node_1,
                expected: "<i></i>",
            };

            let layout2 = TestLayout {
                name: "2",
                node: vref_node_2,
                expected: "<b></b>",
            };

            diff_layouts(vec![layout1, layout2]);
        }
    }
}

mod vtag {

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::html;
        use std::any::TypeId;
        // #[cfg(feature = "std_web")]
        // use stdweb::web::{document, IElement};
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
                .iter()
                .find(|(k, _)| k == &"class")
                .map(|(_, v)| AsRef::as_ref(v))
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
                <div class="class-1 class-2 class-3"></div>
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

            macro_rules! has_class {
                ($vtag:expr) => {
                    $vtag.attributes.iter().any(|(k, _)| k == "class")
                };
            }

            if let VNode::VTag(vtag) = a {
                assert!(!has_class!(vtag));
            } else {
                panic!("vtag expected");
            }

            if let VNode::VTag(vtag) = b {
                assert!(!has_class!(vtag));
            } else {
                panic!("vtag expected");
            }

            if let VNode::VTag(vtag) = c {
                assert!(!has_class!(vtag));
            } else {
                panic!("vtag expected");
            }

            if let VNode::VTag(vtag) = d {
                assert!(!vtag.attributes.iter().any(|(k, _)| k == "class"));
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
                <div class=vec!["class-1", "class-2", "class-3"]></div>
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
                assert_eq!(
                    vtag.attributes
                        .iter()
                        .find(|(k, _)| k == &"aria-controls")
                        .map(|(_, v)| v),
                    Some("it-works")
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
                feature = "static_render" => input_ref.dyn_ref::<InputElement>(),
            };
            cfg_match! {
                feature = "std_web" => input.unwrap().set_raw_value("User input"),
                feature = "web_sys" => input.unwrap().set_value("User input"),
                feature = "static_render" => input.unwrap().set_value("User input"),
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
                builder.push('a');
                builder
            }/> };

            elem.apply(&scope, &parent, NodeRef::default(), None);
            let vtag = assert_vtag(&mut elem);
            // make sure the new tag name is used internally
            assert_eq!(vtag.tag(), "a");

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
            let v: Option<&str> = div_vtag
                .attributes
                .iter()
                .find(|(k, _)| k == &"value")
                .map(|(_, v)| AsRef::as_ref(v));
            assert_eq!(v, Some("Hello"));

            let mut input_el = html! {
                <@{"input"} value="World"/>
            };
            let input_vtag = assert_vtag(&mut input_el);
            assert_eq!(input_vtag.value, Some("World".to_string()));
            assert!(!input_vtag.attributes.iter().any(|(k, _)| k == "value"));
        }

        #[test]
        fn dynamic_tags_handle_weird_capitalization() {
            let mut el = html! {
                <@{"tExTAREa"}/>
            };
            let vtag = assert_vtag(&mut el);
            assert_eq!(vtag.tag(), "textarea");
        }

        #[test]
        fn reset_node_ref() {
            let scope = test_scope();
            let parent = document().create_element("div").unwrap();

            #[cfg(feature = "std_web")]
            document().body().unwrap().append_child(&parent);
            #[cfg(feature = "web_sys")]
            document().body().unwrap().append_child(&parent).unwrap();

            let node_ref = NodeRef::default();
            let mut elem: VNode = html! { <div ref=node_ref.clone()></div> };
            assert_vtag(&mut elem);
            elem.apply(&scope, &parent, NodeRef::default(), None);
            let parent_node = cfg_match! {
                feature = "std_web" => parent.as_node(),
                feature = "web_sys" => parent.deref(),
                feature = "static_render" => parent.deref(),
            };
            assert_eq!(node_ref.get(), parent_node.first_child());
            elem.detach(&parent);
            assert!(node_ref.get().is_none());
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
}

mod vtext {

    #[cfg(test)]
    mod test {
        extern crate self as yew;

        use crate::html;

        #[cfg(feature = "wasm_test")]
        use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

        #[cfg(feature = "wasm_test")]
        wasm_bindgen_test_configure!(run_in_browser);

        #[test]
        fn text_as_root() {
            html! {
                "Text Node As Root"
            };

            html! {
                { "Text Node As Root" }
            };
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
                node: html! { "a" },
                expected: "a",
            };

            let layout2 = TestLayout {
                name: "2",
                node: html! { "b" },
                expected: "b",
            };

            let layout3 = TestLayout {
                name: "3",
                node: html! {
                    <>
                        {"a"}
                        {"b"}
                    </>
                },
                expected: "ab",
            };

            let layout4 = TestLayout {
                name: "4",
                node: html! {
                    <>
                        {"b"}
                        {"a"}
                    </>
                },
                expected: "ba",
            };

            diff_layouts(vec![layout1, layout2, layout3, layout4]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestClass;

    impl TestClass {
        fn as_class(&self) -> &'static str {
            "test-class"
        }
    }

    impl From<TestClass> for Classes {
        fn from(x: TestClass) -> Self {
            Classes::from(x.as_class())
        }
    }

    #[test]
    fn it_is_initially_empty() {
        let subject = Classes::new();
        assert!(subject.is_empty());
    }

    #[test]
    fn it_pushes_value() {
        let mut subject = Classes::new();
        subject.push("foo");
        assert!(!subject.is_empty());
        assert!(subject.contains("foo"));
    }

    #[test]
    fn it_adds_values_via_extend() {
        let mut other = Classes::new();
        other.push("bar");
        let mut subject = Classes::new();
        subject.extend(other);
        assert!(subject.contains("bar"));
    }

    #[test]
    fn it_contains_both_values() {
        let mut other = Classes::new();
        other.push("bar");
        let mut subject = Classes::new();
        subject.extend(other);
        subject.push("foo");
        assert!(subject.contains("foo"));
        assert!(subject.contains("bar"));
    }

    #[test]
    fn it_splits_class_with_spaces() {
        let mut subject = Classes::new();
        subject.push("foo bar");
        assert!(subject.contains("foo"));
        assert!(subject.contains("bar"));
    }

    #[test]
    fn push_and_contains_can_be_used_with_other_objects() {
        let mut subject = Classes::new();
        subject.push(TestClass);
        let other_class: Option<TestClass> = None;
        subject.push(other_class);
        assert!(subject.contains(TestClass.as_class()));
    }

    #[test]
    fn can_be_extended_with_another_class() {
        let mut other = Classes::new();
        other.push("foo");
        other.push("bar");
        let mut subject = Classes::new();
        subject.extend(other);
        assert!(subject.contains("foo"));
        assert!(subject.contains("bar"));
    }
}

// stdweb lacks the `inner_html` method
#[cfg(all(test, feature = "web_sys"))]
mod layout_tests {
    use super::*;
    use crate::html::{AnyScope, Scope};
    use crate::{Component, ComponentLink, Html, ShouldRender};

    struct Comp;
    impl Component for Comp {
        type Message = ();
        type Properties = ();

        fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
            unimplemented!()
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            unimplemented!();
        }

        fn change(&mut self, _: Self::Properties) -> ShouldRender {
            unimplemented!()
        }

        fn view(&self) -> Html {
            unimplemented!()
        }
    }

    pub(crate) struct TestLayout<'a> {
        pub(crate) name: &'a str,
        pub(crate) node: VNode,
        pub(crate) expected: &'a str,
    }

    pub(crate) fn diff_layouts(layouts: Vec<TestLayout<'_>>) {
        let document = crate::utils::document();
        let parent_scope: AnyScope = Scope::<Comp>::new(None).into();
        let parent_element = document.create_element("div").unwrap();
        let parent_node: Node = parent_element.clone().into();
        let end_node = document.create_text_node("END");
        parent_node.append_child(&end_node).unwrap();
        let mut empty_node: VNode = VText::new("").into();

        // Tests each layout independently
        let next_sibling = NodeRef::new(end_node.into());
        for layout in layouts.iter() {
            // Apply the layout
            let mut node = layout.node.clone();
            #[cfg(feature = "wasm_test")]
            wasm_bindgen_test::console_log!("Independently apply layout '{}'", layout.name);
            node.apply(&parent_scope, &parent_element, next_sibling.clone(), None);
            assert_eq!(
                parent_element.inner_html(),
                format!("{}END", layout.expected),
                "Independent apply failed for layout '{}'",
                layout.name,
            );

            // Diff with no changes
            let mut node_clone = layout.node.clone();
            #[cfg(feature = "wasm_test")]
            wasm_bindgen_test::console_log!("Independently reapply layout '{}'", layout.name);
            node_clone.apply(
                &parent_scope,
                &parent_element,
                next_sibling.clone(),
                Some(node),
            );
            assert_eq!(
                parent_element.inner_html(),
                format!("{}END", layout.expected),
                "Independent reapply failed for layout '{}'",
                layout.name,
            );

            // Detach
            empty_node.clone().apply(
                &parent_scope,
                &parent_element,
                next_sibling.clone(),
                Some(node_clone),
            );
            assert_eq!(
                parent_element.inner_html(),
                "END",
                "Independent detach failed for layout '{}'",
                layout.name,
            );
        }

        // Sequentially apply each layout
        let mut ancestor: Option<VNode> = None;
        for layout in layouts.iter() {
            let mut next_node = layout.node.clone();
            #[cfg(feature = "wasm_test")]
            wasm_bindgen_test::console_log!("Sequentially apply layout '{}'", layout.name);
            next_node.apply(
                &parent_scope,
                &parent_element,
                next_sibling.clone(),
                ancestor,
            );
            assert_eq!(
                parent_element.inner_html(),
                format!("{}END", layout.expected),
                "Sequential apply failed for layout '{}'",
                layout.name,
            );
            ancestor = Some(next_node);
        }

        // Sequentially detach each layout
        for layout in layouts.into_iter().rev() {
            let mut next_node = layout.node.clone();
            #[cfg(feature = "wasm_test")]
            wasm_bindgen_test::console_log!("Sequentially detach layout '{}'", layout.name);
            next_node.apply(
                &parent_scope,
                &parent_element,
                next_sibling.clone(),
                ancestor,
            );
            assert_eq!(
                parent_element.inner_html(),
                format!("{}END", layout.expected),
                "Sequential detach failed for layout '{}'",
                layout.name,
            );
            ancestor = Some(next_node);
        }

        // Detach last layout
        empty_node.apply(&parent_scope, &parent_element, next_sibling, ancestor);
        assert_eq!(
            parent_element.inner_html(),
            "END",
            "Failed to detach last layout"
        );
    }
}
