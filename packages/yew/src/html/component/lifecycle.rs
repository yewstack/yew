//! Component lifecycle module

use std::any::Any;
use std::rc::Rc;

#[cfg(feature = "csr")]
use web_sys::Element;

use super::scope::AnyScope;
#[cfg(feature = "hydration")]
use crate::dom_bundle::Fragment;
#[cfg(feature = "csr")]
use crate::dom_bundle::{BSubtree, Bundle};
#[cfg(feature = "csr")]
use crate::html::NodeRef;
#[cfg(feature = "hydration")]
use crate::html::RenderMode;
use crate::html::{Html, RenderError};
use crate::scheduler::{self, Shared};
use crate::suspense::{resume_suspension, suspend_suspension, DispatchSuspension, Suspension};
use crate::{Callback, Context, ContextProvider, FunctionComponent};

pub(crate) enum ComponentRenderState {
    #[cfg(feature = "csr")]
    Render {
        bundle: Bundle,
        root: BSubtree,
        parent: Element,
        next_sibling: NodeRef,
        internal_ref: NodeRef,
    },
    #[cfg(feature = "hydration")]
    Hydration {
        fragment: Fragment,
        root: BSubtree,
        parent: Element,
        next_sibling: NodeRef,
        internal_ref: NodeRef,
    },
    #[cfg(feature = "ssr")]
    Ssr {
        sender: Option<crate::platform::pinned::oneshot::Sender<Html>>,
    },
}

impl std::fmt::Debug for ComponentRenderState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "csr")]
            Self::Render {
                ref bundle,
                root,
                ref parent,
                ref next_sibling,
                ref internal_ref,
            } => f
                .debug_struct("ComponentRenderState::Render")
                .field("bundle", bundle)
                .field("root", root)
                .field("parent", parent)
                .field("next_sibling", next_sibling)
                .field("internal_ref", internal_ref)
                .finish(),

            #[cfg(feature = "hydration")]
            Self::Hydration {
                ref fragment,
                ref parent,
                ref next_sibling,
                ref internal_ref,
                ref root,
            } => f
                .debug_struct("ComponentRenderState::Hydration")
                .field("fragment", fragment)
                .field("root", root)
                .field("parent", parent)
                .field("next_sibling", next_sibling)
                .field("internal_ref", internal_ref)
                .finish(),

            #[cfg(feature = "ssr")]
            Self::Ssr { ref sender } => {
                let sender_repr = match sender {
                    Some(_) => "Some(_)",
                    None => "None",
                };

                f.debug_struct("ComponentRenderState::Ssr")
                    .field("sender", &sender_repr)
                    .finish()
            }
        }
    }
}

#[cfg(feature = "csr")]
impl ComponentRenderState {
    pub(crate) fn shift(&mut self, next_parent: Element, next_next_sibling: NodeRef) {
        match self {
            #[cfg(feature = "csr")]
            Self::Render {
                bundle,
                parent,
                next_sibling,
                ..
            } => {
                bundle.shift(&next_parent, next_next_sibling.clone());

                *parent = next_parent;
                next_sibling.link(next_next_sibling);
            }
            #[cfg(feature = "hydration")]
            Self::Hydration {
                fragment,
                parent,
                next_sibling,
                ..
            } => {
                fragment.shift(&next_parent, next_next_sibling.clone());

                *parent = next_parent;
                next_sibling.link(next_next_sibling);
            }

            #[cfg(feature = "ssr")]
            Self::Ssr { .. } => {
                #[cfg(debug_assertions)]
                panic!("shifting is not possible during SSR");
            }
        }
    }
}

pub(crate) struct ComponentState {
    pub(super) component: FunctionComponent,
    pub(super) context: Context,

    pub(super) render_state: ComponentRenderState,

    #[cfg(feature = "csr")]
    has_rendered: bool,
    #[cfg(feature = "hydration")]
    pending_props: Option<Rc<dyn Any>>,

    suspension: Option<Suspension>,

    pub(crate) comp_id: usize,
}

