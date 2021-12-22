use std::rc::Rc;

use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub inner: String,
}

impl Reducible for Message {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Message { inner: action }.into()
    }
}

pub type MessageContext = UseReducerHandle<Message>;
