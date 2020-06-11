#[macro_use]
extern crate serde;
use yew::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Debug)]
pub enum ComponentEvent {
    /// Sent when a component mounts to the DOM
    Mounted,
    /// Sent when a component unmounts from the DOM
    Unmounted,
    /// Sent when a component updates itself
    Updated,
    /// Sent when a component is created
    Created,
    /// Sent when a component is destroyed
    Destroyed,
}

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
    ws_service: yew::services::websocket::WebSocketService,
    component_tree: indextree::Arena<ComponentMessage>,
    root_node: Option<indextree::NodeId>,
}

enum DevToolsExtensionMsg {
    ReceiveWSMessage(String),
    ReceiveWSStatus(yew::services::websocket::WebSocketStatus),
    Nop,
}

struct ComponentRepr {
    name: String,
    selector: String,
    pending: bool,
}

fn compute_depth(string: String) -> usize {
    string.matches('/').count() + 1
}

impl Component for DevToolsExtension {
    type Message = DevToolsExtensionMsg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut ws_service = yew::services::WebSocketService::new();
        let ws_task = ws_service
            .connect_text(
                "ws://localhost:8017/ws",
                link.callback(|text| match text {
                    Ok(t) => DevToolsExtensionMsg::ReceiveWSMessage(t),
                    Err(_) => {
                        yew::web_sys::console::error_1(
                            &"There was an error with the WebSocket connection.".into(),
                        );
                        DevToolsExtensionMsg::Nop
                    }
                }),
                link.callback(|status| DevToolsExtensionMsg::ReceiveWSStatus(status)),
            )
            .unwrap();
        DevToolsExtension {
            ws_service,
            ws_task,
            component_tree: indextree::Arena::new(),
            root_node: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            DevToolsExtensionMsg::ReceiveWSMessage(message) => {
                let message: ComponentMessage = match serde_json::from_str(&message) {
                    Ok(t) => t,
                    Err(e) => {
                        yew::web_sys::console::error_1(
                            &"Received an invalid message from the DevTools server.".into(),
                        );
                        return false;
                    }
                };
                match message.event {
                    ComponentEvent::Mounted => {}
                    ComponentEvent::Updated => {}
                    ComponentEvent::Created => {}
                    ComponentEvent::Destroyed => {}
                    ComponentEvent::Unmounted => {}
                }
                false
            }
            DevToolsExtensionMsg::ReceiveWSStatus(status) => {
                if let yew::services::websocket::WebSocketStatus::Error = status {
                    yew::web_sys::console::error_1(
                        &"An error occurred with the WebSocket connection. If this \
                        persists, consider filing a bug report at github.com/yewstack/yew"
                            .into(),
                    )
                }
                false
            }
            DevToolsExtensionMsg::Nop => false,
        }
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
