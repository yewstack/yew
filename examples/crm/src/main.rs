use add_client::AddClientForm;
use serde::{Deserialize, Serialize};
use yew::format::Json;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_services::storage::Area;
use yew_services::{DialogService, StorageService};

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
    link: ComponentLink<Self>,
    storage: StorageService,
    clients: Vec<Client>,
    scene: Scene,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let Json(clients) = storage.restore(KEY);
        let clients = clients.ok().unwrap_or_else(Vec::new);
        Self {
            link,
            storage,
            clients,
            scene: Scene::ClientsList,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SwitchTo(scene) => {
                self.scene = scene;
                true
            }
            Msg::AddClient(client) => {
                self.clients.push(client);
                self.storage.store(KEY, Json(&self.clients));
                // we only need to re-render if we're currently displaying the clients
                matches!(self.scene, Scene::ClientsList)
            }
            Msg::ClearClients => {
                if DialogService::confirm("Do you really want to clear the data?") {
                    self.clients.clear();
                    self.storage.remove(KEY);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.scene {
            Scene::ClientsList => html! {
                <div class="crm">
                    <h1>{"List of clients"}</h1>
                    <div class="clients">
                        { for self.clients.iter().map(Client::render) }
                    </div>
                    <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::NewClientForm))>{ "Add New" }</button>
                    <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::Settings))>{ "Settings" }</button>
                </div>
            },
            Scene::NewClientForm => html! {
                <div class="crm">
                    <h1>{"Add a new client"}</h1>
                    <AddClientForm on_add=self.link.callback(Msg::AddClient) on_abort=self.link.callback(|_| Msg::SwitchTo(Scene::ClientsList)) />
                </div>
            },
            Scene::Settings => html! {
                <div>
                    <h1>{"Settings"}</h1>
                    <button onclick=self.link.callback(|_| Msg::ClearClients)>{ "Remove all clients" }</button>
                    <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::ClientsList))>{ "Go Back" }</button>
                </div>
            },
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
