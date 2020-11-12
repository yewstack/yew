//! Function components are a simplified version of normal components.
//! They consist of a single function that receives props and determines what should be rendered by returning [`Html`].
//! Basically, it's a component that's been reduced to just the [`Component::view`] method.
//! On its own that would be quite limiting because you can only create pure components, but that's where Hooks come in.
//! Hooks allow function components to use state and other Yew features without implementing the [`Component`] trait.
//!
//! ```rust
//! # use yew_functional::function_component;
//! # use yew::prelude::*;
//!
//! #[function_component(HelloWorld)]
//! fn hello_world() -> Html {
//!     html! { "Hello world" }
//! }
//! ```
//!
//! # Defining function components
//!
//! The simplest way to define function components is use the [`function_component`] attribute.
//! It is also possible to define them manually by implementing [`FunctionProvider`] trait and
//! using [`FunctionComponent`] struct. [`function_component`] provides an abstraction over
//! implementing [`FunctionProvider`] and exposing [`FunctionComponent`] type.
//!
//! # Hooks
//!
//! Hooks are simply functions that let you “hook into” components' state and/or lifecycle and perform actions. Yew comes with a few pre-defined Hooks.
//! You can also create your own by using [`use_hook`] function.
//!
//! A full guide for consuming Hooks and define custom ones can be found on [Yew's website](https://yew.rs/)
//!
//! ## Why do Hooks return an [`Rc`]?
//!
//! In most cases, you'll be cloning the values returned from the Hooks.
//! As it may be expensive to clone such values, they're `Rc`ed, so they can be cloned relatively cheaply.
//! An example of such a situation would be when you have to clone a a large HashMap or Vector.
//! It will be much cheaper to clone an `Rc` than to clone such a type.
//!
//! It is worth noting that this problem doesn't occur with premitives and with types that implment [`Copy`]
//!
//! Following example shows one of the most common cases which requires cloning the values:
//!
//! ```rust
//! # use yew_functional::{function_component, use_state};
//! # use yew::prelude::*;
//! # use std::rc::Rc;
//!
//! fn component() -> Html {
//!     let (data, set_data) = use_state(|| "".to_string());
//!     let onclick = {
//!         let data = Rc::clone(&data);
//!         // Values must be moved into this closure so in order to be able to use them later on, they must be cloned
//!         Callback::from(move |_| set_data(format!("{}MoreData", data)))
//!     };
//!
//!     html! { <>
//!         // If `data` wasn't cloned above, it would've been impossible to use it here
//!         { &data }
//!         <button onclick=onclick>{"Show data"}</button>
//!     </>}
//! }
//! ```
//!

use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use yew::html::AnyScope;
use yew::{Component, ComponentLink, Html, Properties};

mod use_context_hook;
pub use use_context_hook::*;
/// This attribute creates a function component from a normal Rust function.
///
/// Functions with this attribute **must** return `Html` and can optionally take an argument for props.
/// Note that the function only receives a reference to the props.
///
/// When using this attribute you need to provide a name for the component:
/// `#[function_component(ComponentName)]`.
/// The attribute will then automatically create a [`FunctionComponent`] with the given identifier
/// which you can use like a normal component.
///
/// # Example
/// ```rust
/// # use yew_functional::function_component;
/// # use yew::prelude::*;
/// #
/// # #[derive(Properties, Clone, PartialEq)]
/// # pub struct Props {
/// #     text: String
/// # }
/// #
/// #[function_component(NameOfComponent)]
/// pub fn component(props: &Props) -> Html {
///     html! {
///         <p>{ &props.text }</p>
///     }
/// }
/// ```
pub use yew_functional_macro::function_component;

thread_local! {
    static CURRENT_HOOK: RefCell<Option<HookState>> = RefCell::new(None);
}

pub trait Hook {
    fn tear_down(&mut self) {}
}

type Msg = Box<dyn FnOnce() -> bool>;
type ProcessMessage = Rc<dyn Fn(Msg, bool)>;

struct HookState {
    counter: usize,
    scope: AnyScope,
    process_message: ProcessMessage,
    hooks: Vec<Rc<RefCell<dyn std::any::Any>>>,
    destroy_listeners: Vec<Box<dyn FnOnce()>>,
}

pub trait FunctionProvider {
    type TProps: Properties + PartialEq;
    fn run(props: &Self::TProps) -> Html;
}

#[derive(Clone, Default)]
struct MsgQueue(Rc<RefCell<Vec<Msg>>>);

impl MsgQueue {
    fn push(&self, msg: Msg) {
        self.0.borrow_mut().push(msg);
    }

    fn drain(&self) -> Vec<Msg> {
        self.0.borrow_mut().drain(..).collect()
    }
}

