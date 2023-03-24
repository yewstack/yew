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
            <div class="app">
                <div class="parent">
                    <h2>{ "Parent-to-Child communication example" }</h2>
                    <div class="button-panel">
                        <button class="button" {onclick}>{"Click here!"}</button>
                    </div>
                    <Child {clicks} />
                </div>
            </div>
        }
    }
}