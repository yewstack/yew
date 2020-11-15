#![recursion_limit = "256"]

use stdweb::web::Date;
use yew::services::ConsoleService;
use yew::{html, Component, Context, Html, ShouldRender};

pub struct Model {
    value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Model { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                ConsoleService::log("plus one");
            }
            Msg::Decrement => {
                self.value -= 1;
                ConsoleService::log("minus one");
            }
        }
        true
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
                <p>{ self.value }</p>
                <p>{ Date::new().to_string() }</p>
            </div>
        }
    }
}
