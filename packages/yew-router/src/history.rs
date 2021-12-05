//! A module that provides universal session history and location information.

pub use gloo::history::{
    AnyHistory, BrowserHistory, HashHistory, History, HistoryError, HistoryResult, Location,
    MemoryHistory,
};
