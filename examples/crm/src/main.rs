#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use yew::html::*;
use yew::services::alert::AlertService;
use yew::services::storage::{StorageService, Scope};

const KEY: &'static str = "yew.crm";

#[derive(Serialize, Deserialize)]
struct Client {
    first_name: String,
    last_name: String,
}

struct Model {
    clients: Vec<Client>,
    first_name_value: String,
    last_name_value: String,
}

enum Msg {
    AddNew,
    UpdateFirstName(String),
    UpdateLastName(String),
    Store,
    Restore,
    Clear,
    Nope,
}

fn update(context: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::AddNew => {
            let client = Client {
                first_name: model.first_name_value.clone(),
                last_name: model.last_name_value.clone(),
            };
            model.clients.push(client);
            model.first_name_value = "".to_string();
            model.last_name_value = "".to_string();
        }
        Msg::UpdateFirstName(val) => {
            println!("Input: {}", val);
            model.first_name_value = val;
        }
        Msg::UpdateLastName(val) => {
            println!("Input: {}", val);
            model.last_name_value = val;
        }
        Msg::Store => {
            context.store_value(Scope::Local, KEY, &model.clients);
        }
        Msg::Restore => {
            if let Ok(clients) = context.restore_value::<Vec<Client>>(Scope::Local, KEY) {
                model.clients = clients;
            } else {
                context.alert("Oh no! Storage was corrupted!");
            }
        }
        Msg::Clear => {
            model.clients.clear();
            context.remove_value(Scope::Local, KEY);
        }
        Msg::Nope => {}
    }
}

fn view(model: &Model) -> Html<Msg> {
    let view_client = |client: &Client| {
        html! {
            <div class="client",>
                <p>{ format!("First Name: {}", client.first_name) }</p>
                <p>{ format!("Last Name: {}", client.last_name) }</p>
            </div>
        }
    };
    html! {
        <div class="crm",>
            <div class="clients",>
                { for model.clients.iter().map(view_client) }
            </div>
            <div class="names",>
                { view_first_name_input(&model) }
                { view_last_name_input(&model) }
            </div>
            <button onclick=|_| Msg::AddNew,>{ "AddNew" }</button>
            <button onclick=|_| Msg::Store,>{ "Store" }</button>
            <button onclick=|_| Msg::Restore,>{ "Restore" }</button>
            <button onclick=|_| Msg::Clear,>{ "Clear" }</button>
        </div>
    }
}

fn view_first_name_input(model: &Model) -> Html<Msg> {
    html! {
        <input class="new-client-firstname",
               placeholder="First name",
               value=&model.first_name_value,
               oninput=|e: InputData| Msg::UpdateFirstName(e.value),
               onkeypress=|e: KeyData| {
                   if e.key == "Enter" { Msg::AddNew } else { Msg::Nope }
               }, />
    }
}

fn view_last_name_input(model: &Model) -> Html<Msg> {
    html! {
        <input class="new-client-lastname",
               placeholder="Last name",
               value=&model.last_name_value,
               oninput=|e: InputData| Msg::UpdateLastName(e.value),
               onkeypress=|e: KeyData| {
                   if e.key == "Enter" { Msg::AddNew } else { Msg::Nope }
               }, />
    }
}

fn main() {
    let model = Model {
        clients: Vec::new(),
        first_name_value: "".into(),
        last_name_value: "".into(),
    };
    program(model, update, view);
}
