//! Error types with sanity.

use thiserror::Error;

/// A generic error type for all `arfur`-related errors. Implements `From<E>` where `E` is an error type from this crate.
#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown")]
    Unknown,
}

/// A wrapper around [`std::result::Result`] that uses [`Error`] as the error type.
pub type Result<T> = std::result::Result<T, Error>;