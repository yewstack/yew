use super::{use_hook, Hook};
use std::rc::Rc;

/// This hook is an alternative to [`use_state`]. It is used to handle component's state and is used
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
///     let (
///         counter, // the state
///         // function to update the state
///         // as the same suggests, it dispatches the values to the reducer function
///         dispatch
///     ) = use_reducer(
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
///         let dispatch = Rc::clone(&dispatch);
///         Callback::from(move |_| dispatch(Action::Double))
///     };
///     let square_onclick = Callback::from(move |_| dispatch(Action::Square));
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
pub fn use_reducer<Action: 'static, Reducer, State: 'static>(
    reducer: Reducer,
    initial_state: State,
) -> (Rc<State>, Rc<impl Fn(Action)>)
where
    Reducer: Fn(Rc<State>, Action) -> State + 'static,
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
///     let (counter, dispatch) = use_reducer_with_init(
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
///             <button onclick=Callback::from(move |_| dispatch(10))>{"Increment by 10"}</button>
///         </>
///     }
/// }
/// ```
pub fn use_reducer_with_init<Action: 'static, Reducer, State: 'static, InitialState, InitFn>(
    reducer: Reducer,
    initial_state: InitialState,
    init: InitFn,
) -> (Rc<State>, Rc<impl Fn(Action)>)
where
    Reducer: Fn(Rc<State>, Action) -> State + 'static,
    InitFn: Fn(InitialState) -> State,
{
    struct UseReducerState<State> {
        current_state: Rc<State>,
    }
    impl<T> Hook for UseReducerState<T> {}
    let init = Box::new(init);
    let reducer = Rc::new(reducer);
    use_hook(
        |internal_hook_change: &mut UseReducerState<State>, hook_callback| {
            (
                internal_hook_change.current_state.clone(),
                Rc::new(move |action: Action| {
                    let reducer = reducer.clone();
                    hook_callback(
                        move |internal_hook_change: &mut UseReducerState<State>| {
                            internal_hook_change.current_state = Rc::new((reducer)(
                                internal_hook_change.current_state.clone(),
                                action,
                            ));
                            true
                        },
                        false, // run pre render
                    );
                }),
            )
        },
        move || UseReducerState {
            current_state: Rc::new(init(initial_state)),
        },
    )
}
