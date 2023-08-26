use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use yew::Reducible;

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
