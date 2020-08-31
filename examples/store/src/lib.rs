mod agents;
mod post;
mod text_input;

use web_sys::console;
use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::agents::posts::{PostId, PostStore, Request};
use crate::post::Post;
use crate::text_input::TextInput;

pub struct App {
    link: ComponentLink<Self>,
    post_ids: Vec<PostId>,
    post_store: Box<dyn Bridge<StoreWrapper<PostStore>>>,
}

pub enum Msg {
    CreatePost(String),
    PostStoreMsg(ReadOnly<PostStore>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::PostStoreMsg);
        let post_store = PostStore::bridge(callback);
        Self {
            link,
            post_ids: Vec::new(),
            post_store,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CreatePost(text) => {
                self.post_store.send(Request::CreatePost(text));
                false
            }
            Msg::PostStoreMsg(state) => {
                // We can see this is logged once before we click any button.
                // The state of the store is sent when we open a bridge.
                console::log_1(&"Received update".into());

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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let posts = self.post_ids.iter().map(|&id| html!(<Post key=id id=id />));
        html! {
            <div>
                <TextInput value="New post" onsubmit=self.link.callback(Msg::CreatePost) />

                <div>
                    { for posts }
                </div>
            </div>
        }
    }
}
