//! This module is a container of servies to interact with the external resources.
//!
//! It carries a similar role as subscriptions in Elm, but can be used directly
//! from the `update` method.

pub mod console;
pub mod dialog;
pub mod fetch;
pub mod interval;
pub mod keyboard;
pub mod reader;
pub mod render;
pub mod resize;
pub mod storage;
pub mod timeout;
pub mod websocket;

#[doc(inline)]
pub use self::console::ConsoleService;
#[doc(inline)]
pub use self::dialog::DialogService;
pub use self::fetch::FetchService;
#[doc(inline)]
pub use self::interval::IntervalService;
#[doc(inline)]
pub use self::reader::ReaderService;
#[doc(inline)]
pub use self::render::RenderService;
#[doc(inline)]
pub use self::resize::ResizeService;
#[doc(inline)]
pub use self::storage::StorageService;
#[doc(inline)]
pub use self::timeout::TimeoutService;
#[doc(inline)]
pub use self::websocket::WebSocketService;

use std::time::Duration;

/// An universal task of a service.
/// It have to be canceled when dropped.
pub trait Task: Drop {
    /// Returns `true` if task is active.
    fn is_active(&self) -> bool;
}

#[doc(hidden)]
fn to_ms(duration: Duration) -> u32 {
    let ms = duration.subsec_millis();
    ms + duration.as_secs() as u32 * 1000
}
