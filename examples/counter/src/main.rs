use js_sys::Date;
use yew::component::{Component, Context};
use yew::services::ConsoleService;
use yew::{html, Html, ShouldRender};

pub enum Msg {
    Increment,
    Decrement,
}

pub struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                ConsoleService::log("plus one");
                true
            }
            Msg::Decrement => {
                self.value -= 1;
                ConsoleService::log("minus one");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=ctx.callback(|_| Msg::Increment)>
                        { "Increment" }
                    </button>
                    <button onclick=ctx.callback(|_| Msg::Decrement)>
                        { "Decrement" }
                    </button>
                    <button onclick=ctx.batch_callback(|_| vec![Msg::Increment, Msg::Increment])>
                        { "Increment Twice" }
                    </button>
                </nav>
                <p>
                    <b>{ "Current value: " }</b>
                    { self.value }
                </p>
                <p>
                    <b>{ "Rendered at: " }</b>
                    { String::from(Date::new_0().to_string()) }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
