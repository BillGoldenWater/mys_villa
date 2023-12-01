/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::error::Error;
use std::fmt::Debug;

use crate::api::api_error::ApiError;
use crate::bot::bot_permission::BotPermission;

#[derive(Debug, thiserror::Error)]
pub enum VError {
  #[error("permission denied, require: {0}")]
  PermissionDenied(BotPermission),

  #[error("api error: {0}")]
  Api(#[from] ApiError),
  #[error("request executor error: {0}")]
  RequestExecutor(Box<dyn Error + Send + Sync>),
  #[error("custom error: {0}")]
  Custom(#[from] Box<dyn Error + Send + Sync>),
}

pub type VResult<T> = Result<T, VError>;
