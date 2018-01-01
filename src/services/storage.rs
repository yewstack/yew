//! This module contains the implementation of a service to
//! use local and session storage of a browser.

use stdweb::Value;
use format::{Storable, Restorable};

/// A scope to keep the data in.
pub enum Scope {
    /// Use `localStorage` of a browser.
    Local,
    /// Use `sessionStorage` of a browser.
    Session,
}

/// A storage service attached to a context.
pub struct StorageService {
    scope: Scope,
}

impl StorageService {

    /// Creates a new storage service instance with specified storate scope.
    pub fn new(scope: Scope) -> Self {
        StorageService { scope }
    }

    /// Stores value to the storage.
    pub fn store<T>(&mut self, key: &str, value: T)
    where
        T: Into<Storable>
    {
        if let Some(data) = value.into() {
            match self.scope {
                Scope::Local => { js! { localStorage.setItem(@{key}, @{data}); } },
                Scope::Session => { js! { sessionStorage.setItem(@{key}, @{data}); } },
            }
        }
    }

    /// Restores value from the storage.
    pub fn restore<T>(&mut self, key: &str) -> T
    where
        T : From<Restorable>
    {
        let value: Value = {
            match self.scope {
                Scope::Local => js! { return localStorage.getItem(@{key}); },
                Scope::Session => js! { return sessionStorage.getItem(@{key}); },
            }
        };
        let data = value.into_string().ok_or_else(|| "can't read string from storage".into());
        T::from(data)
    }

    /// Removes value from the storage.
    pub fn remove(&mut self, key: &str) {
        {
            match self.scope {
                Scope::Local => js! { localStorage.removeItem(@{key}); },
                Scope::Session => js! { sessionStorage.removeItem(@{key}); },
            }
        };
    }
}
