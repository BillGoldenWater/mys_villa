use crate::bot::bot_permission::BotPermission;
use crate::response::retcode::RetCode;
use std::env::VarError;

/// defines the errors that this lib can generate
#[derive(Debug, thiserror::Error)]
pub enum VError {
  /// failed to load environment variable
  #[error("failed to load environment variable")]
  EnvVarErr(#[from] VarError),
  /// error from api
  #[error("api returned an error, code: {retcode:?}, message: {message}")]
  ApiError {
    /// retcode
    retcode: RetCode,
    /// message
    message: String,
  },
  /// bot doesn't have required permission
  #[error("permission denied: this operation require permission {0:?}")]
  PermissionDenied(BotPermission),
  /// error from serde_json
  #[error("failed to (de)serialize data: {0}")]
  SerdeJsonError(#[from] serde_json::Error),
  /// other error
  #[error("{0}")]
  Other(String),
}

/// result type with error type predefined
pub type VResult<T> = Result<T, VError>;
