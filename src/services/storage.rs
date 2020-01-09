//! This module contains the implementation of a service to
//! use local and session storage of a browser.

use crate::format::Text;
use std::fmt;
use stdweb::web::{window, Storage};
use thiserror::Error;

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
    pub fn new(area: Area) -> Self {
        let storage = {
            match area {
                Area::Local => window().local_storage(),
                Area::Session => window().session_storage(),
            }
        };
        StorageService { storage }
    }

    /// Stores value to the storage.
    pub fn store<T>(&mut self, key: &str, value: T)
    where
        T: Into<Text>,
    {
        if let Ok(data) = value.into() {
            self.storage
                .insert(key, &data)
                .expect("can't insert value to a storage");
        }
    }

    /// Restores value from the storage.
    pub fn restore<T>(&self, key: &str) -> T
    where
        T: From<Text>,
    {
        let data = self
            .storage
            .get(key)
            .ok_or_else(|| StorageError::CantRestore.into());
        T::from(data)
    }

    /// Removes value from the storage.
    pub fn remove(&mut self, key: &str) {
        self.storage.remove(key);
    }
}
