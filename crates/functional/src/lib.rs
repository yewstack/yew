use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use yew::{Component, ComponentLink, Html, Properties};

thread_local! {
    static CURRENT_HOOK: RefCell<Option<HookState>> = RefCell::new(None);
}

struct HookState {
    counter: usize,
    process_message: Rc<dyn Fn(Box<dyn FnOnce() -> bool>)>,
    hooks: Vec<Rc<RefCell<dyn std::any::Any>>>,
}

pub trait FunctionProvider {
    type TProps: Properties + PartialEq;
    fn run(props: &Self::TProps) -> Html;
}

pub struct FunctionComponent<T: FunctionProvider> {
    _never: std::marker::PhantomData<T>,
    props: T::TProps,
    hook_state: RefCell<Option<HookState>>,
}

impl<T: 'static> Component for FunctionComponent<T>
where
    T: FunctionProvider,
{
    type Message = Box<dyn FnOnce() -> bool>;
    type Properties = T::TProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FunctionComponent {
            _never: std::marker::PhantomData::default(),
            props,
            hook_state: RefCell::new(Some(HookState {
                counter: 0,
                process_message: Rc::new(move |msg| link.send_message(msg)),
                hooks: vec![],
            })),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        msg()
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        let mut props = props;
        std::mem::swap(&mut self.props, &mut props);
        props == self.props
    }

    //noinspection DuplicatedCode
    fn view(&self) -> Html {
        // Reset hook
        self.hook_state
            .try_borrow_mut()
            .expect("Unexpected concurrent/nested view call")
            .as_mut()
            .unwrap()
            .counter = 0;
        // Load hook
        CURRENT_HOOK.with(|previous_hook| {
            std::mem::swap(
                previous_hook
                    .try_borrow_mut()
                    .expect("Previous hook still borrowed")
                    .deref_mut(),
                self.hook_state.borrow_mut().deref_mut(),
            );
        });

        let ret = T::run(&self.props);

        // Unload hook
        CURRENT_HOOK.with(|previous_hook| {
            std::mem::swap(
                previous_hook
                    .try_borrow_mut()
                    .expect("Previous hook still borrowed")
                    .deref_mut(),
                self.hook_state.borrow_mut().deref_mut(),
            );
        });

        return ret;
    }
}

pub fn use_ref<T: 'static, InitialProvider>(initial_value: InitialProvider) -> Rc<RefCell<T>>
where
    InitialProvider: FnOnce() -> T,
{
    type UseRefState<T> = Rc<RefCell<T>>;

    use_hook(
        |state: &mut UseRefState<T>, pretrigger_change_acceptor| {
            let _ignored = || pretrigger_change_acceptor(|_| false); // we need it to be a specific closure type, even if we never use it
            return state.clone();
        },
        move || Rc::new(RefCell::new(initial_value())),
    )
}

pub fn use_reducer<Action: 'static, Reducer, State: 'static>(
    reducer: Reducer,
    initial_state: State,
) -> (Rc<State>, Box<impl Fn(Action)>)
where
    Reducer: Fn(Rc<State>, Action) -> State + 'static,
{
    return use_reducer_with_init(reducer, initial_state, |a| a);
}

pub fn use_reducer_with_init<Action: 'static, Reducer, State: 'static, InitialState, InitFn>(
    reducer: Reducer,
    initial_state: InitialState,
    init: InitFn,
) -> (Rc<State>, Box<impl Fn(Action)>)
where
    Reducer: Fn(Rc<State>, Action) -> State + 'static,
    InitFn: Fn(InitialState) -> State,
{
    struct UseReducerState<State> {
        current_state: Rc<State>,
    }
    let init = Box::new(init);
    let reducer = Rc::new(reducer);
    let ret = use_hook(
        |internal_hook_change: &mut UseReducerState<State>, pretrigger_change_runner| {
            return (
                internal_hook_change.current_state.clone(),
                Box::new(move |action: Action| {
                    let reducer = reducer.clone();
                    pretrigger_change_runner(
                        move |internal_hook_change: &mut UseReducerState<State>| {
                            internal_hook_change.current_state = Rc::new((reducer)(
                                internal_hook_change.current_state.clone(),
                                action,
                            ));
                            true
                        },
                    );
                }),
            );
        },
        move || UseReducerState {
            current_state: Rc::new(init(initial_state)),
        },
    );
    return ret;
}

pub fn use_state<T, F>(initial_state_fn: F) -> (Rc<T>, Box<impl Fn(T)>)
where
    F: FnOnce() -> T,
    T: 'static,
{
    struct UseStateState<T2> {
        current: Rc<T2>,
    }
    return use_hook(
        |prev: &mut UseStateState<T>, hook_update| {
            let current = prev.current.clone();
            return (
                current,
                Box::new(move |o: T| {
                    hook_update(|state: &mut UseStateState<T>| {
                        state.current = Rc::new(o);
                        true
                    });
                }),
            );
        },
        move || UseStateState {
            current: Rc::new(initial_state_fn()),
        },
    );
}

pub fn use_hook<InternalHookState, HookRunner, R, InitialStateProvider, PretriggerChange: 'static>(
    hook_runner: HookRunner,
    initial_state_producer: InitialStateProvider,
) -> R
where
    HookRunner: FnOnce(&mut InternalHookState, Box<dyn Fn(PretriggerChange)>) -> R,
    InternalHookState: 'static,
    InitialStateProvider: FnOnce() -> InternalHookState,
    PretriggerChange: FnOnce(&mut InternalHookState) -> bool,
{
    // Extract current hook
    let (hook, process_message) = CURRENT_HOOK.with(|hook_state_holder| {
        let hook_state_holder = hook_state_holder.try_borrow_mut();
        let mut hook_state_holder = hook_state_holder.expect("Nested hooks not supported");
        let mut hook_state = hook_state_holder
            .as_mut()
            .expect("No current hook. Hooks can only be called inside functional components");

        // Determine which hook position we're at and increment for the next hook
        let hook_pos = hook_state.counter;
        hook_state.counter += 1;

        // Initialize hook if this is the first call
        if hook_pos >= hook_state.hooks.len() {
            let initial_state = Rc::new(RefCell::new(initial_state_producer()));
            hook_state.hooks.push(initial_state);
        }

        let hook = hook_state.hooks[hook_pos].clone();

        return (hook, hook_state.process_message.clone());
    });

    let trigger = {
        let hook = hook.clone();
        Box::new(move |pretrigger_change: PretriggerChange| {
            let hook = hook.clone();
            process_message(Box::new(move || {
                let mut hook = hook.borrow_mut();
                let hook = hook.downcast_mut::<InternalHookState>();
                let hook = hook.expect(
                    "Incompatible hook type. Hooks must always be called in the same order",
                );
                pretrigger_change(hook)
            }));
        })
    };
    let mut hook = hook.borrow_mut();
    let hook = hook.downcast_mut::<InternalHookState>();
    let mut hook =
        hook.expect("Incompatible hook type. Hooks must always be called in the same order");

    // Execute the actual hook closure we were given. Let it mutate the hook state and let
    // it create a callback that takes the mutable hook state.
    hook_runner(&mut hook, trigger)
}
