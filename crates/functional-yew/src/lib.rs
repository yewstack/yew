use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use yew::{Component, ComponentLink, Html, Properties};

thread_local! {
    static CURRENT_HOOK: Rc<RefCell<Option<Rc<RefCell<HookState>>>>> = Rc::new(RefCell::new(None));
}

struct HookState {
    counter: usize,
    render_call: Rc<dyn Fn()>,
    hooks: Vec<Option<Box<dyn std::any::Any>>>,
}

pub trait FunctionProvider {
    type TProps: Properties + PartialEq;
    fn run(props: &Self::TProps) -> Html;
}

pub struct FunctionComponent<T: FunctionProvider> {
    _never: Option<std::marker::PhantomData<T>>,
    props: T::TProps,
    hook_state: RefCell<Option<Rc<RefCell<HookState>>>>,
}

impl<T: 'static> Component for FunctionComponent<T>
    where
        T: FunctionProvider,
{
    type Message = ();
    type Properties = T::TProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let render_call = Rc::new(move || link.send_message(()));
        FunctionComponent {
            _never: None,
            props,
            hook_state: RefCell::new(Some(Rc::new(RefCell::new(HookState {
                counter: 0,
                render_call,
                hooks: vec![],
            })))),
        }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        let mut props = props;
        std::mem::swap(&mut self.props, &mut props);
        props == self.props
    }

    fn view(&self) -> Html {
        self.hook_state
            .try_borrow_mut()
            .expect("Unexpected concurrent/nested view call")
            .as_mut()
            .map(|a| {
                a.clone()
                    .try_borrow_mut()
                    .expect("Unexpected borrow for hook state")
                    .counter = 0
            });
        let previous_hook = CURRENT_HOOK.with(|previous_hook| previous_hook.clone());
        std::mem::swap(
            previous_hook
                .try_borrow_mut()
                .expect("Previous hook still borrowed")
                .deref_mut(),
            self.hook_state.try_borrow_mut().unwrap().deref_mut(),
        );

        let ret = T::run(&self.props);

        std::mem::swap(
            previous_hook.borrow_mut().deref_mut(),
            self.hook_state.borrow_mut().deref_mut(),
        );
        return ret;
    }
}

pub fn use_ref<T: 'static, InitialProvider>(initial_value: InitialProvider) -> Rc<RefCell<T>>
    where
        InitialProvider: FnOnce() -> T,
{
    struct UseRefState<T> {
        o: Rc<RefCell<T>>,
    }

    let initial_value = Box::new(initial_value);

    use_hook(
        |state: &mut UseRefState<T>, pretrigger_change_acceptor| {
            let _ignored = || pretrigger_change_acceptor(|_| false); // we need it to be a specific closure type, even if we never use it
            return state.o.clone();
        },
        move || UseRefState {
            o: Rc::new(RefCell::new(initial_value())),
        },
    )
}

pub fn use_reducer1<Action: 'static, Reducer, State: 'static>(
    reducer: Reducer,
    initial_state: State,
) -> (Rc<State>, Box<impl Fn(Action)>)
    where
        Reducer: Fn(Rc<State>, Action) -> State + 'static,
{
    return use_reducer2(reducer, initial_state, |a| a);
}

