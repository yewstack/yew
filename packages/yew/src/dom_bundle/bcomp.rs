//! This module contains the bundle implementation of a virtual component [BComp].

use std::any::TypeId;
use std::borrow::Borrow;
use std::fmt;

use web_sys::Element;

use super::{BNode, BSubtree, DomSlot, DynamicDomSlot, Reconcilable, ReconcileTarget};
use crate::html::{AnyScope, Scoped};
use crate::virtual_dom::{Key, VComp};

/// A virtual component. Compare with [VComp].
pub(super) struct BComp {
    type_id: TypeId,
    scope: Box<dyn Scoped>,
    /// An internal [`DomSlot`] passed around to track this components position. This
    /// will dynamically adjust when a lifecycle changes the render state of this component.
    own_position: DynamicDomSlot,
    key: Option<Key>,
}

impl BComp {
    /// Get the key of the underlying component
    pub fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }
}

impl fmt::Debug for BComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BComp")
            .field("root", &self.scope.as_ref().render_state())
            .finish()
    }
}

impl ReconcileTarget for BComp {
    fn detach(self, _root: &BSubtree, _parent: &Element, parent_to_detach: bool) {
        self.scope.destroy_boxed(parent_to_detach);
    }

    fn shift(&self, next_parent: &Element, slot: DomSlot) -> DomSlot {
        self.scope.shift_node(next_parent.clone(), slot);

        self.own_position.to_position()
    }
}

impl Reconcilable for VComp {
    type Bundle = BComp;

    fn attach(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
    ) -> (DomSlot, Self::Bundle) {
        let VComp {
            type_id,
            mountable,
            key,
            ..
        } = self;
        let internal_ref = DynamicDomSlot::new_debug_trapped();

        let scope = mountable.mount(
            root,
            parent_scope,
            parent.to_owned(),
            slot,
            internal_ref.clone(),
        );

        (
            internal_ref.to_position(),
            BComp {
                type_id,
                own_position: internal_ref,
                key,
                scope,
            },
        )
    }

    fn reconcile_node(
        self,
        root: &BSubtree,
        parent_scope: &AnyScope,
        parent: &Element,
        slot: DomSlot,
        bundle: &mut BNode,
    ) -> DomSlot {
        match bundle {
            // If the existing bundle is the same type, reuse it and update its properties
            BNode::Comp(ref mut bcomp)
                if self.type_id == bcomp.type_id && self.key == bcomp.key =>
            {
                self.reconcile(root, parent_scope, parent, slot, bcomp)
            }
            _ => self.replace(root, parent_scope, parent, slot, bundle),
        }
    }

    fn reconcile(
        self,
        _root: &BSubtree,
        _parent_scope: &AnyScope,
        _parent: &Element,
        slot: DomSlot,
        bcomp: &mut Self::Bundle,
    ) -> DomSlot {
        let VComp { mountable, key, .. } = self;

        bcomp.key = key;
        mountable.reuse(bcomp.scope.borrow(), slot);
        bcomp.own_position.to_position()
    }
}

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;
    use crate::dom_bundle::{Fragment, Hydratable};

    impl Hydratable for VComp {
        fn hydrate(
            self,
            root: &BSubtree,
            parent_scope: &AnyScope,
            parent: &Element,
            fragment: &mut Fragment,
        ) -> Self::Bundle {
            let VComp {
                type_id,
                mountable,
                key,
                ..
            } = self;
            let internal_ref = DynamicDomSlot::new_debug_trapped();

            let scoped = mountable.hydrate(
                root.clone(),
                parent_scope,
                parent.clone(),
                internal_ref.clone(),
                fragment,
            );

            BComp {
                type_id,
                scope: scoped,
                own_position: internal_ref,
                key,
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    use gloo::utils::document;
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    use web_sys::Element;

    use super::*;
    use crate::dom_bundle::Reconcilable;
    use crate::virtual_dom::{Key, VChild, VNode};
    use crate::{html, scheduler, Children, Component, Context, Html, Properties};

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
        let (_, mut bundle) = comp.attach(&root, &scope, &parent, DomSlot::at_end());
        scheduler::start_now();

        for _ in 0..10000 {
            let node = html! { <Comp></Comp> };
            node.reconcile_node(&root, &scope, &parent, DomSlot::at_end(), &mut bundle);
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
    fn vchild_partialeq() {
        let vchild1: VChild<Comp> = VChild::new(
            Props {
                field_1: 1,
                field_2: 1,
            },
            None,
        );

        let vchild2: VChild<Comp> = VChild::new(
            Props {
                field_1: 1,
                field_2: 1,
            },
            None,
        );

        let vchild3: VChild<Comp> = VChild::new(
            Props {
                field_1: 2,
                field_2: 2,
            },
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

        fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
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

    fn setup_parent() -> (BSubtree, AnyScope, Element) {
        let scope = AnyScope::test();
        let parent = document().create_element("div").unwrap();
        let root = BSubtree::create_root(&parent);

        document().body().unwrap().append_child(&parent).unwrap();

        (root, scope, parent)
    }

    fn get_html(node: Html, root: &BSubtree, scope: &AnyScope, parent: &Element) -> String {
        // clear parent
        parent.set_inner_html("");

        node.attach(root, scope, parent, DomSlot::at_end());
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
        <ul><li><span>a</span></li><li><span>b</span></li><li><span>c</span></li></ul>";

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
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod layout_tests {
    extern crate self as yew;

    use std::marker::PhantomData;

    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use crate::tests::layout_tests::{diff_layouts, TestLayout};
    use crate::{html, Children, Component, Context, Html, Properties};

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