pub struct FunctionComponent<T: FunctionProvider + 'static> {
    _never: std::marker::PhantomData<T>,
    props: T::TProps,
    link: ComponentLink<Self>,
    hook_state: RefCell<Option<HookState>>,
    message_queue: MsgQueue,
}

impl<T> FunctionComponent<T>
where
    T: FunctionProvider,
{
    fn swap_hook_state(&self) {
        CURRENT_HOOK.with(|previous_hook| {
            std::mem::swap(
                &mut *previous_hook
                    .try_borrow_mut()
                    .expect("Previous hook still borrowed"),
                &mut *self.hook_state.borrow_mut(),
            );
        });
    }
}

impl<T: 'static> Component for FunctionComponent<T>
where
    T: FunctionProvider,
{
    type Message = Box<dyn FnOnce() -> bool>;
    type Properties = T::TProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let scope = AnyScope::from(link.clone());
        let message_queue = MsgQueue::default();
        Self {
            _never: std::marker::PhantomData::default(),
            props,
            link: link.clone(),
            message_queue: message_queue.clone(),
            hook_state: RefCell::new(Some(HookState {
                counter: 0,
                scope,
                process_message: Rc::new(move |msg, post_render| {
                    if post_render {
                        message_queue.push(msg);
                    } else {
                        link.send_message(msg);
                    }
                }),
                hooks: vec![],
                destroy_listeners: vec![],
            })),
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        for msg in self.message_queue.drain() {
            self.link.send_message(msg);
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        msg()
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        let mut props = props;
        std::mem::swap(&mut self.props, &mut props);
        props != self.props
    }

    fn view(&self) -> Html {
        // Reset hook
        self.hook_state
            .try_borrow_mut()
            .expect("Unexpected concurrent/nested view call")
            .as_mut()
            .unwrap()
            .counter = 0;

        // Load hook
        self.swap_hook_state();

        let ret = T::run(&self.props);

        // Restore previous hook
        self.swap_hook_state();

        ret
    }

    fn destroy(&mut self) {
        if let Some(ref mut hook_state) = *self.hook_state.borrow_mut() {
            for hook in hook_state.destroy_listeners.drain(..) {
                hook()
            }
        }
    }
}

/// `use_ref` is used for obtaining a mutable reference to a stateful value.
/// Its state persists across renders.
///
/// It is important to note that you do not get notified of state changes.
/// If you need the component to be re-rendered on state change, consider using [`use_state`].
///
/// # Example
///
/// ```rust
/// # use yew_functional::{function_component, use_state, use_ref};
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// # use std::cell::RefCell;
/// # use std::ops::{Deref, DerefMut};
///
/// #[function_component(UseRef)]
/// fn ref_hook() -> Html {
///     let (outer_html, set_outer_html) = use_state(|| "".to_string());
///
///     let node_ref: Rc<RefCell<NodeRef>> = use_ref(|| NodeRef::default());
///
///     let on_click = {
///         let node_ref = Rc::clone(&node_ref);
///
///         Callback::from(move |e| {
///             let to_set = (*node_ref.borrow().deref())
///                 .cast::<yew::web_sys::Element>()
///                 .unwrap()
///                 .outer_html();
///             set_outer_html(to_set)
///         })
///     };
///     html! {
///         <>
///             <div id="result" ref=(*node_ref.borrow_mut().deref_mut()).clone()>{"Filler"}</div>
///             {outer_html}
///             <br />
///             <button onclick=on_click>{"Refresh"}</button>
///         </>
///     }
/// }
/// ```
pub fn use_ref<T: 'static, InitialProvider>(initial_value: InitialProvider) -> Rc<RefCell<T>>
where
    InitialProvider: FnOnce() -> T,
{
    #[derive(Clone)]
    struct UseRefState<T>(Rc<RefCell<T>>);
    impl<T> Hook for UseRefState<T> {}

    use_hook(
        |state: &mut UseRefState<T>, hook_callback| {
            // we need it to be a specific closure type, even if we never use it
            let _ignored = || hook_callback(|_| false, false);
            state.0.clone()
        },
        move || UseRefState(Rc::new(RefCell::new(initial_value()))),
    )
}
/// `use_reducer` is an alternative to [`use_state`]. It is used to handle component's state and is used
/// when complex actions needs to be performed on said state.
///
/// It accepts a reducer function and initial state and returns [`Rc`] of the state, and a dispatch function.
/// The dispatch function takes one argument of `Action`. When called, the action and current value
/// are passed to the reducer function which computes a new state which is returned,
/// and the component is re-rendered.
///
/// For lazy initialization, consider using [`use_reducer_with_init`] instead.
///
/// # Example
///
/// ```rust
/// # use yew_functional::{function_component, use_reducer};
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// # use std::ops::DerefMut;
///
/// #[function_component(UseReducer)]
/// fn reducer() -> Html {
///     //// reducer's Action
///     enum Action {
///         Double,
///         Square,
///     }
///
///     //// reducer's State
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
///     let double_onclick = {
///         let dispatch = Rc::clone(&dispatch);
///         Callback::from(move |_| dispatch(Action::Double))
///     };
///     let square_onclick = Callback::from(move |_| dispatch(Action::Square));
///
///     html! {
///         <>
///             <div id="result">{counter.counter}</div>
///
///             <button onclick=double_onclick>{"Double"}</button>
///             <button onclick=square_onclick>{"Square"}</button>
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

/// [`use_reducer`] but with init argument. The Hook is passed the initial state
/// which is then passed down to `init` function which initializes the state and returns it.
/// The hook then returns this state.
///
/// This is useful for lazy initialization where it is beneficial not to perform expensive
/// computation up-front
///
/// # Example
/// ```rust
/// # use yew_functional::{function_component, use_reducer_with_init};
/// # use yew::prelude::*;
/// # use std::rc::Rc;
///
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
    impl<T> Hook for UseReducerState<T> {};
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

/// `use_state` is used to mange state in a function component.
/// It returns a `Rc` of the stateful value, and a setter function.
///
/// Initially, the state is set to the result of the function passed.
/// This value remains up-to-date on subsequent renders.
///
/// The setter function is used to update the value and trigger a re-render.
///
/// # Example
///
/// ```rust
/// # use yew_functional::{function_component, use_state, use_ref};
/// # use yew::prelude::*;
/// # use std::rc::Rc;
///
/// #[function_component(UseState)]
/// fn state() -> Html {
///     let (
///         counter, // the returned state
///         set_counter // setter to update the state
///     ) = use_state(|| 0);
///     let onclick = {
///         let counter = Rc::clone(&counter);
///         Callback::from(move |_| set_counter(*counter + 1))
///     };
///
///     html! {
///         <div>
///             <button onclick=onclick>{ "Increment value" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 { counter }
///             </p>
///         </div>
///     }
/// }
/// ```
pub fn use_state<T, F>(initial_state_fn: F) -> (Rc<T>, Rc<impl Fn(T)>)
where
    F: FnOnce() -> T,
    T: 'static,
{
    struct UseStateState<T2> {
        current: Rc<T2>,
    }
    impl<T> Hook for UseStateState<T> {}
    use_hook(
        |prev: &mut UseStateState<T>, hook_callback| {
            let current = prev.current.clone();
            (
                current,
                Rc::new(move |o: T| {
                    hook_callback(
                        |state: &mut UseStateState<T>| {
                            state.current = Rc::new(o);
                            true
                        },
                        false, // run pre render
                    )
                }),
            )
        },
        move || UseStateState {
            current: Rc::new(initial_state_fn()),
        },
    )
}

/// `use_effect` is used for hooking into the component's lifecycle.
/// Similar to `rendered` method of [`Component`] trait,
/// `use_effect` takes a function which is called after the render finishes.
///
/// The said function returns another function, the destructor function,
/// which called when the component is destroyed. It can be used to clean up the effects introduced.
/// This is similar to `destroyed` method of [`Component`] trait.
///
/// # Example
///
/// ```rust
/// # use yew_functional::{function_component, use_effect, use_state};
/// # use yew::prelude::*;
/// # use std::rc::Rc;
///
/// #[function_component(UseEffect)]
/// fn effect() -> Html {
///     let (counter, set_counter) = use_state(|| 0);
///
///     let counter_one = counter.clone();
///     use_effect(move || {
///         // Make a call to DOM API after component is rendered
///         yew::utils::document().set_title(&format!("You clicked {} times", counter_one));
///
///         // Perform the cleanup
///         || yew::utils::document().set_title(&format!("You clicked 0 times"))
///     });
///
///     let onclick = {
///         let counter = Rc::clone(&counter);
///         Callback::from(move |_| set_counter(*counter + 1))
///     };
///
///     return html! {<>
///         <button onclick=onclick>{ format!("Increment to {}", counter) }</button>
///     </>};
/// }
/// ```
pub fn use_effect<F, Destructor>(callback: F)
where
    F: FnOnce() -> Destructor + 'static,
    Destructor: FnOnce() + 'static,
{
    struct UseEffectState<Destructor> {
        destructor: Option<Box<Destructor>>,
    }
    impl<T: FnOnce() + 'static> Hook for UseEffectState<T> {
        fn tear_down(&mut self) {
            if let Some(destructor) = self.destructor.take() {
                destructor()
            }
        }
    }

    let callback = Box::new(callback);

    use_hook(
        |_: &mut UseEffectState<Destructor>, hook_callback| {
            hook_callback(
                move |state: &mut UseEffectState<Destructor>| {
                    if let Some(de) = state.destructor.take() {
                        de();
                    }
                    let new_destructor = callback();
                    state.destructor.replace(Box::new(new_destructor));
                    false
                },
                true, // run post render
            );
        },
        || UseEffectState { destructor: None },
    );
}

