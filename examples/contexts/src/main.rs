mod msg_ctx;
mod producer;
mod subscriber;

use producer::Producer;
use subscriber::Subscriber;
use yew::prelude::*;

use msg_ctx::MessageProvider;

#[function_component]
pub fn Model() -> Html {
    html! {
        <MessageProvider>
            <Producer />
            <Subscriber />
        </MessageProvider>
    }
}

fn main() {
    yew::start_app::<Model>();
}
