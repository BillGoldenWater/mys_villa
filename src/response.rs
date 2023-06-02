use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use retcode::RetCode;

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
  pub fn new_ok(data: Body) -> Self {
    Self {
      retcode: RetCode::Ok,
      message: "".to_string(),
      data: Some(data),
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