/// Sometimes, it's needed to manually define dependencies for [`use_effect`].
/// In such cases, we use `use_effect_with_deps`.
///
/// # Example
///
/// ```rust
/// use yew_functional::{function_component, use_effect_with_deps};
/// use yew::prelude::*;
///
/// # #[function_component(UseEffectWithDeps)]
/// # fn effect_with_deps() -> Html {
///     use_effect_with_deps(
///         move |_| {
///             // ...
///             || {}
///         },
///         (), // dependents
///     );
///
/// #    html! {}
/// # }
/// ```
pub fn use_effect_with_deps<F, Destructor, Dependents>(callback: F, deps: Dependents)
where
    F: FnOnce(&Dependents) -> Destructor + 'static,
    Destructor: FnOnce() + 'static,
    Dependents: PartialEq + 'static,
{
    struct UseEffectState<Dependents, Destructor> {
        deps: Rc<Dependents>,
        destructor: Option<Box<Destructor>>,
    }
    impl<Dependents, Destructor: FnOnce() + 'static> Hook for UseEffectState<Dependents, Destructor> {
        fn tear_down(&mut self) {
            if let Some(destructor) = self.destructor.take() {
                destructor()
            }
        }
    }

    let deps = Rc::new(deps);
    let deps_c = deps.clone();

    use_hook(
        move |_state: &mut UseEffectState<Dependents, Destructor>, hook_callback| {
            hook_callback(
                move |state: &mut UseEffectState<Dependents, Destructor>| {
                    if state.deps != deps {
                        if let Some(de) = state.destructor.take() {
                            de();
                        }
                        let new_destructor = callback(deps.borrow());
                        state.deps = deps;
                        state.destructor.replace(Box::new(new_destructor));
                    } else if state.destructor.is_none() {
                        state
                            .destructor
                            .replace(Box::new(callback(state.deps.borrow())));
                    }
                    false
                },
                true, // run post render
            );
        },
        || UseEffectState {
            deps: deps_c,
            destructor: None,
        },
    );
}

