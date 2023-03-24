use super::*;

/// The `Parent` component holds some state that is updated when its children are clicked
pub struct Parent {
    /// The total number of clicks received
    total_clicks: u32,
    /// The name of the child that was last clicked
    last_updated: Option<AttrValue>,
}
impl Component for Parent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            total_clicks: 0,
            last_updated: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClick(childs_name) => {
                // Keep track of the name of the child that was clicked
                self.last_updated = Some(childs_name);

                // Increment the total number of clicks
                self.total_clicks += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let msg = format!("My children have been clicked {} times.", self.total_clicks);

        let last_updated_msg = if let Some(last_updated) = self.last_updated.as_ref() {
            format!("The last child you clicked was {last_updated}.")
        } else {
            "Waiting for you to click a child...".to_string()
        };

        let on_clicked = ctx.link().callback(Msg::ButtonClick);
        html! {
            <div class="app">
                <div>
                    <h2>{ "Child-to-Parent Communication Example" }</h2>
                    <div class="parent">
                        <div class="tab">
                            <span class="bg-green">{ "Parent" }</span>
                        </div>
                        <div class="parent-body-1">
                            <div>{ msg }</div>
                            <div>{ last_updated_msg }</div>
                            <div class="spacer" />
                            <div class="parent-body-2">
                                <Child name="Alice" on_clicked={on_clicked.clone()} />
                                <Child name="Bob" {on_clicked} />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}