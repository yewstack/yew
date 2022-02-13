//! Component lifecycle module

use super::{AnyScope, BaseComponent, Scope};
use crate::html::{RenderError, RenderResult};
use crate::scheduler::{self, Runnable, Shared};
use crate::suspense::{BaseSuspense, Suspension};
#[cfg(feature = "hydration")]
use crate::virtual_dom::{Fragment, VHydrate};
use crate::virtual_dom::{VDiff, VNode};
use crate::Callback;
use crate::{Context, NodeRef};
#[cfg(feature = "ssr")]
use futures::channel::oneshot;
use std::any::Any;
use std::rc::Rc;
use web_sys::Element;

/// A State to track current component rendering status.
pub(crate) enum Rendered {
    Render {
        parent: Element,

        next_sibling: NodeRef,
        node_ref: NodeRef,

        root_node: VNode,
    },
    #[cfg(feature = "hydration")]
    Hydration {
        parent: Element,

        next_sibling: NodeRef,
        node_ref: NodeRef,

        fragment: Fragment,
    },
    #[cfg(feature = "ssr")]
    Ssr {
        sender: Option<oneshot::Sender<VNode>>,
    },
}

impl Rendered {
    pub fn root_vnode(&self) -> Option<&VNode> {
        match self {
            Rendered::Render { ref root_node, .. } => Some(root_node),

            #[cfg(feature = "hydration")]
            Rendered::Hydration { .. } => None,

            #[cfg(feature = "ssr")]
            Rendered::Ssr { .. } => None,
        }
    }
}

pub(crate) struct CompStateInner<COMP>
where
    COMP: BaseComponent,
{
    pub(crate) component: COMP,
    pub(crate) context: Context<COMP>,
}

/// A trait to provide common,
/// generic free behaviour across all components to reduce code size.
///
/// Mostly a thin wrapper that passes the context to a component's lifecycle
/// methods.
pub(crate) trait Stateful {
    fn id(&self) -> usize;

    fn view(&self) -> RenderResult<VNode>;
    fn rendered(&mut self, first_render: bool);
    fn destroy(&mut self);

    fn any_scope(&self) -> AnyScope;

