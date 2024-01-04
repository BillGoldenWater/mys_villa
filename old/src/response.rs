/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use retcode::RetCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::error::{VError, VResult};

/// define return code from api
pub mod retcode;

/// response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Response<Body> {
  retcode: RetCode,
  message: String,
  data: Option<Body>,
}

impl<Body: DeserializeOwned> Response<Body> {
  /// initialize a ok response with data
  pub fn new_ok(data: Option<Body>) -> Self {
    Self {
      retcode: RetCode::Ok,
      message: "".to_string(),
      data,
    }
  }

  /// to result
  pub fn to_result(self) -> VResult<Body> {
    if matches!(self.retcode, RetCode::Ok) {
      Ok(self.data.unwrap())
    } else {
      Err(VError::ApiError {
        retcode: self.retcode,
        message: self.message,
      })
    }
  }
}

impl Response<()> {
  /// initialize a err response with retcode and message
  pub fn new_err(retcode: RetCode, message: impl Into<String>) -> Self {
    Self {
      retcode,
      message: message.into(),
      data: Option::<()>::None,
    }
  }
}
