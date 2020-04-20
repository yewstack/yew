#![recursion_limit = "256"]

use stdweb::web::Date;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    console: ConsoleService,
    value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            console: ConsoleService::new(),
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                self.console.log("plus one");
            }
            Msg::Decrement => {
                self.value -= 1;
                self.console.log("minus one");
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg);
                    self.console.log("Bulk action");
                }
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::Increment)>
                        { "Increment" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::Decrement)>
                        { "Decrement" }
                    </button>
                    <button onclick=self.link.batch_callback(|_| vec![Msg::Increment, Msg::Increment])>
                        { "Increment Twice" }
                    </button>
                </nav>
                <p>{ self.value }</p>
                <p>{ Date::new().to_string() }</p>
            </div>
        }
    }
}
