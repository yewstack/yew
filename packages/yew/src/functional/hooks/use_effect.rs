use std::cell::RefCell;

use crate::functional::{hook, Effect, Hook, HookContext};

/// Trait describing the destructor of [`use_effect`] hook.
pub trait TearDown: Sized + 'static {
    /// The function that is executed when destructor is called
    fn tear_down(self);
}

impl TearDown for () {
    fn tear_down(self) {}
}

impl<F: FnOnce() + 'static> TearDown for F {
    fn tear_down(self) {
        self()
    }
}

struct UseEffectHook<T, F, D: TearDown> {
    deps_and_runner: Option<(T, F)>,
    last_destructor: Option<D>,
    last_deps: Option<T>,
}

impl<T, F, D: TearDown> Drop for UseEffectHook<T, F, D> {
    fn drop(&mut self) {
        if let Some(de) = self.last_destructor.take() {
            de.tear_down()
        }
    }
}

impl<T, F, D> Effect for RefCell<UseEffectHook<T, F, D>>
where
    T: PartialEq + 'static,
    F: FnOnce(&T) -> D + 'static,
    D: TearDown,
{
    fn rendered(&self) {
        let mut this = self.borrow_mut();

        if let Some((deps, f)) = this.deps_and_runner.take() {
            if Some(&deps) == this.last_deps.as_ref() {
                return;
            }

            if let Some(de) = this.last_destructor.take() {
                de.tear_down();
            }

            this.last_destructor = Some(f(&deps));
            this.last_deps = Some(deps);
        }
    }
}

impl<T, F, D> Hook for UseEffectHook<T, F, D>
where
    T: PartialEq + 'static,
    F: FnOnce(&T) -> D + 'static,
    D: TearDown,
{
    type Output = ();

    fn run(self, ctx: &mut HookContext) -> Self::Output {
        ctx.next_effect(|_| RefCell::new(self));
    }
}

/// `use_effect` is used for hooking into the component's lifecycle and creating side effects.
///
/// The callback is called every time after the component's render has finished.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// # use std::rc::Rc;
///
/// #[function_component(UseEffect)]
/// fn effect() -> Html {
///     let counter = use_state(|| 0);
///
///     let counter_one = counter.clone();
///     use_effect(move || {
///         // Make a call to DOM API after component is rendered
///         gloo::utils::document().set_title(&format!("You clicked {} times", *counter_one));
///
///         // Perform the cleanup
///         || gloo::utils::document().set_title(&format!("You clicked 0 times"))
///     });
///
///     let onclick = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.set(*counter + 1))
///     };
///
///     html! {
///         <button {onclick}>{ format!("Increment to {}", *counter) }</button>
///     }
/// }
/// ```
///
/// # Destructor
///
/// Any type implementing [`TearDown`] can be used as destructor, which is called when the component
/// is re-rendered
///
/// ## Tip
///
/// The callback can return [`()`] if there is no destructor to run.
#[hook]
pub fn use_effect<F, D>(f: F)
where
    F: FnOnce() -> D + 'static,
    D: TearDown,
{
    struct NeverEq;
    impl PartialEq for NeverEq {
        fn eq(&self, _other: &Self) -> bool {
            false
        }
    }

    // Never equals, so this will be called every render
    use_effect_with(NeverEq, |_| f());
}

/// This hook is similar to [`use_effect`] but it accepts dependencies.
///
/// Whenever the dependencies are changed, the effect callback is called again.
/// To detect changes, dependencies must implement [`PartialEq`].
///
/// # Note
/// The destructor also runs when dependencies change.
///
/// # Example
///
/// ```rust
/// use yew::{function_component, html, use_effect_with, Html, Properties};
/// # use gloo::console::log;
///
/// #[derive(Properties, PartialEq)]
/// pub struct Props {
///     pub is_loading: bool,
/// }
///
/// #[function_component]
/// fn HelloWorld(props: &Props) -> Html {
///     let is_loading = props.is_loading.clone();
///
///     use_effect_with(is_loading, move |_| {
///         log!(" Is loading prop changed!");
///     });
///
///     html! {
///         <>{"Am I loading? - "}{is_loading}</>
///     }
/// }
/// ```
///
/// # Tips
///
/// ## Only on first render
///
/// Provide a empty tuple `()` as dependencies when you need to do something only on the first
/// render of a component.
///
/// ```rust
/// use yew::{function_component, html, use_effect_with, Html};
/// # use gloo::console::log;
///
/// #[function_component]
/// fn HelloWorld() -> Html {
///     use_effect_with((), move |_| {
///         log!("I got rendered, yay!");
///     });
///
///     html! { "Hello" }
/// }
/// ```
///
/// ## On destructing or last render
///
/// Use [Only on first render](#only-on-first-render) but put the code in the cleanup function.
/// It will only get called when the component is removed from view / gets destroyed.
///
/// ```rust
/// use yew::{function_component, html, use_effect_with, Html};
/// # use gloo::console::log;
///
/// #[function_component]
/// fn HelloWorld() -> Html {
///     use_effect_with((), move |_| {
///         || {
///             log!("Noo dont kill me, ahhh!");
///         }
///     });
///
///     html! { "Hello" }
/// }
/// ```
///
/// Any type implementing [`TearDown`] can be used as destructor
///
/// ### Tip
///
/// The callback can return [`()`] if there is no destructor to run.
pub fn use_effect_with<T, F, D>(deps: T, f: F) -> impl Hook<Output = ()>
where
    T: PartialEq + 'static,
    F: FnOnce(&T) -> D + 'static,
    D: TearDown,
{
    UseEffectHook {
        deps_and_runner: Some((deps, f)),
        last_destructor: None,
        last_deps: None,
    }
}

/// use effect with deps
#[hook]
#[deprecated = "use `use_effect_with` instead"]
pub fn use_effect_with_deps<T, F, D>(f: F, deps: T)
where
    T: PartialEq + 'static,
    F: FnOnce(&T) -> D + 'static,
    D: TearDown,
{
    use_effect_with(deps, f)
}
