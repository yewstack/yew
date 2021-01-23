//! Component lifecycle module

use super::{Component, Scope};
use crate::scheduler::{scheduler, ComponentRunnableType, Runnable, Shared};
use crate::virtual_dom::{VDiff, VNode};
use crate::NodeRef;
use web_sys::Element;

pub(crate) struct ComponentState<COMP: Component> {
    pub(crate) component: Box<COMP>,
    pub(crate) root_node: VNode,

    scope: Scope<COMP>,
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
        props: COMP::Properties,
    ) -> Self {
        let component = Box::new(COMP::create(props, scope.clone()));
        Self {
            component,
            root_node,
            scope,
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
            scheduler()
                .component
                .push_update_batch(self.pending_updates.drain(..).map(|update| {
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

impl<COMP: Component> ComponentLifecycleEvent<COMP> {
    pub(crate) fn as_runnable_type(&self) -> ComponentRunnableType {
        match self {
            Self::Create(_) => ComponentRunnableType::Create,
            Self::Update(_) => ComponentRunnableType::Update,
            Self::Render => ComponentRunnableType::Render,
            Self::Rendered => ComponentRunnableType::Rendered,
            Self::Destroy => ComponentRunnableType::Destroy,
        }
    }
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
    pub(crate) props: COMP::Properties,
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
    Properties(COMP::Properties, NodeRef, NodeRef),
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
                        UpdateEvent::Message(message) => state.component.update(message),
                        UpdateEvent::MessageBatch(messages) => {
                            let component = &mut state.component;
                            messages
                                .into_iter()
                                .fold(false, |acc, msg| component.update(msg) || acc)
                        }
                        UpdateEvent::Properties(props, node_ref, next_sibling) => {
                            // When components are updated, a new node ref could have been passed in
                            state.node_ref = node_ref;
                            // When components are updated, their siblings were likely also updated
                            state.next_sibling = next_sibling;
                            state.component.change(props)
                        }
                    };

                    if should_render {
                        state.pending_root = Some(state.component.view());
                        state.scope.process(ComponentLifecycleEvent::Render);
                    };
                }
            }
            ComponentLifecycleEvent::Render => {
                if let Some(state) = current_state.as_mut() {
                    if let Some(mut new_root) = state.pending_root.take() {
                        std::mem::swap(&mut new_root, &mut state.root_node);
                        let ancestor = Some(new_root);
                        let new_root = &mut state.root_node;
                        let scope = state.scope.clone().into();
                        let next_sibling = state.next_sibling.clone();
                        let node = new_root.apply(&scope, &state.parent, next_sibling, ancestor);
                        state.node_ref.link(node);
                        state.scope.process(ComponentLifecycleEvent::Rendered);
                    }
                }
            }
            ComponentLifecycleEvent::Rendered => {
                if let Some(mut state) = current_state.as_mut() {
                    let first_render = !state.has_rendered;
                    state.component.rendered(first_render);
                    state.has_rendered = true;
                    state.drain_pending_updates(&self.state);
                }
            }
            ComponentLifecycleEvent::Destroy => {
                if let Some(mut state) = current_state.take() {
                    state.component.destroy();
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

    #[derive(Clone, Properties, Default)]
    struct ChildProps {
        lifecycle: Rc<RefCell<Vec<String>>>,
    }

    struct Child {
        props: ChildProps,
    }

    impl Component for Child {
        type Message = ();
        type Properties = ChildProps;

        fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
            Child { props }
        }

        fn rendered(&mut self, _first_render: bool) {
            self.props
                .lifecycle
                .borrow_mut()
                .push("child rendered".into());
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            false
        }

        fn change(&mut self, _: Self::Properties) -> ShouldRender {
            false
        }

        fn view(&self) -> Html {
            html! {}
        }
    }

    #[derive(Clone, Properties, Default)]
    struct Props {
        lifecycle: Rc<RefCell<Vec<String>>>,
        create_message: Option<bool>,
        update_message: RefCell<Option<bool>>,
        view_message: RefCell<Option<bool>>,
        rendered_message: RefCell<Option<bool>>,
    }

    struct Comp {
        props: Props,
        link: ComponentLink<Self>,
    }

    impl Component for Comp {
        type Message = bool;
        type Properties = Props;

        fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
            props.lifecycle.borrow_mut().push("create".into());
            if let Some(msg) = props.create_message {
                link.send_message(msg);
            }
            Comp { props, link }
        }

        fn rendered(&mut self, first_render: bool) {
            if let Some(msg) = self.props.rendered_message.borrow_mut().take() {
                self.link.send_message(msg);
            }
            self.props
                .lifecycle
                .borrow_mut()
                .push(format!("rendered({})", first_render));
        }

        fn update(&mut self, msg: Self::Message) -> ShouldRender {
            if let Some(msg) = self.props.update_message.borrow_mut().take() {
                self.link.send_message(msg);
            }
            self.props
                .lifecycle
                .borrow_mut()
                .push(format!("update({})", msg));
            msg
        }

        fn change(&mut self, _: Self::Properties) -> ShouldRender {
            self.props.lifecycle.borrow_mut().push("change".into());
            false
        }

        fn view(&self) -> Html {
            if let Some(msg) = self.props.view_message.borrow_mut().take() {
                self.link.send_message(msg);
            }
            self.props.lifecycle.borrow_mut().push("view".into());
            html! { <Child lifecycle=self.props.lifecycle.clone() /> }
        }
    }

    impl Drop for Comp {
        fn drop(&mut self) {
            self.props.lifecycle.borrow_mut().push("drop".into());
        }
    }

    fn test_lifecycle(props: Props, expected: &[String]) {
        let document = crate::utils::document();
        let scope = Scope::<Comp>::new(None);
        let el = document.create_element("div").unwrap();
        let lifecycle = props.lifecycle.clone();

        lifecycle.borrow_mut().clear();
        scope.mount_in_place(el, NodeRef::default(), NodeRef::default(), props);

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
