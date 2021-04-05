use js_sys::Date;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_services::ConsoleService;

// Define possible messages
pub enum Msg {
    Increment,
    Decrement,
}

// Define model for this component
pub struct Model {
    link: ComponentLink<Self>,
    value: i64, // This will store the counter value
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                ConsoleService::log("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.value -= 1;
                ConsoleService::log("minus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="panel">
                    // A button to send the Increment message
                    <button class="button" onclick=self.link.callback(|_| Msg::Increment)>
                        { "+1" }
                    </button>

                    // A button to send the Decrement message
                    <button onclick=self.link.callback(|_| Msg::Decrement)>
                        { "-1" }
                    </button>

                    // A button to send two Increment messages
                    <button onclick=self.link.batch_callback(|_| vec![Msg::Increment, Msg::Increment])>
                        { "+1, +1" }
                    </button>

                </div>

                // Display the contents of the component
                <p class="counter">
                    { self.value }
                </p>

                // Display the current date and time the page was rendered
                <p class="footer">
                    { "Rendered: " }
                    { String::from(Date::new_0().to_string()) }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
