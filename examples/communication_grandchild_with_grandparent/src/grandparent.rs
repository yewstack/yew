use super::*;

pub enum Msg {
    ButtonClick(AttrValue),
}

/// Our top-level (grandparent) component that holds a reference to the shared state.
pub struct GrandParent {
    state: Rc<AppState>,
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

        let detail_msg = if let Some(last_clicked) = &self.state.last_clicked {
            format!("The last child you clicked was {last_clicked}.")
        } else {
            "Waiting for you to click a granchild...".to_string()
        };

        html! {
            <ContextProvider<Rc<AppState>> context={app_state}>
                <div class="bg-zinc-900 text-zinc-100 min-h-screen min-w-screen flex flex-col justify-center items-center">
                    <div>
                        <h2 class="text-4xl mb-8">{ "Grandchild-with-Grandparent Communication Example" }</h2>
                        <div class="border-4 border-green-600 rounded-2xl">
                            <div class="bg-green-600 rounded-t px-3 pb-1 font-medium">
                                <span>{ "Grandparent" }</span>
                            </div>
                            <div class="flex flex-col px-5 pb-5 pt-3">
                                <span class="text-xl">{ "My grandchildren have been clicked " }<span class="font-bold">{ self.state.total_clicks }</span>{ " times." }</span>
                                <div class="text-xl my-3">{detail_msg}</div>
                                <Parent />
                            </div>
                        </div>
                    </div>
                </div>
            </ContextProvider<Rc<AppState>>>
        }
    }
}
