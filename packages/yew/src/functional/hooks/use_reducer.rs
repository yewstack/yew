use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use crate::functional::use_hook;

type DispatchFn<T> = Rc<dyn Fn(<T as Reducible>::Action)>;

/// A trait that implements a reducer function of a type.
pub trait Reducible {
    /// The action type of the reducer.
    type Action;

    /// The reducer function.
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self>;
}

struct UseReducer<T>
where
    T: Reducible,
{
    current_state: Rc<T>,

    // To be replaced with OnceCell once it becomes available in std.
    dispatch: RefCell<Option<DispatchFn<T>>>,
}

/// State handle for [`use_reducer`] and [`use_reducer_eq`] hook
pub struct UseReducerHandle<T>
where
    T: Reducible,
{
    value: Rc<T>,
    dispatch: DispatchFn<T>,
}

impl<T> UseReducerHandle<T>
where
    T: Reducible,
{
    /// Dispatch the given action to the reducer.
    pub fn dispatch(&self, value: T::Action) {
        (self.dispatch)(value)
    }

    /// Returns the dispatcher of the current state.
    pub fn dispatcher(&self) -> UseReducerDispatcher<T> {
        UseReducerDispatcher {
            dispatch: self.dispatch.clone(),
        }
    }
}

impl<T> Deref for UseReducerHandle<T>
where
    T: Reducible,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

impl<T> Clone for UseReducerHandle<T>
where
    T: Reducible,
{
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
            dispatch: Rc::clone(&self.dispatch),
        }
    }
}

impl<T> fmt::Debug for UseReducerHandle<T>
where
    T: Reducible + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseReducerHandle")
            .field("value", &format!("{:?}", self.value))
            .finish()
    }
}

impl<T> PartialEq for UseReducerHandle<T>
where
    T: Reducible + PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.value == rhs.value
    }
}

/// Dispatcher handle for [`use_reducer`] and [`use_reducer_eq`] hook
pub struct UseReducerDispatcher<T>
where
    T: Reducible,
{
    dispatch: DispatchFn<T>,
}

impl<T> Clone for UseReducerDispatcher<T>
where
    T: Reducible,
{
    fn clone(&self) -> Self {
        Self {
            dispatch: Rc::clone(&self.dispatch),
        }
    }
}

impl<T> fmt::Debug for UseReducerDispatcher<T>
where
    T: Reducible + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseReducerDispatcher").finish()
    }
}

impl<T> PartialEq for UseReducerDispatcher<T>
where
    T: Reducible,
{
    fn eq(&self, rhs: &Self) -> bool {
        #[allow(clippy::vtable_address_comparisons)]
        Rc::ptr_eq(&self.dispatch, &rhs.dispatch)
    }
}

impl<T> UseReducerDispatcher<T>
where
    T: Reducible,
{
    /// Dispatch the given action to the reducer.
    pub fn dispatch(&self, value: T::Action) {
        (self.dispatch)(value)
    }
}

/// The base function of [`use_reducer`] and [`use_reducer_eq`]
fn use_reducer_base<T, F, R>(initial_fn: F, should_render_fn: R) -> UseReducerHandle<T>
where
    T: Reducible + 'static,
    F: FnOnce() -> T,
    R: (Fn(&T, &T) -> bool) + 'static,
{
    use_hook(
        move || UseReducer {
            current_state: Rc::new(initial_fn()),
            dispatch: RefCell::default(),
        },
        |s, updater| {
            let mut dispatch_ref = s.dispatch.borrow_mut();

            // Create dispatch once.
            let dispatch = match *dispatch_ref {
                Some(ref m) => (*m).to_owned(),
                None => {
                    let should_render_fn = Rc::new(should_render_fn);

                    let dispatch: Rc<dyn Fn(T::Action)> = Rc::new(move |action: T::Action| {
                        let should_render_fn = should_render_fn.clone();

                        updater.callback(move |state: &mut UseReducer<T>| {
                            let next_state = state.current_state.clone().reduce(action);
                            let should_render = should_render_fn(&next_state, &state.current_state);
                            state.current_state = next_state;

                            should_render
                        });
                    });

                    *dispatch_ref = Some(dispatch.clone());

                    dispatch
                }
            };

            UseReducerHandle {
                value: Rc::clone(&s.current_state),
                dispatch,
            }
        },
        |_| {},
    )
}

/// This hook is an alternative to [`use_state`](super::use_state()).
/// It is used to handle component's state and is used when complex actions needs to be performed on said state.
///
/// The state is expected to implement the [`Reducible`] trait which provides an `Action` type and a reducer
/// function.
///
/// # Example
/// ```rust
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// #
///
/// /// reducer's Action
/// enum CounterAction {
///     Double,
///     Square,
/// }
///
/// /// reducer's State
/// struct CounterState {
///     counter: i32,
/// }
///
/// impl Default for CounterState {
///     fn default() -> Self {
///         Self { counter: 1 }
///     }
/// }
///
/// impl Reducible for CounterState {
///     /// Reducer Action Type
///     type Action = CounterAction;
///
///     /// Reducer Function
///     fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
///         let next_ctr = match action {
///             CounterAction::Double => self.counter * 2,
///             CounterAction::Square => self.counter.pow(2)
///         };
///
///         Self { counter: next_ctr }.into()
///     }
/// }
///
/// #[function_component(UseReducer)]
/// fn reducer() -> Html {
///     // The use_reducer hook takes an initialization function which will be called only once.
///     let counter = use_reducer(CounterState::default);
///
///    let double_onclick = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.dispatch(CounterAction::Double))
///     };
///     let square_onclick = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.dispatch(CounterAction::Square))
///     };
///
///     html! {
///         <>
///             <div id="result">{ counter.counter }</div>
///
///             <button onclick={double_onclick}>{ "Double" }</button>
///             <button onclick={square_onclick}>{ "Square" }</button>
///         </>
///     }
/// }
/// ```
pub fn use_reducer<T, F>(initial_fn: F) -> UseReducerHandle<T>
where
    T: Reducible + 'static,
    F: FnOnce() -> T,
{
    use_reducer_base(initial_fn, |_, _| true)
}

/// [`use_reducer`] but only re-renders when `prev_state != next_state`.
///
/// This requires the state to implement [`PartialEq`] in addition to the [`Reducible`] trait
/// required by [`use_reducer`].
pub fn use_reducer_eq<T, F>(initial_fn: F) -> UseReducerHandle<T>
where
    T: Reducible + PartialEq + 'static,
    F: FnOnce() -> T,
{
    use_reducer_base(initial_fn, T::ne)
}
