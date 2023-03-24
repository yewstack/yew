use super::*;

pub enum Msg {
    ButtonClick(AttrValue),
}

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
        let last_updated_msg = if let Some(last_updated) = self.last_updated.as_ref() {
            format!("The last child you clicked was {last_updated}.")
        } else {
            "Waiting for you to click a child...".to_string()
        };

        let on_clicked = ctx.link().callback(Msg::ButtonClick);
        html! {
            <div class="bg-zinc-900 text-zinc-100 min-h-screen min-w-screen flex flex-col justify-center items-center">
                <div>
                    <h2 class="text-4xl mb-8">{ "Child-to-Parent Communication Example" }</h2>
                    <div class="border-4 border-green-600 rounded-2xl">
                        <div class="bg-green-600 rounded-t px-3 pb-1 font-medium">
                            <span>{ "Parent" }</span>
                        </div>
                        <div class="flex flex-col px-5 pb-5 pt-3">
                            <span class="text-xl">{ "My children have been clicked " }<span class="font-bold">{ self.total_clicks }</span>{ " times." }</span>
                            <span class="text-xl my-3">{ last_updated_msg }</span>
                            <div class="flex mt-3 gap-x-5">
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
