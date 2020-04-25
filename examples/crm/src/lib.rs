#![recursion_limit = "128"]

#[macro_use]
extern crate serde_derive;

mod markdown;

use yew::format::Json;
use yew::services::storage::Area;
use yew::services::{DialogService, StorageService};
use yew::{html, Component, ComponentLink, Html, InputData, Renderable, ShouldRender};

const KEY: &str = "yew.crm.database";

#[derive(Serialize, Deserialize)]
struct Database {
    clients: Vec<Client>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    first_name: String,
    last_name: String,
    description: String,
}

impl Client {
    fn empty() -> Self {
        Client {
            first_name: "".into(),
            last_name: "".into(),
            description: "".into(),
        }
    }
}

#[derive(Debug)]
pub enum Scene {
    ClientsList,
    NewClientForm(Client),
    Settings,
}

pub struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    dialog: DialogService,
    database: Database,
    scene: Scene,
}

#[derive(Debug)]
pub enum Msg {
    SwitchTo(Scene),
    AddNew,
    UpdateFirstName(String),
    UpdateLastName(String),
    UpdateDescription(String),
    Clear,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let Json(database) = storage.restore(KEY);
        let database = database.unwrap_or_else(|_| Database {
            clients: Vec::new(),
        });
        Model {
            link,
            storage,
            dialog: DialogService::new(),
            database,
            scene: Scene::ClientsList,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut new_scene = None;
        match self.scene {
            Scene::ClientsList => match msg {
                Msg::SwitchTo(Scene::NewClientForm(client)) => {
                    new_scene = Some(Scene::NewClientForm(client));
                }
                Msg::SwitchTo(Scene::Settings) => {
                    new_scene = Some(Scene::Settings);
                }
                unexpected => {
                    panic!(
                        "Unexpected message when clients list shown: {:?}",
                        unexpected
                    );
                }
            },
            Scene::NewClientForm(ref mut client) => match msg {
                Msg::UpdateFirstName(val) => {
                    println!("Input: {}", val);
                    client.first_name = val;
                }
                Msg::UpdateLastName(val) => {
                    println!("Input: {}", val);
                    client.last_name = val;
                }
                Msg::UpdateDescription(val) => {
                    println!("Input: {}", val);
                    client.description = val;
                }
                Msg::AddNew => {
                    let mut new_client = Client::empty();
                    ::std::mem::swap(client, &mut new_client);
                    self.database.clients.push(new_client);
                    self.storage.store(KEY, Json(&self.database));
                }
                Msg::SwitchTo(Scene::ClientsList) => {
                    new_scene = Some(Scene::ClientsList);
                }
                unexpected => {
                    panic!(
                        "Unexpected message during new client editing: {:?}",
                        unexpected
                    );
                }
            },
            Scene::Settings => match msg {
                Msg::Clear => {
                    let ok = { self.dialog.confirm("Do you really want to clear the data?") };
                    if ok {
                        self.database.clients.clear();
                        self.storage.remove(KEY);
                    }
                }
                Msg::SwitchTo(Scene::ClientsList) => {
                    new_scene = Some(Scene::ClientsList);
                }
                unexpected => {
                    panic!("Unexpected message for settings scene: {:?}", unexpected);
                }
            },
        }
        if let Some(new_scene) = new_scene.take() {
            self.scene = new_scene;
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.scene {
            Scene::ClientsList => html! {
                <div class="crm">
                    <div class="clients">
                        { for self.database.clients.iter().map(Renderable::render) }
                    </div>
                    <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::NewClientForm(Client::empty())))>{ "Add New" }</button>
                    <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::Settings))>{ "Settings" }</button>
                </div>
            },
            Scene::NewClientForm(ref client) => html! {
                <div class="crm">
                    <div class="names">
                        { client.view_first_name_input(&self.link) }
                        { client.view_last_name_input(&self.link) }
                        { client.view_description_textarea(&self.link) }
                    </div>
                    <button disabled=client.first_name.is_empty() || client.last_name.is_empty()
                            onclick=self.link.callback(|_| Msg::AddNew)>{ "Add New" }</button>
                    <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::ClientsList))>{ "Go Back" }</button>
                </div>
            },
            Scene::Settings => html! {
                <div>
                    <button onclick=self.link.callback(|_| Msg::Clear)>{ "Clear Database" }</button>
                    <button onclick=self.link.callback(|_| Msg::SwitchTo(Scene::ClientsList))>{ "Go Back" }</button>
                </div>
            },
        }
    }
}

impl Renderable for Client {
    fn render(&self) -> Html {
        html! {
            <div class="client">
                <p>{ format!("First Name: {}", self.first_name) }</p>
                <p>{ format!("Last Name: {}", self.last_name) }</p>
                <p>{ "Description:" }</p>
                { markdown::render_markdown(&self.description) }
            </div>
        }
    }
}

impl Client {
    fn view_first_name_input(&self, link: &ComponentLink<Model>) -> Html {
        html! {
            <input class="new-client firstname"
                   placeholder="First name"
                   value=&self.first_name
                   oninput=link.callback(|e: InputData| Msg::UpdateFirstName(e.value)) />
        }
    }

    fn view_last_name_input(&self, link: &ComponentLink<Model>) -> Html {
        html! {
            <input class="new-client lastname"
                   placeholder="Last name"
                   value=&self.last_name
                   oninput=link.callback(|e: InputData| Msg::UpdateLastName(e.value)) />
        }
    }
    fn view_description_textarea(&self, link: &ComponentLink<Model>) -> Html {
        html! {
            <textarea class=("new-client", "description")
               placeholder="Description"
               value=&self.description
               oninput=link.callback(|e: InputData| Msg::UpdateDescription(e.value)) />
        }
    }
}
