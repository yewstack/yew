//! This module contains declaration of `Component` trait which used
//! to create own UI-components.

use html::{Html, LocalSender};

/// An interface of a UI-component. Uses `self` as a model.
pub trait Component<CTX>: Default {
    /// Message type which `update` loop get.
    type Msg;
    /// Initialization routine which could use a context.
    fn initialize(&mut self, _context: LocalSender<Self::Msg, CTX>) {
        // Do nothing by default
    }
    /// Called everytime when a messages of `Msg` type received. It also takes a
    /// reference to a context.
    fn update(&mut self, msg: Self::Msg, context: LocalSender<Self::Msg, CTX>);
    /// Called by rendering loop.
    fn view(&self) -> Html<Self::Msg, CTX>;
}

