//! Router Component.

use crate::prelude::*;
use yew::prelude::*;

/// Props for [`Router`]
#[derive(Properties, PartialEq, Clone)]
pub struct RouterProps<H>
where
    H: History + 'static,
{
    pub children: Children,
    pub history: H,
}

/// A context for [`History`]
#[derive(Clone)]
pub(crate) struct RouterState<H>
where
    H: History + 'static,
{
    history: H,
    // Counter to force update.
    ctr: u32,
}

impl<H> RouterState<H>
where
    H: History + 'static,
{
    pub fn history(&self) -> H {
        self.history.clone()
    }
}

impl<H> PartialEq for RouterState<H>
where
    H: History + 'static,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.ctr == rhs.ctr
    }
}

#[doc(hidden)]
pub enum Msg {
    ReRender,
}

/// The router component.
///
/// When a route can't be matched, it looks for the route with `not_found` attribute.
/// If such a route is provided, it redirects to the specified route.
/// Otherwise `html! {}` is rendered and a message is logged to console
/// stating that no route can be matched.
/// See the [crate level document][crate] for more information.
pub struct Router<H>
where
    H: History + 'static,
{
    _listener: HistoryListener,
    history: H,
    ctr: u32,
}

impl<H> Component for Router<H>
where
    H: History + 'static,
{
    type Message = Msg;
    type Properties = RouterProps<H>;

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
        let typed_state = RouterState {
            history: self.history.clone(),
            ctr: self.ctr,
        };

        let any_state = RouterState {
            history: self.history.clone().into_any_history(),
            ctr: self.ctr,
        };

        html! {
            <ContextProvider<RouterState<H>> context={typed_state}>
                <ContextProvider<RouterState<AnyHistory>> context={any_state}>
                    {ctx.props().children.clone()}
                </ContextProvider<RouterState<AnyHistory>>>
            </ContextProvider<RouterState<H>>>
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct BrowserRouterProps {
    pub children: Children,
}

#[function_component(BrowserRouter)]
pub fn browser_router(props: &BrowserRouterProps) -> Html {
    let history = use_state(BrowserHistory::new);
    let children = props.children.clone();

    html! {
        <Router<BrowserHistory> history={(*history).clone()}>
            {children}
        </Router<BrowserHistory>>
    }
}
