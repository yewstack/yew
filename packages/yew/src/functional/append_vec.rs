use std::cell::{Cell, UnsafeCell};

pub struct AppendOnlyList<T: ?Sized> {
    state: UnsafeCell<Vec<Box<T>>>,
    counter: Cell<usize>,
    #[cfg(debug_assertions)]
    total_counter: Option<usize>,
}

#[cfg(test)]
#[test]
fn assert_state_list_not_sync() {
    struct Invalid;
    trait AmbiguousIfImpl<A> {
        fn some_item() {}
    }

    impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
    impl<T: ?Sized + Sync> AmbiguousIfImpl<Invalid> for T {}
    // This will fail to compile (ambiguous impl) if T: Sync ==> StateList<T>: Sync
    // StateList<T> should never be sync!
    fn _assert_generically<T: Sync>() {
        let _ = <AppendOnlyList<T> as AmbiguousIfImpl<_>>::some_item;
    }
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
        // SAFETY: Because we are not Sync, we don't need to atomically increase here
        let position = self.counter.get();
        self.counter.set(position + 1);
        // SAFETY: we drop this ref before returning.
        //  Because we are not Sync, no other thread can have concurrent access to this.
        //  Because we assert that we don't re-enter in the initializer, this is legal
        //   to access even afterwards.
        let state = unsafe { &mut *self.state.get() };
        assert!(position <= state.len(), "Detected re-entrant initializer()");
        if position == state.len() {
            state.push(initializer());
        }
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
}

#[cfg(test)]
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
