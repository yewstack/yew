use exitcode;
use thiserror::Error;

use std::convert::From;

#[derive(Debug, Error)]
pub enum RunError {
    #[error("cannot run multiple projects at once")]
    SpawnServerError,

    #[error("{0}")]
    BuildError(BuildError),
}

impl From<RunError> for i32 {
    fn from(error: RunError) -> i32 {
        match error {
            RunError::SpawnServerError => exitcode::USAGE,
            RunError::BuildError(e) => e.into(),
        }
    }
}

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("{0} doesn't have a Cargo.toml file")]
    NoCargoToml(String),

    #[error("invalid scheme: {0}")]
    InvalidScheme(String),

    #[error("build failed with error code: {0}")]
    BuildExitCode(i32)
}

impl From<BuildError> for i32 {
    fn from(error: BuildError) -> i32 {
        match error {
            BuildError::NoCargoToml(_) => exitcode::NOINPUT,
            BuildError::InvalidScheme(_) => exitcode::USAGE,
            BuildError::BuildExitCode(_) => exitcode::DATAERR,
        }
    }
}

#[derive(Debug, Error)]
pub enum SubcommandError {
    #[error("{0}")]
    RunError(RunError),

    #[error("{0}")]
    BuildError(BuildError),
}

impl From<SubcommandError> for i32 {
    fn from(error: SubcommandError) -> i32 {
        error.into()
    }
}
