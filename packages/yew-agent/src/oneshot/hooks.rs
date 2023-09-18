use yew::prelude::*;

use super::provider::OneshotProviderState;
use super::Oneshot;

/// Hook handle for [`use_oneshot_runner`]
#[derive(Debug)]
pub struct UseOneshotRunnerHandle<T>
where
    T: Oneshot + 'static,
{
    state: OneshotProviderState<T>,
}

impl<T> UseOneshotRunnerHandle<T>
where
    T: Oneshot + 'static,
{
    /// Runs an oneshot agent.
    pub async fn run(&self, input: T::Input) -> T::Output {
        self.state.create_bridge().run(input).await
    }
}

impl<T> Clone for UseOneshotRunnerHandle<T>
where
    T: Oneshot + 'static,
{
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl<T> PartialEq for UseOneshotRunnerHandle<T>
where
    T: Oneshot,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.state == rhs.state
    }
}

/// A hook to create a runner to an oneshot agent.
#[hook]
pub fn use_oneshot_runner<T>() -> UseOneshotRunnerHandle<T>
where
    T: Oneshot + 'static,
{
    let state = use_context::<OneshotProviderState<T>>().expect("failed to find worker context");

    UseOneshotRunnerHandle { state }
}
