//! Component lifecycle module

use super::{Component, Scope};
use crate::scheduler::{self, Runnable, Shared};
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
        }
    }
}

pub(crate) struct CreateRunner<COMP: Component> {
    pub(crate) parent: Element,
    pub(crate) next_sibling: NodeRef,
    pub(crate) placeholder: VNode,
    pub(crate) node_ref: NodeRef,
    pub(crate) props: COMP::Properties,
    pub(crate) scope: Scope<COMP>,
}

impl<COMP: Component> Runnable for CreateRunner<COMP> {
    fn run(self: Box<Self>) {
        let mut current_state = self.scope.state.borrow_mut();
        if current_state.is_none() {
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
    Properties(COMP::Properties, NodeRef, NodeRef),
}

pub(crate) struct UpdateRunner<COMP: Component> {
    pub(crate) state: Shared<Option<ComponentState<COMP>>>,
    pub(crate) event: UpdateEvent<COMP>,
}

impl<COMP: Component> Runnable for UpdateRunner<COMP> {
    fn run(self: Box<Self>) {
        if let Some(mut state) = self.state.borrow_mut().as_mut() {
            let schedule_render = match self.event {
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
            state.component.destroy();
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
            let mut new_root = state.component.view();
            std::mem::swap(&mut new_root, &mut state.root_node);
            let ancestor = Some(new_root);
            let new_root = &mut state.root_node;
            let scope = state.scope.clone().into();
            let next_sibling = state.next_sibling.clone();
            let node = new_root.apply(&scope, &state.parent, next_sibling, ancestor);
            state.node_ref.link(node);
        }
    }
}

struct RenderedRunner<COMP: Component> {
    pub(crate) state: Shared<Option<ComponentState<COMP>>>,
}

impl<COMP: Component> Runnable for RenderedRunner<COMP> {
    fn run(self: Box<Self>) {
        if let Some(state) = self.state.borrow_mut().as_mut() {
            let first_render = !state.has_rendered;
            state.component.rendered(first_render);
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
        #[allow(dead_code)]
        #[cfg(feature = "wasm_test")]
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
            #[cfg(feature = "wasm_test")]
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
            html! { <Child lifecycle={self.props.lifecycle.clone()} /> }
        }
    }

    impl Drop for Comp {
        fn drop(&mut self) {
            self.props.lifecycle.borrow_mut().push("drop".into());
        }
    }

    fn test_lifecycle(props: Props, expected: &[&str]) {
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
                "view",
                "rendered(false)",
                "update(true)",
                "view",
                "rendered(false)",
            ],
        );

        // TODO: test render deduplication
    }
}
