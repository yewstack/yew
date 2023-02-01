use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use crate::functional::{hook, Hook, HookContext};

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
    current_state: Rc<RefCell<Rc<T>>>,

    dispatch: DispatchFn<T>,
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
        &self.value
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
fn use_reducer_base<'hook, T>(
    init_fn: impl 'hook + FnOnce() -> T,
    should_render_fn: fn(&T, &T) -> bool,
) -> impl 'hook + Hook<Output = UseReducerHandle<T>>
where
    T: Reducible + 'static,
{
    struct HookProvider<'hook, T, F>
    where
        T: Reducible + 'static,
        F: 'hook + FnOnce() -> T,
    {
        _marker: PhantomData<&'hook ()>,

        init_fn: F,
        should_render_fn: fn(&T, &T) -> bool,
    }

    impl<'hook, T, F> Hook for HookProvider<'hook, T, F>
    where
        T: Reducible + 'static,
        F: 'hook + FnOnce() -> T,
    {
        type Output = UseReducerHandle<T>;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            let Self {
                init_fn,
                should_render_fn,
                ..
            } = self;

            let state = ctx.next_state(move |re_render| {
                let val = Rc::new(RefCell::new(Rc::new(init_fn())));
                let should_render_fn = Rc::new(should_render_fn);

                UseReducer {
                    current_state: val.clone(),
                    dispatch: Rc::new(move |action: T::Action| {
                        let should_render = {
                            let should_render_fn = should_render_fn.clone();
                            let mut val = val.borrow_mut();
                            let next_val = (*val).clone().reduce(action);
                            let should_render = should_render_fn(&next_val, &val);
                            *val = next_val;

                            should_render
                        };

                        // Currently, this triggers a render immediately, so we need to release the
                        // borrowed reference first.
                        if should_render {
                            re_render()
                        }
                    }),
                }
            });

            let value = state.current_state.borrow().clone();
            let dispatch = state.dispatch.clone();

            UseReducerHandle { value, dispatch }
        }
    }

    HookProvider {
        _marker: PhantomData,
        init_fn,
        should_render_fn,
    }
}

/// This hook is an alternative to [`use_state`](super::use_state()).
/// It is used to handle component's state and is used when complex actions needs to be performed on
/// said state.
///
/// The state is expected to implement the [`Reducible`] trait which provides an `Action` type and a
/// reducer function.
///
/// The state object returned by the initial state function is required to
/// implement a `Reducible` trait which defines the associated `Action` type and a
/// reducer function.
///
/// This hook will always trigger a re-render upon receiving an action. See
/// [`use_reducer_eq`] if you want the component to only re-render when the state changes.
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
///             CounterAction::Square => self.counter.pow(2),
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
///     let double_onclick = {
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
///
/// # Tip
///
/// The dispatch function is guaranteed to be the same across the entire
/// component lifecycle. You can safely omit the `UseReducerHandle` from the
/// dependents of `use_effect_with_deps` if you only intend to dispatch
/// values from within the hooks.
///
/// # Caution
///
/// The value held in the handle will reflect the value of at the time the
/// handle is returned by the `use_reducer`. It is possible that the handle does
/// not dereference to an up to date value if you are moving it into a
/// `use_effect_with_deps` hook. You can register the
/// state to the dependents so the hook can be updated when the value changes.
#[hook]
pub fn use_reducer<T, F>(init_fn: F) -> UseReducerHandle<T>
where
    T: Reducible + 'static,
    F: FnOnce() -> T,
{
    use_reducer_base(init_fn, |_, _| true)
}

/// [`use_reducer`] but only re-renders when `prev_state != next_state`.
///
/// This requires the state to implement [`PartialEq`] in addition to the [`Reducible`] trait
/// required by [`use_reducer`].
#[hook]
pub fn use_reducer_eq<T, F>(init_fn: F) -> UseReducerHandle<T>
where
    T: Reducible + PartialEq + 'static,
    F: FnOnce() -> T,
{
    use_reducer_base(init_fn, T::ne)
}
