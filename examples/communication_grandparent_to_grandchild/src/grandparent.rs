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