use crate::agents::posts::{PostId, PostRequest, PostStore};
use crate::text_input::TextInput;
use yew::prelude::*;
use yew_agent::utils::store::{Bridgeable, ReadOnly, StoreWrapper};
use yew_agent::Bridge;

pub enum Msg {
    UpdateText(String),
    Delete,
    PostStore(ReadOnly<PostStore>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: PostId,
}

pub struct Post {
    id: PostId,
    text: Option<String>,
    post_store: Box<dyn Bridge<StoreWrapper<PostStore>>>,
}

impl Component for Post {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::PostStore);
        Self {
            id: ctx.props().id,
            text: None,
            post_store: PostStore::bridge(callback),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateText(text) => {
                self.post_store.send(PostRequest::Update(self.id, text));
                false
            }
            Msg::Delete => {
                self.post_store.send(PostRequest::Remove(self.id));
                false
            }
            Msg::PostStore(state) => {
                let state = state.borrow();

                // Only update if the post changed.
                if let Some(text) = state.posts.get(&self.id) {
                    if self.text.as_ref().map(|it| *it != *text).unwrap_or(false) {
                        self.text = Some(text.clone());
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.id = ctx.props().id;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text = self.text.as_deref().unwrap_or("<pending>");

        html! {
            <div>
                <h2>{ format!("Post #{}", self.id) }</h2>
                <p>{text}</p>

                <TextInput value={text.to_owned()} onsubmit={ctx.link().callback(Msg::UpdateText)} />
                <button on:click={ctx.link().callback(|_| Msg::Delete)}>
                    { "Delete" }
                </button>
            </div>
        }
    }
}
