use yew::prelude::*;

use super::provider::OneshotProviderState;
use super::Oneshot;

/// Handle for [use_bridge_oneshot]
#[derive(Debug)]
pub struct UseBridgeOneshotHandle<T>
where
    T: Oneshot + 'static,
{
    state: OneshotProviderState<T>,
}

impl<T> UseBridgeOneshotHandle<T>
where
    T: Oneshot + 'static,
{
    /// Runs an oneshot agent.
    pub async fn run(&self, input: T::Input) -> T::Output {
        self.state.create_bridge().run(input).await
    }
}

impl<T> Clone for UseBridgeOneshotHandle<T>
where
    T: Oneshot + 'static,
{
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl<T> PartialEq for UseBridgeOneshotHandle<T>
where
    T: Oneshot,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.state == rhs.state
    }
}

/// A hook to bridge to an oneshot agent.
#[hook]
pub fn use_bridge_oneshot<T>() -> UseBridgeOneshotHandle<T>
where
    T: Oneshot + 'static,
{
    let state = use_context::<OneshotProviderState<T>>().expect("failed to find worker context");

    UseBridgeOneshotHandle { state }
}
