//! Component lifecycle module

use std::rc::Rc;

use web_sys::Element;

use super::scope::Scope;
use crate::dom_bundle::{DomSlot, Realized};
#[cfg(feature = "hydration")]
use crate::html::RenderMode;
use crate::html::{Context, Html, Intrinsical, NodeRef, RenderError};
use crate::suspense::{resume_suspension, suspend_suspension, DispatchSuspension, Suspension};
use crate::{scheduler, Callback, ContextProvider, HookContext};

pub(crate) struct ComponentState {
    pub(super) component: HookContext,
    pub(super) context: Context,

    pub slot: DomSlot,

    #[cfg(feature = "hydration")]
    pending_mountable: Option<Rc<dyn Intrinsical>>,

    suspension: Option<Suspension>,
}

impl ComponentState {
    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        name = "create",
        skip_all,
        fields(component.id = context.link().id()),
    )]
    fn new(context: Context, slot: DomSlot) -> Self {
        Self {
            component: HookContext::new(&context),
            context,
            suspension: None,

            slot,

            #[cfg(feature = "hydration")]
            pending_mountable: None,
        }
    }

    pub fn run_create(context: Context, slot: DomSlot) {
        let scope = context.scope.clone();
        let mut current_state = scope.state_cell().borrow_mut();

        if current_state.is_none() {
            let mut self_ = Self::new(context, slot);
            self_.render();

            // We are safe to assign afterwards as we mutably borrow the state and don't release it
            // until this function returns.
            *current_state = Some(self_);
        }
    }

    pub fn run_render(scope: &Scope) {
        if let Some(state) = scope.state_cell().borrow_mut().as_mut() {
            state.render();
        }
    }

    pub fn run_shift(scope: &Scope, next_parent: Element, next_sibling: NodeRef) {
        if let Some(state) = scope.state_cell().borrow_mut().as_mut() {
            state.shift(next_parent, next_sibling);
        }
    }

    pub fn run_update(
        scope: &Scope,
        mountable: Option<Rc<dyn Intrinsical>>,
        next_sibling: Option<NodeRef>,
    ) {
        if let Some(state) = scope.state_cell().borrow_mut().as_mut() {
            state.changed(mountable, next_sibling);
        }
    }

    pub fn run_destroy(scope: &Scope, parent_to_detach: bool) {
        if let Some(state) = scope.state_cell().borrow_mut().take() {
            state.destroy(parent_to_detach);
        }
    }

    fn resume_existing_suspension(&mut self) {
        if let Some(m) = self.suspension.take() {
            let comp_scope = self.context.link();

            let suspense_scope = comp_scope
                .find_parent_scope::<ContextProvider<DispatchSuspension>>()
                .unwrap();
            resume_suspension(suspense_scope, m);
        }
    }

    pub fn shift(&mut self, next_parent: Element, next_next_sibling: NodeRef) {
        match self.slot.content {
            Realized::Bundle(ref mut bundle) => {
                bundle.shift(&next_parent, next_next_sibling.clone());
            }
            #[cfg(feature = "hydration")]
            Realized::Fragement(ref mut fragment) => {
                fragment.shift(&next_parent, next_next_sibling.clone());
            }
        }

        self.slot.parent = next_parent;
        self.slot.next_sibling.link(next_next_sibling);
    }

    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        skip(self),
        fields(component.id = self.context.link().id())
    )]
    fn destroy(mut self, parent_to_detach: bool) {
        self.component.destroy();
        self.resume_existing_suspension();

        match self.slot.content {
            Realized::Bundle(bundle) => {
                bundle.detach(&self.slot.root, &self.slot.parent, parent_to_detach);
            }
            // We need to detach the hydrate fragment if the component is not hydrated.
            #[cfg(feature = "hydration")]
            Realized::Fragement(fragment) => {
                fragment.detach(&self.slot.root, &self.slot.parent, parent_to_detach);
            }
        }

        self.slot.internal_ref.set(None);
    }

    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        skip_all,
        fields(component.id = self.context.link().id())
    )]
    fn render(&mut self) {
        match self.context.intrisic().render(&mut self.component) {
            Ok(vnode) => self.commit_render(vnode),
            Err(RenderError::Suspended(susp)) => self.suspend(susp),
        };
    }

    fn suspend(&mut self, suspension: Suspension) {
        // Currently suspended, we re-use previous root node and send
        // suspension to parent element.

        if suspension.resumed() {
            self.render();
        } else {
            // We schedule a render after current suspension is resumed.
            let comp_scope = self.context.link();

            let suspense_scope = comp_scope
                .find_parent_scope::<ContextProvider<DispatchSuspension>>()
                .expect("To suspend rendering, a <Suspense /> component is required.");

            {
                let scope = self.context.link().clone();
                suspension.listen(Callback::from(move |_| {
                    let scope = scope.clone();
                    scheduler::push(move || ComponentState::run_render(&scope));
                }));
            }

            if let Some(ref last_suspension) = self.suspension {
                if &suspension != last_suspension {
                    // We remove previous suspension from the suspense.
                    resume_suspension(suspense_scope, last_suspension.clone())
                }
            }
            self.suspension = Some(suspension.clone());

            suspend_suspension(suspense_scope, suspension);
        }
    }

    fn commit_render(&mut self, new_root: Html) {
        // Currently not suspended, we remove any previous suspension and update
        // normally.
        self.resume_existing_suspension();

        match self.slot.content {
            Realized::Bundle(ref mut bundle) => {
                let scope = self.context.link();

                let new_node_ref = bundle.reconcile(
                    &self.slot.root,
                    scope,
                    &self.slot.parent,
                    self.slot.next_sibling.clone(),
                    new_root,
                );
                self.slot.internal_ref.link(new_node_ref);

                let has_pending_props = self.rendered();
                if has_pending_props {
                    self.changed(None, None);
                }
            }

            #[cfg(feature = "hydration")]
            Realized::Fragement(ref mut fragment) => {
                use crate::dom_bundle::Bundle;

                let scope = self.context.link();

                let (node, bundle) = Bundle::hydrate(
                    &self.slot.root,
                    scope,
                    &self.slot.parent,
                    fragment,
                    new_root,
                );

                // We trim all text nodes before checking as it's likely these are whitespaces.
                fragment.trim_start_text_nodes(&self.slot.parent);

                assert!(fragment.is_empty(), "expected end of component, found node");

                self.slot.internal_ref.link(node);

                self.slot.content = Realized::Bundle(bundle);
            }
        };
    }

    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        skip(self, mountable),
        fields(component.id = self.context.link().id())
    )]
    pub(super) fn changed(
        &mut self,
        mountable: Option<Rc<dyn Intrinsical>>,
        next_sibling: Option<NodeRef>,
    ) {
        if let Some(next_sibling) = next_sibling {
            // When components are updated, their siblings were likely also updated
            // We also need to shift the bundle so next sibling will be synced to child
            // components.
            self.slot.next_sibling.link(next_sibling);
        }

        // Only trigger changed if props were changed / next sibling has changed.
        let schedule_render = '_block: {
            #[cfg(feature = "hydration")]
            if self.context.creation_mode() == RenderMode::Hydration {
                break '_block if let Some(mountable) =
                    mountable.or_else(|| self.pending_mountable.take())
                {
                    match self.slot.content {
                        Realized::Bundle { .. } => {
                            self.pending_mountable = None;
                            if !self.context.mountable.intrinsic_eq(mountable.as_ref()) {
                                self.context.mountable = mountable;
                            }
                            true
                        }
                        Realized::Fragement { .. } => {
                            self.pending_mountable = Some(mountable);
                            false
                        }
                    }
                } else {
                    false
                };
            }

            mountable
                .and_then(|m| (!self.context.mountable.intrinsic_eq(m.as_ref())).then_some(m))
                .map(|m| {
                    self.context.mountable = m;
                    true
                })
                .unwrap_or(false)
        };

        tracing::trace!("props_update(schedule_render={})", schedule_render);

        if schedule_render {
            self.render()
        }
    }

    #[tracing::instrument(
        level = tracing::Level::DEBUG,
        skip(self),
        fields(component.id = self.context.link().id())
    )]
    pub(super) fn rendered(&mut self) -> bool {
        if self.suspension.is_none() {
            self.component.rendered();
        }

        #[cfg(feature = "hydration")]
        {
            self.pending_mountable.is_some()
        }
        #[cfg(not(feature = "hydration"))]
        {
            false
        }
    }
}

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
