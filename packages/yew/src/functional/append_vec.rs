use std::cell::{Cell, UnsafeCell};

/// A list of items (effects or states) that can be accessed in a serial function component.
///
/// The speciality is that it provides *mutable* access to the stored items, one after the other.
/// If there are not enough items, a new one is create from a given initializer.
///
/// The list then allows to restart this dealing process, for the next view function run.
pub struct AppendOnlyList<T: ?Sized> {
    state: UnsafeCell<Vec<Box<T>>>,
    /// SAFETY: the items [0, counter) of the state have been given out as mutable borrows.
    ///  The items [counter, ..) are not yet claimed.
    counter: Cell<usize>,
    #[cfg(debug_assertions)]
    total_counter: Option<usize>,
}

impl<T: ?Sized> AppendOnlyList<T> {
    pub fn new() -> Self {
        Self {
            state: UnsafeCell::default(),
            counter: Cell::new(0),
            #[cfg(debug_assertions)]
            total_counter: None,
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn next_state(&self, initializer: impl FnOnce() -> Box<T>) -> &mut T {
        // SAFETY: there should be test-cases run under miri that verify these claims.
        // SAFETY: Because we are not Sync, we don't need to atomically increase here
        let position = self.counter.get();
        self.counter.set(position + 1);
        // SAFETY: we drop this ref before returning.
        //  Because we are not Sync, no other thread can have concurrent access to this.
        //  Because we assert that we don't re-enter in the initializer, this is legal
        //   to access even afterwards.
        let state = unsafe { &mut *self.state.get() };
        // The exact rules are a bit murky to me. Technically a re-rentrant call takes
        // a second mutable borrow to state, but then only uses it immutably before running
        // into this assert. If this panic is caught and ignored, and control returns
        // to the first call, the mutable borrow *might* technically have to be renewed.
        assert!(position <= state.len(), "Detected re-entrant initializer()");
        // This hook/state is new. Run the initializer and push it. As explained above,
        // during the initializer nobody else can modify state.
        if position == state.len() {
            // SAFETY: this push can lead to the vec re-allocating. This is why we store
            //  Boxes to the state. The Boxes get moved, but their contents does not, so
            //  the &mut we gave out to earlier items are still okay.
            // NOTE: Box is a bit special with uniqueness handling. We might have to store
            //  the Box::into_raw form in the vec and convert it back so that no Boxes
            //  are mutably touched during re-allocations. This would warrant a custom Drop
            //  impl and complicate the situation.
            state.push(initializer());
        }
        // Finally access the item
        state[position].as_mut()
    }

    pub fn restart(&mut self) {
        // SAFETY: Because we have exclusive access to self, there can't be any surviving borrows to states
        //  This makes it safe to restart and serve states from the front again.
        self.counter.set(0);
    }

    pub fn get_mut(&mut self) -> &mut Vec<Box<T>> {
        self.state.get_mut()
    }

    /// asserts hook counter.
    ///
    /// This function asserts that the number of hooks matches for every render.
    #[cfg(debug_assertions)]
    pub fn assert_hook_context(&mut self, render_ok: bool) {
        // Procedural Macros can catch most conditionally called hooks at compile time, but it cannot
        // detect early return (as the return can be Err(_), Suspension).
        let counter = self.counter.get();
        match (render_ok, self.total_counter) {
            // First rendered,
            // we store the hook counter.
            (true, None) => {
                self.total_counter = Some(counter);
            }
            // Component is suspended before it's first rendered.
            // We don't have a total count to compare with.
            (false, None) => {}

            // Subsequent render,
            // we compare stored total count and current render count.
            (true, Some(total_counter)) => {
                assert_eq!(total_counter, counter, "Hooks are called conditionally.")
            }

            // Subsequent suspension,
            // components can have less hooks called when suspended, but not more.
            (false, Some(total_counter)) => {
                assert!(counter <= total_counter, "Hooks are called conditionally.")
            }
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn assert_hook_context(&mut self, _render_ok: bool) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    /// If it were sync, we could have multiple threads accessing it, which is not safe
    /// The position is kept in a simple unsync Cell.
    #[test]
    fn assert_append_only_list_not_sync() {
        /// Idea taken from the static_assertions crate
        /// <U as AmbiguousIfSync<_>>::some_item is ambiguous if and only if U: Sync
        /// ```compile_fail
        /// let _ = <u32 as AmbiguousIfSync<_>>::some_item;
        /// ```
        trait AmbiguousIfSync<A> {
            fn some_item() {}
        }
        impl<T: ?Sized> AmbiguousIfSync<()> for T {}
        struct Overload;
        impl<T: ?Sized + Sync> AmbiguousIfSync<Overload> for T {}

        let _ = <AppendOnlyList<u32> as AmbiguousIfSync<_>>::some_item;
        // This will fail to compile (ambiguous impl) if T: Sync ==> StateList<T>: Sync
        // StateList<T> should never be sync!
        fn _assert_generically<T: Sync>() {
            let _ = <AppendOnlyList<T> as AmbiguousIfSync<_>>::some_item;
            let _ = <AppendOnlyList<Box<T>> as AmbiguousIfSync<_>>::some_item;
        }
    }

    #[test]
    fn append_only_list_works() {
        // DO run this with miri enabled!
        let mut list = AppendOnlyList::new();
        {
            list.next_state(|| Box::new(0));
            list.next_state(|| Box::new(42));
        }
        list.assert_hook_context(false); // suspending
        list.restart();
        {
            let fst = list.next_state(|| Box::new(0xbad));
            let snd = list.next_state(|| Box::new(0xbad));
            let thrd = list.next_state(|| {
                //list.next_state(|| Box::new(0xdead)); // Illegal, safely panics
                Box::new(0xbeaf)
            });
            assert_eq!(*fst, 0);
            assert_eq!(*snd, 42);
            assert_eq!(*thrd, 0xbeaf);
            *fst = 101;
            *snd = 202;
            *thrd = 303;
        }
        list.assert_hook_context(true); // rendering okay
        list.restart();
        {
            list.next_state(|| Box::new(0xbad));
            list.next_state(|| Box::new(0xbad));
            list.next_state(|| Box::new(0xbeaf));
        }
        list.assert_hook_context(true); // assert same count of hooks
        let contents = list.get_mut().iter().map(|bi| **bi).collect::<Vec<_>>();
        assert_eq!(contents, vec![101, 202, 303]);
    }
}
