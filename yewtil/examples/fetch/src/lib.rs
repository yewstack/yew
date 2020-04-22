use crate::Msg::SetMarkdownFetchState;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewtil::fetch::{Fetch, FetchAction, FetchRequest, FetchState, Json, MethodBody};
use yewtil::future::LinkFuture;

#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<Model>();
}

struct Model {
    markdown: Fetch<Request, Vec<Employee>>,
    link: ComponentLink<Self>,
}

#[derive(Default, Debug, Clone)]
pub struct Request;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Employee {
    id: String,
    employee_name: String,
    employee_salary: String,
    employee_age: String,
    profile_image: String,
}

impl FetchRequest for Request {
    type RequestBody = ();
    type ResponseBody = Vec<Employee>;
    type Format = Json;

    fn url(&self) -> String {
        // Given that this is an external resource, this may fail sometime in the future.
        // Please report any regressions related to this.
        "http://dummy.restapiexample.com/api/v1/employees".to_string()
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        vec![]
    }

    fn use_cors(&self) -> bool {
        true
    }
}

enum Msg {
    SetMarkdownFetchState(FetchAction<Vec<Employee>>),
    GetMarkdown,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            markdown: Default::default(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetMarkdownFetchState(fetch_state) => {
                self.markdown.apply(fetch_state);
                true
            }
            Msg::GetMarkdown => {
                self.link
                    .send_future(self.markdown.fetch(Msg::SetMarkdownFetchState));
                self.link
                    .send_message(SetMarkdownFetchState(FetchAction::Fetching));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.markdown.as_ref().state() {
            FetchState::NotFetching(_) => {
                html! {<button onclick=self.link.callback(|_| Msg::GetMarkdown)>{"Get employees"}</button>}
            }
            FetchState::Fetching(_) => html! {"Fetching"},
            FetchState::Fetched(data) => data.iter().map(render_employee).collect(),
            FetchState::Failed(_, err) => html! {&err},
        }
    }
}

fn render_employee(e: &Employee) -> Html {
    html! {
        <div>
            <div>
                {"Name: "}
                {&e.employee_name}
            </div>
            <div>
                {"Salary: "}
                {&e.employee_salary}
            </div>

            <div>
                {"Age: "}
                {&e.employee_age}
            </div>
            <br/>
        </div>
    }
}
