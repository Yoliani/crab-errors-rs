#[cfg(feature = "libsql_error")]
use libsql::Error as LibSqlError;


#[allow(unused)]
use crate::AppError;


#[cfg(feature = "libsql_error")]
impl From<LibSqlError> for AppError {
    fn from(e: LibSqlError) -> Self {
        AppError::DatasourceError(e.to_string())
    }
}