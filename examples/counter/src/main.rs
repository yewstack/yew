use gloo_console as console;
use gloo::timers::callback::Interval;
use js_sys::Date;
use yew::{html, Component, Context, Html};

// Define the possible messages which can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
    HoldDown,
    HoldUp,
    DropInterval,
}

pub struct App {
    value: i64, // This will store the counter value
    interval: Option<Interval>, // Option<T> because we will cancel it via None
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0, interval: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                console::log!("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.value -= 1;
                console::log!("minus one");
                true
            }
            Msg::HoldDown => {
                console::log!("hold down");
                let interval = {
                    let link = _ctx.link().clone();
                    Interval::new(200, move|| {
                        link.send_message(Msg::Decrement)
                    })
                };
                self.interval = Some(interval);
                true
            }
            Msg::HoldUp => {
                console::log!("hold up");
                let interval = {
                    let link = _ctx.link().clone();
                    Interval::new(200, move || {
                        link.send_message(Msg::Increment)
                    })
                };
                self.interval = Some(interval);
                true
            }
            Msg::DropInterval => {
                console::log!("drop interval");
                if self.interval.is_some() {
                    self.interval = None; // This will cancel our interval
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="panel">
                    // A button to send HoldDown message
                    <button
                        onmousedown={ctx.link().callback(|_| Msg::HoldDown)}
                        onmouseup={ctx.link().callback(|_| Msg::DropInterval)}
                    >
                        { "<<" }
                    </button>

                    // A button to send the Decrement message
                    <button onclick={ctx.link().callback(|_| Msg::Decrement)}>
                        { "-1" }
                    </button>

                    // A button to send the Increment message
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
                        { "+1" }
                    </button>

                    // A button to send two Increment messages
                    <button onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
                        { "+1, +1" }
                    </button>

                    // A button to send HoldUp message
                    <button
                        onmousedown={ctx.link().callback(|_| Msg::HoldUp)}
                        onmouseup={ctx.link().callback(|_| Msg::DropInterval)}
                    >
                        { ">>" }
                    </button>

                </div>

                // Display the current value of the counter
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
    yew::Renderer::<App>::new().render();
}
