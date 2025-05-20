use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wasm_bindgen::UnwrapThrowExt;
use yew::Reducible;

/// Convenience function to avoid repeating expect logic.
pub fn window() -> web_sys::Window {
    web_sys::window().expect_throw("Can't find the global Window")
}

/// Gets a unique worker id
pub(crate) fn get_next_id() -> usize {
    static CTR: AtomicUsize = AtomicUsize::new(0);

    CTR.fetch_add(1, Ordering::SeqCst)
}

#[derive(Default, PartialEq)]
pub(crate) struct BridgeIdState {
    pub inner: usize,
}

impl Reducible for BridgeIdState {
    type Action = ();

    fn reduce(self: Rc<Self>, _: Self::Action) -> Rc<Self> {
        Self {
            inner: self.inner + 1,
        }
        .into()
    }
}

pub(crate) enum OutputsAction<T> {
    Push(Rc<T>),
    Close,
    Reset,
}

pub(crate) struct OutputsState<T> {
    pub ctr: usize,
    pub inner: Vec<Rc<T>>,
    pub closed: bool,
}

impl<T> Clone for OutputsState<T> {
    fn clone(&self) -> Self {
        Self {
            ctr: self.ctr,
            inner: self.inner.clone(),
            closed: self.closed,
        }
    }
}

impl<T> Reducible for OutputsState<T> {
    type Action = OutputsAction<T>;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        {
            let this = Rc::make_mut(&mut self);
            this.ctr += 1;

            match action {
                OutputsAction::Push(m) => this.inner.push(m),
                OutputsAction::Close => {
                    this.closed = true;
                }
                OutputsAction::Reset => {
                    this.closed = false;
                    this.inner = Vec::new();
                }
            }
        }

        self
    }
}

impl<T> Default for OutputsState<T> {
    fn default() -> Self {
        Self {
            ctr: 0,
            inner: Vec::new(),
            closed: false,
        }
    }
}