impl ComponentState {
    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        name = "create",
        skip_all,
        fields(component.id = scope.get_id()),
    )]
    fn new(
        component: FunctionComponent,
        context: Context,
        initial_render_state: ComponentRenderState,
        scope: AnyScope,
    ) -> Self {
        let comp_id = scope.get_id();

        Self {
            component,
            context,
            render_state: initial_render_state,
            suspension: None,

            #[cfg(feature = "csr")]
            has_rendered: false,
            #[cfg(feature = "hydration")]
            pending_props: None,

            comp_id,
        }
    }

    fn resume_existing_suspension(&mut self) {
        if let Some(m) = self.suspension.take() {
            let comp_scope = self.context.link();

            let suspense_scope = comp_scope
                .find_parent_scope::<ContextProvider<DispatchSuspension>>()
                .unwrap();
            resume_suspension(&suspense_scope, m);
        }
    }
}

pub(crate) struct CreateRunner {
    pub initial_render_state: ComponentRenderState,
    pub scope: AnyScope,
    pub component: FunctionComponent,
    pub context: Context,
}

impl CreateRunner {
    pub fn run(self) {
        let mut current_state = self.scope.state.borrow_mut();
        if current_state.is_none() {
            *current_state = Some(ComponentState::new(
                self.component,
                self.context,
                self.initial_render_state,
                self.scope.clone(),
            ));
        }
    }
}

pub(crate) struct DestroyRunner {
    pub state: Shared<Option<ComponentState>>,
    pub parent_to_detach: bool,
}

impl ComponentState {
    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        skip(self),
        fields(component.id = self.comp_id)
    )]
    fn destroy(mut self, parent_to_detach: bool) {
        self.component.destroy();
        self.resume_existing_suspension();

        match self.render_state {
            #[cfg(feature = "csr")]
            ComponentRenderState::Render {
                bundle,
                ref parent,
                ref internal_ref,
                ref root,
                ..
            } => {
                bundle.detach(root, parent, parent_to_detach);

                internal_ref.set(None);
            }
            // We need to detach the hydrate fragment if the component is not hydrated.
            #[cfg(feature = "hydration")]
            ComponentRenderState::Hydration {
                ref root,
                fragment,
                ref parent,
                ref internal_ref,
                ..
            } => {
                fragment.detach(root, parent, parent_to_detach);

                internal_ref.set(None);
            }

            #[cfg(feature = "ssr")]
            ComponentRenderState::Ssr { .. } => {
                let _ = parent_to_detach;
            }
        }
    }
}

impl DestroyRunner {
    pub fn run(self) {
        if let Some(state) = self.state.borrow_mut().take() {
            state.destroy(self.parent_to_detach);
        }
    }
}

pub(crate) struct RenderRunner {
    pub state: Shared<Option<ComponentState>>,
}

