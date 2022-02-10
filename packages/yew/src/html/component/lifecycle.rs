//! Component lifecycle module

use super::{AnyScope, BaseComponent, Scope};
use crate::html::{RenderError, RenderResult};
use crate::scheduler::{self, Runnable, Shared};
use crate::suspense::{Suspense, Suspension};
#[cfg(feature = "hydration")]
use crate::virtual_dom::{trim_start_text_nodes, VHydrate};
use crate::virtual_dom::{VDiff, VNode};
use crate::Callback;
use crate::{Context, NodeRef};
#[cfg(feature = "ssr")]
use futures::channel::oneshot;
use std::any::Any;
#[cfg(feature = "hydration")]
use std::collections::VecDeque;
use std::rc::Rc;
use web_sys::Element;
#[cfg(feature = "hydration")]
use web_sys::Node;

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

    pub(crate) root_node: VNode,

    /// When a component has no parent, it means that it should not be rendered.
    parent: Option<Element>,

    next_sibling: NodeRef,
    node_ref: NodeRef,
    has_rendered: bool,

    suspension: Option<Suspension>,

    #[cfg(feature = "hydration")]
    hydrate_fragment: Option<VecDeque<Node>>,

    #[cfg(feature = "ssr")]
    html_sender: Option<oneshot::Sender<VNode>>,

    // Used for debug logging
    #[cfg(debug_assertions)]
    pub(crate) vcomp_id: usize,
}

pub(crate) struct CreateRunner<COMP: BaseComponent> {
    pub(crate) parent: Option<Element>,
    pub(crate) next_sibling: NodeRef,
    pub(crate) placeholder: VNode,
    pub(crate) node_ref: NodeRef,
    pub(crate) props: Rc<COMP::Properties>,
    pub(crate) scope: Scope<COMP>,
    #[cfg(feature = "ssr")]
    pub(crate) html_sender: Option<oneshot::Sender<VNode>>,
    #[cfg(feature = "hydration")]
    pub(crate) hydrate_fragment: Option<VecDeque<Node>>,
}

