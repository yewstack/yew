#![allow(missing_docs)]

//! Components wrapped with context including properties, state, and link

use crate::html::{Html, Properties, Scope, ShouldRender};

/// Context
pub type Context<T> = Scope<T>;

// Link to component scope
// pub type Link<T> = Scope<T>;

/// Yew component
pub trait Component: Sized + 'static {
    type Message: 'static;
    type Properties: Properties;

    fn create(ctx: &Context<Self>) -> Self;
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> ShouldRender {
        false
    }
    fn changed(&mut self, _ctx: &Context<Self>, _new_props: &Self::Properties) -> ShouldRender {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html;
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
