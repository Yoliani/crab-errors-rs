#[cfg(feature = "libsql_error")]
use libsql::Error as LibSqlError;


#[allow(unused)]
use crate::CrabError;


#[cfg(feature = "libsql_error")]
impl From<LibSqlError> for CrabError {
    fn from(e: LibSqlError) -> Self {
        CrabError::DatasourceError(e.to_string())
    }
}