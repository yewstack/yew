use super::*;

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
        html! {

            <div class="border-4 border-purple-600 rounded-2xl flex-grow mt-2">
                <div class="bg-purple-600 rounded-t px-3 pb-1 font-medium">
                    <span>{ "Child" }</span>
                </div>
                <div class="px-5 py-3">
                    <span class="text-xl">{ "My grandparent has been clicked " }<span class="font-bold">{ self.state.total_clicks }</span>{ " times." }</span>
                </div>
            </div>
        }
    }
}
