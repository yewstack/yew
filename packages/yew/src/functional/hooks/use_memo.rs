use std::cell::RefCell;
use std::rc::Rc;

use crate::functional::{hook, use_state};

/// Get a immutable reference to a memoized value
///
/// Memoization means it will only get recalculated when provided dependencies update/change
#[hook]
pub fn use_memo<T, D>(memo_fn: impl FnOnce(&D) -> T, deps: D) -> Rc<T>
where
    T: 'static,
    D: 'static + PartialEq,
{
    let val = use_state(|| -> RefCell<Option<Rc<T>>> { RefCell::new(None) });
    let last_deps = use_state(|| -> RefCell<Option<D>> { RefCell::new(None) });

    let mut val = val.borrow_mut();
    let mut last_deps = last_deps.borrow_mut();

    match (
        val.as_ref(),
        last_deps.as_ref().and_then(|m| (m != &deps).then(|| ())),
    ) {
        // Previous value exists and last_deps == deps
        (Some(m), None) => m.clone(),
        _ => {
            let new_val = Rc::new(memo_fn(&deps));
            *last_deps = Some(deps);

            *val = Some(new_val.clone());

            new_val
        }
    }
}
