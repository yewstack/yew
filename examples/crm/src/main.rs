use add_client::AddClientForm;
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::{html, Component, Context, Html};

mod add_client;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Client {
    pub first_name: String,
    pub last_name: String,
    pub description: String,
}

impl Client {
    pub fn render(&self) -> Html {
        html! {
            <div class="client" style="margin-bottom: 50px">
                <p>{ format!("First Name: {}", self.first_name) }</p>
                <p>{ format!("Last Name: {}", self.last_name) }</p>
                <p>{ "Description:" }</p>
                { &self.description }
            </div>
        }
    }
}

/// storage key for the clients
const KEY: &str = "yew.crm.clients";

#[derive(Debug)]
pub enum Scene {
    ClientsList,
    NewClientForm,
    Settings,
}

#[derive(Debug)]
pub enum Msg {
    SwitchTo(Scene),
    AddClient(Client),
    ClearClients,
}

pub struct Model {
    clients: Vec<Client>,
    scene: Scene,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let clients = LocalStorage::get(KEY).unwrap_or_else(|_| Vec::new());
        Self {
            clients,
            scene: Scene::ClientsList,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SwitchTo(scene) => {
                self.scene = scene;
                true
            }
            Msg::AddClient(client) => {
                self.clients.push(client);
                LocalStorage::set(KEY, &self.clients).expect("failed to set");
                // we only need to re-render if we're currently displaying the clients
                matches!(self.scene, Scene::ClientsList)
            }
            Msg::ClearClients => {
                if gloo::dialogs::confirm("Do you really want to clear the data?") {
                    self.clients.clear();
                    LocalStorage::delete(KEY);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.scene {
            Scene::ClientsList => html! {
                <div class="crm">
                    <h1>{"List of clients"}</h1>
                    <div class="clients">
                        { for self.clients.iter().map(Client::render) }
                    </div>
                    <button onclick={ctx.link().callback(|_| Msg::SwitchTo(Scene::NewClientForm))}>{ "Add New" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::SwitchTo(Scene::Settings))}>{ "Settings" }</button>
                </div>
            },
            Scene::NewClientForm => html! {
                <div class="crm">
                    <h1>{"Add a new client"}</h1>
                    <AddClientForm on_add={ctx.link().callback(Msg::AddClient)} on_abort={ctx.link().callback(|_| Msg::SwitchTo(Scene::ClientsList))} />
                </div>
            },
            Scene::Settings => html! {
                <div>
                    <h1>{"Settings"}</h1>
                    <button onclick={ctx.link().callback(|_| Msg::ClearClients)}>{ "Remove all clients" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::SwitchTo(Scene::ClientsList))}>{ "Go Back" }</button>
                </div>
            },
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
