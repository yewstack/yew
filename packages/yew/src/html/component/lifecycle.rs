//! Component lifecycle module

use super::{Component, Scope};
use crate::scheduler::{self, Runnable, Shared};
use crate::virtual_dom::{VDiff, VNode};
use crate::{Context, NodeRef};
use std::rc::Rc;
use web_sys::Element;

pub(crate) struct ComponentState<COMP: Component> {
    pub(crate) component: Box<COMP>,
    pub(crate) root_node: VNode,

    context: Context<COMP>,
    parent: Element,
    next_sibling: NodeRef,
    node_ref: NodeRef,
    has_rendered: bool,

    // Used for debug logging
    #[cfg(debug_assertions)]
    pub(crate) vcomp_id: u64,
}

impl<COMP: Component> ComponentState<COMP> {
    pub(crate) fn new(
        parent: Element,
        next_sibling: NodeRef,
        root_node: VNode,
        node_ref: NodeRef,
        scope: Scope<COMP>,
        props: Rc<COMP::Properties>,
    ) -> Self {
        #[cfg(debug_assertions)]
        let vcomp_id = {
            use super::Scoped;

            scope.to_any().vcomp_id
        };
        let context = Context { scope, props };

        let component = Box::new(COMP::create(&context));
        Self {
            component,
            root_node,
            context,
            parent,
            next_sibling,
            node_ref,
            has_rendered: false,

            #[cfg(debug_assertions)]
            vcomp_id,
        }
    }
}

pub(crate) struct CreateRunner<COMP: Component> {
    pub(crate) parent: Element,
    pub(crate) next_sibling: NodeRef,
    pub(crate) placeholder: VNode,
    pub(crate) node_ref: NodeRef,
    pub(crate) props: Rc<COMP::Properties>,
    pub(crate) scope: Scope<COMP>,
}

impl<COMP: Component> Runnable for CreateRunner<COMP> {
    fn run(self: Box<Self>) {
        let mut current_state = self.scope.state.borrow_mut();
        if current_state.is_none() {
            #[cfg(debug_assertions)]
            crate::virtual_dom::vcomp::log_event(self.scope.vcomp_id, "create");

            *current_state = Some(ComponentState::new(
                self.parent,
                self.next_sibling,
                self.placeholder,
                self.node_ref,
                self.scope.clone(),
                self.props,
            ));
        }
    }
}

pub(crate) enum UpdateEvent<COMP: Component> {
    /// Wraps messages for a component.
    Message(COMP::Message),
    /// Wraps batch of messages for a component.
    MessageBatch(Vec<COMP::Message>),
    /// Wraps properties, node ref, and next sibling for a component.
    Properties(Rc<COMP::Properties>, NodeRef, NodeRef),
}

pub(crate) struct UpdateRunner<COMP: Component> {
    pub(crate) state: Shared<Option<ComponentState<COMP>>>,
    pub(crate) event: UpdateEvent<COMP>,
}

impl<COMP: Component> Runnable for UpdateRunner<COMP> {
    fn run(self: Box<Self>) {
        if let Some(mut state) = self.state.borrow_mut().as_mut() {
            let schedule_render = match self.event {
                UpdateEvent::Message(message) => state.component.update(&state.context, message),
                UpdateEvent::MessageBatch(messages) => {
                    messages.into_iter().fold(false, |acc, msg| {
                        state.component.update(&state.context, msg) || acc
                    })
                }
                UpdateEvent::Properties(props, node_ref, next_sibling) => {
                    // When components are updated, a new node ref could have been passed in
                    state.node_ref = node_ref;
                    // When components are updated, their siblings were likely also updated
                    state.next_sibling = next_sibling;
                    // Only trigger changed if props were changed
                    if state.context.props != props {
                        state.context.props = Rc::clone(&props);
                        state.component.changed(&state.context)
                    } else {
                        false
                    }
                }
            };

            #[cfg(debug_assertions)]
            crate::virtual_dom::vcomp::log_event(
                state.vcomp_id,
                format!("update(schedule_render={})", schedule_render),
            );

            if schedule_render {
                scheduler::push_component_render(
                    self.state.as_ptr() as usize,
                    RenderRunner {
                        state: self.state.clone(),
                    },
                    RenderedRunner {
                        state: self.state.clone(),
                    },
                );
                // Only run from the scheduler, so no need to call `scheduler::start()`
            }
        }
    }
}

