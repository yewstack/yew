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
        deps: Option<Rc<Dependents>>,
        val: Option<Rc<T>>,
    }

    use_hook(
        || {
            let state: UseMemo<T, Dependents> = UseMemo {
                val: None,
                deps: None,
            };

            state
        },
        move |state, _updater| match state.val.clone() {
            Some(m) => {
                if Some(&deps) != state.deps.as_ref() {
                    let val = Rc::new(memo_fn(&deps));

                    state.val = Some(val.clone());
                    state.deps = Some(deps);

                    val
                } else {
                    m
                }
            }
            None => {
                let val = Rc::new(memo_fn(&deps));

                state.val = Some(val.clone());
                state.deps = Some(deps);

                val
            }
        },
        |_| {},
    )
}
