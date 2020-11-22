use crate::agents::posts::{PostId, PostStore, Request};
use crate::text_input::TextInput;
use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};
use yewtil::NeqAssign;

pub enum Msg {
    UpdateText(String),
    Delete,
    PostStoreMsg(ReadOnly<PostStore>),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: PostId,
}

pub struct Post {
    text: Option<String>,
    post_store: Box<dyn Bridge<StoreWrapper<PostStore>>>,
}

impl Component for Post {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.callback(Msg::PostStoreMsg);
        Self {
            text: None,
            post_store: PostStore::bridge(callback),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateText(text) => {
                self.post_store
                    .send(Request::UpdatePost(ctx.props.id, text));
                false
            }
            Msg::Delete => {
                self.post_store.send(Request::RemovePost(ctx.props.id));
                false
            }
            Msg::PostStoreMsg(state) => {
                let state = state.borrow();

                // Only update if the post changed.
                if let Some(text) = state.posts.get(&ctx.props.id) {
                    self.text.neq_assign(Some(text.clone()))
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text = self.text.as_deref().unwrap_or("<pending>");

        html! {
            <div>
                <h2>{ format!("Post #{}", ctx.props.id) }</h2>
                <p>{text}</p>

                <TextInput value=text onsubmit=ctx.callback(Msg::UpdateText) />
                <button onclick=ctx.callback(|_| Msg::Delete)>
                    { "Delete" }
                </button>
            </div>
        }
    }
}
