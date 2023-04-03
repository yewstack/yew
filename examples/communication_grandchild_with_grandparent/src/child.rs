use super::*;

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
        let name = format!("I'm {my_name}.");

        // Here we emit the callback to the grandparent component, whenever the button is clicked.
        let onclick = self.state.child_clicked.reform(move |_| (my_name.clone()));

        html! {
            <div class="child-body">
                <div class="child-tag">
                    <span>{ "Child" }</span>
                </div>
                <div>
                    <span>{ "We've been clicked " }<span>{ self.state.total_clicks }</span>{ " times." }</span>
                </div>
                <div class="child-content">
                    <span>{ name }</span>
                    <button {onclick}>{"Click"}</button>
                </div>
            </div>
        }
    }
}
