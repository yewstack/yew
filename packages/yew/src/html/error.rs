use thiserror::Error;

/// Render Error.
#[derive(Error, Debug, Clone)]
pub enum RenderError {
    /// Component Rendering Suspended
    #[error("component rendering is suspended.")]
    Suspended,
}

/// Render Result.
pub type RenderResult<T> = std::result::Result<T, RenderError>;
