#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use yew::html::*;
use yew::format::Json;
use yew::services::dialog::DialogService;
use yew::services::storage::{StorageService, Scope};

const KEY: &'static str = "yew.crm.database";

struct Context {
    storage: StorageService,
    dialog: DialogService,
}

#[derive(Serialize, Deserialize)]
struct Database {
    clients: Vec<Client>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Client {
    first_name: String,
    last_name: String,
}

impl Client {
    fn empty() -> Self {
        Client {
            first_name: "".into(),
            last_name: "".into(),
        }
    }
}

#[derive(Debug)]
enum Scene {
    Initialization,
    ClientsList,
    NewClientForm(Client),
    Settings,
}

struct Model {
    database: Database,
    scene: Scene,
}

#[derive(Debug)]
enum Msg {
    SwitchTo(Scene),
    AddNew,
    UpdateFirstName(String),
    UpdateLastName(String),
    Clear,
}

fn load_database(context: &mut Context) -> Database {
    let Json(database) = context.storage.restore(KEY);
    database.unwrap_or_else(|_| Database {
        clients: Vec::new(),
    })
}

fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    let mut new_scene = None;
    match model.scene {
        Scene::Initialization => {
            match msg {
                Msg::SwitchTo(Scene::ClientsList) => {
                    new_scene = Some(Scene::ClientsList);
                }
                unexpected => {
                    panic!("Unexpected message during initialization: {:?}", unexpected);
                }
            }
        }
        Scene::ClientsList => {
            match msg {
                Msg::SwitchTo(Scene::NewClientForm(client)) => {
                    new_scene = Some(Scene::NewClientForm(client));
                }
                Msg::SwitchTo(Scene::Settings) => {
                    new_scene = Some(Scene::Settings);
                }
                unexpected => {
                    panic!("Unexpected message when clients list shown: {:?}", unexpected);
                }
            }
        }
        Scene::NewClientForm(ref mut client) => {
            match msg {
                Msg::UpdateFirstName(val) => {
                    println!("Input: {}", val);
                    client.first_name = val;
                }
                Msg::UpdateLastName(val) => {
                    println!("Input: {}", val);
                    client.last_name = val;
                }
                Msg::AddNew => {
                    let mut new_client = Client::empty();
                    ::std::mem::swap(client, &mut new_client);
                    model.database.clients.push(new_client);
                    context.storage.store(KEY, Json(&model.database));
                }
                Msg::SwitchTo(Scene::ClientsList) => {
                    new_scene = Some(Scene::ClientsList);
                }
                unexpected => {
                    panic!("Unexpected message during new client editing: {:?}", unexpected);
                }
            }
        }
        Scene::Settings => {
            match msg {
                Msg::Clear => {
                    if context.dialog.confirm("Do you really want to clear the data?") {
                        model.database.clients.clear();
                        context.storage.remove(KEY);
                    }
                }
                Msg::SwitchTo(Scene::ClientsList) => {
                    new_scene = Some(Scene::ClientsList);
                }
                unexpected => {
                    panic!("Unexpected message for settings scene: {:?}", unexpected);
                }
            }
        }
    }
    if let Some(new_scene) = new_scene.take() {
        model.scene = new_scene;
    }
}

fn view(model: &Model) -> Html<Msg> {
    match model.scene {
        Scene::Initialization => html! {
            <div>{ "Loading..." }</div>
        },
        Scene::ClientsList => html! {
            <div class="crm",>
                <div class="clients",>
                    { for model.database.clients.iter().map(view_client) }
                </div>
                <button onclick=|_| Msg::SwitchTo(Scene::NewClientForm(Client::empty())),>{ "Add New" }</button>
                <button onclick=|_| Msg::SwitchTo(Scene::Settings),>{ "Settings" }</button>
            </div>
        },
        Scene::NewClientForm(ref client) => html! {
            <div class="crm",>
                <div class="names",>
                    { view_first_name_input(client) }
                    { view_last_name_input(client) }
                </div>
                <button disabled=client.first_name.is_empty() || client.last_name.is_empty(),
                        onclick=|_| Msg::AddNew,>{ "Add New" }</button>
                <button onclick=|_| Msg::SwitchTo(Scene::ClientsList),>{ "Go Back" }</button>
            </div>
        },
        Scene::Settings => html! {
            <div>
                <button onclick=|_| Msg::Clear,>{ "Clear Database" }</button>
                <button onclick=|_| Msg::SwitchTo(Scene::ClientsList),>{ "Go Back" }</button>
            </div>
        },
    }
}

fn view_client(client: &Client) -> Html<Msg> {
    html! {
        <div class="client",>
            <p>{ format!("First Name: {}", client.first_name) }</p>
            <p>{ format!("Last Name: {}", client.last_name) }</p>
        </div>
    }
}

fn view_first_name_input(client: &Client) -> Html<Msg> {
    html! {
        <input class=("new-client", "firstname"),
               placeholder="First name",
               value=&client.first_name,
               oninput=|e: InputData| Msg::UpdateFirstName(e.value),
               />
    }
}

fn view_last_name_input(client: &Client) -> Html<Msg> {
    html! {
        <input class=("new-client", "lastname"),
               placeholder="Last name",
               value=&client.last_name,
               oninput=|e: InputData| Msg::UpdateLastName(e.value),
               />
    }
}

fn main() {
    yew::initialize();
    let mut app = App::new();
    let mut context = Context {
        storage: StorageService::new(Scope::Local),
        dialog: DialogService,
    };
    let database = load_database(&mut context);
    let scene = Scene::Initialization;
    let model = Model { database, scene };
    app.sender().send(Msg::SwitchTo(Scene::ClientsList));
    app.mount(context, model, update, view);
    yew::run_loop();
}
