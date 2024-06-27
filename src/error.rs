use localic_std::errors::LocalError;
use serde_json::Error as SerdeJsonError;
use std::io::Error as IoError;
use thiserror::Error;

/// General error during testing.
#[derive(Error, Debug)]
pub enum Error {
    #[error("local interchain failure: `{0}`")]
    LocalInterchain(#[from] LocalError),
    #[error("IO failure: `{0}`")]
    Io(#[from] IoError),
    #[error("serialization failed: `{0}`")]
    Serialization(#[from] SerdeJsonError),
    #[error("failed to query container with cmd `{0}`")]
    ContainerCmd(String),
    #[error("an unknown error occurred: `{0}`")]
    Misc(String),
    #[error("test context missing variable `{0}`")]
    MissingContextVariable(String),
}
