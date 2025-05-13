use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};

/// Identifier to send output to bridges.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub struct HandlerId(usize);

impl HandlerId {
    pub(crate) fn new() -> Self {
        static CTR: AtomicUsize = AtomicUsize::new(0);

        let id = CTR.fetch_add(1, Ordering::SeqCst);

        HandlerId(id)
    }
}
