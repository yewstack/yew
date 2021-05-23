use crate::use_hook;
use std::ops::Deref;
use std::rc::Rc;

struct UseReducer<State> {
    current_state: Rc<State>,
}

/// This hook is an alternative to [`use_state`](super::use_state()). It is used to handle component's state and is used
/// when complex actions needs to be performed on said state.
///
/// For lazy initialization, consider using [`use_reducer_with_init`] instead.
///
/// # Example
/// ```rust
/// # use yew_functional::{function_component, use_reducer};
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// # use std::ops::DerefMut;
/// #
/// #[function_component(UseReducer)]
/// fn reducer() -> Html {
///     /// reducer's Action
///     enum Action {
///         Double,
///         Square,
///     }
///
///     /// reducer's State
///     struct CounterState {
///         counter: i32,
///     }
///
///     let counter = use_reducer(
///         // the reducer function
///         |prev: Rc<CounterState>, action: Action| CounterState {
///             counter: match action {
///                 Action::Double => prev.counter * 2,
///                 Action::Square => prev.counter * prev.counter,
///             }
///         },
///         // initial state
///         CounterState { counter: 1 },
///     );
///
///    let double_onclick = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.dispatch(Action::Double))
///     };
///     let square_onclick = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.dispatch(Action::Square))
///     };
///
///     html! {
///         <>
///             <div id="result">{ counter.counter }</div>
///
///             <button onclick=double_onclick>{ "Double" }</button>
///             <button onclick=square_onclick>{ "Square" }</button>
///         </>
///     }
/// }
/// ```
pub fn use_reducer<Action, Reducer, State>(
    reducer: Reducer,
    initial_state: State,
) -> UseReducerHandle<State, Action>
where
    Action: 'static,
    Reducer: Fn(Rc<State>, Action) -> State + 'static,
    State: 'static,
{
    use_reducer_with_init(reducer, initial_state, |a| a)
}

/// [`use_reducer`] but with init argument.
///
/// This is useful for lazy initialization where it is beneficial not to perform expensive
/// computation up-front
///
/// # Example
/// ```rust
/// # use yew_functional::{function_component, use_reducer_with_init};
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// #
/// #[function_component(UseReducerWithInit)]
/// fn reducer_with_init() -> Html {
///     struct CounterState {
///         counter: i32,
///     }
///     let counter = use_reducer_with_init(
///         |prev: Rc<CounterState>, action: i32| CounterState {
///             counter: prev.counter + action,
///         },
///         0,
///         |initial: i32| CounterState {
///             counter: initial + 10,
///         },
///     );
///
///     html! {
///         <>
///             <div id="result">{counter.counter}</div>
///
///             <button onclick=Callback::from(move |_| counter.dispatch(10))>{"Increment by 10"}</button>
///         </>
///     }
/// }
/// ```
pub fn use_reducer_with_init<Reducer, Action, State, InitialState, InitFn>(
    reducer: Reducer,
    initial_state: InitialState,
    init: InitFn,
) -> UseReducerHandle<State, Action>
where
    Reducer: Fn(Rc<State>, Action) -> State + 'static,
    Action: 'static,
    State: 'static,
    InitialState: 'static,
    InitFn: Fn(InitialState) -> State + 'static,
{
    let init = Box::new(init);
    let reducer = Rc::new(reducer);
    use_hook(
        move || UseReducer {
            current_state: Rc::new(init(initial_state)),
        },
        |s, updater| {
            let setter: Rc<dyn Fn(Action)> = Rc::new(move |action: Action| {
                let reducer = reducer.clone();
                // We call the callback, consumer the updater
                // Required to put the type annotations on Self so the method knows how to downcast
                updater.callback(move |state: &mut UseReducer<State>| {
                    let new_state = reducer(state.current_state.clone(), action);
                    state.current_state = Rc::new(new_state);
                    true
                });
            });

            UseReducerHandle {
                value: Rc::clone(&s.current_state),
                setter,
            }
        },
        |_| {},
    )
}

/// State handle for [`use_reducer`] hook
pub struct UseReducerHandle<State, Action> {
    value: Rc<State>,
    setter: Rc<dyn Fn(Action)>,
}

impl<State, Action> UseReducerHandle<State, Action> {
    pub fn dispatch(&self, value: Action) {
        (self.setter)(value)
    }
}

impl<State, Action> Deref for UseReducerHandle<State, Action> {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

impl<State, Action> Clone for UseReducerHandle<State, Action> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
            setter: Rc::clone(&self.setter),
        }
    }
}
