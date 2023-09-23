use yew::prelude::*;

use super::msg_ctx::MessageContext;

#[function_component]
pub fn Producer() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    html! {
        <button onclick={move |_| msg_ctx.dispatch("Message Received.".to_string())}>
            {"PRESS ME"}
        </button>
    }
}
