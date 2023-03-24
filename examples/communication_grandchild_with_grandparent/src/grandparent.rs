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
        let msg = format!(
            "My grandchildren have been clicked {} times",
            self.state.total_clicks
        );

        let detail_msg = if let Some(last_clicked) = &self.state.last_clicked {
            format!("{last_clicked} was clicked last")
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