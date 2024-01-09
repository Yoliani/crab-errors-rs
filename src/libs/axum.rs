#[cfg(feature = "axum_error")]
use axum::{Error, BoxError};

#[allow(unused)]
use crate::CrabError;

#[cfg(feature = "axum_error")]
impl From<Error> for CrabError {
    fn from(e: Error) -> Self {
        CrabError::ServerError(e.to_string())
    }
}

#[cfg(feature = "axum_error")]
impl From<BoxError> for CrabError {
    fn from(e: BoxError) -> Self {
        CrabError::ServerError(e.to_string())
    }
}
