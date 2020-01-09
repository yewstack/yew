//! This module contains the implementation of a service to
//! use local and session storage of a browser.

use crate::format::Text;
use failure::Fail;
use std::fmt;
#[cfg(feature = "std_web")]
use stdweb::web::{window, Storage};
#[cfg(feature = "web_sys")]
use ::{wasm_bindgen::JsValue, web_sys::Storage};

/// Represents errors of a storage.
#[derive(Debug, Fail)]
enum StorageError {
    #[fail(display = "restore error")]
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
            #[cfg(feature = "std_web")]
            let window = window();
            #[cfg(feature = "web_sys")]
            let window = web_sys::window().unwrap();
            match area {
                Area::Local => window.local_storage(),
                Area::Session => window.session_storage(),
            }
        };
        #[cfg(feature = "web_sys")]
        let storage = storage
            .and_then(|storage| storage.ok_or(JsValue::NULL))
            .expect("failed to aquire storage");
        StorageService { storage }
    }

    /// Stores value to the storage.
    pub fn store<T>(&mut self, key: &str, value: T)
    where
        T: Into<Text>,
    {
        if let Ok(data) = value.into() {
            #[cfg(feature = "std_web")]
            let result = self.storage.insert(key, &data);
            #[cfg(feature = "web_sys")]
            let result = self.storage.set_item(key, &data);
            result.expect("can't insert value to a storage");
        }
    }

    /// Restores value from the storage.
    pub fn restore<T>(&self, key: &str) -> T
    where
        T: From<Text>,
    {
        #[cfg(feature = "std_web")]
        let data = self.storage.get(key);
        #[cfg(feature = "web_sys")]
        let data = self.storage.get_item(key).unwrap();
        let data = data.ok_or_else(|| StorageError::CantRestore.into());
        T::from(data)
    }

    /// Removes value from the storage.
    pub fn remove(&mut self, key: &str) {
        #[cfg(feature = "std_web")]
        self.storage.remove(key);
        #[cfg(feature = "web_sys")]
        self.storage.remove_item(key).unwrap();
    }
}
