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
            <div class="parent">
                <div>
                    <h2 class="title">{ "Parent-to-Child Communication Example" }</h2>
                    <div class="parent-body">
                        <div class="parent-tag">
                            <span>{ "Parent" }</span>
                        </div>
                        <div class="parent-content">
                            <button {onclick}>{"Click"}</button>
                            <Child {clicks} />
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
