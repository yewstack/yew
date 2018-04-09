extern crate stdweb;
#[macro_use]
extern crate yew;

use stdweb::web::Date;
use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct Model {
    value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsRef<ConsoleService>,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model { value: 0 }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                context.as_ref().log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                context.as_ref().log("minus one");
            }
            Msg::Bulk(list) => for msg in list {
                self.update(msg, context);
                context.as_ref().log("Bulk action");
            },
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsRef<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment]),>{ "Increment Twice" }</button>
                </nav>
                <p>{ self.value }</p>
                <p>{ Date::new().to_string() }</p>
            </div>
        }
    }
}

