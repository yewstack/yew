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
    _ws_task: yew::services::websocket::WebSocketTask,
    component_tree: indextree::Arena<ComponentRepr>,
    root_node: Option<indextree::NodeId>,
}

enum DevToolsExtensionMsg {
    ReceiveWSMessage(String),
    ReceiveWSStatus(yew::services::websocket::WebSocketStatus),
    Noop,
}

struct ComponentRepr {
    name: String,
    selector: String,
    is_in_dom: bool,
}

impl Component for DevToolsExtension {
    type Message = DevToolsExtensionMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ws_task = yew::services::WebSocketService::new()
            .connect_text(
                "ws://localhost:8017/ws",
                link.callback(|text| match text {
                    Ok(t) => DevToolsExtensionMsg::ReceiveWSMessage(t),
                    Err(_) => {
                        yew::web_sys::console::error_1(
                            &"There was an error with the WebSocket connection.".into(),
                        );
                        DevToolsExtensionMsg::Noop
                    }
                }),
                link.callback(DevToolsExtensionMsg::ReceiveWSStatus),
            )
            .unwrap();
        DevToolsExtension {
            _ws_task: ws_task,
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
                            &format!(
                                "Received an invalid message from the DevTools server. \
                            Error message: `{:?}`",
                                e
                            )
                            .into(),
                        );
                        return false;
                    }
                };
                match message.event {
                    ComponentEvent::Mounted => {
                        let selector = message.data.unwrap().selector.unwrap();
                        let relevant_node = self
                            .component_tree
                            .iter()
                            .find(|node| node.get().selector == selector)
                            .unwrap()
                            .get_mut();
                        relevant_node.is_in_dom = true;
                    }
                    ComponentEvent::Updated => {}
                    ComponentEvent::Created => {
                        if let Some(root_node) = self.root_node {
                            let selector = message.data.unwrap().selector.unwrap();
                            let youngest_parent =
                                self.component_tree.iter().fold(root_node, |acc, val| {
                                    if self
                                        .component_tree
                                        .get(root_node)
                                        .unwrap()
                                        .get()
                                        .selector
                                        .starts_with(&selector)
                                        && self
                                            .component_tree
                                            .get(root_node)
                                            .unwrap()
                                            .get()
                                            .selector
                                            .starts_with(&val.get().selector)
                                    {
                                        self.component_tree.get_node_id(val).unwrap()
                                    } else {
                                        root_node
                                    }
                                });
                            let component_node = self.component_tree.new_node(ComponentRepr {
                                name: "".to_string(),
                                selector,
                                is_in_dom: false,
                            });
                            youngest_parent.append(component_node, &mut self.component_tree);
                        } else {
                            self.root_node = Some(self.component_tree.new_node(ComponentRepr {
                                name: "".to_string(),
                                selector: "".to_string(),
                                is_in_dom: false,
                            }))
                        }
                    }
                    ComponentEvent::Destroyed => {
                        let found_node = self
                            .component_tree
                            .iter()
                            .find(|node| {
                                node.get().selector == message.data.unwrap().selector.unwrap()
                            })
                            .unwrap();
                        let found_node = self.component_tree.get_node_id(found_node).unwrap();
                        found_node.remove(&mut self.component_tree);
                    }
                    ComponentEvent::Unmounted => {
                        let selector = message.data.unwrap().selector.unwrap();
                        let relevant_node = self
                            .component_tree
                            .iter()
                            .find(|node| node.get().selector == selector)
                            .unwrap()
                            .get_mut();
                        relevant_node.is_in_dom = false;
                    }
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
            DevToolsExtensionMsg::Noop => false,
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
