use std::collections::HashMap;
use yew_agent::AgentLink;
use yewtil::store::{Store, StoreWrapper};

pub type PostId = u32;

#[derive(Debug)]
pub enum Request {
    CreatePost(String),
    UpdatePost(PostId, String),
    RemovePost(PostId),
}

#[derive(Debug)]
pub enum Action {
    SetPost(Option<PostId>, String),
    RemovePost(PostId),
}

pub struct PostStore {
    pub posts: HashMap<PostId, String>,
    // Stores can have private state too
    id_counter: PostId,
}

impl Store for PostStore {
    type Action = Action;
    type Input = Request;

    fn new() -> Self {
        let mut posts = HashMap::new();

        // We insert one post to show the initial send of state
        // when a bridge is opened.
        posts.insert(0, "Magic first post".to_owned());

        PostStore {
            posts,
            id_counter: 1,
        }
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            Request::CreatePost(text) => {
                link.send_message(Action::SetPost(None, text));
            }
            Request::UpdatePost(id, text) => {
                link.send_message(Action::SetPost(Some(id), text));
            }
            Request::RemovePost(id) => {
                link.send_message(Action::RemovePost(id));
            }
        }
    }

    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::SetPost(id, text) => {
                let id = id.unwrap_or_else(|| self.next_id());
                self.posts.insert(id, text);
            }
            Action::RemovePost(id) => {
                self.posts.remove(&id);
            }
        }
    }
}

impl PostStore {
    fn next_id(&mut self) -> PostId {
        let tmp = self.id_counter;
        self.id_counter += 1;
        tmp
    }
}
