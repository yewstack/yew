use super::msg_ctx::MessageContext;

use yew::prelude::*;

#[function_component(Subscriber)]
pub fn subscriber() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    let message = msg_ctx.inner.to_owned();

    html! {
        <h1>{ message }</h1>
    }
}
