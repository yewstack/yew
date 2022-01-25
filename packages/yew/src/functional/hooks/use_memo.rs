use crate::functional::{hook, use_hook};
use std::rc::Rc;

/// Get a immutable reference to a memoized value
///
/// Memoization means it will only get recalculated when provided dependencies update/change
#[hook]
pub fn use_memo<T, D>(memo_fn: impl FnOnce(&D) -> T, deps: D) -> Rc<T>
where
    T: 'static,
    D: 'static + PartialEq,
{
    let deps = Rc::new(deps);

    pub struct UseMemo<T, D>
    where
        T: 'static,
        D: 'static + PartialEq,
    {
        inner: Option<(Rc<D>, Rc<T>)>,
    }

    use_hook(
        || -> UseMemo<T, D> { UseMemo { inner: None } },
        move |state, _updater| {
            state
                .inner
                .as_ref()
                .and_then(|(m, n)| (m.as_ref() == &*deps).then(|| n.clone()))
                .unwrap_or_else(|| {
                    let val = Rc::new(memo_fn(&deps));

                    state.inner = Some((deps, val.clone()));

                    val
                })
        },
    )
}
