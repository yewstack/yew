use crate::hook;

use std::rc::Rc;

use crate::suspense::SuspensionResult;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[hook]
pub fn use_prepared_state<T, D, F>(f: F, deps: D) -> Option<Rc<T>>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
    F: FnOnce(&D) -> T,
{
    todo!()
}

#[hook]
pub fn use_prepared_state_with_suspension<T, D, F>(f: F, deps: D) -> SuspensionResult<Option<Rc<T>>>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
    F: FnOnce(&D) -> T,
{
    todo!()
}
