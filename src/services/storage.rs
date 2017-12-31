//! This module contains the implementation of a service to
//! use local and session storage of a browser.

use stdweb::Value;
use html::Context;
use services::format::{Storable, Restorable};

/// A scope to keep the data in.
pub enum Scope {
    /// Use `localStorage` of a browser.
    Local,
    /// Use `sessionStorage` of a browser.
    Session,
}

/// An abstract storage service attached to a context.
pub trait StorageService {
    /// Stores value to the storage.
    fn store_value<T>(&mut self, scope: Scope, key: &str, value: T)
    where
        T: Into<Storable>;

    /// Restores value from the storage.
    fn restore_value<T>(&mut self, scope: Scope, key: &str) -> T
    where
        T : From<Restorable>;

    /// Removes value from the storage.
    fn remove_value(&mut self, scope: Scope, key: &str);
}

impl<MSG: 'static> StorageService for Context<MSG> {
    fn store_value<T>(&mut self, scope: Scope, key: &str, value: T)
    where
        T: Into<Storable>
    {
        if let Some(data) = value.into() {
            match scope {
                Scope::Local => { js! { localStorage.setItem(@{key}, @{data}); } },
                Scope::Session => { js! { sessionStorage.setItem(@{key}, @{data}); } },
            }
        }
    }

    // TODO Use erorr-chain
    fn restore_value<T>(&mut self, scope: Scope, key: &str) -> T
    where
        T : From<Restorable>
    {
        let value: Value = {
            match scope {
                Scope::Local => js! { return localStorage.getItem(@{key}); },
                Scope::Session => js! { return sessionStorage.getItem(@{key}); },
            }
        };
        let data = value.into_string().ok_or_else(|| "can't read string from storage".into());
        T::from(data)
    }

    fn remove_value(&mut self, scope: Scope, key: &str) {
        {
            match scope {
                Scope::Local => js! { localStorage.removeItem(@{key}); },
                Scope::Session => js! { sessionStorage.removeItem(@{key}); },
            }
        };
    }
}
