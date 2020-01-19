//! This module contains the implementation of a service to
//! use local and session storage of a browser.

use crate::format::Text;
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use std::fmt;
use thiserror::Error;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        #[allow(unused_imports)]
        use stdweb::{_js_impl, js};
        use stdweb::unstable::TryFrom;
        use stdweb::web::{Storage};
    } else if #[cfg(feature = "web_sys")] {
        use crate::utils;
        use web_sys::Storage;
    }
}

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
        let storage = cfg_match! {
            feature = "std_web" => ({
                let storage_name = match area {
                    Area::Local => "localStorage",
                    Area::Session => "sessionStorage",
                };
                let storage = js! {
                    try {
                        return window[@{storage_name}];
                    } catch(error) {
                        return error;
                    }
                };
                Storage::try_from(js!( return @{storage.as_ref()}; ))
            }),
            feature = "web_sys" => ({
                let storage = {
                    match area {
                        Area::Local => utils::window().local_storage(),
                        Area::Session => utils::window().session_storage(),
                    }
                };
                storage.map(Option::unwrap)
            }),
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
            let result = cfg_match! {
                feature = "std_web" => self.storage.insert(key, &data),
                feature = "web_sys" => self.storage.set_item(key, &data),
            };
            result.expect("can't insert value to a storage");
        }
    }

    /// Restores value from the storage.
    pub fn restore<T>(&self, key: &str) -> T
    where
        T: From<Text>,
    {
        let data = cfg_match! {
            feature = "std_web" => self.storage.get(key),
            feature = "web_sys" => self.storage.get_item(key).unwrap(),
        };
        let data = data.ok_or_else(|| StorageError::CantRestore.into());
        T::from(data)
    }

    /// Removes value from the storage.
    pub fn remove(&mut self, key: &str) {
        cfg_match! {
            feature = "std_web" => self.storage.remove(key),
            feature = "web_sys" => self.storage.remove_item(key).unwrap(),
        };
    }
}