pub(crate) struct DestroyRunner<COMP: Component> {
    pub(crate) state: Shared<Option<ComponentState<COMP>>>,
}

impl<COMP: Component> Runnable for DestroyRunner<COMP> {
    fn run(self: Box<Self>) {
        if let Some(mut state) = self.state.borrow_mut().take() {
            #[cfg(debug_assertions)]
            crate::virtual_dom::vcomp::log_event(state.vcomp_id, "destroy");

            state.component.destroy(&state.context);
            state.root_node.detach(&state.parent);
            state.node_ref.set(None);
        }
    }
}

pub(crate) struct RenderRunner<COMP: Component> {
    pub(crate) state: Shared<Option<ComponentState<COMP>>>,
}

impl<COMP: Component> Runnable for RenderRunner<COMP> {
    fn run(self: Box<Self>) {
        if let Some(state) = self.state.borrow_mut().as_mut() {
            #[cfg(debug_assertions)]
            crate::virtual_dom::vcomp::log_event(state.vcomp_id, "render");

            let mut new_root = state.component.view(&state.context);
            std::mem::swap(&mut new_root, &mut state.root_node);
            let ancestor = Some(new_root);
            let new_root = &mut state.root_node;
            let scope = state.context.scope.clone().into();
            let next_sibling = state.next_sibling.clone();
            let node = new_root.apply(&scope, &state.parent, next_sibling, ancestor);
            state.node_ref.link(node);
        }
    }
}

pub(crate) struct RenderedRunner<COMP: Component> {
    pub(crate) state: Shared<Option<ComponentState<COMP>>>,
}

impl<COMP: Component> Runnable for RenderedRunner<COMP> {
    fn run(self: Box<Self>) {
        if let Some(state) = self.state.borrow_mut().as_mut() {
            #[cfg(debug_assertions)]
            crate::virtual_dom::vcomp::log_event(state.vcomp_id, "rendered");

            let first_render = !state.has_rendered;
            state.component.rendered(&state.context, first_render);
            state.has_rendered = true;
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate self as yew;

    use crate::html;
    use crate::html::*;
    use crate::Properties;
    use std::ops::Deref;
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Clone, Properties, Default, PartialEq)]
    struct ChildProps {
        lifecycle: Rc<RefCell<Vec<String>>>,
    }

    struct Child {}

    impl Component for Child {
        type Message = ();
        type Properties = ChildProps;

        fn create(_ctx: &Context<Self>) -> Self {
            Child {}
        }

        fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
            ctx.props()
                .lifecycle
                .borrow_mut()
                .push("child rendered".into());
        }

        fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
            false
        }

        fn changed(&mut self, _ctx: &Context<Self>) -> bool {
            false
        }

        fn view(&self, _ctx: &Context<Self>) -> Html {
            html! {}
        }
    }

    #[derive(Clone, Properties, Default, PartialEq)]
    struct Props {
        lifecycle: Rc<RefCell<Vec<String>>>,
        #[allow(dead_code)]
        #[cfg(feature = "wasm_test")]
        create_message: Option<bool>,
        update_message: RefCell<Option<bool>>,
        view_message: RefCell<Option<bool>>,
        rendered_message: RefCell<Option<bool>>,
    }

    struct Comp {
        lifecycle: Rc<RefCell<Vec<String>>>,
    }

    impl Component for Comp {
        type Message = bool;
        type Properties = Props;

        fn create(ctx: &Context<Self>) -> Self {
            ctx.props().lifecycle.borrow_mut().push("create".into());
            #[cfg(feature = "wasm_test")]
            if let Some(msg) = ctx.props().create_message {
                ctx.link().send_message(msg);
            }
            Comp {
                lifecycle: Rc::clone(&ctx.props().lifecycle),
            }
        }

        fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
            if let Some(msg) = ctx.props().rendered_message.borrow_mut().take() {
                ctx.link().send_message(msg);
            }
            ctx.props()
                .lifecycle
                .borrow_mut()
                .push(format!("rendered({})", first_render));
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            if let Some(msg) = ctx.props().update_message.borrow_mut().take() {
                ctx.link().send_message(msg);
            }
            ctx.props()
                .lifecycle
                .borrow_mut()
                .push(format!("update({})", msg));
            msg
        }

        fn changed(&mut self, ctx: &Context<Self>) -> bool {
            self.lifecycle = Rc::clone(&ctx.props().lifecycle);
            self.lifecycle.borrow_mut().push("change".into());
            false
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            if let Some(msg) = ctx.props().view_message.borrow_mut().take() {
                ctx.link().send_message(msg);
            }
            self.lifecycle.borrow_mut().push("view".into());
            html! { <Child lifecycle={self.lifecycle.clone()} /> }
        }
    }

    impl Drop for Comp {
        fn drop(&mut self) {
            self.lifecycle.borrow_mut().push("drop".into());
        }
    }

    fn test_lifecycle(props: Props, expected: &[&str]) {
        let document = gloo_utils::document();
        let scope = Scope::<Comp>::new(None);
        let el = document.create_element("div").unwrap();
        let lifecycle = props.lifecycle.clone();

        lifecycle.borrow_mut().clear();
        scope.mount_in_place(el, NodeRef::default(), NodeRef::default(), Rc::new(props));

        assert_eq!(&lifecycle.borrow_mut().deref()[..], expected);
    }

    #[test]
    fn lifecycle_tests() {
        let lifecycle: Rc<RefCell<Vec<String>>> = Rc::default();

        test_lifecycle(
            Props {
                lifecycle: lifecycle.clone(),
                ..Props::default()
            },
            &["create", "view", "child rendered", "rendered(true)"],
        );

        test_lifecycle(
            Props {
                lifecycle: lifecycle.clone(),
                #[cfg(feature = "wasm_test")]
                create_message: Some(false),
                ..Props::default()
            },
            &[
                "create",
                "view",
                "child rendered",
                "rendered(true)",
                "update(false)",
            ],
        );

        test_lifecycle(
            Props {
                lifecycle: lifecycle.clone(),
                view_message: RefCell::new(Some(true)),
                ..Props::default()
            },
            &[
                "create",
                "view",
                "child rendered",
                "rendered(true)",
                "update(true)",
                "view",
                "rendered(false)",
            ],
        );

        test_lifecycle(
            Props {
                lifecycle: lifecycle.clone(),
                view_message: RefCell::new(Some(false)),
                ..Props::default()
            },
            &[
                "create",
                "view",
                "child rendered",
                "rendered(true)",
                "update(false)",
            ],
        );

        test_lifecycle(
            Props {
                lifecycle: lifecycle.clone(),
                rendered_message: RefCell::new(Some(false)),
                ..Props::default()
            },
            &[
                "create",
                "view",
                "child rendered",
                "rendered(true)",
                "update(false)",
            ],
        );

        test_lifecycle(
            Props {
                lifecycle: lifecycle.clone(),
                rendered_message: RefCell::new(Some(true)),
                ..Props::default()
            },
            &[
                "create",
                "view",
                "child rendered",
                "rendered(true)",
                "update(true)",
                "view",
                "rendered(false)",
            ],
        );

        // This also tests render deduplication after the first render
        test_lifecycle(
            Props {
                lifecycle,
                #[cfg(feature = "wasm_test")]
                create_message: Some(true),
                update_message: RefCell::new(Some(true)),
                ..Props::default()
            },
            &[
                "create",
                "view",
                "child rendered",
                "rendered(true)",
                "update(true)",
                "update(true)",
                "view",
                "rendered(false)",
            ],
        );
    }
}