    fn flush_messages(&mut self) -> bool;
    fn props_changed(&mut self, props: Rc<dyn Any>) -> bool;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<COMP> Stateful for CompStateInner<COMP>
where
    COMP: BaseComponent,
{
    fn id(&self) -> usize {
        self.context.scope.id
    }

    fn view(&self) -> RenderResult<VNode> {
        self.component.view(&self.context)
    }

    fn rendered(&mut self, first_render: bool) {
        self.component.rendered(&self.context, first_render)
    }

    fn destroy(&mut self) {
        self.component.destroy(&self.context);
    }

    fn any_scope(&self) -> AnyScope {
        self.context.link().clone().into()
    }

    fn flush_messages(&mut self) -> bool {
        self.context
            .link()
            .pending_messages
            .drain()
            .into_iter()
            .fold(false, |acc, msg| {
                self.component.update(&self.context, msg) || acc
            })
    }

    fn props_changed(&mut self, props: Rc<dyn Any>) -> bool {
        let props = match Rc::downcast::<COMP::Properties>(props) {
            Ok(m) => m,
            _ => return false,
        };

        if self.context.props != props {
            self.context.props = Rc::clone(&props);
            self.component.changed(&self.context)
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub(crate) struct ComponentState {
    pub(crate) inner: Box<dyn Stateful>,

    pub(crate) rendered: Rendered,

    has_rendered: bool,

    suspension: Option<Suspension>,
}

pub(crate) struct CreateRunner<COMP: BaseComponent> {
    pub(crate) rendered: Rendered,
    pub(crate) props: Rc<COMP::Properties>,
    pub(crate) scope: Scope<COMP>,
}

impl<COMP: BaseComponent> Runnable for CreateRunner<COMP> {
    fn run(self: Box<Self>) {
        let state = self.scope.state.clone();
        let mut current_state = state.borrow_mut();

        if current_state.is_none() {
            #[cfg(debug_assertions)]
            crate::virtual_dom::vcomp::log_event(self.scope.id, "create");

            let Self {
                props,
                rendered,
                scope,
            } = *self;

            let context = Context { scope, props };

            let inner = Box::new(CompStateInner {
                component: COMP::create(&context),
                context,
            });

            let state = ComponentState {
                inner,
                rendered,
                suspension: None,
                has_rendered: false,
            };

            *current_state = Some(state);
        }
    }
}

pub(crate) enum UpdateEvent {
    /// Drain messages for a component.
    Message,
    /// Wraps properties, node ref, and next sibling for a component.
    Properties(Rc<dyn Any>, NodeRef, NodeRef),
    /// Shift Scope.
    Shift(Element, NodeRef),
}

pub(crate) struct UpdateRunner {
    pub(crate) state: Shared<Option<ComponentState>>,
    pub(crate) event: UpdateEvent,
}

impl Runnable for UpdateRunner {
    fn run(self: Box<Self>) {
        if let Some(state) = self.state.borrow_mut().as_mut() {
            let schedule_render = match self.event {
                UpdateEvent::Message => state.inner.flush_messages(),
                UpdateEvent::Properties(props, next_node_ref, next_sibling) => {
                    match state.rendered {
                        Rendered::Render {
                            ref mut node_ref,
                            next_sibling: ref mut current_next_sibling,
                            ..
                        } => {
                            // When components are updated, a new node ref could have been passed in
                            *node_ref = next_node_ref;
                            // When components are updated, their siblings were likely also updated
                            *current_next_sibling = next_sibling;
                            // Only trigger changed if props were changed
                            state.inner.props_changed(props)
                        }

                        #[cfg(feature = "hydration")]
                        Rendered::Hydration {
                            ref mut node_ref,
                            next_sibling: ref mut current_next_sibling,
                            ..
                        } => {
                            // When components are updated, a new node ref could have been passed in
                            *node_ref = next_node_ref;
                            // When components are updated, their siblings were likely also updated
                            *current_next_sibling = next_sibling;
                            // Only trigger changed if props were changed
                            state.inner.props_changed(props)
                        }

                        #[cfg(feature = "ssr")]
                        Rendered::Ssr { .. } => state.inner.props_changed(props),
                    }
                }

                UpdateEvent::Shift(next_parent, next_sibling) => {
                    match state.rendered {
                        Rendered::Render {
                            ref root_node,
                            ref mut parent,
                            next_sibling: ref mut current_next_sibling,
                            ..
                        } => {
                            root_node.shift(parent, &next_parent, next_sibling.clone());

                            *parent = next_parent;
                            *current_next_sibling = next_sibling;
                        }

                        // We need to shift the hydrate fragment if the component is not hydrated.
                        #[cfg(feature = "hydration")]
                        Rendered::Hydration {
                            ref fragment,
                            ref mut parent,
                            next_sibling: ref mut current_next_sibling,
                            ..
                        } => {
                            fragment.shift(parent, &next_parent, next_sibling.clone());

                            *parent = next_parent;
                            *current_next_sibling = next_sibling;
                        }

                        // Shifting is not possible during SSR.
                        #[cfg(feature = "ssr")]
                        Rendered::Ssr { .. } => {
                            #[cfg(debug_assertions)]
                            panic!("shifting is not possible during SSR");
                        }
                    }

                    false
                }
            };

            #[cfg(debug_assertions)]
            crate::virtual_dom::vcomp::log_event(
                state.inner.id(),
                format!("update(schedule_render={})", schedule_render),
            );

            if schedule_render {
                scheduler::push_component_render(
                    state.inner.id(),
                    RenderRunner {
                        state: self.state.clone(),
                    },
                );
                // Only run from the scheduler, so no need to call `scheduler::start()`
            }
        }
    }
}

pub(crate) struct DestroyRunner {
    pub(crate) state: Shared<Option<ComponentState>>,
    pub(crate) parent_to_detach: bool,
}

impl Runnable for DestroyRunner {
    fn run(self: Box<Self>) {
        if let Some(mut state) = self.state.borrow_mut().take() {
            #[cfg(debug_assertions)]
            crate::virtual_dom::vcomp::log_event(state.inner.id(), "destroy");

            state.inner.destroy();

            match state.rendered {
                Rendered::Render {
                    ref mut root_node,
                    ref parent,
                    ref node_ref,
                    ..
                } => {
                    root_node.detach(parent, self.parent_to_detach);

                    node_ref.set(None);
                }
                // We need to detach the hydrate fragment if the component is not hydrated.
                #[cfg(feature = "hydration")]
                Rendered::Hydration {
                    ref fragment,
                    ref parent,
                    ref node_ref,
                    ..
                } => {
                    for node in fragment.iter() {
                        parent
                            .remove_child(node)
                            .expect("failed to remove fragment node.");
                    }

                    node_ref.set(None);
                }

                #[cfg(feature = "ssr")]
                Rendered::Ssr { .. } => {}
            }
        }
    }
}

pub(crate) struct RenderRunner {
    pub(crate) state: Shared<Option<ComponentState>>,
}

impl RenderRunner {
    fn render(&self, state: &mut ComponentState, new_root: VNode) {
        // Currently not suspended, we remove any previous suspension and update
        // normally.

        if let Some(m) = state.suspension.take() {
            let comp_scope = state.inner.any_scope();

            let suspense_scope = comp_scope.find_parent_scope::<BaseSuspense>().unwrap();
            let suspense = suspense_scope.get_component().unwrap();

            suspense.resume(m);
        }

        let scope = state.inner.any_scope();

        match state.rendered {
            Rendered::Render {
                root_node: ref mut current_root,
                ref parent,
                ref next_sibling,
                ref node_ref,
                ..
            } => {
                let mut root = new_root;
                std::mem::swap(&mut root, current_root);

                let ancestor = root;

                let node = current_root.apply(&scope, parent, next_sibling.clone(), Some(ancestor));

                node_ref.link(node);

                let first_render = !state.has_rendered;
                state.has_rendered = true;

                scheduler::push_component_rendered(
                    state.inner.id(),
                    RenderedRunner {
                        state: self.state.clone(),
                        first_render,
                    },
                    first_render,
                );
            }

            #[cfg(feature = "hydration")]
            Rendered::Hydration {
                ref mut fragment,
                ref parent,
                ref node_ref,
                ref next_sibling,
            } => {
                // We schedule a "first" render to run immediately after hydration,
                // for the following reason:
                // 1. Fix NodeRef (first_node and next_sibling)
                // 2. Switch from fallback UI to children UI for <Suspense /> component (if it is
                //    not meant to be suspended.).
                scheduler::push_component_render(
                    state.inner.id(),
                    RenderRunner {
                        state: self.state.clone(),
                    },
                );

                let mut root = new_root;

                // This first node is not guaranteed to be correct here.
                // As it may be a comment node that is removed afterwards.
                // but we link it anyways.
                let node = root.hydrate(&scope, parent, fragment);

                // We trim all text nodes before checking as it's likely these are whitespaces.
                fragment.trim_start_text_nodes(parent);

                assert!(fragment.is_empty(), "expected end of component, found node");

                node_ref.link(node);

                state.rendered = Rendered::Render {
                    root_node: root,
                    parent: parent.clone(),
                    node_ref: node_ref.clone(),
                    next_sibling: next_sibling.clone(),
                };
            }

            #[cfg(feature = "ssr")]
            Rendered::Ssr { ref mut sender } => {
                if let Some(tx) = sender.take() {
                    tx.send(new_root).unwrap();
                }
            }
        };
    }

    fn suspend(&self, state: &mut ComponentState, suspension: Suspension) {
        // Currently suspended, we re-use previous root node and send
        // suspension to parent element.
        let shared_state = self.state.clone();

        if suspension.resumed() {
            // schedule a render immediately if suspension is resumed.

            scheduler::push_component_render(
                state.inner.id(),
                RenderRunner {
                    state: shared_state,
                },
            );
        } else {
            // We schedule a render after current suspension is resumed.

            let comp_scope = state.inner.any_scope();

            let suspense_scope = comp_scope
                .find_parent_scope::<BaseSuspense>()
                .expect("To suspend rendering, a <Suspense /> component is required.");
            let suspense = suspense_scope.get_component().unwrap();

            let comp_id = state.inner.id();

            suspension.listen(Callback::from(move |_| {
                scheduler::push_component_render(
                    comp_id,
                    RenderRunner {
                        state: shared_state.clone(),
                    },
                );
                scheduler::start();
            }));

            if let Some(ref last_suspension) = state.suspension {
                if &suspension != last_suspension {
                    // We remove previous suspension from the suspense.
                    suspense.resume(last_suspension.clone());
                }
            }
            state.suspension = Some(suspension.clone());

            suspense.suspend(suspension);
        }
    }
}

impl Runnable for RenderRunner {
    fn run(self: Box<Self>) {
        if let Some(state) = self.state.borrow_mut().as_mut() {
            #[cfg(debug_assertions)]
            crate::virtual_dom::vcomp::log_event(state.inner.id(), "render");

            match state.inner.view() {
                Ok(m) => self.render(state, m),
                Err(RenderError::Suspended(m)) => self.suspend(state, m),
            };
        }
    }
}

pub(crate) struct RenderedRunner {
    pub(crate) state: Shared<Option<ComponentState>>,
    first_render: bool,
}

impl Runnable for RenderedRunner {
    fn run(self: Box<Self>) {
        if let Some(state) = self.state.borrow_mut().as_mut() {
            #[cfg(debug_assertions)]
            crate::virtual_dom::vcomp::log_event(state.inner.id(), "rendered");

            match state.rendered {
                #[cfg(feature = "ssr")]
                Rendered::Ssr { .. } => {}
                #[cfg(feature = "hydration")]
                Rendered::Hydration { .. } => {}

                // We only call rendered when the component is rendered & not suspended.
                Rendered::Render { .. } => {
                    if state.suspension.is_none() {
                        state.inner.rendered(self.first_render);
                    }
                }
            }
        }
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
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
        crate::scheduler::start_now();

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
