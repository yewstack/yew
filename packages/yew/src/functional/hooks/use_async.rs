use std::{fmt, future::Future};

use crate::{functional::hook, use_effect_with, use_state, UseStateHandle};

#[hook]
pub fn use_async<Arg, F, Fut, Res>(deps: Arg, f: F) -> UseAsyncHandle<Res>
where
    Arg: 'static + PartialEq,
    F: 'static + FnOnce(&Arg) -> Fut,
    Fut: 'static + Future<Output = Res>,
    Res: 'static,
{
    let result = use_state(|| None);
    use_effect_with(deps, {
        let result = result.clone();
        move |deps| {
            result.set(None);
            let future = f(deps);
            wasm_bindgen_futures::spawn_local(async move {
                let res = future.await;
                result.set(Some(res));
            });
        }
    });
    UseAsyncHandle { inner: result }
}

#[derive(Clone, PartialEq)]
pub struct UseAsyncHandle<T> {
    inner: UseStateHandle<Option<T>>,
}

#[derive(Debug)]
pub enum UseAsyncResult<'a, T> {
    Pending,
    Ready(&'a T),
}

impl<T: fmt::Debug> fmt::Debug for UseAsyncHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner_value: &Option<T> = &*self.inner;
        match inner_value {
            None => f.write_str("Pending"),
            Some(val) => f.debug_tuple("Ready").field(val).finish(),
        }
    }
}

impl<T> UseAsyncHandle<T> {
    pub fn status(&self) -> UseAsyncResult<'_, T> {
        match &*self.inner {
            None => UseAsyncResult::Pending,
            Some(res) => UseAsyncResult::Ready(res),
        }
    }
}
