use crate::fetch::FetchError;

/// Holds the state of the request being made and response
/// (if any has been made successfully at any prior point).
#[derive(Clone, Debug, PartialEq)]
pub enum FetchState<RES> {
    NotFetching(Option<RES>),
    Fetching(Option<RES>),
    Fetched(RES),
    Failed(Option<RES>, FetchError),
}

impl<RES> Default for FetchState<RES> {
    fn default() -> Self {
        FetchState::NotFetching(None)
    }
}

impl<RES> FetchState<RES> {
    /// Determines if there is a different discriminant between the fetch states.
    pub(crate) fn discriminant_differs(&self, other: &Self) -> bool {
        std::mem::discriminant(self) != std::mem::discriminant(other)
    }

    pub(crate) fn not_fetching(self) -> Self {
        match self {
            FetchState::NotFetching(res) => FetchState::NotFetching(res),
            FetchState::Fetching(res) => FetchState::NotFetching(res),
            FetchState::Fetched(res) => FetchState::NotFetching(Some(res)),
            FetchState::Failed(res, _err) => FetchState::NotFetching(res),
        }
    }

    pub(crate) fn fetching(self) -> Self {
        match self {
            FetchState::NotFetching(res) => FetchState::Fetching(res),
            FetchState::Fetching(res) => FetchState::Fetching(res),
            FetchState::Fetched(res) => FetchState::Fetching(Some(res)),
            FetchState::Failed(res, _err) => FetchState::Fetching(res),
        }
    }

    pub(crate) fn fetched(self, res: RES) -> Self {
        match self {
            FetchState::NotFetching(_res) => FetchState::Fetched(res),
            FetchState::Fetching(_res) => FetchState::Fetched(res),
            FetchState::Fetched(_res) => FetchState::Fetched(res),
            FetchState::Failed(_res, _err) => FetchState::Fetched(res),
        }
    }

    pub(crate) fn failed(self, err: FetchError) -> Self {
        match self {
            FetchState::NotFetching(res) => FetchState::Failed(res, err),
            FetchState::Fetching(res) => FetchState::Failed(res, err),
            FetchState::Fetched(res) => FetchState::Failed(Some(res), err),
            FetchState::Failed(res, _err) => FetchState::Failed(res, err),
        }
    }
}
