use futures_signals::signal::Mutable;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use yew::agent::*;
use yew::services::fetch::FetchTask;

use super::services::{Fetcher, ResponseWrapper};

#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    pub ip: Mutable<Option<String>>,
}

impl Default for State {
    fn default() -> State {
        State {
            ip: Mutable::new(None),
        }
    }
}

pub type ArcState = Arc<State>;

pub type TaskHandlerId = Rc<String>;

pub type TaskBundle = (TaskHandlerId, ResponseWrapper);

#[derive(Deserialize, Serialize)]
pub enum ActionType {
    GetIp,
    ClearIp,
}

#[derive(Deserialize, Serialize)]
pub enum StoreInput {
    Action(ActionType),
    // Mutations aren't used in this example, but I've left this here to show you things you can send through the store
    Mutation,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StoreOutput {
    StateInstance(Arc<State>),
}

pub struct Store {
    link: AgentLink<Store>,
    state: Arc<State>,
    fetcher: Fetcher,
    fetch_tasks: HashMap<Rc<String>, FetchTask>,
}

pub enum Msg {
    FetchResponse(TaskBundle),
}

impl Store {
    fn init_state() -> Arc<State> {
        Arc::new(State {
            ..Default::default()
        })
    }
    fn register_task(&mut self, task_name: Rc<String>, task: FetchTask) {
        self.fetch_tasks.insert(task_name, task);
    }
    fn unregister_task(&mut self, task: Rc<String>) {
        if self.fetch_tasks.contains_key(task.as_ref()) {
            self.fetch_tasks.remove_entry(task.as_ref());
        }
    }
}

impl Agent for Store {
    type Reach = Context;
    type Message = Msg;
    type Input = StoreInput;
    type Output = StoreOutput;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            state: Self::init_state(),
            fetcher: Fetcher::new(),
            fetch_tasks: HashMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::FetchResponse(r) => match r {
                (id, ResponseWrapper::IpResponse(data)) => {
                    if data.is_ok() {
                        self.state.as_ref().ip.set(Some(data.unwrap().ip));
                    } // else: handle error

                    // Clean up the task that just returned
                    self.unregister_task(id);
                }
            },
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            StoreInput::Action(a) => match a {
                ActionType::GetIp => {
                    let callback = self
                        .link
                        .callback(|resp: TaskBundle| Msg::FetchResponse(resp));

                    // This example doesn't account for duplicate task names.
                    // In your projects you'll want to use a unique identifier so you don't clear the wrong request
                    let task_name = Rc::new(String::from("ip_fetcher"));

                    let task_clone = task_name.clone();

                    let task = self.fetcher.get_ip(task_name, callback);

                    self.register_task(task_clone, task);
                }
                ActionType::ClearIp => {
                    self.state.ip.set(None);
                }
            },
            _ => {}
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.link
            .respond(id, StoreOutput::StateInstance(self.state.clone()));
    }
}
