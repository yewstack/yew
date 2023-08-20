use std::cell::Cell;
use std::fmt;
use std::future::Future;
use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

/// This hook is used to await a future in a suspending context.
///
/// A [Suspension] is created from the passed future and the result of the future
/// is the output of the suspension.
pub struct UseFutureHandle<O> {
    inner: UseStateHandle<Option<O>>,
}

impl<O> Deref for UseFutureHandle<O> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<T: fmt::Debug> fmt::Debug for UseFutureHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseFutureHandle")
            .field("value", &format!("{:?}", self.inner))
            .finish()
    }
}

/// Use the result of an async computation, suspending while waiting.
///
/// Awaits the future returned from the first call to `init_f`, and returns
/// its result in a [`UseFutureHandle`]. Always suspends initially, even if
/// the future is immediately [ready].
///
/// [ready]: std::task::Poll::Ready
///
/// # Example
///
/// ```
/// # use yew::prelude::*;
/// # use yew::suspense::use_future;
/// use gloo::net::http::Request;
///
/// const URL: &str = "https://en.wikipedia.org/w/api.php?\
///                    action=query&origin=*&format=json&generator=search&\
///                    gsrnamespace=0&gsrlimit=5&gsrsearch='New_England_Patriots'";
///
/// #[function_component]
/// fn WikipediaSearch() -> HtmlResult {
///     let res = use_future(|| async { Request::get(URL).send().await?.text().await })?;
///     let result_html = match *res {
///         Ok(ref res) => html! { res },
///         Err(ref failure) => failure.to_string().into(),
///     };
///     Ok(html! {
///         <p>
///             {"Wikipedia search result: "}
///             {result_html}
///         </p>
///     })
/// }
/// ```
#[hook]
pub fn use_future<F, T, O>(init_f: F) -> SuspensionResult<UseFutureHandle<O>>
where
    F: FnOnce() -> T,
    T: Future<Output = O> + 'static,
    O: 'static,
{
    use_future_with((), move |_| init_f())
}

/// Use the result of an async computation with dependencies, suspending while waiting.
///
/// Awaits the future returned from `f` for the latest `deps`. Even if the future is immediately
/// [ready], the hook suspends at least once. If the dependencies
/// change while a future is still pending, the result is never used. This guarantees that your
/// component always sees up-to-date values while it is not suspended.
///
/// [ready]: std::task::Poll::Ready
#[hook]
pub fn use_future_with<F, D, T, O>(deps: D, f: F) -> SuspensionResult<UseFutureHandle<O>>
where
    F: FnOnce(Rc<D>) -> T,
    T: Future<Output = O> + 'static,
    O: 'static,
    D: PartialEq + 'static,
{
    let output = use_state(|| None);
    // We only commit a result if it comes from the latest spawned future. Otherwise, this
    // might trigger pointless updates or even override newer state.
    let latest_id = use_state(|| Cell::new(0u32));

    let suspension = {
        let output = output.clone();

        use_memo_base(
            move |deps| {
                let self_id = latest_id.get().wrapping_add(1);
                // As long as less than 2**32 futures are in flight wrapping_add is fine
                (*latest_id).set(self_id);
                let deps = Rc::new(deps);
                let task = f(deps.clone());
                let suspension = Suspension::from_future(async move {
                    let result = task.await;
                    if latest_id.get() == self_id {
                        output.set(Some(result));
                    }
                });
                (suspension, deps)
            },
            deps,
        )
    };

    if suspension.resumed() {
        Ok(UseFutureHandle { inner: output })
    } else {
        Err((*suspension).clone())
    }
}
