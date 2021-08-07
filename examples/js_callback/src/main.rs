use wasm_bindgen::prelude::*;
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
    link: ComponentLink<Model>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            payload: String::default(),
            debugged_payload: format!("{:?}", ""),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
                let callback = self.link.callback(Msg::Payload);
                bindings::get_payload_later(Closure::once_into_js(move |payload: String| {
                    callback.emit(payload)
                }));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <textarea
                    class="code-block"
                    oninput={self.link.callback(|input: InputData| Msg::Payload(input.value))}
                    value={self.payload.clone()}
                />
                <button onclick={self.link.callback(|_| Msg::Payload(bindings::get_payload()))}>
                    { "Get the payload!" }
                </button>
                <button onclick={self.link.callback(|_| Msg::AsyncPayload)} >
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
