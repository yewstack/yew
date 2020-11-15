mod agents;
mod post;
mod text_input;

use agents::posts::{PostId, PostStore, Request};
use post::Post;
use text_input::TextInput;
use yew::{prelude::*, services::ConsoleService};
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

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
        let callback = ctx.callback(Msg::PostStoreMsg);
        Self {
            post_ids: Vec::new(),
            post_store: PostStore::bridge(callback),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CreatePost(text) => {
                self.post_store.send(Request::CreatePost(text));
                false
            }
            Msg::PostStoreMsg(state) => {
                // We can see this is logged once before we click any button.
                // The state of the store is sent when we open a bridge.
                ConsoleService::log("Received update");

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
                <TextInput value="New post" onsubmit=ctx.callback(Msg::CreatePost) />

                <div>
                    { for self.post_ids.iter().map(|&id| html!{ <Post key=id id=id /> }) }
                </div>
            </>
        }
    }
}
fn main() {
    yew::start_app::<Model>();
}
