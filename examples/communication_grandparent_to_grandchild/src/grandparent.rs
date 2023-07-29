use super::*;

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
                <div class="grandparent">
                    <div>
                        <h2 class="title">{ "Grandparent-to-Grandchild Communication Example" }</h2>

                        <div class="grandparent-body">
                            <div class="grandparent-tag">
                                <span>{ "Grandparent" }</span>
                            </div>
                            <div class="grandparent-content">
                                <button {onclick}>{"Click"}</button>
                                <Parent />
                            </div>
                        </div>
                    </div>
                </div>
            </ContextProvider<Rc<AppState>>>
        }
    }
}
