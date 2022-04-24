use crate::functional::PreparedState;

#[cfg(feature = "ssr")]
use std::future::Future;
#[cfg(feature = "ssr")]
use std::pin::Pin;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(feature = "hydration")]
mod feat_hydration;
#[cfg(all(feature = "hydration", feature = "ssr"))]
mod feat_hydration_ssr;
#[cfg(not(any(feature = "hydration", feature = "ssr")))]
mod feat_none;
#[cfg(feature = "ssr")]
mod feat_ssr;

#[cfg(all(feature = "hydration", not(feature = "ssr")))]
pub use feat_hydration::*;
#[cfg(all(feature = "ssr", feature = "hydration"))]
pub use feat_hydration_ssr::*;
#[cfg(not(any(feature = "hydration", feature = "ssr")))]
pub use feat_none::*;
#[cfg(all(feature = "ssr", not(feature = "hydration")))]
pub use feat_ssr::*;

struct PreparedStateBase<T, D>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
{
    state: Option<Rc<T>>,
    #[allow(dead_code)]
    deps: Option<Rc<D>>,
}

#[cfg(feature = "hydration")]
impl<T, D> PreparedStateBase<T, D>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
{
    fn decode(buf: &[u8]) -> Self {
        let (state, deps) =
            bincode::deserialize::<(T, D)>(buf).expect("failed to deserialize state");

        PreparedStateBase {
            state: Some(Rc::new(state)),
            deps: Some(Rc::new(deps)),
        }
    }
}

impl<T, D> PreparedState for PreparedStateBase<T, D>
where
    D: Serialize + DeserializeOwned + PartialEq + 'static,
    T: Serialize + DeserializeOwned + 'static,
{
    #[cfg(feature = "ssr")]
    fn prepare(&self) -> Pin<Box<dyn Future<Output = Vec<u8>>>> {
        let state = bincode::serialize(&(self.state.as_deref(), self.deps.as_deref()))
            .expect("failed to prepare state");

        Box::pin(async move { state })
    }
}
