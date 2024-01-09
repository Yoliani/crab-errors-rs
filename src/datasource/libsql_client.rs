#[cfg(feature = "libsql_error")]
use libsql::errors::{Error};

#[allow(unused)]
use crate::CrabError;

#[cfg(feature = "libsql_error")]
impl From<Error> for CrabError {
    fn from(e: Error) -> Self {
        CrabError::DatasourceError(e.to_string())
    }
}