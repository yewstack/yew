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
                <div class="bg-zinc-900 text-zinc-100 min-h-screen min-w-screen flex flex-col justify-center items-center">
                    <div>
                        <h2 class="text-4xl mb-8">{ "Grandparent-to-Grandchild Communication Example" }</h2>

                        <div class="border-4 border-green-600 rounded-2xl">
                            <div class="bg-green-600 rounded-t px-3 pb-1 font-medium">
                                <span>{ "Grandparent" }</span>
                            </div>
                            <div class="flex flex-col px-5 pb-5 pt-5">
                                <button class="bg-green-600 hover:bg-green-800 rounded-xl text-lg pt-1 pb-2 px-3 font-medium mb-5 w-32" {onclick}>{"Click"}</button>
                                <Parent />
                            </div>
                        </div>
                    </div>
                </div>
            </ContextProvider<Rc<AppState>>>
        }
    }
}
