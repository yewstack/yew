//! Router Component.

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

#[doc(hidden)]
pub enum Msg {
    ReRender,
}

/// The Router component.
///
/// This provides [`History`] context to its children and switches.
///
/// You only need one `<Router />` for each application.
pub struct Router {
    _listener: HistoryListener,
    history: AnyHistory,
    ctr: u32,
}

impl Component for Router {
    type Message = Msg;
    type Properties = RouterProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();

        let listener = ctx
            .props()
            .history
            .listen(move || link.send_message(Msg::ReRender));

        Self {
            _listener: listener,
            history: ctx.props().history.clone(),
            ctr: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReRender => {
                self.ctr += 1;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let link = ctx.link().clone();

        if self.history != ctx.props().history {
            self._listener = ctx
                .props()
                .history
                .listen(move || link.send_message(Msg::ReRender));

            self.history = ctx.props().history.clone();

            true
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = RouterState {
            history: self.history.clone().into_any_history(),
            ctr: self.ctr,
        };

        html! {
            <ContextProvider<RouterState> context={state}>
                {ctx.props().children.clone()}
            </ContextProvider<RouterState>>
        }
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
