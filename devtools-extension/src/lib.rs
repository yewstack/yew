//! Developer tools extension for Yew.
//!
//! This is shipped as the browser extension.

#[macro_use]
extern crate serde;

use yew::prelude::*;
use yew::services::websocket::WebSocketTask;

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
    #[cfg(not(feature = "logic_test"))]
    /// The WebSocket task – this handles the receiving data from the WebSocket connection.
    ///
    /// It isn't directly used, but it needs to be kept alive for the duration of the program.
    _ws_task: yew::services::websocket::WebSocketTask,
    /// Stores the component tree.
    component_tree: indextree::Arena<ComponentRepr>,
    /// The root component. Storing this is useful and makes it easier to implement some methods
    /// than it otherwise would be.
    root_node: Option<indextree::NodeId>,
}

enum DevToolsExtensionMsg {
    ReceiveWSMessage(String),
    ReceiveWSStatus(yew::services::websocket::WebSocketStatus),
    Noop,
}

#[derive(Debug, Clone, PartialEq)]
/// Used to store all the relevant information about a component in the extension.
struct ComponentRepr {
    /// The name of the component – this is obtained inside of the `yew` crate by using
    /// `std::any::type_name::<COMP>()` and corresponds to the name of the item (e.g. a struct or an
    /// enum) which the relevant implementation of `Component` refers to.
    name: String,
    selector: String,
    /// Whether or not a component is in the browser's DOM.
    is_in_dom: bool,
    /// Whether or not a component's children should be shown or not.
    collapsed: bool,
}

impl std::convert::Into<Html> for &ComponentRepr {
    /// Converst a `ComponentRepr` into `Html` so that it can be rendered to the DOM.
    fn into(self) -> Html {
        html! {
            <>
                <h3>{self.name.clone()}</h3>
                <p>{self.selector.clone()}</p>
                <p>{match self.is_in_dom {
                    true => "In DOM",
                    false => "Not in DOM"
                }}</p>
            </>
        }
    }
}

impl Component for DevToolsExtension {
    type Message = DevToolsExtensionMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        #[cfg(not(feature = "logic_test"))]
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
            #[cfg(not(feature = "logic_test"))]
            _ws_task: ws_task,
            component_tree: indextree::Arena::new(),
            root_node: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            DevToolsExtensionMsg::ReceiveWSMessage(message) => self.handle_message(message),
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

    /// Renders the DevTools extension into the DOM.
    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"Yew DevTools"}</h1>
                <div>
                    {
                        if let Some(root_node) = self.root_node {
                            self.render_component_tree(root_node)
                        } else {
                            html! {
                                <p>{"No data received (yet)..."}</p>
                            }
                        }
                    }
                </div>
            </>
        }
    }
}

impl DevToolsExtension {
    #[cfg(feature = "logic_test")]
    /// This method is used internally for testing – it's just a utility function which makes it
    /// slightly more concise to initialize an instance of `DevToolsExtension`.
    fn new() -> Self {
        Self {
            component_tree: indextree::Arena::new(),
            root_node: None,
        }
    }

    /// Recursively renders the current component state.
    fn render_component_tree(&self, top_node_id: indextree::NodeId) -> Html {
        let top_node = self.component_tree.get(top_node_id).unwrap().get();
        let top_node_html: Html = top_node.into();
        let children = top_node_id.children(&self.component_tree);
        html! {
            <div class="node">
                <div class="parent">
                    {top_node_html}
                </div>
                <div class="children">
                    {
                        children.filter_map(|child| {
                            if !child.is_removed() {
                                Some(self.render_component_tree(child)).collect::<Html>()
                            }
                            else {
                                None
                            }
                        })
                    }
                </div>
            </div>
        }
    }

    /// Processes a message from the WebSocket.
    ///
    /// Returns `true` if the state has changed and `false` if the state hasn't changed. This
    /// is used as the output of the `view` function.
    ///
    /// This has been written this way partly to make it easier to test it.
    fn handle_message(&mut self, message: String) -> bool {
        let message = match DevToolsExtension::extract_message(&message) {
            Some(t) => t,
            None => return false,
        };
        match message.event {
            ComponentEvent::Mounted => self.set_dom_status(&message, true),
            ComponentEvent::Updated => {}
            ComponentEvent::Created => self.create_component(&message),
            ComponentEvent::Destroyed => self.delete_component(&message),
            ComponentEvent::Unmounted => self.set_dom_status(&message, false),
        }
        false
    }

