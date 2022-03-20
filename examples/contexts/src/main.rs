mod msg_ctx;
mod producer;
mod struct_component_subscriber;
mod subscriber;

use producer::Producer;
use struct_component_subscriber::StructComponentSubscriber;
use subscriber::Subscriber;
use yew::prelude::*;

use msg_ctx::MessageProvider;

#[function_component]
pub fn App() -> Html {
    html! {
        <MessageProvider>
            <Producer />
            <Subscriber />
            <StructComponentSubscriber />
        </MessageProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