impl ComponentState {
    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        skip_all,
        fields(component.id = self.comp_id)
    )]
    fn render(&mut self, shared_state: &Shared<Option<ComponentState>>) {
        match self.component.render(self.context.props()) {
            Ok(vnode) => self.commit_render(shared_state, vnode),
            Err(RenderError::Suspended(susp)) => self.suspend(shared_state, susp),
        };
    }

    fn suspend(&mut self, shared_state: &Shared<Option<ComponentState>>, suspension: Suspension) {
        // Currently suspended, we re-use previous root node and send
        // suspension to parent element.

        if suspension.resumed() {
            let runner = RenderRunner {
                state: shared_state.clone(),
            };

            // schedule a render immediately if suspension is resumed.
            scheduler::push_component_render(self.comp_id, move || runner.run());
        } else {
            // We schedule a render after current suspension is resumed.
            let comp_scope = self.context.link();

            let suspense_scope = comp_scope
                .find_parent_scope::<ContextProvider<DispatchSuspension>>()
                .expect("To suspend rendering, a <Suspense /> component is required.");

            let comp_id = self.comp_id;
            let shared_state = shared_state.clone();
            suspension.listen(Callback::from(move |_| {
                let runner = RenderRunner {
                    state: shared_state.clone(),
                };

                scheduler::push_component_render(comp_id, move || runner.run());
                scheduler::start();
            }));

            if let Some(ref last_suspension) = self.suspension {
                if &suspension != last_suspension {
                    // We remove previous suspension from the suspense.
                    resume_suspension(&suspense_scope, last_suspension.clone())
                }
            }
            self.suspension = Some(suspension.clone());

            suspend_suspension(&suspense_scope, suspension);
        }
    }

    fn commit_render(&mut self, shared_state: &Shared<Option<ComponentState>>, new_root: Html) {
        // Currently not suspended, we remove any previous suspension and update
        // normally.
        self.resume_existing_suspension();

        match self.render_state {
            #[cfg(feature = "csr")]
            ComponentRenderState::Render {
                ref mut bundle,
                ref parent,
                ref root,
                ref next_sibling,
                ref internal_ref,
                ..
            } => {
                let scope = self.context.link();

                #[cfg(feature = "hydration")]
                next_sibling.debug_assert_not_trapped();

                let new_node_ref =
                    bundle.reconcile(root, scope, parent, next_sibling.clone(), new_root);
                internal_ref.link(new_node_ref);

                let first_render = !self.has_rendered;
                self.has_rendered = true;

                let runner = RenderedRunner {
                    state: shared_state.clone(),
                };

                scheduler::push_component_rendered(
                    self.comp_id,
                    move || runner.run(),
                    first_render,
                );
            }

            #[cfg(feature = "hydration")]
            ComponentRenderState::Hydration {
                ref mut fragment,
                ref parent,
                ref internal_ref,
                ref next_sibling,
                ref root,
            } => {
                let scope = self.context.link();

                // This first node is not guaranteed to be correct here.
                // As it may be a comment node that is removed afterwards.
                // but we link it anyways.
                let (node, bundle) = Bundle::hydrate(root, scope, parent, fragment, new_root);

                // We trim all text nodes before checking as it's likely these are whitespaces.
                fragment.trim_start_text_nodes(parent);

                assert!(fragment.is_empty(), "expected end of component, found node");

                internal_ref.link(node);

                self.render_state = ComponentRenderState::Render {
                    root: root.clone(),
                    bundle,
                    parent: parent.clone(),
                    internal_ref: internal_ref.clone(),
                    next_sibling: next_sibling.clone(),
                };
            }

            #[cfg(feature = "ssr")]
            ComponentRenderState::Ssr { ref mut sender } => {
                let _ = shared_state;
                if let Some(tx) = sender.take() {
                    tx.send(new_root).unwrap();
                }
            }
        };
    }
}

impl RenderRunner {
    pub fn run(self) {
        let mut state = self.state.borrow_mut();
        let state = match state.as_mut() {
            None => return, // skip for components that have already been destroyed
            Some(state) => state,
        };

        state.render(&self.state);
    }
}

#[cfg(feature = "csr")]
mod feat_csr {
    use super::*;

    pub(crate) struct PropsUpdateRunner {
        pub state: Shared<Option<ComponentState>>,
        pub props: Option<Rc<dyn Any>>,
        pub next_sibling: Option<NodeRef>,
    }

