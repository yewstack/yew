use std::{
    fmt::{self, Debug},
    rc::Rc,
};

use yew::{Component, ComponentLink, Html, Properties};

use crate::{router::RouterListener, Routable, Router};

/// Wraps `Rc` around `Fn` so it can be passed as a prop.
pub struct RenderFn<T: Routable>(Rc<dyn Fn(&Router<T>) -> Html>);

impl<T: Routable> RenderFn<T> {
    /// Creates a new [`RenderFn`]
    ///
    /// It is recommended that you use [`Router::render`] instead
    pub fn new(value: impl Fn(&Router<T>) -> Html + 'static) -> Self {
        Self(Rc::new(value))
    }
}

impl<T: Routable> Debug for RenderFn<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RenderFn").finish()
    }
}

impl<T: Routable> Clone for RenderFn<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T: Routable> PartialEq for RenderFn<T> {
    fn eq(&self, other: &Self) -> bool {
        // https://github.com/rust-lang/rust-clippy/issues/6524
        #[allow(clippy::vtable_address_comparisons)]
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/// Props for [`RouterConsumer`]
#[derive(Clone, PartialEq, Properties)]
pub struct RouterConsumerProps<T: Routable> {
    /// Callback which returns [`Html`] to be rendered for the current route.
    pub render: RenderFn<T>,
}

/// The context provider component.
///
/// Every child (direct or indirect) of this component may access the context value.
/// In order to consume contexts, [`ComponentLink::context`][Scope::context] method is used,
/// In function components the `use_context` hook is used.
#[derive(Debug)]
pub struct RouterConsumer<T: Routable> {
    render: RenderFn<T>,
    router: Router<T>,
    _listener: RouterListener<T>,
}

impl<T: Routable> Component for RouterConsumer<T> {
    type Message = ();
    type Properties = RouterConsumerProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = Router::new(link.clone());
        let _listener = router.register(link.callback(|_| ()));
        Self {
            render: props.render,
            router,
            _listener,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.render != props.render {
            self.render = props.render;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        (&self.render.0)(&self.router)
    }
}
