use std::any::{Any, TypeId};
use std::rc::Rc;

#[cfg(feature = "ssr")]
use futures::future::{FutureExt, LocalBoxFuture};
use web_sys::Element;

use super::{BaseComponent, Context, Scope};
use crate::dom_bundle::BSubtree;
#[cfg(feature = "hydration")]
use crate::dom_bundle::Fragment;
use crate::functional::FunctionComponent;
use crate::html::NodeRef;
#[cfg(feature = "ssr")]
use crate::platform::fmt::BufWriter;
#[cfg(any(feature = "hydration", feature = "ssr"))]
use crate::virtual_dom::Collectable;

pub(crate) trait Mountable {
    fn type_id(&self) -> TypeId;
    fn props(&self) -> &dyn Any;

    fn create_component(&self, ctx: &Context) -> FunctionComponent;
    #[cfg(any(feature = "hydration", feature = "ssr"))]
    fn create_collectable(&self) -> Collectable;

    #[cfg(feature = "csr")]
    fn mount(
        self: Rc<Self>,
        root: &BSubtree,
        parent_scope: &Scope,
        parent: Element,
        internal_ref: NodeRef,
        next_sibling: NodeRef,
    ) -> Scope;

    #[cfg(feature = "csr")]
    fn reuse(self: Rc<Self>, scope: &Scope, next_sibling: NodeRef);

    #[cfg(feature = "ssr")]
    fn render_into_stream<'a>(
        self: Rc<Self>,
        w: &'a mut BufWriter,
        parent_scope: &'a Scope,
        hydratable: bool,
    ) -> LocalBoxFuture<'a, ()>;

    #[cfg(feature = "hydration")]
    fn hydrate(
        self: Rc<Self>,
        root: BSubtree,
        parent_scope: &Scope,
        parent: Element,
        internal_ref: NodeRef,
        fragment: &mut Fragment,
    ) -> Scope;
}

pub(crate) struct ComponentIntriustic<COMP: BaseComponent> {
    props: Rc<COMP::Properties>,
}

impl<COMP: BaseComponent> ComponentIntriustic<COMP> {
    pub fn new(props: Rc<COMP::Properties>) -> Self {
        Self { props }
    }
}

impl<COMP: BaseComponent> Mountable for ComponentIntriustic<COMP> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<COMP>()
    }

    fn props(&self) -> &dyn Any {
        self.props.as_ref()
    }

    fn create_component(&self, ctx: &Context) -> FunctionComponent {
        COMP::create(ctx)
    }

    #[cfg(any(feature = "hydration", feature = "ssr"))]
    fn create_collectable(&self) -> Collectable {
        Collectable::for_component::<COMP>()
    }

    #[cfg(feature = "csr")]
    fn mount(
        self: Rc<Self>,
        root: &BSubtree,
        parent_scope: &Scope,
        parent: Element,
        internal_ref: NodeRef,
        next_sibling: NodeRef,
    ) -> Scope {
        let scope = Scope::new(self.as_ref(), Some(parent_scope.clone()));
        scope.mount(self, root.clone(), parent, next_sibling, internal_ref);

        scope
    }

    #[cfg(feature = "csr")]
    fn reuse(self: Rc<Self>, scope: &Scope, next_sibling: NodeRef) {
        scope.reuse(self, next_sibling);
    }

    #[cfg(feature = "ssr")]
    fn render_into_stream<'a>(
        self: Rc<Self>,
        w: &'a mut BufWriter,
        parent_scope: &'a Scope,
        hydratable: bool,
    ) -> LocalBoxFuture<'a, ()> {
        let scope: Scope = Scope::new(self.as_ref(), Some(parent_scope.clone()));

        async move {
            scope.render_into_stream(self, w, hydratable).await;
        }
        .boxed_local()
    }

    #[cfg(feature = "hydration")]
    fn hydrate(
        self: Rc<Self>,
        root: BSubtree,
        parent_scope: &Scope,
        parent: Element,
        internal_ref: NodeRef,
        fragment: &mut Fragment,
    ) -> Scope {
        let scope: Scope = Scope::new(self.as_ref(), Some(parent_scope.clone()));

        // This is very helpful to see which component is failing during hydration
        // which means this component may not having a stable layout / differs between
        // client-side and server-side.
        tracing::trace!(
            component.id = scope.id(),
            "hydration(type = {})",
            std::any::type_name::<COMP>()
        );

        scope.hydrate(self, root, parent, fragment, internal_ref);

        scope
    }
}