pub fn use_reducer2<Action: 'static, Reducer, State: 'static, InitialState, InitFn>(
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

pub fn use_effect<F, Destructor>(callback: F)
    where
        F: FnOnce() -> Destructor,
        Destructor: FnOnce() + 'static,
{
    let callback = Box::new(callback);
    use_effect5(
        Box::new(|_: &(), _: &(), _: &(), _: &(), _: &()| callback()),
        (),
        (),
        (),
        (),
        (),
    );
}

pub fn use_effect1<F, Destructor, T1>(callback: F, o1: T1)
    where
        F: FnOnce(&T1) -> Destructor,
        Destructor: FnOnce() + 'static,
        T1: PartialEq + 'static,
{
    let callback = Box::new(callback);
    use_effect5(
        Box::new(|a: &T1, _: &(), _: &(), _: &(), _: &()| callback(a)),
        o1,
        (),
        (),
        (),
        (),
    );
}

pub fn use_effect2<F, Destructor, T1, T2>(callback: F, o1: T1, o2: T2)
    where
        F: FnOnce(&T1, &T2) -> Destructor,
        Destructor: FnOnce() + 'static,
        T1: PartialEq + 'static,
        T2: PartialEq + 'static,
{
    let callback = Box::new(callback);
    use_effect5(
        Box::new(|a: &T1, b: &T2, _: &(), _: &(), _: &()| callback(a, b)),
        o1,
        o2,
        (),
        (),
        (),
    );
}

pub fn use_effect3<F, Destructor, T1, T2, T3>(callback: F, o1: T1, o2: T2, o3: T3)
    where
        F: FnOnce(&T1, &T2, &T3) -> Destructor,
        Destructor: FnOnce() + 'static,
        T1: PartialEq + 'static,
        T2: PartialEq + 'static,
        T3: PartialEq + 'static,
{
    let callback = Box::new(callback);
    use_effect5(
        Box::new(|a: &T1, b: &T2, c: &T3, _: &(), _: &()| callback(a, b, c)),
        o1,
        o2,
        o3,
        (),
        (),
    );
}

pub fn use_effect4<F, Destructor, T1, T2, T3, T4>(callback: F, o1: T1, o2: T2, o3: T3, o4: T4)
    where
        F: FnOnce(&T1, &T2, &T3, &T4) -> Destructor,
        Destructor: FnOnce() + 'static,
        T1: PartialEq + 'static,
        T2: PartialEq + 'static,
        T3: PartialEq + 'static,
        T4: PartialEq + 'static,
{
    let callback = Box::new(callback);
    use_effect5(
        Box::new(|a: &T1, b: &T2, c: &T3, d: &T4, _: &()| callback(a, b, c, d)),
        o1,
        o2,
        o3,
        o4,
        (),
    );
}


pub fn use_effect5<F, Destructor, T1, T2, T3, T4, T5>(
    callback: Box<F>,
    o1: T1,
    o2: T2,
    o3: T3,
    o4: T4,
    o5: T5,
) where
    F: FnOnce(&T1, &T2, &T3, &T4, &T5) -> Destructor,
    Destructor: FnOnce() + 'static,
    T1: PartialEq + 'static,
    T2: PartialEq + 'static,
    T3: PartialEq + 'static,
    T4: PartialEq + 'static,
    T5: PartialEq + 'static,
{
    struct UseEffectState<T1, T2, T3, T4, T5, Destructor> {
        o1: Rc<T1>,
        o2: Rc<T2>,
        o3: Rc<T3>,
        o4: Rc<T4>,
        o5: Rc<T5>,
        destructor: Option<Box<Destructor>>,
    }
    let o1 = Rc::new(o1);
    let o2 = Rc::new(o2);
    let o3 = Rc::new(o3);
    let o4 = Rc::new(o4);
    let o5 = Rc::new(o5);
    let o1_c = o1.clone();
    let o2_c = o2.clone();
    let o3_c = o3.clone();
    let o4_c = o4.clone();
    let o5_c = o5.clone();
    use_hook(
        move |state: &mut UseEffectState<T1, T2, T3, T4, T5, Destructor>, hook_update| {
            let mut should_update = !(*state.o1 == *o1
                && *state.o2 == *o2
                && *state.o3 == *o3
                && *state.o4 == *o4
                && *state.o5 == *o5);

            if should_update {
                if let Some(de) = state.destructor.take() {
                    de();
                }
                let new_destructor = callback(
                    o1.borrow(),
                    o2.borrow(),
                    o3.borrow(),
                    o4.borrow(),
                    o5.borrow(),
                );
                state.o1 = o1.clone();
                state.o2 = o2.clone();
                state.o3 = o3.clone();
                state.o4 = o4.clone();
                state.o5 = o5.clone();
                state.destructor.replace(Box::new(new_destructor));
            } else if state.destructor.is_none() {
                should_update = true;
                state.destructor.replace(Box::new(callback(
                    state.o1.borrow(),
                    state.o2.borrow(),
                    state.o3.borrow(),
                    state.o4.borrow(),
                    state.o5.borrow(),
                )));
            }
            return move || {
                if should_update {
                    hook_update(
                        move |state: &mut UseEffectState<T1, T2, T3, T4, T5, Destructor>| true,
                    )
                }
            };
        },
        || UseEffectState {
            o1: o1_c,
            o2: o2_c,
            o3: o3_c,
            o4: o4_c,
            o5: o5_c,
            destructor: None,
        },
    )();
}

pub fn use_hook<InternalHookState, HookRunner, R, InitialStateProvider, PretriggerChange: 'static>(
    hook_runner: HookRunner,
    initial_state_producer: InitialStateProvider,
) -> R
    where
        HookRunner: FnOnce(&mut InternalHookState, Rc<dyn Fn(PretriggerChange)>) -> R,
        InternalHookState: 'static,
        InitialStateProvider: FnOnce() -> InternalHookState,
        PretriggerChange: FnOnce(&mut InternalHookState) -> bool,
{
    return CURRENT_HOOK.with(|current_hook_field| {
        // Obtain a persistent Rc to the hook
        let current_hook = current_hook_field.clone();
        let hook_ref = current_hook.try_borrow_mut()
            .expect("Nested hooks not supported");
        let persistent_hook_state = hook_ref.clone().expect(
            "No current hook. Hooks can only be called inside functional components",
        );
        // ...and release the mutable ref to the global hook state immediately
        std::mem::drop(hook_ref);


        // Determine which hook position we're at and increment for the next hook
        let mut mut_hook_state = persistent_hook_state.try_borrow_mut()
            .expect("This hook state is already in use");
        let hook_pos = mut_hook_state.counter;
        mut_hook_state.counter += 1;

        // Check if this is the first time this hook is run
        if let Some(preexisting_hook_outer) = mut_hook_state.hooks.get_mut(hook_pos) {
            // Nope, has been run previously -> Has a state we are managing

            // Take the current hook state out of its optional and release the mut ref to the
            // per component hook state right away so that nested hooks can act
            let mut hook_state_inner = preexisting_hook_outer.take()
                .expect("Hook nested in itself (1)");
            std::mem::drop(mut_hook_state);
            // Cast to the expected hook state
            let preexisting_hook = hook_state_inner
                .downcast_mut::<InternalHookState>()
                .expect("Incompatible hook type. Hooks must always be called in the same order");

            // Need a copy that is not moved into the closure
            let persistent_hook_state_c = persistent_hook_state.clone();

            // Execute the actual hook closure we were given. Let it mutate the hook state and let
            // it create a callback that takes the mutable hook state.
            let ret = hook_runner(preexisting_hook, Rc::new(move |pretrigger_change| {
                // We are called with a closure the hook wants to execute, borrowing component
                // hook state.
                let mut borrowed_hook_state = persistent_hook_state.try_borrow_mut()
                    .expect("Could not borrow hook state. Note: you cannot nest hooks.");
                // We get the render call for later
                let render_call = borrowed_hook_state.render_call.clone();

                // We take the internal hook state out of its place
                let mut internal_hook_state = borrowed_hook_state.hooks
                    .get_mut(hook_pos)
                    .expect("Hook should have been initialized at this point")
                    .take()
                    .expect("Hook already in use elsewhere");
                std::mem::drop(borrowed_hook_state);

                // ...and cast it to the appropriate type
                let internal_hook_state_ref: &mut InternalHookState = internal_hook_state
                    .downcast_mut::<InternalHookState>()
                    .expect("Unexpected hook type change. Hooks must always be called in the same order");

                // It's time to call the callback. We remember whether it wants to rerender or not.
                let should_rerender = pretrigger_change(internal_hook_state_ref);
                // We put the internal hook state back into its place
                persistent_hook_state
                    .try_borrow_mut()
                    .expect("Hook state unexpectedly in use.")
                    .hooks
                    .get_mut(hook_pos)
                    .expect("Hook should have been initialized at this point")
                    .replace(internal_hook_state);
                // If a render was requested, we trigger it
                if should_rerender {
                    render_call();
                }
            })); // End of hook runner execution
            // We can now put the internal hook state back into place (keeping in mind we are not
            // inside the closure callback yet)
            persistent_hook_state_c.try_borrow_mut()
                .expect("Hook state already borrowed. Note that hooks cannot be nested")
                .hooks
                .get_mut(hook_pos).expect("Hook removed itself").replace(hook_state_inner);
            // Return whatever the hook runner returned
            return ret;
        } else {
            // This is the first execution
            // Get the initial state
            let mut new_state: InternalHookState = initial_state_producer();
            // We are currently using the state ourselves, so let's place a placeholder
            mut_hook_state.hooks.push(None);
            // We pushed it, now we can drop our mutable reference
            std::mem::drop(mut_hook_state);
            // Clone for later
            let persistent_hook_state_c = persistent_hook_state.clone();
            // Run the provided hook runner with the mutable internal hook state and the callback-acceptor
            let ret = hook_runner(&mut new_state, Rc::new(move |pretrigger_change| {
                // The component wishes to modify its state. First we need to get hold of the state
                let mut borrowed_hook_state = persistent_hook_state.try_borrow_mut()
                    .expect("Hook state currently borrowed. Note: you cannot nest hooks.");
                // While we're at it, get the render call
                let render_call = borrowed_hook_state.render_call.clone();
                // Take the internal hook state out of its optional
                let internal_hook_state_opt = borrowed_hook_state.hooks
                    .get_mut(hook_pos)
                    .expect("Hook should have been initialized at this point");
                let mut hook_state = internal_hook_state_opt.take()
                    .expect("Hook nested in itself (2)");
                // ...so that we can drop our reference
                std::mem::drop(borrowed_hook_state);

                // Cast the hook state
                let internal_hook_state: &mut InternalHookState = hook_state
                    .downcast_mut::<InternalHookState>()
                    .expect("Unexpected hook type change. Hooks must always be called in the same order");

                // Run the actual trigger with the state we now hold
                let ret = pretrigger_change(internal_hook_state);

                // We put the hook state back into it's original place
                let mut borrowed_hook_state = persistent_hook_state.try_borrow_mut()
                    .expect("Hook is being mutated. You cannot call hooks inside hooks");
                let internal_hook_state_opt = borrowed_hook_state.hooks.get_mut(hook_pos)
                    .expect("Hook should have been initialized at this point");
                internal_hook_state_opt.replace(hook_state);
                std::mem::drop(borrowed_hook_state);
                // If a rerender was requested, do it
                if ret {
                    render_call();
                }
            }));
            // After the initial call, we place the internal state into the spot for the first time
            persistent_hook_state_c.try_borrow_mut().expect("Hook state could not be borrowed. Note that hooks cannot be called inside hooks.")
                .hooks
                .get_mut(hook_pos)
                .expect("Double borrow of hook state")
                .replace(Box::new(new_state));
            return ret;
        }
    });
}