    impl ComponentState {
        #[tracing::instrument(
            level = tracing::Level::DEBUG,
            skip(self),
            fields(component.id = self.comp_id)
        )]
        fn changed(&mut self, props: Option<Rc<dyn Any>>, next_sibling: Option<NodeRef>) -> bool {
            if let Some(next_sibling) = next_sibling {
                // When components are updated, their siblings were likely also updated
                // We also need to shift the bundle so next sibling will be synced to child
                // components.
                match self.render_state {
                    #[cfg(feature = "csr")]
                    ComponentRenderState::Render {
                        next_sibling: ref current_next_sibling,
                        ..
                    } => {
                        current_next_sibling.link(next_sibling);
                    }

                    #[cfg(feature = "hydration")]
                    ComponentRenderState::Hydration {
                        next_sibling: ref current_next_sibling,
                        ..
                    } => {
                        current_next_sibling.link(next_sibling);
                    }

                    #[cfg(feature = "ssr")]
                    ComponentRenderState::Ssr { .. } => {
                        #[cfg(debug_assertions)]
                        panic!("properties do not change during SSR");
                    }
                }
            }

            let should_render =
                |_props: Option<Rc<dyn Any>>, _state: &mut ComponentState| -> bool {
                    // TODO: Add Props Change back.
                    true
                };

            #[cfg(feature = "hydration")]
            let should_render_hydration =
                |props: Option<Rc<dyn Any>>, state: &mut ComponentState| -> bool {
                    if let Some(props) = props.or_else(|| state.pending_props.take()) {
                        match state.has_rendered {
                            true => {
                                state.pending_props = None;
                                // state.inner.props_changed(props)
                                // TODO: Add Props Change back.
                                true
                            }
                            false => {
                                state.pending_props = Some(props);
                                false
                            }
                        }
                    } else {
                        false
                    }
                };

            // Only trigger changed if props were changed / next sibling has changed.
            let schedule_render = {
                #[cfg(feature = "hydration")]
                {
                    if self.context.creation_mode() == RenderMode::Hydration {
                        should_render_hydration(props, self)
                    } else {
                        should_render(props, self)
                    }
                }

                #[cfg(not(feature = "hydration"))]
                should_render(props, self)
            };

            tracing::trace!(
                "props_update(has_rendered={} schedule_render={})",
                self.has_rendered,
                schedule_render
            );
            schedule_render
        }
    }

    impl PropsUpdateRunner {
        pub fn run(self) {
            let Self {
                next_sibling,
                props,
                state: shared_state,
            } = self;

            if let Some(state) = shared_state.borrow_mut().as_mut() {
                let schedule_render = state.changed(props, next_sibling);

                let runner = RenderRunner {
                    state: shared_state.clone(),
                };

                if schedule_render {
                    scheduler::push_component_render(state.comp_id, move || runner.run());
                    // Only run from the scheduler, so no need to call `scheduler::start()`
                }
            };
        }
    }

    pub(crate) struct RenderedRunner {
        pub state: Shared<Option<ComponentState>>,
    }

    impl ComponentState {
        #[tracing::instrument(
            level = tracing::Level::DEBUG,
            skip(self),
            fields(component.id = self.comp_id)
        )]
        fn rendered(&mut self) -> bool {
            if self.suspension.is_none() {
                self.component.rendered();
            }

            #[cfg(feature = "hydration")]
            {
                self.pending_props.is_some()
            }
            #[cfg(not(feature = "hydration"))]
            {
                false
            }
        }
    }

    impl RenderedRunner {
        pub fn run(self) {
            if let Some(state) = self.state.borrow_mut().as_mut() {
                let has_pending_props = state.rendered();

                if has_pending_props {
                    let runner = PropsUpdateRunner {
                        state: self.state.clone(),
                        props: None,
                        next_sibling: None,
                    };

                    scheduler::push(move || runner.run());
                }
            }
        }
    }
}

#[cfg(feature = "csr")]
pub(super) use feat_csr::*;

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    extern crate self as yew;

    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    use super::*;
    use crate::dom_bundle::BSubtree;
    use crate::html::*;
    use crate::{html, Properties};

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

        fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
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
        #[cfg(target_arch = "wasm32")]
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
            #[cfg(target_arch = "wasm32")]
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

        fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
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
        let document = gloo::utils::document();
        let scope = Scope::<Comp>::new(None);
        let parent = document.create_element("div").unwrap();
        let root = BSubtree::create_root(&parent);

        let lifecycle = props.lifecycle.clone();

        lifecycle.borrow_mut().clear();
        scope.mount_in_place(
            root,
            parent,
            NodeRef::default(),
            NodeRef::default(),
            Rc::new(props),
        );
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
                #[cfg(target_arch = "wasm32")]
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
                #[cfg(target_arch = "wasm32")]
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
