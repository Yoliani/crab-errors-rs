#[cfg(feature = "async_graphql")]
use async_graphql::{ErrorExtensionValues, ErrorExtensions, FieldError};

#[allow(unused)]
use crate::{AppError, AppErrorRetry};

#[cfg(feature = "async_graphql")]
pub fn create_extension(
    e: &mut ErrorExtensionValues,
    reason: &String,
    code: &String,
    level: AppErrorRetry,
) -> () {
    e.set("reason", reason.clone());
    e.set("code", code.clone());
    e.set("level", level.to_string());
}

#[cfg(feature = "async_graphql")]
impl ErrorExtensions for AppError {
    // lets define our base extensions
    fn extend(&self) -> FieldError {
        self.extend_with(|_, e| match self {
            AppError::NotFound => create_extension(
                e,
                &"Could not find resource".to_string(),
                &"NOT_FOUND".to_string(),
                AppErrorRetry::None,
            ),
            AppError::ServerError(reason) => create_extension(
                e,
                &reason,
                &"SERVER_ERROR".to_string(),
                AppErrorRetry::Cancel,
            ),
            AppError::DatasourceError(reason) => create_extension(
                e,
                &reason,
                &"DATA_SOURCE_ERROR".to_string(),
                AppErrorRetry::WaitAndRetry,
            ),
            AppError::ValidationError { reason, code } => {
                create_extension(e, &reason, &code, AppErrorRetry::None)
            }
            AppError::MaxFileSizeError(reason) => create_extension(
                e,
                &reason,
                &"MAX_FILE_SIZE_ERROR".to_string(),
                AppErrorRetry::Cancel,
            ),
            AppError::ContentTypeError(reason) => create_extension(
                e,
                &reason,
                &"CONTENT_TYPE_ERROR".to_string(),
                AppErrorRetry::Cancel,
            ),
            AppError::Anyhow(error) => create_extension(
                e,
                &error.to_string(),
                &"SERVER_ERROR".to_string(),
                AppErrorRetry::Cancel,
            ),
            AppError::ErrorWithoutExtensions => {}
            AppError::Unauthorized => create_extension(
                e,
                &"UNAUTHORIZED".to_string(),
                &"UNAUTHORIZED".to_string(),
                AppErrorRetry::Cancel,
            ),
            AppError::Forbidden => create_extension(
                e,
                &"FORBIDDEN".to_string(),
                &"FORBIDDEN".to_string(),
                AppErrorRetry::Cancel,
            ),
        })
    }
}
