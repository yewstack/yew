use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use yew::html::AnyScope;
use yew::{Component, ComponentLink, Html, Properties};

mod use_context_hook;
pub use use_context_hook::*;
/// This macro autogenerates the requirements for function components which allows any function to
/// be annotated and used as a Yew component.
///
/// Functions to which this macro is applied to **must** return `Html` and can optionally take an
/// argument for props. The name for the component is passed as an attribute to the macro.
///
/// This attribute generates a struct which implements [`FunctionProvider`] trait.
/// A type alias `FunctionComponent<CreatedStruct>` is then created.
/// [`FunctionComponent`] is a struct which implements `yew::Component`
/// and handles the magic required for making function components work.
/// In most cases, you don't need to use the trait implementation,
/// all you need is the aliased `type` which can be used as any other component.
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

pub fn use_reducer<Action: 'static, Reducer, State: 'static>(
    reducer: Reducer,
    initial_state: State,
) -> (Rc<State>, Rc<impl Fn(Action)>)
where
    Reducer: Fn(Rc<State>, Action) -> State + 'static,
{
    use_reducer_with_init(reducer, initial_state, |a| a)
}

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
