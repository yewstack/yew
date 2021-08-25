mod agents;
mod post;
mod text_input;

use agents::posts::{PostId, PostRequest, PostStore};
use post::Post;
use text_input::TextInput;
use weblog::console_log;
use yew::prelude::*;
use yew_agent::utils::store::{Bridgeable, ReadOnly, StoreWrapper};
use yew_agent::Bridge;

pub enum Msg {
    CreatePost(String),
    PostStoreMsg(ReadOnly<PostStore>),
}

pub struct Model {
    post_ids: Vec<PostId>,
    post_store: Box<dyn Bridge<StoreWrapper<PostStore>>>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::PostStoreMsg);
        Self {
            post_ids: Vec::new(),
            post_store: PostStore::bridge(callback),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CreatePost(text) => {
                self.post_store.send(PostRequest::Create(text));
                false
            }
            Msg::PostStoreMsg(state) => {
                // We can see this is logged once before we click any button.
                // The state of the store is sent when we open a bridge.
                console_log!("Received update");

                let state = state.borrow();
                if state.posts.len() != self.post_ids.len() {
                    self.post_ids = state.posts.keys().copied().collect();
                    self.post_ids.sort_unstable();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <TextInput value="New post" onsubmit={ctx.link().callback(Msg::CreatePost)} />

                <div>
                    { for self.post_ids.iter().map(|&id| html!{ <Post key={id} {id} /> }) }
                </div>
            </>
        }
    }
}
fn main() {
    yew::start_app::<Model>();
}