impl<COMP: BaseComponent> Runnable for CreateRunner<COMP> {
    fn run(self: Box<Self>) {
        let scope = self.scope.clone();

        let mut current_state = self.scope.state.borrow_mut();
        if current_state.is_none() {
            #[cfg(debug_assertions)]
            crate::virtual_dom::vcomp::log_event(self.scope.vcomp_id, "create");

            let Self {
                props,
                placeholder,
                parent,
                next_sibling,
                node_ref,

                #[cfg(feature = "hydration")]
                hydrate_fragment,

                #[cfg(feature = "ssr")]
                html_sender,
                ..
            } = *self;

            #[cfg(debug_assertions)]
            let vcomp_id = scope.vcomp_id;
            let context = Context { scope, props };

            let inner = Box::new(CompStateInner {
                component: COMP::create(&context),
                context,
            });

            let state = ComponentState {
                inner,
                root_node: placeholder,
                parent,
                next_sibling,
                node_ref,
                suspension: None,
                has_rendered: false,

                #[cfg(feature = "hydration")]
                hydrate_fragment,

                #[cfg(feature = "ssr")]
                html_sender,

                #[cfg(debug_assertions)]
                vcomp_id,
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
        if let Some(mut state) = self.state.borrow_mut().as_mut() {
            let schedule_render = match self.event {
                UpdateEvent::Message => state.inner.flush_messages(),
                UpdateEvent::Properties(props, node_ref, next_sibling) => {
                    // When components are updated, a new node ref could have been passed in
                    state.node_ref = node_ref;
                    // When components are updated, their siblings were likely also updated
                    state.next_sibling = next_sibling;
                    // Only trigger changed if props were changed

                    state.inner.props_changed(props)
                }
                UpdateEvent::Shift(parent, next_sibling) => {
                    // We need to shift the hydrate fragment if the component is not hydrated.
                    #[cfg(feature = "hydration")]
                    {
                        use crate::virtual_dom::shift_fragment;

                        if let Some(ref m) = state.hydrate_fragment {
                            shift_fragment(
                                m,
                                state.parent.as_ref().unwrap(),
                                &parent,
                                next_sibling.clone(),
                            );
                        }
                    }

                    state.root_node.shift(
                        state.parent.as_ref().unwrap(),
                        &parent,
                        next_sibling.clone(),
                    );

                    state.parent = Some(parent);
                    state.next_sibling = next_sibling;

                    false
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
            crate::virtual_dom::vcomp::log_event(state.vcomp_id, "destroy");

            state.inner.destroy();

            if let Some(ref m) = state.parent {
                state.root_node.detach(m, self.parent_to_detach);
                state.node_ref.set(None);
            }
        }
    }
}

pub(crate) struct RenderRunner {
    pub(crate) state: Shared<Option<ComponentState>>,
}

impl Runnable for RenderRunner {
    fn run(self: Box<Self>) {
        if let Some(state) = self.state.borrow_mut().as_mut() {
            #[cfg(debug_assertions)]
            crate::virtual_dom::vcomp::log_event(state.vcomp_id, "render");

            match state.inner.view() {
                Ok(m) => {
                    // Currently not suspended, we remove any previous suspension and update
                    // normally.
                    let mut root = m;
                    if state.parent.is_some() {
                        std::mem::swap(&mut root, &mut state.root_node);
                    }

                    if let Some(m) = state.suspension.take() {
                        let comp_scope = state.inner.any_scope();

                        let suspense_scope = comp_scope.find_parent_scope::<Suspense>().unwrap();
                        let suspense = suspense_scope.get_component().unwrap();

                        suspense.resume(m);
                    }

                    if let Some(ref m) = state.parent {
                        let ancestor = Some(root);
                        let new_root = &mut state.root_node;
                        let scope = state.inner.any_scope();
                        let next_sibling = state.next_sibling.clone();

                        #[cfg(not(feature = "hydration"))]
                        let node = new_root.apply(&scope, m, next_sibling, ancestor);

                        #[cfg(feature = "hydration")]
                        let node = match state.hydrate_fragment.take() {
                            Some(mut fragment) => {
                                let first_node = new_root.hydrate(&scope, m, &mut fragment);

                                // We trim all text nodes before checking it's likely these are whitespaces.
                                trim_start_text_nodes(m, &mut fragment);

                                assert!(
                                    fragment.is_empty(),
                                    "expected end of component, found node"
                                );

                                first_node
                            }
                            None => new_root.apply(&scope, m, next_sibling, ancestor),
                        };

                        state.node_ref.link(node);

                        let first_render = !state.has_rendered;
                        state.has_rendered = true;

                        scheduler::push_component_rendered(
                            self.state.as_ptr() as usize,
                            RenderedRunner {
                                state: self.state.clone(),
                                first_render,
                            },
                            first_render,
                        );
                    } else {
                        #[cfg(feature = "ssr")]
                        if let Some(tx) = state.html_sender.take() {
                            tx.send(root).unwrap();
                        }
                    }
                }

                Err(RenderError::Suspended(m)) => {
                    // Currently suspended, we re-use previous root node and send
                    // suspension to parent element.
                    let shared_state = self.state.clone();

                    if m.resumed() {
                        // schedule a render immediately if suspension is resumed.

                        scheduler::push_component_render(
                            shared_state.as_ptr() as usize,
                            RenderRunner {
                                state: shared_state.clone(),
                            },
                        );
                    } else {
                        // We schedule a render after current suspension is resumed.

                        let comp_scope = state.inner.any_scope();

                        let suspense_scope = comp_scope
                            .find_parent_scope::<Suspense>()
                            .expect("To suspend rendering, a <Suspense /> component is required.");
                        let suspense = suspense_scope.get_component().unwrap();

                        m.listen(Callback::from(move |_| {
                            scheduler::push_component_render(
                                shared_state.as_ptr() as usize,
                                RenderRunner {
                                    state: shared_state.clone(),
                                },
                            );
                            scheduler::start();
                        }));

                        if let Some(ref last_m) = state.suspension {
                            if &m != last_m {
                                // We remove previous suspension from the suspense.
                                suspense.resume(last_m.clone());
                            }
                        }
                        state.suspension = Some(m.clone());

                        suspense.suspend(m);
                    }
                }
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
            crate::virtual_dom::vcomp::log_event(state.vcomp_id, "rendered");

            if state.suspension.is_none() && state.parent.is_some() {
                state.inner.rendered(self.first_render);
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
