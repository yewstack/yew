//! Router Component.
use std::rc::Rc;

use crate::prelude::*;
use yew::prelude::*;

/// Props for [`Router`].
#[derive(Properties, PartialEq, Clone)]
pub struct RouterProps {
    pub children: Children,
    pub history: AnyHistory,
}

/// A context for [`Router`]
#[derive(Clone)]
pub(crate) struct RouterState {
    history: AnyHistory,
    // Counter to force update.
    ctr: u32,
}

impl RouterState {
    pub fn history(&self) -> AnyHistory {
        self.history.clone()
    }
}

impl PartialEq for RouterState {
    fn eq(&self, rhs: &Self) -> bool {
        self.ctr == rhs.ctr
    }
}

pub(crate) enum RouterStateAction {
    Navigate,
    ReplaceHistory(AnyHistory),
}

impl Reducible for RouterState {
    type Action = RouterStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let history = match action {
            RouterStateAction::Navigate => self.history(),
            RouterStateAction::ReplaceHistory(m) => m,
        };

        Self {
            history,
            ctr: self.ctr + 1,
        }
        .into()
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

    let state = use_reducer(|| RouterState {
        history: history.clone(),
        ctr: 0,
    });

    {
        let state_dispatcher = state.dispatcher();

        use_effect_with_deps(
            move |history| {
                state_dispatcher.dispatch(RouterStateAction::ReplaceHistory(history.clone()));

                let listener =
                    history.listen(move || state_dispatcher.dispatch(RouterStateAction::Navigate));

                // We hold the listener in the destructor.
                move || {
                    std::mem::drop(listener);
                }
            },
            history,
        );
    }

    html! {
        <ContextProvider<RouterState> context={(*state).clone()}>
            {children}
        </ContextProvider<RouterState>>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct BrowserRouterProps {
    pub children: Children,
}

/// A [`Router`] thats provides history via [`BrowserHistory`].
///
/// This Router uses browser's native history to manipulate session history
/// and uses regular URL as route.
#[function_component(BrowserRouter)]
pub fn browser_router(props: &BrowserRouterProps) -> Html {
    let history = use_state(BrowserHistory::new);
    let children = props.children.clone();

    html! {
        <Router history={(*history).clone().into_any_history()}>
            {children}
        </Router>
    }
}
