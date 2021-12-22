mod msg_ctx;
mod producer;
mod subscriber;

use producer::Producer;
use subscriber::Subscriber;
use yew::prelude::*;

use msg_ctx::{Message, MessageContext};

#[function_component(Model)]
pub fn model() -> Html {
    let msg = use_reducer(|| Message {
        inner: "No message yet.".to_string(),
    });

    html! {
        <ContextProvider<MessageContext> context={msg}>
            <Producer />
            <Subscriber />
        </ContextProvider<MessageContext>>
    }
}

fn main() {
    yew::start_app::<Model>();
}
