//! Smart pointers for use within Yew.
//!
//! These all offer similar semantics to `std::rc::Rc`, but offer better ergonomics within Yew,
//! or functionality not available in `Rc`.
#[cfg(feature = "mrc_irc")]
mod irc;
#[cfg(feature = "lrc")]
mod lrc;
#[cfg(feature = "mrc_irc")]
mod mrc;
mod rc_box;
mod takeable;

#[cfg(feature = "mrc_irc")]
pub use irc::Irc;
#[deprecated]
#[cfg(feature = "lrc")]
pub use lrc::Lrc;
#[cfg(feature = "mrc_irc")]
pub use mrc::Mrc;

pub(crate) type IsZero = bool;
