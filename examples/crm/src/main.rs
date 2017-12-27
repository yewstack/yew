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
}

enum Msg {
    AddNew,
    Store,
    Restore,
    Clear,
}

fn update(context: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::AddNew => {
            // TODO Add form to fill client's info
            let client = Client {
                first_name: "Denis".into(),
                last_name: "Kolodin".into(),
            };
            model.clients.push(client);
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
            <button onclick=|_| Msg::AddNew,>{ "AddNew" }</button>
            <button onclick=|_| Msg::Store,>{ "Store" }</button>
            <button onclick=|_| Msg::Restore,>{ "Restore" }</button>
            <button onclick=|_| Msg::Clear,>{ "Clear" }</button>
        </div>
    }
}

fn main() {
    let model = Model {
        clients: Vec::new(),
    };
    program(model, update, view);
}

