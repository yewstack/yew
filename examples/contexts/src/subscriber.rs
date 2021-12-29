use super::msg_ctx::MessageContext;

use yew::prelude::*;

#[function_component]
pub fn Subscriber() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    let message = msg_ctx.inner.to_owned();

    html! {
        <h1>{ message }</h1>
    }
}
