/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use crate::api::{
  api_error::{ApiError, ApiResult},
  villa_bot_api::villa_response::retcode::RetCode,
};

pub mod retcode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VillaResponse<Data> {
  retcode: RetCode,
  message: String,
  data: Option<Data>,
}

impl<Data> VillaResponse<Data> {
  pub fn ok_with(data: impl Into<Option<Data>>) -> Self {
    Self {
      retcode: RetCode::Ok,
      message: "".to_string(),
      data: data.into(),
    }
  }

  pub fn into_result(self) -> ApiResult<Data> {
    if matches!(self.retcode, RetCode::Ok) {
      if let Some(data) = self.data {
        Ok(data)
      } else {
        Err(ApiError::VillaEmptyData {
          retcode: self.retcode,
          message: self.message,
        })
      }
    } else {
      Err(ApiError::Villa {
        retcode: self.retcode,
        message: self.message,
      })
    }
  }
}
