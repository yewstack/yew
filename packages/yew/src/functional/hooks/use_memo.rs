use std::borrow::Borrow;
use std::rc::Rc;

use super::use_mut_ref;
use crate::functional::hook;

/// Get a immutable reference to a memoized value.
///
/// This version allows for a key cache key derivation that only borrows
/// like the original argument. For example, using ` K = Rc<D>`, we only
/// create a shared reference to dependencies *after* they change.
#[hook]
pub(crate) fn use_memo_base<T, F, D, K>(f: F, deps: D) -> Rc<T>
where
    T: 'static,
    F: FnOnce(D) -> (T, K),
    K: 'static + Borrow<D>,
    D: PartialEq,
{
    struct MemoState<T, K> {
        memo_key: K,
        result: Rc<T>,
    }
    let state = use_mut_ref(|| -> Option<MemoState<T, K>> { None });

    let mut state = state.borrow_mut();
    match &*state {
        Some(existing) if existing.memo_key.borrow() != &deps => {
            // Drop old state if it's outdated
            *state = None;
        }
        _ => {}
    };
    let state = state.get_or_insert_with(|| {
        let (result, memo_key) = f(deps);
        let result = Rc::new(result);
        MemoState { result, memo_key }
    });
    state.result.clone()
}

/// Get a immutable reference to a memoized value.
///
/// Memoization means it will only get recalculated when provided dependencies update/change.
///
/// It can be useful for keeping things in scope for the lifetime of the component,
/// so long as you don't store a clone of the resulting `Rc` anywhere that outlives the component.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
///
/// #[derive(PartialEq, Properties)]
/// pub struct Props {
///     pub step: usize,
/// }
///
/// #[function_component(UseMemo)]
/// fn memo(props: &Props) -> Html {
///     // Will only get recalculated if `props.step` value changes
///     let message = use_memo(
///         |step| format!("{}. Do Some Expensive Calculation", step),
///         props.step,
///     );
///
///     html! {
///         <div>
///             <span>{ (*message).clone() }</span>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_memo<T, F, D>(f: F, deps: D) -> Rc<T>
where
    T: 'static,
    F: FnOnce(&D) -> T,
    D: 'static + PartialEq,
{
    use_memo_base(|d| (f(&d), d), deps)
}
