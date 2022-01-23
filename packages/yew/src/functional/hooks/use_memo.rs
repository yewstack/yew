use crate::functional::{hook, use_hook};
use std::rc::Rc;

/// This hook is used for obtaining a immutable reference to a value that is recalculated when it's
/// dependency changes.
#[hook]
pub fn use_memo<T, Dependents>(memo_fn: impl FnOnce(&Dependents) -> T, deps: Dependents) -> Rc<T>
where
    T: 'static,
    Dependents: 'static + PartialEq,
{
    let deps = Rc::new(deps);

    pub struct UseMemo<T, Dependents>
    where
        T: 'static,
        Dependents: 'static + PartialEq,
    {
        inner: Option<(Rc<Dependents>, Rc<T>)>,
    }

    use_hook::<UseMemo<T, Dependents>, _, _, _, _>(
        || UseMemo { inner: None },
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
        |_| {},
    )
}
