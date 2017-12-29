#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use yew::html::*;
use yew::services::Format;
use yew::services::fetch::{FetchService, Method, Request};

struct Model {
    fetching: bool,
    data: Option<Status>,
}

enum Msg {
    FetchData,
    DataReady(Result<Status, ()>),
}

#[derive(Deserialize, Debug)]
struct Status {
    value: u32,
}


fn update(context: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::FetchData => {
            let request = Request {
                method: Method::Get,
                in_format: Format::Json,
                out_format: Format::Json,
                url: "./data.json".into(),
            };
            context.fetch::<_,(),Status>(request, None, Msg::DataReady);
        }
        Msg::DataReady(response) => {
            model.fetching = false;
            match response {
                Ok(data) => {
                    model.data = Some(data);
                }
                Err(_) => {
                }
            }
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <nav class="menu",>
                <button onclick=|_| Msg::FetchData,>{ "Fetch Data" }</button>
                { view_data(model) }
            </nav>
        </div>
    }
}

fn view_data(model: &Model) -> Html<Msg> {
    if let Some(ref data) = model.data {
        html! {
            <p>{ data.value }</p>
        }
    } else {
        html! {
            <p>{ "Data hasn't fetched yet." }</p>
        }
    }
}

fn main() {
    let model = Model {
        fetching: false,
        data: None,
    };
    program(model, update, view);
}
