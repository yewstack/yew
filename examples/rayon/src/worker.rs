use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use yew_agent::{Agent, AgentLink, HandlerId, Public};

use crate::app::STATE;

pub(crate) struct Worker {
    link: AgentLink<Self>,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum Request {
    Compute,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum Response {
    Acknowledged,
    Finished,
}

impl Agent for Worker {
    type Reach = Public<Self>;
    type Message = ();
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {
        // no messaging
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            Request::Compute => {
                log::debug!("Worker thread: got request");
                self.link.respond(id, Response::Acknowledged);

                log::debug!("Worker thread: processing");
                // artificial delay
                let link = self.link.clone();
                gloo_timers::callback::Timeout::new(1_000, move || {
                    let mut s = STATE.lock().unwrap();
                    s.computed_sum = Some((1..=s.n).into_par_iter().sum());

                    log::debug!("Worker thread: finished");
                    link.respond(id, Response::Finished);
                })
                .forget();
            }
        }
    }

    fn name_of_resource() -> &'static str {
        "yew_rayon_demo.js"
    }

    fn is_module() -> bool {
        true
    }
}
