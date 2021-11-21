use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

mod bindings;

pub enum Msg {
    Payload(String),
    AsyncPayload,
}

pub struct Model {
    payload: String,
    // Pointless field just to have something that's been manipulated
    debugged_payload: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            payload: String::default(),
            debugged_payload: format!("{:?}", ""),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Payload(payload) => {
                if payload != self.payload {
                    self.debugged_payload = format!("{:?}", payload);
                    self.payload = payload;
                    true
                } else {
                    false
                }
            }
            Msg::AsyncPayload => {
                let callback = ctx.link().callback(Msg::Payload);
                bindings::get_payload_later(Closure::once_into_js(move |payload: String| {
                    callback.emit(payload)
                }));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <textarea
                    class="code-block"
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input: HtmlTextAreaElement = e.target_unchecked_into();
                        Msg::Payload(input.value())
                    })}
                    value={self.payload.clone()}
                />
                <button onclick={ctx.link().callback(|_| Msg::Payload(bindings::get_payload()))}>
                    { "Get the payload!" }
                </button>
                <button onclick={ctx.link().callback(|_| Msg::AsyncPayload)} >
                    { "Get the payload later!" }
                </button>
                <p class="code-block">
                    { &self.debugged_payload }
                </p>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
