use crate::fetch::FetchError;
use crate::NeqAssign;

/// Represents a state change to Fetch wrapper.
#[derive(Clone, PartialEq, Debug)]
pub enum FetchAction<T> {
    NotFetching,
    Fetching,
    Fetched(T),
    Failed(FetchError),
}

impl<T> Default for FetchAction<T> {
    fn default() -> Self {
        FetchAction::NotFetching
    }
}

impl<T> FetchAction<T> {
    /// Returns a reference to the Success case
    pub fn success(&self) -> Option<&T> {
        match self {
            FetchAction::Fetched(value) => Some(value),
            _ => None,
        }
    }

    /// Gets the value out of the fetch state if it is a `Success` variant.
    pub fn unwrap(self) -> T {
        if let FetchAction::Fetched(value) = self {
            value
        } else {
            panic!("Could not unwrap value of FetchState");
        }
    }

    /// Transforms the FetchState into another FetchState using the given function.
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> FetchAction<U> {
        match self {
            FetchAction::NotFetching => FetchAction::NotFetching,
            FetchAction::Fetching => FetchAction::NotFetching,
            FetchAction::Fetched(t) => FetchAction::Fetched(f(t)),
            FetchAction::Failed(e) => FetchAction::Failed(e),
        }
    }

    /// Applies a function that mutates the response if the Action is the success case.
    pub fn alter<F: Fn(&mut T)>(&mut self, f: F) {
        if let FetchAction::Fetched(t) = self {
            f(t)
        }
    }

    /// Converts the FetchAction to contain a reference to the success case.
    pub fn as_ref(&self) -> FetchAction<&T> {
        match self {
            FetchAction::NotFetching => FetchAction::NotFetching,
            FetchAction::Fetching => FetchAction::NotFetching,
            FetchAction::Fetched(t) => FetchAction::Fetched(t),
            FetchAction::Failed(e) => FetchAction::Failed(e.clone()),
        }
    }
}

impl<T: PartialEq> FetchAction<T> {
    /// Sets the fetch state to be fetching.
    /// If it wasn't already in a fetch state, it will return `true`,
    /// to indicate that the component should re-render.
    pub fn set_fetching(&mut self) -> bool {
        self.neq_assign(FetchAction::Fetching)
    }
}