    /// Deletes a component from the component tree.
    fn delete_component(&mut self, message: &ComponentMessage) {
        if message.data.as_ref().unwrap().selector.as_ref().unwrap()
            == &self
                .component_tree
                .get(self.root_node.unwrap())
                .unwrap()
                .get()
                .selector
        {
            self.root_node = None;
        };
        let found_node = self
            .component_tree
            .iter()
            .find(|node| {
                &node.get().selector == message.data.as_ref().unwrap().selector.as_ref().unwrap()
            })
            .unwrap();
        let found_node = self.component_tree.get_node_id(found_node).unwrap();
        found_node.remove(&mut self.component_tree);
    }

    /// Updates the status of the DOM status (e.g. when a component is mounted or unmounted).
    fn set_dom_status(&mut self, message: &ComponentMessage, status: bool) {
        let selector = message.data.as_ref().unwrap().selector.as_ref().unwrap();
        let node = self
            .component_tree
            .iter()
            .find(|node| &node.get().selector == selector)
            .unwrap();
        let node_id = self.component_tree.get_node_id(node).unwrap();
        self.component_tree
            .get_mut(node_id)
            .unwrap()
            .get_mut()
            .is_in_dom = status;
    }

    /// Adds a new component to the component tree.
    ///
    /// If the root node doesn't exist, then the component will be set as the root component. This
    /// makes the assumption that a child component cannot be mounted or rendered before a child
    /// component can.
    fn create_component(&mut self, message: &ComponentMessage) {
        if let Some(root_node) = self.root_node {
            self.create_with_existing_root_node(message, root_node);
        } else {
            self.root_node = Some(
                self.component_tree.new_node(ComponentRepr {
                    name: message.data.as_ref().unwrap().name.clone(),
                    selector: message
                        .data
                        .as_ref()
                        .unwrap()
                        .selector
                        .as_ref()
                        .unwrap()
                        .clone(),
                    is_in_dom: false,
                    collapsed: false,
                }),
            );
        }
    }

    /// Computes where in the component tree a new component should go and then inserts it as a
    /// child node of the parent whose 'selector' matches the closest. For example, if there are
    /// three nodes – one with selector `body`, another with selector `body/h1` and the final one
    /// with selector `body/h1/a/h1` and a new node to be added with a selector `body/h1/p` then
    /// the node will be added as the child of the component at `body/h1/`.
    fn create_with_existing_root_node(
        &mut self,
        message: &ComponentMessage,
        root_node: indextree::NodeId,
    ) {
        let selector = message.data.as_ref().unwrap().selector.as_ref().unwrap();
        let youngest_parent = self.component_tree.iter().fold(root_node, |acc, val| {
            if val
                .get()
                .selector
                .starts_with(&self.component_tree.get(acc).unwrap().get().selector)
                && message
                    .data
                    .as_ref()
                    .unwrap()
                    .selector
                    .as_ref()
                    .unwrap()
                    .as_str()
                    .starts_with(val.get().selector.as_str())
            {
                self.component_tree.get_node_id(val).unwrap()
            } else {
                acc
            }
        });

        let component_node = self.component_tree.new_node(ComponentRepr {
            name: message.data.as_ref().unwrap().name.clone(),
            selector: selector.to_string(),
            is_in_dom: false,
            collapsed: false,
        });
        youngest_parent.append(component_node, &mut self.component_tree);
    }

    /// Uses `serde_json` to parse a JSON message.
    /// Please don't assume that the browser extension will always use JSON. The data serialization
    /// format used might be changed at any point (e.g. to a binary message format such as
    /// MessagePack or a serialization-less format such as Cap'n Proto).
    fn extract_message(message: &String) -> Option<ComponentMessage> {
        match serde_json::from_str(&message) {
            Ok(t) => Some(t),
            Err(e) => {
                yew::web_sys::console::error_1(
                    &format!(
                        "Received an invalid message from the DevTools server. \
                            Error message: `{:?}`",
                        e
                    )
                    .into(),
                );
                None
            }
        }
    }
}
