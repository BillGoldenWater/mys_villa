/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api::villa_bot_api::villa_response::retcode::RetCode;
use crate::http::response::Response;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
  #[error("non ok response: {}", .0.status_code)]
  NonOkResponse(Response),
  #[error("villa api error: {retcode:?}, {message}")]
  Villa { retcode: RetCode, message: String },
  #[error("empty data from ok response: {retcode:?}, {message}")]
  VillaEmptyData { retcode: RetCode, message: String },
  #[error("failed when (de)serialization: {0}")]
  SerdeJson(#[from] serde_json::Error),
}

pub type ApiResult<T> = Result<T, ApiError>;