pub fn use_hook<InternalHookState, HookRunner, R, InitialStateProvider, HookUpdate: 'static>(
    hook_runner: HookRunner,
    initial_state_producer: InitialStateProvider,
) -> R
where
    HookRunner: FnOnce(&mut InternalHookState, Box<dyn Fn(HookUpdate, bool)>) -> R,
    InternalHookState: Hook + 'static,
    InitialStateProvider: FnOnce() -> InternalHookState,
    HookUpdate: FnOnce(&mut InternalHookState) -> bool,
{
    // Extract current hook
    let (hook, process_message) = CURRENT_HOOK.with(|hook_state_holder| {
        let hook_state_holder = hook_state_holder.try_borrow_mut();
        let mut hook_state_holder = hook_state_holder.expect("Nested hooks not supported");
        let mut hook_state = hook_state_holder
            .as_mut()
            .expect("No current hook. Hooks can only be called inside function components");

        // Determine which hook position we're at and increment for the next hook
        let hook_pos = hook_state.counter;
        hook_state.counter += 1;

        // Initialize hook if this is the first call
        if hook_pos >= hook_state.hooks.len() {
            let initial_state = Rc::new(RefCell::new(initial_state_producer()));
            hook_state.hooks.push(initial_state.clone());
            hook_state.destroy_listeners.push(Box::new(move || {
                initial_state.borrow_mut().deref_mut().tear_down();
            }));
        }

        let hook = hook_state.hooks[hook_pos].clone();

        (hook, hook_state.process_message.clone())
    });

    let hook_callback = {
        let hook = hook.clone();
        Box::new(move |update: HookUpdate, post_render| {
            let hook = hook.clone();
            process_message(
                Box::new(move || {
                    let mut hook = hook.borrow_mut();
                    let hook = hook.downcast_mut::<InternalHookState>();
                    let hook = hook.expect(
                        "Incompatible hook type. Hooks must always be called in the same order",
                    );
                    update(hook)
                }),
                post_render,
            );
        })
    };
    let mut hook = hook.borrow_mut();
    let hook = hook.downcast_mut::<InternalHookState>();
    let mut hook =
        hook.expect("Incompatible hook type. Hooks must always be called in the same order");

    // Execute the actual hook closure we were given. Let it mutate the hook state and let
    // it create a callback that takes the mutable hook state.
    hook_runner(&mut hook, hook_callback)
}

pub(crate) fn get_current_scope() -> Option<AnyScope> {
    CURRENT_HOOK.with(|cell| cell.borrow().as_ref().map(|state| state.scope.clone()))
}
