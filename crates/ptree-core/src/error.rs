use std::io;

use thiserror::Error;

// TODO: [ref] : Within the crate
// it's better to use Error and Result,
// and on export you `use crate::error::{Error as PTreeError, Result as PTreeResult};`
// This keeps the error handle _CODE_ consistent across the crate, and only renames it for external
// use, which is more intuitive for users and developers of the crate.

pub type PTreeResult<T> = std::result::Result<T, PTreeError>;

#[derive(Error, Debug)]
pub enum PTreeError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),

    #[error("Invalid drive: {0}")]
    InvalidDrive(String),

    #[error("Lock timeout: {0}")]
    LockTimeout(String),

    #[error("Traversal error: {0}")]
    Traversal(String),
    // // NOTE: Then we implement
    // // From<ChildCrateError> for MainLibraryError, we can use the `?` operator to automatically
    // // convert errors from the child crate into our main library error type.
    // // This allows us to handle errors from the child crate seamlessly within our main library,
    // // without having to manually convert them each time (or use .map_err() everywhere).
    // #[error("CLI error: {0}")]
    // Cli(#[from] PtreeCliError),
}
