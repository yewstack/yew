use yew::prelude::*;

use super::msg_ctx::MessageContext;

pub struct StructComponentProducer;

impl Component for StructComponentProducer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (msg_ctx, _) = ctx
            .link()
            .context::<MessageContext>(Callback::noop())
            .expect("No Message Context Provided");

        html! {
            <button onclick={move |_| msg_ctx.dispatch("Other message received.".to_owned())}>
                {"OR ME"}
            </button>
        }
    }
}
