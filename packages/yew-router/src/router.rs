//! Router Component.
use std::rc::Rc;

use crate::history::{AnyHistory, BrowserHistory, HashHistory, History, Location};
// use crate::prelude::*;
use crate::navigator::Navigator;
use yew::prelude::*;

/// Props for [`Router`].
#[derive(Properties, PartialEq, Clone)]
pub struct RouterProps {
    pub children: Children,
    pub history: AnyHistory,
}

/// A context for [`Router`]
#[derive(Clone)]
pub(crate) struct LocationContext {
    location: Location,
    // Counter to force update.
    ctr: u32,
}

impl LocationContext {
    pub fn location(&self) -> Location {
        self.location.clone()
    }
}

impl PartialEq for LocationContext {
    fn eq(&self, rhs: &Self) -> bool {
        self.ctr == rhs.ctr
    }
}

impl Reducible for LocationContext {
    type Action = Location;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self {
            location: action,
            ctr: self.ctr + 1,
        }
        .into()
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct NavigatorContext {
    navigator: Navigator,
}

impl NavigatorContext {
    pub fn navigator(&self) -> Navigator {
        self.navigator.clone()
    }
}

/// The Router component.
///
/// This provides [`History`] context to its children and switches.
///
/// You only need one `<Router />` for each application.
#[function_component(Router)]
pub fn router(props: &RouterProps) -> Html {
    let RouterProps { history, children } = props.clone();

    let loc_ctx = use_reducer(|| LocationContext {
        location: history.location(),
        ctr: 0,
    });

    let navi_ctx = NavigatorContext {
        navigator: Navigator::new(history.clone()),
    };

    {
        let loc_ctx_dispatcher = loc_ctx.dispatcher();

        use_effect_with_deps(
            move |history| {
                let history = history.clone();
                // Force location update when history changes.
                loc_ctx_dispatcher.dispatch(history.location());

                let history_cb = {
                    let history = history.clone();
                    move || loc_ctx_dispatcher.dispatch(history.location())
                };

                let listener = history.listen(history_cb);

                // We hold the listener in the destructor.
                move || {
                    std::mem::drop(listener);
                }
            },
            history,
        );
    }

    html! {
        <ContextProvider<NavigatorContext> context={navi_ctx}>
            <ContextProvider<LocationContext> context={(*loc_ctx).clone()}>
                {children}
            </ContextProvider<LocationContext>>
        </ContextProvider<NavigatorContext>>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ConcreteRouterProps {
    pub children: Children,
}

/// A [`Router`] thats provides history via [`BrowserHistory`].
///
/// This Router uses browser's native history to manipulate session history
/// and uses regular URL as route.
#[function_component(BrowserRouter)]
pub fn browser_router(props: &ConcreteRouterProps) -> Html {
    let history = use_state(|| AnyHistory::from(BrowserHistory::new()));
    let children = props.children.clone();

    html! {
        <Router history={(*history).clone()}>
            {children}
        </Router>
    }
}

/// A [`Router`] thats provides history via [`HashHistory`].
///
/// This Router uses browser's native history to manipulate session history
/// and uses regular URL as route.
#[function_component(HashRouter)]
pub fn hash_router(props: &ConcreteRouterProps) -> Html {
    let history = use_state(|| AnyHistory::from(HashHistory::new()));
    let children = props.children.clone();

    html! {
        <Router history={(*history).clone()}>
            {children}
        </Router>
    }
}
