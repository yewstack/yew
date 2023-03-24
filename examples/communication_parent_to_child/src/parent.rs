use super::*;

/// The `Parent` component holds some state that is passed down to the children.
pub struct Parent {
    /// The total number of clicks received
    nr_of_clicks: u32,
}

pub enum Msg {
    ButtonClick,
}

impl Component for Parent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { nr_of_clicks: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClick => {
                self.nr_of_clicks += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::ButtonClick);

        // Here we pass down "our" nr_of_clicks to the child by setting the "clicks" property.
        let clicks = self.nr_of_clicks;

        html! {
            <div class="bg-zinc-900 text-zinc-100 min-h-screen min-w-screen flex flex-col justify-center items-center">
                <div>
                    <h2 class="text-4xl mb-8">{ "Parent-to-Child Communication Example" }</h2>
                    <div class="border-4 border-green-600 rounded-2xl">
                        <div class="bg-green-600 rounded-t px-3 pb-1 font-medium">
                            <span>{ "Parent" }</span>
                        </div>
                        <div class="flex flex-col px-5 pb-5 pt-5">
                            <button class="bg-green-600 hover:bg-green-800 rounded-xl text-lg pt-1 pb-2 px-3 font-medium mb-5" {onclick}>{"Click"}</button>
                            <Child {clicks} />
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
