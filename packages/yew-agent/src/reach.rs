/// The reachability of an agent.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Reach {
    /// Public Reachability.
    Public,
    /// Private Reachability.
    Private,
}
