#[cfg(feature = "async_graphql")]
use async_graphql::{ErrorExtensionValues, ErrorExtensions, FieldError};

#[allow(unused)]
use crate::{CrabError, CrabErrorRetry};

#[cfg(feature = "async_graphql")]
pub fn create_extension(
    e: &mut ErrorExtensionValues,
    reason: &String,
    code: &String,
    level: CrabErrorRetry,
) -> () {
    e.set("reason", reason.clone());
    e.set("code", code.clone());
    e.set("level", level.to_string());
}

#[cfg(feature = "async_graphql")]
impl ErrorExtensions for CrabError {
    // lets define our base extensions
    fn extend(&self) -> FieldError {
        self.extend_with(|_, e| match self {
            CrabError::NotFound => create_extension(
                e,
                &"Could not find resource".to_string(),
                &"NOT_FOUND".to_string(),
                CrabErrorRetry::None,
            ),
            CrabError::ServerError(reason) => create_extension(
                e,
                &reason,
                &"SERVER_ERROR".to_string(),
                CrabErrorRetry::Cancel,
            ),
            CrabError::DatasourceError(reason) => create_extension(
                e,
                &reason,
                &"DATA_SOURCE_ERROR".to_string(),
                CrabErrorRetry::WaitAndRetry,
            ),
            CrabError::ValidationError { reason, code } => {
                create_extension(e, &reason, &code, CrabErrorRetry::None)
            }
            CrabError::MaxFileSizeError(reason) => create_extension(
                e,
                &reason,
                &"MAX_FILE_SIZE_ERROR".to_string(),
                CrabErrorRetry::Cancel,
            ),
            CrabError::ContentTypeError(reason) => create_extension(
                e,
                &reason,
                &"CONTENT_TYPE_ERROR".to_string(),
                CrabErrorRetry::Cancel,
            ),
            CrabError::Anyhow(error) => create_extension(
                e,
                &error.to_string(),
                &"SERVER_ERROR".to_string(),
                CrabErrorRetry::Cancel,
            ),
            CrabError::ErrorWithoutExtensions => {}
            CrabError::Unauthorized => create_extension(
                e,
                &"UNAUTHORIZED".to_string(),
                &"UNAUTHORIZED".to_string(),
                CrabErrorRetry::Cancel,
            ),
            CrabError::Forbidden => create_extension(
                e,
                &"FORBIDDEN".to_string(),
                &"FORBIDDEN".to_string(),
                CrabErrorRetry::Cancel,
            ),
        })
    }
}
