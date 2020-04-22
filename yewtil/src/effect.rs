use std::rc::Rc;
use yew::ShouldRender;

/// Alternative to using Message enums.
///
/// Using Effects instead of Messages allows you to define the mutation to your component's state
/// from inside `html!` macros instead of from within update functions.
pub struct Effect<COMP>(Box<dyn Fn(&mut COMP) -> ShouldRender>);

impl<COMP> Default for Effect<COMP> {
    fn default() -> Self {
        Effect::new(|_| false)
    }
}

impl<COMP> Effect<COMP> {
    /// Wraps a function in an Effect wrapper.
    pub fn new(f: impl Fn(&mut COMP) -> ShouldRender + 'static) -> Self {
        Effect(Box::new(f))
    }

    /// Runs the effect, causing a mutation to the component state.
    pub fn call(self, component: &mut COMP) -> ShouldRender {
        (self.0)(component)
    }
}

/// Terser wrapper function to be used instead of `Effect::new()`.
pub fn effect<COMP>(f: impl Fn(&mut COMP) -> ShouldRender + 'static) -> Effect<COMP> {
    Effect::new(f)
}

#[allow(dead_code)]
mod wip {
    use super::*;

    // TODO, once function_traits stabalize, this `to_effect()` will be able to be replaced with just a () call, making this more ergonomic.
    // https://github.com/rust-lang/rust/issues/29625
    // TODO, change naming of this.

    // TODO. Consider an arbitrary state holder: Hashmap<&str, Box<dyn Any + 'static>. It might be possible to write a function that returns a &T, and a StateHook<COMP, T>
    // TODO for any given T that could be inserted into the holder. This might work well if the state holder itself is a component.
    // Actually, as long as order is preserved, a Vec<Box<dyn Any + 'static>>, might work just as well.
    // This would replicate the useState hook in react https://reactjs.org/docs/hooks-state.html

    /// Wrapper around a mutable accessor to one of the component's (or another construct capabale of storing state's) fields.
    pub struct StateHook<STORE, T>(Rc<dyn Fn(&mut STORE) -> &mut T>);

    impl<STORE: 'static, T: 'static> StateHook<STORE, T> {
        /// Creates a new state hook.
        pub fn new(mutable_accessor: impl Fn(&mut STORE) -> &mut T + 'static) -> Self {
            StateHook(Rc::new(mutable_accessor))
        }

        /// Creates an effect using the wrapped accessor and a mutation function for the `T`.
        pub fn to_effect(&self, f: impl Fn(&mut T) -> ShouldRender + 'static) -> Effect<STORE> {
            let mutable_accessor = self.0.clone();
            Effect::new(move |comp| {
                let t = (mutable_accessor)(comp);
                f(t)
            })
        }
    }
}
