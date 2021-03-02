//! This module contains Yew's implementation of a service to
//! use local and session storage of a browser.

use std::fmt;
use thiserror::Error;
use web_sys::Storage;
use yew::format::Text;
use yew::utils;

/// Represents errors of a storage.
#[derive(Debug, Error)]
enum StorageError {
    #[error("restore error")]
    CantRestore,
}

/// An area to keep the data in.
#[derive(Debug)]
pub enum Area {
    /// Use `localStorage` of a browser.
    Local,
    /// Use `sessionStorage` of a browser.
    Session,
}

/// A storage service attached to a context.
pub struct StorageService {
    storage: Storage,
}

impl fmt::Debug for StorageService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StorageService")
    }
}

impl StorageService {
    /// Creates a new storage service instance with specified storage area.
    pub fn new(area: Area) -> Result<Self, &'static str> {
        let storage = {
            let storage = {
                match area {
                    Area::Local => utils::window().local_storage(),
                    Area::Session => utils::window().session_storage(),
                }
            };
            storage.map(Option::unwrap)
        };

        storage
            .map(|storage| StorageService { storage })
            .map_err(|_| "couldn't aquire storage")
    }

    /// Stores value to the storage.
    pub fn store<T>(&mut self, key: &str, value: T)
    where
        T: Into<Text>,
    {
        if let Ok(data) = value.into() {
            let result = self.storage.set_item(key, &data);
            result.expect("can't insert value to a storage");
        }
    }

    /// Restores value from the storage.
    pub fn restore<T>(&self, key: &str) -> T
    where
        T: From<Text>,
    {
        let data = self.storage.get_item(key).unwrap();
        let data = data.ok_or_else(|| StorageError::CantRestore.into());
        T::from(data)
    }

    /// Removes value from the storage.
    pub fn remove(&mut self, key: &str) {
        self.storage.remove_item(key).unwrap();
    }
}
