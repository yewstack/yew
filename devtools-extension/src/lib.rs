#[macro_use]
extern crate serde;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentMessage {
    /// The event which is to be logged.
    event: ComponentEvent,
    /// Optional additional data about the event (e.g. the component's location in the DOM).
    data: Option<DebugComponent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DebugComponent {
    /// The name of the component
    name: String,
    selector: Option<String>,
}

struct DevToolsExtension {
    ws_task: yew::services::websocket::WebSocketTask,
    component_tree: indextree::Arena<ComponentMessage>,
}

enum DevToolsExtensionMsg {
    ReceiveWSMessage(String),
    ReceiveWSStatus(yew::services::websocket::WebSocketStatus),
}

impl Component for DevToolsExtension {
    type Message = DevToolsExtensionMsg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ws_task = yew::services::WebSocketService::new()
            .connect(
                "ws://localhost:8017/ws",
                link.callback(|text| DevToolsExtensionMsg::ReceiveWSMessage(text)),
                link.callback(|status| DevToolsExtensionMsg::ReceiveWSStatus(status)),
            )
            .unwrap();
        DevToolsExtension {
            ws_task,
            component_tree: indextree::Arena::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            DevToolsExtensionMsg::ReceiveWSMessage(message) => {}
            DevToolsExtensionMsg::ReceiveWSStatus(status) => {}
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <h1>{"Yew DevTools"}</h1>
        }
    }
}
