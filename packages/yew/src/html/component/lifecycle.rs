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
    pending_root: Option<VNode>,
    pending_updates: Vec<UpdateEvent<COMP>>,
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
            pending_root: None,
            pending_updates: Vec::new(),
        }
    }

    fn drain_pending_updates(&mut self, state: &Shared<Option<ComponentState<COMP>>>) {
        if !self.pending_updates.is_empty() {
            scheduler::push_component_updates(self.pending_updates.drain(..).map(|update| {
                Box::new(ComponentRunnable {
                    state: state.clone(),
                    event: update.into(),
                }) as Box<dyn Runnable>
            }));
        }
    }
}

/// Internal Component lifecycle event
pub(crate) enum ComponentLifecycleEvent<COMP: Component> {
    Create(CreateEvent<COMP>),
    Update(UpdateEvent<COMP>),
    Render,
    Rendered,
    Destroy,
}

impl<COMP: Component> From<CreateEvent<COMP>> for ComponentLifecycleEvent<COMP> {
    fn from(create: CreateEvent<COMP>) -> Self {
        Self::Create(create)
    }
}

pub(crate) struct CreateEvent<COMP: Component> {
    pub(crate) parent: Element,
    pub(crate) next_sibling: NodeRef,
    pub(crate) placeholder: VNode,
    pub(crate) node_ref: NodeRef,
    pub(crate) props: Rc<COMP::Properties>,
    pub(crate) scope: Scope<COMP>,
}

impl<COMP: Component> From<UpdateEvent<COMP>> for ComponentLifecycleEvent<COMP> {
    fn from(update: UpdateEvent<COMP>) -> Self {
        Self::Update(update)
    }
}

pub(crate) enum UpdateEvent<COMP: Component> {
    /// First update
    First,
    /// Wraps messages for a component.
    Message(COMP::Message),
    /// Wraps batch of messages for a component.
    MessageBatch(Vec<COMP::Message>),
    /// Wraps properties, node ref, and next sibling for a component.
    Properties(Rc<COMP::Properties>, NodeRef, NodeRef),
}

pub(crate) struct ComponentRunnable<COMP: Component> {
    pub(crate) state: Shared<Option<ComponentState<COMP>>>,
    pub(crate) event: ComponentLifecycleEvent<COMP>,
}

impl<COMP: Component> Runnable for ComponentRunnable<COMP> {
    fn run(self: Box<Self>) {
        let mut current_state = self.state.borrow_mut();
        match self.event {
            ComponentLifecycleEvent::Create(event) => {
                if current_state.is_none() {
                    *current_state = Some(ComponentState::new(
                        event.parent,
                        event.next_sibling,
                        event.placeholder,
                        event.node_ref,
                        event.scope.clone(),
                        event.props,
                    ));
                }
            }
            ComponentLifecycleEvent::Update(event) => {
                if let Some(mut state) = current_state.as_mut() {
                    if state.pending_root.is_some() {
                        state.pending_updates.push(event);
                        return;
                    }

                    let should_render = match event {
                        UpdateEvent::First => true,
                        UpdateEvent::Message(message) => {
                            state.component.update(&state.context, message)
                        }
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

                    if should_render {
                        state.pending_root = Some(state.component.view(&state.context));
                        state.context.scope.process(ComponentLifecycleEvent::Render);
                    };
                }
            }
            ComponentLifecycleEvent::Render => {
                if let Some(state) = current_state.as_mut() {
                    if let Some(mut new_root) = state.pending_root.take() {
                        std::mem::swap(&mut new_root, &mut state.root_node);
                        let ancestor = Some(new_root);
                        let new_root = &mut state.root_node;
                        let scope = state.context.scope.clone().into();
                        let next_sibling = state.next_sibling.clone();
                        let node = new_root.apply(&scope, &state.parent, next_sibling, ancestor);
                        state.node_ref.link(node);
                        state
                            .context
                            .scope
                            .process(ComponentLifecycleEvent::Rendered);
                    }
                }
            }
            ComponentLifecycleEvent::Rendered => {
                if let Some(mut state) = current_state.as_mut() {
                    let first_render = !state.has_rendered;
                    state.component.rendered(&state.context, first_render);
                    state.has_rendered = true;
                    state.drain_pending_updates(&self.state);
                }
            }
            ComponentLifecycleEvent::Destroy => {
                if let Some(mut state) = current_state.take() {
                    state.component.destroy(&state.context);
                    state.root_node.detach(&state.parent);
                    state.node_ref.set(None);
                }
            }
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

        fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> ShouldRender {
            false
        }

        fn changed(&mut self, _ctx: &Context<Self>) -> ShouldRender {
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

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
            if let Some(msg) = ctx.props().update_message.borrow_mut().take() {
                ctx.link().send_message(msg);
            }
            ctx.props()
                .lifecycle
                .borrow_mut()
                .push(format!("update({})", msg));
            msg
        }

        fn changed(&mut self, ctx: &Context<Self>) -> ShouldRender {
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

    fn test_lifecycle(props: Props, expected: &[String]) {
        let document = crate::utils::document();
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
            &[
                "create".to_string(),
                "view".to_string(),
                "child rendered".to_string(),
                "rendered(true)".to_string(),
            ],
        );

        test_lifecycle(
            Props {
                lifecycle: lifecycle.clone(),
                #[cfg(feature = "wasm_test")]
                create_message: Some(false),
                ..Props::default()
            },
            &[
                "create".to_string(),
                "view".to_string(),
                "child rendered".to_string(),
                "rendered(true)".to_string(),
                "update(false)".to_string(),
            ],
        );

        test_lifecycle(
            Props {
                lifecycle: lifecycle.clone(),
                view_message: RefCell::new(Some(true)),
                ..Props::default()
            },
            &[
                "create".to_string(),
                "view".to_string(),
                "child rendered".to_string(),
                "rendered(true)".to_string(),
                "update(true)".to_string(),
                "view".to_string(),
                "rendered(false)".to_string(),
            ],
        );

        test_lifecycle(
            Props {
                lifecycle: lifecycle.clone(),
                view_message: RefCell::new(Some(false)),
                ..Props::default()
            },
            &[
                "create".to_string(),
                "view".to_string(),
                "child rendered".to_string(),
                "rendered(true)".to_string(),
                "update(false)".to_string(),
            ],
        );

        test_lifecycle(
            Props {
                lifecycle: lifecycle.clone(),
                rendered_message: RefCell::new(Some(false)),
                ..Props::default()
            },
            &[
                "create".to_string(),
                "view".to_string(),
                "child rendered".to_string(),
                "rendered(true)".to_string(),
                "update(false)".to_string(),
            ],
        );

        test_lifecycle(
            Props {
                lifecycle: lifecycle.clone(),
                rendered_message: RefCell::new(Some(true)),
                ..Props::default()
            },
            &[
                "create".to_string(),
                "view".to_string(),
                "child rendered".to_string(),
                "rendered(true)".to_string(),
                "update(true)".to_string(),
                "view".to_string(),
                "rendered(false)".to_string(),
            ],
        );

        test_lifecycle(
            Props {
                lifecycle,
                #[cfg(feature = "wasm_test")]
                create_message: Some(true),
                update_message: RefCell::new(Some(true)),
                ..Props::default()
            },
            &[
                "create".to_string(),
                "view".to_string(),
                "child rendered".to_string(),
                "rendered(true)".to_string(),
                "update(true)".to_string(),
                "view".to_string(),
                "rendered(false)".to_string(),
                "update(true)".to_string(),
                "view".to_string(),
                "rendered(false)".to_string(),
            ],
        );
    }
}
