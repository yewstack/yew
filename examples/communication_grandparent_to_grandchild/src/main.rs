use std::rc::Rc;

use yew::{function_component, html, Component, Context, ContextHandle, ContextProvider, Html};

/// This is the shared state between the parent and child components.
#[derive(Clone, Eq, PartialEq)]
pub struct AppState {
    /// The total number of clicks received.
    total_clicks: u32,
}

/// Our top-level (grandparent) component that holds a reference to the shared state.
pub struct GrandParent {
    state: Rc<AppState>,
}

pub enum Msg {
    ButtonClick,
}

impl Component for GrandParent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let state = Rc::new(AppState { total_clicks: 0 });
        Self { state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClick => {
                Rc::make_mut(&mut self.state).total_clicks += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::ButtonClick);
        let app_state = self.state.clone();

        html! {
            <ContextProvider<Rc<AppState>> context={app_state}>
                <div class="app">
                    <div class="parent">
                        <h2>{ "Grandparent-to-Grandchild communication example" }</h2>
                        <div class="button-panel">
                            <button class="button" {onclick}>{"Click here!"}</button>
                        </div>
                        <Parent />
                    </div>
                </div>
            </ContextProvider<Rc<AppState>>>
        }
    }
}

/// The `Parent` component is the parent of the `Child` component. It has no logic, and is here to
/// show there is no direct relation between grandchild and grandparent.
#[function_component]
fn Parent() -> Html {
    html! {
        <Child />
    }
}

/// The `Child` component is the child of the `Parent` component, and will receive updates from the
/// grandparent using the context.
pub struct Child {
    state: Rc<AppState>,
    _listener: ContextHandle<Rc<AppState>>,
}

pub enum ChildMsg {
    ContextChanged(Rc<AppState>),
}

impl Component for Child {
    type Message = ChildMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Here we fetch the shared state from the context. For a demonstration on the use of
        // context in a functional component, have a look at the `examples/contexts` code.
        let (state, _listener) = ctx
            .link()
            .context::<Rc<AppState>>(ctx.link().callback(ChildMsg::ContextChanged))
            .expect("context to be set");

        Self { state, _listener }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChildMsg::ContextChanged(state) => {
                self.state = state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let msg = format!(
            "My grandparent has been clicked {} times",
            self.state.total_clicks
        );

        html! {
            <div class="child">
                <div>{msg}</div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<GrandParent>::new().render();
}
