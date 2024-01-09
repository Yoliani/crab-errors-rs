#[cfg(feature = "libsql_error")]
use libsql::Error as LibSqlError;
use libsql::errors::{Error};


#[allow(unused)]
use crate::CrabError;


#[cfg(feature = "libsql_error")]
impl From<LibSqlError> for CrabError {
    fn from(e: LibSqlError) -> Self {
        CrabError::DatasourceError(e.to_string())
    }
}

#[cfg(feature = "libsql_error")]
impl From<Error> for CrabError {
    fn from(e: Error) -> Self {
        CrabError::DatasourceError(e.to_string())
    }
}