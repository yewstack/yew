use thiserror::Error;

use crate::suspense::Suspension;

/// Render Error.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum RenderError {
    /// Component Rendering Suspended
    #[error("component rendering is suspended.")]
    Suspended(#[from] Suspension),
}

/// Render Result.
pub type RenderResult<T> = std::result::Result<T, RenderError>;
