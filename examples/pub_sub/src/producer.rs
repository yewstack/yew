use yew::prelude::*;

use super::msg_ctx::MessageContext;

#[function_component(Producer)]
pub fn producer() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    let onclick = Callback::from(move |_| msg_ctx.dispatch("Message Received.".to_string()));

    html! {
        <button {onclick}>
            {"PRESS ME"}
        </button>
    }
}
