#![recursion_limit = "128"]
#![deny(warnings)]

use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsValue};
use yew::prelude::*;

pub struct Model {
    payload: String,
    // Pointless field just to have something that's been manipulated
    debugged_payload: String,
    link: ComponentLink<Model>,
}

pub enum Msg {
    Payload(String),
    AsyncPayload,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let payload = String::default();
        let debugged_payload = format!("{:?}", payload);
        Self {
            payload,
            debugged_payload,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            Payload(payload) => {
                if payload != self.payload {
                    self.debugged_payload = format!("{:?}", payload);
                    self.payload = payload;
                    true
                } else {
                    false
                }
            }
            AsyncPayload => {
                get_payload_later(self.link.callback(Msg::Payload));
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <textarea oninput=self.link.callback(move |input: InputData| Msg::Payload(input.value))
                    style="font-family: 'Monaco' monospace;"
                    value={ &self.payload }>
                </textarea>
                <button onclick=self.link.callback(|_| Msg::Payload(get_payload()))>
                    { "Get the payload!" }
                </button>
                <button onclick=self.link.callback(|_| Msg::AsyncPayload) >
                    { "Get the payload later!" }
                </button>
                <p style="font-family: 'Monaco', monospace;">
                    { nbsp(self.debugged_payload.as_str()) }
                </p>
            </div>
        }
    }
}

fn nbsp<T>(string: T) -> String
where
    String: From<T>,
{
    String::from(string).replace(' ', "\u{00a0}")
}

#[wasm_bindgen]
extern "C" {
    fn get_payload() -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "get_payload_later")]
    fn get_payload_later_js(payload_callback: JsValue);
}

fn get_payload_later(payload_callback: Callback<String>) {
    let callback = Closure::once_into_js(move |payload: String| payload_callback.emit(payload));
    get_payload_later_js(callback);
}
