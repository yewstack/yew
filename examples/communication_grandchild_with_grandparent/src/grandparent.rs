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
                let shared_state = Rc::make_mut(&mut self.state);
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
            "Waiting for you to click a grandchild...".to_string()
        };

        html! {
            <ContextProvider<Rc<AppState>> context={app_state}>
                <div class="grandparent">
                    <div>
                        <h2 class="title">{ "Grandchild-with-Grandparent Communication Example" }</h2>
                        <div class="grandparent-body">
                            <div class="grandparent-tag">
                                <span>{ "Grandparent" }</span>
                            </div>
                            <div class="grandparent-content">
                                <span>{ "My grandchildren have been clicked " }<span>{ self.state.total_clicks }</span>{ " times." }</span>
                                <span>{detail_msg}</span>
                                <Parent />
                            </div>
                        </div>
                    </div>
                </div>
            </ContextProvider<Rc<AppState>>>
        }
    }
}
