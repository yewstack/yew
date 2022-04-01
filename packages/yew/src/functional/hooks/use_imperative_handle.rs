use super::use_effect;
use crate::functional::hook;
use crate::html::HtmlRef;

/// A hook to register an imperative handle.
#[hook]
pub fn use_imperative_handle<T, F>(ref_: HtmlRef<T>, f: F)
where
    T: Clone + 'static,
    F: 'static + FnOnce() -> T,
{
    use_effect(move || {
        ref_.set(Some(f()));

        move || {
            ref_.set::<T>(None);
        }
    });
}
