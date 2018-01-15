#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::format::Json;
use yew::services::dialog::DialogService;
use yew::services::storage::{StorageService, Area};

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

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(context: &mut Env<Context, Self>) -> Self {
        let Json(database) = context.storage.restore(KEY);
        let database = database.unwrap_or_else(|_| Database {
            clients: Vec::new(),
        });
        Model {
            database,
            scene: Scene::ClientsList,
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        let mut new_scene = None;
        match self.scene {
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
                        self.database.clients.push(new_client);
                        context.storage.store(KEY, Json(&self.database));
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
                            self.database.clients.clear();
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
            self.scene = new_scene;
        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        match self.scene {
            Scene::ClientsList => html! {
                <div class="crm",>
                    <div class="clients",>
                        { for self.database.clients.iter().map(Renderable::view) }
                    </div>
                    <button onclick=|_| Msg::SwitchTo(Scene::NewClientForm(Client::empty())),>{ "Add New" }</button>
                    <button onclick=|_| Msg::SwitchTo(Scene::Settings),>{ "Settings" }</button>
                </div>
            },
            Scene::NewClientForm(ref client) => html! {
                <div class="crm",>
                    <div class="names",>
                        { client.view_first_name_input() }
                        { client.view_last_name_input() }
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
}

impl Renderable<Context, Model> for Client {
    fn view(&self) -> Html<Context, Model> {
        html! {
            <div class="client",>
                <p>{ format!("First Name: {}", self.first_name) }</p>
                <p>{ format!("Last Name: {}", self.last_name) }</p>
            </div>
        }
    }
}

impl Client {
    fn view_first_name_input(&self) -> Html<Context, Model> {
        html! {
            <input class=("new-client", "firstname"),
                   placeholder="First name",
                   value=&self.first_name,
                   oninput=|e: InputData| Msg::UpdateFirstName(e.value),
                   />
        }
    }

    fn view_last_name_input(&self) -> Html<Context, Model> {
        html! {
            <input class=("new-client", "lastname"),
                   placeholder="Last name",
                   value=&self.last_name,
                   oninput=|e: InputData| Msg::UpdateLastName(e.value),
                   />
        }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        storage: StorageService::new(Area::Local),
        dialog: DialogService,
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
