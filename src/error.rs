/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::env::VarError;

use crate::bot::bot_permission::BotPermission;
use crate::bot::command::CommandTryFromEventError;
use crate::bot::villa::room::message::message_chain::MessageChainParseError;
use crate::bot::villa::room::message::message_chain_matcher::mhy_text_matcher::MhyTextMatchError;
use crate::response::retcode::RetCode;

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
  /// request finished with a non OK status code
  #[error("request finished with a non OK status code: {0}")]
  RequestNonOk(u16),
  /// bot doesn't have required permission
  #[error("permission denied: this operation require permission {0:?}")]
  PermissionDenied(BotPermission),
  /// failed to parse into command
  #[error("failed to parse into command {0:?}")]
  CommandParseFailed(#[from] CommandTryFromEventError),
  /// failed to parse message object into message chain
  #[error("failed to parse message object into message chain {0:?}")]
  MessageChainParse(#[from] MessageChainParseError),
  /// failed to match mhy text
  #[error("match failed {0:?}")]
  MhyTextMatch(#[from] MhyTextMatchError),
  /// error from serde_json
  #[error("failed to (de)serialize data: {0}")]
  SerdeJsonError(#[from] serde_json::Error),
  /// other error
  #[error("{0}")]
  Other(String),
}

/// result type with error type predefined
pub type VResult<T> = Result<T, VError>;
