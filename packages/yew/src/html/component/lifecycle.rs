//! Component lifecycle module

use super::scope::{AnyScope, Scope};
use super::BaseComponent;
use crate::dom_bundle::ComponentRenderState;
use crate::html::RenderError;
use crate::scheduler::{self, Runnable, Shared};
use crate::suspense::{Suspense, Suspension};
use crate::{Callback, Context, HtmlResult, NodeRef};
use std::any::Any;
use std::rc::Rc;

pub struct CompStateInner<COMP>
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
pub trait Stateful {
    fn view(&self) -> HtmlResult;
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
    fn view(&self) -> HtmlResult {
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

pub struct ComponentState {
    pub(super) inner: Box<dyn Stateful>,

    pub(super) render_state: ComponentRenderState,
    node_ref: NodeRef,
    has_rendered: bool,

    suspension: Option<Suspension>,

    // Used for debug logging
    #[cfg(debug_assertions)]
    pub(crate) vcomp_id: usize,
}

impl ComponentState {
    pub(crate) fn new<COMP: BaseComponent>(
        initial_render_state: ComponentRenderState,
        node_ref: NodeRef,
        scope: Scope<COMP>,
        props: Rc<COMP::Properties>,
    ) -> Self {
        #[cfg(debug_assertions)]
        let vcomp_id = scope.vcomp_id;
        let context = Context { scope, props };

        let inner = Box::new(CompStateInner {
            component: COMP::create(&context),
            context,
        });

        Self {
            inner,
            render_state: initial_render_state,
            node_ref,
            suspension: None,
            has_rendered: false,

            #[cfg(debug_assertions)]
            vcomp_id,
        }
    }
}

pub struct CreateRunner<COMP: BaseComponent> {
    pub initial_render_state: ComponentRenderState,
    pub node_ref: NodeRef,
    pub props: Rc<COMP::Properties>,
    pub scope: Scope<COMP>,
}

impl<COMP: BaseComponent> Runnable for CreateRunner<COMP> {
    fn run(self: Box<Self>) {
        let mut current_state = self.scope.state.borrow_mut();
        if current_state.is_none() {
            #[cfg(debug_assertions)]
            super::log_event(self.scope.vcomp_id, "create");

            *current_state = Some(ComponentState::new(
                self.initial_render_state,
                self.node_ref,
                self.scope.clone(),
                self.props,
            ));
        }
    }
}

pub enum UpdateEvent {
    /// Drain messages for a component.
    Message,
    /// Wraps properties, node ref, and next sibling for a component.
    Properties(Rc<dyn Any>, NodeRef, NodeRef),
}

pub struct UpdateRunner {
    pub state: Shared<Option<ComponentState>>,
    pub event: UpdateEvent,
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
                    state.render_state.reuse(next_sibling);
                    // Only trigger changed if props were changed

                    state.inner.props_changed(props)
                }
            };

            #[cfg(debug_assertions)]
            super::log_event(
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

pub struct DestroyRunner {
    pub state: Shared<Option<ComponentState>>,
    pub parent_to_detach: bool,
}

impl Runnable for DestroyRunner {
    fn run(self: Box<Self>) {
        if let Some(mut state) = self.state.borrow_mut().take() {
            #[cfg(debug_assertions)]
            super::log_event(state.vcomp_id, "destroy");

            state.inner.destroy();
            state.render_state.detach(self.parent_to_detach);
            state.node_ref.set(None);
        }
    }
}

pub struct RenderRunner {
    pub state: Shared<Option<ComponentState>>,
}

impl Runnable for RenderRunner {
    fn run(self: Box<Self>) {
        if let Some(state) = self.state.borrow_mut().as_mut() {
            #[cfg(debug_assertions)]
            super::log_event(state.vcomp_id, "render");

            match state.inner.view() {
                Ok(root) => {
                    // Currently not suspended, we remove any previous suspension and update
                    // normally.
                    if let Some(m) = state.suspension.take() {
                        let comp_scope = state.inner.any_scope();

                        let suspense_scope = comp_scope.find_parent_scope::<Suspense>().unwrap();
                        let suspense = suspense_scope.get_component().unwrap();

                        suspense.resume(m);
                    }

                    let scope = state.inner.any_scope();
                    let node = state.render_state.reconcile(root, &scope);
                    state.node_ref.link(node);

                    if state.render_state.should_trigger_rendered() {
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

struct RenderedRunner {
    state: Shared<Option<ComponentState>>,
    first_render: bool,
}

impl Runnable for RenderedRunner {
    fn run(self: Box<Self>) {
        if let Some(state) = self.state.borrow_mut().as_mut() {
            #[cfg(debug_assertions)]
            super::log_event(state.vcomp_id, "rendered");

            if state.suspension.is_none() {
                state.inner.rendered(self.first_render);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate self as yew;

    use crate::dom_bundle::{BSubtree, ComponentRenderState};
    use crate::html;
    use crate::html::*;
    use crate::Properties;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;
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
        let parent = document.create_element("div").unwrap();
        let root = BSubtree::create_root(&parent);

        let node_ref = NodeRef::default();
        let render_state = ComponentRenderState::new(root, parent, NodeRef::default(), &node_ref);
        let lifecycle = props.lifecycle.clone();

        lifecycle.borrow_mut().clear();
        scope.mount_in_place(render_state, node_ref, Rc::new(props));
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
