use serde::{Serialize, Deserialize};
use serde_json;
use stdweb::Value;
use html::Context;

pub enum Scope {
    Local,
    Session,
}

pub trait StorageService {
    fn store_value<T>(&mut self, scope: Scope, key: &str, value: &T)
    where
        T: Serialize;
    fn restore_value<T>(&mut self, scope: Scope, key: &str) -> Result<T, ()>
    where
        T : for <'de> Deserialize<'de>;
    fn remove_value(&mut self, scope: Scope, key: &str);
}

impl<MSG: 'static> StorageService for Context<MSG> {
    fn store_value<T>(&mut self, scope: Scope, key: &str, value: &T)
    where
        T: Serialize
    {
        let data = serde_json::to_string(value).expect("can't serialize data to store");
        match scope {
            Scope::Local => { js! { localStorage.setItem(@{key}, @{data}); } },
            Scope::Session => { js! { sessionStorage.setItem(@{key}, @{data}); } },
        }
    }

    // TODO Use erorr-chain
    fn restore_value<T>(&mut self, scope: Scope, key: &str) -> Result<T, ()>
    where
        T : for <'de> Deserialize<'de>
    {
        let value: Value = {
            match scope {
                Scope::Local => js! { return localStorage.getItem(@{key}); },
                Scope::Session => js! { return sessionStorage.getItem(@{key}); },
            }
        };
        if let Some(data) = value.into_string() {
            serde_json::from_str::<T>(&data).map_err(drop)
        } else {
            Err(()) // No data sored with this key
        }
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
