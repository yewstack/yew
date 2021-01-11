//! Component lifecycle module

use super::{Component, Scope};
use crate::scheduler::{scheduler, ComponentRunnableType, Runnable, Shared};
use crate::virtual_dom::{VDiff, VNode};
use crate::NodeRef;
use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::web::Element;
    } else if #[cfg(feature = "web_sys")] {
        use web_sys::Element;
    }
}

pub(crate) struct ComponentState<COMP: Component> {
    parent: Element,
    next_sibling: NodeRef,
    node_ref: NodeRef,

    scope: Scope<COMP>,
    pub(crate) component: Box<COMP>,

    pub(crate) placeholder: Option<VNode>,
    pub(crate) last_root: Option<VNode>,
    new_root: Option<VNode>,
    has_rendered: bool,
    pending_updates: Vec<UpdateTask<COMP>>,
}

impl<COMP: Component> ComponentState<COMP> {
    /// Creates a new `ComponentState`, also invokes the `create()`
    /// method on component to create it.
    pub(crate) fn new(
        parent: Element,
        next_sibling: NodeRef,
        placeholder: Option<VNode>,
        node_ref: NodeRef,
        scope: Scope<COMP>,
        props: COMP::Properties,
    ) -> Self {
        let component = Box::new(COMP::create(props, scope.clone()));
        Self {
            parent,
            next_sibling,
            node_ref,
            scope,
            component,
            placeholder,
            last_root: None,
            new_root: None,
            has_rendered: false,
            pending_updates: Vec::new(),
        }
    }
}

/// Internal Component runnable tasks
pub(crate) enum ComponentTask<COMP: Component> {
    Create(CreateTask<COMP>),
    Update(UpdateTask<COMP>),
    Render(bool),
    Rendered(bool),
    Destroy,
}

impl<COMP: Component> ComponentTask<COMP> {
    pub(crate) fn as_runnable_type(&self) -> ComponentRunnableType {
        match self {
            Self::Create(_) => ComponentRunnableType::Create,
            Self::Update(_) => ComponentRunnableType::Update,
            Self::Render(_) => ComponentRunnableType::Render,
            Self::Rendered(_) => ComponentRunnableType::Rendered,
            Self::Destroy => ComponentRunnableType::Destroy,
        }
    }
}

impl<COMP: Component> From<CreateTask<COMP>> for ComponentTask<COMP> {
    fn from(create: CreateTask<COMP>) -> Self {
        Self::Create(create)
    }
}

pub(crate) struct CreateTask<COMP: Component> {
    pub(crate) parent: Element,
    pub(crate) next_sibling: NodeRef,
    pub(crate) placeholder: Option<VNode>,
    pub(crate) node_ref: NodeRef,
    pub(crate) props: COMP::Properties,
    pub(crate) scope: Scope<COMP>,
}

impl<COMP: Component> From<UpdateTask<COMP>> for ComponentTask<COMP> {
    fn from(update: UpdateTask<COMP>) -> Self {
        Self::Update(update)
    }
}

pub(crate) enum UpdateTask<COMP: Component> {
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
    pub(crate) task: ComponentTask<COMP>,
}

impl<COMP: Component> Runnable for ComponentRunnable<COMP> {
    fn run(self: Box<Self>) {
        let mut current_state = self.state.borrow_mut();
        match self.task {
            ComponentTask::Create(this) => {
                if current_state.is_none() {
                    *current_state = Some(ComponentState::new(
                        this.parent,
                        this.next_sibling,
                        this.placeholder,
                        this.node_ref,
                        this.scope.clone(),
                        this.props,
                    ));
                }
            }
            ComponentTask::Render(first_render) => {
                if let Some(mut state) = current_state.as_mut() {
                    // Skip render if we haven't seen the "first render" yet
                    if !first_render && state.last_root.is_none() {
                        return;
                    }

                    if let Some(mut new_root) = state.new_root.take() {
                        let last_root = state.last_root.take().or_else(|| state.placeholder.take());
                        let parent_scope = state.scope.clone().into();
                        let next_sibling = state.next_sibling.clone();
                        let node =
                            new_root.apply(&parent_scope, &state.parent, next_sibling, last_root);
                        state.node_ref.link(node);
                        state.last_root = Some(new_root);
                        state.scope.run(ComponentTask::Rendered(first_render));
                    }
                }
            }
            ComponentTask::Rendered(first_render) => {
                if let Some(mut state) = current_state.as_mut() {
                    // Don't call rendered if we haven't seen the "first render" yet
                    if !first_render && !state.has_rendered {
                        return;
                    }

                    state.has_rendered = true;
                    state.component.rendered(first_render);
                    if !state.pending_updates.is_empty() {
                        scheduler().push_comp_update_batch(state.pending_updates.drain(..).map(
                            |update| {
                                Box::new(ComponentRunnable {
                                    state: self.state.clone(),
                                    task: update.into(),
                                }) as Box<dyn Runnable>
                            },
                        ));
                    }
                }
            }
            ComponentTask::Update(event) => {
                if let Some(mut state) = current_state.as_mut() {
                    if state.new_root.is_some() {
                        state.pending_updates.push(event);
                        return;
                    }

                    let first_update = matches!(event, UpdateTask::First);

                    let should_update = match event {
                        UpdateTask::First => true,
                        UpdateTask::Message(message) => state.component.update(message),
                        UpdateTask::MessageBatch(messages) => {
                            let component = &mut state.component;
                            messages
                                .into_iter()
                                .fold(false, |acc, msg| component.update(msg) || acc)
                        }
                        UpdateTask::Properties(props, node_ref, next_sibling) => {
                            // When components are updated, a new node ref could have been passed in
                            state.node_ref = node_ref;
                            // When components are updated, their siblings were likely also updated
                            state.next_sibling = next_sibling;
                            state.component.change(props)
                        }
                    };

                    if should_update {
                        state.new_root = Some(state.component.view());
                        state.scope.run(ComponentTask::Render(first_update));
                    };
                }
            }
            ComponentTask::Destroy => {
                if let Some(mut state) = current_state.take() {
                    state.component.destroy();
                    if let Some(last_frame) = &mut state.last_root {
                        last_frame.detach(&state.parent);
                    }
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
        scope.mount_in_place(el, NodeRef::default(), None, NodeRef::default(), props);

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
