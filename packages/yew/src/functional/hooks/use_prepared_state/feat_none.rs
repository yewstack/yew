use crate::hook;

use std::rc::Rc;

use crate::suspense::SuspensionResult;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// The noop variant. This is used when its client side rendering and hydration is not enabled.
#[doc(hidden)]
#[hook]
pub fn use_prepared_state<T, D>(_deps: D) -> Option<Rc<T>>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
{
    None
}

/// The with suspension variant for use_prepared_state_with_noop.
#[doc(hidden)]
#[hook]
pub fn use_prepared_state_with_suspension<T, D>(_deps: D) -> SuspensionResult<Option<Rc<T>>>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
{
    Ok(None)
}
