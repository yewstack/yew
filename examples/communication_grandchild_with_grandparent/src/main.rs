use std::rc::Rc;

use yew::{
    function_component, html, AttrValue, Callback, Component, Context, ContextHandle,
    ContextProvider, Html, Properties,
};

/// This is the shared state between the parent and child components.
#[derive(Clone, PartialEq)]
pub struct AppState {
    /// Total number of clicks received.
    total_clicks: u32,
    /// Callback used when a child is clicked. The AttrValue is the name of the child that was
    /// clicked.
    child_clicked: Callback<AttrValue>,
    /// The name of the child that was last clicked.
    last_clicked: Option<AttrValue>,
}

/// Our top-level (grandparent) component that holds a reference to the shared state.
pub struct GrandParent {
    state: Rc<AppState>,
}

pub enum Msg {
    ButtonClick(AttrValue),
}

impl Component for GrandParent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let child_clicked = ctx.link().callback(Msg::ButtonClick);
        let state = Rc::new(AppState {
            total_clicks: 0,
            child_clicked,
            last_clicked: None,
        });
        Self { state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClick(childs_name) => {
                // Update the shared state
                let mut shared_state = Rc::make_mut(&mut self.state);
                shared_state.total_clicks += 1;
                shared_state.last_clicked = Some(childs_name);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let app_state = self.state.clone();
        let msg = format!(
            "My grandchildren have been clicked {} times",
            self.state.total_clicks
        );

        let detail_msg = if let Some(last_clicked) = &self.state.last_clicked {
            format!("{} was clicked last", last_clicked)
        } else {
            "No one has been clicked yet".to_string()
        };

        html! {
            <ContextProvider<Rc<AppState>> context={app_state}>
                <div class="app">
                    <div class="parent">
                        <h2>{ "Grandchild-with-Grandparent communication example" }</h2>
                        <div>{msg}</div>
                        <div>{detail_msg}</div>
                        <div class="spacer" />
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
        <>
            <Child name="Alice" />
            <Child name="Bob" />
        </>
    }
}

/// The `Child` component is the child of the `Parent` component, and will send and receive updates
/// to/from the grandparent using the context.
pub struct Child {
    state: Rc<AppState>,
    _listener: ContextHandle<Rc<AppState>>,
}

pub enum ChildMsg {
    ContextChanged(Rc<AppState>),
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct ChildProps {
    pub name: AttrValue,
}

impl Component for Child {
    type Message = ChildMsg;
    type Properties = ChildProps;

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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let my_name = ctx.props().name.clone();
        let name = format!("{}: ", my_name);

        // Here we emit the callback to the grandparent component, whenever the button is clicked.
        let onclick = self.state.child_clicked.reform(move |_| (my_name.clone()));

        let msg = format!("We've been clicked: {} times", self.state.total_clicks);

        html! {
            <div class="child">
                <div class="child-name">
                    <div>{name}</div>
                </div>
                <div class="button-panel">
                    <button class="button" {onclick}>{"Click here"}</button>
                </div>
                <div class="status-message">
                    <div>{msg}</div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<GrandParent>::new().render();
}
