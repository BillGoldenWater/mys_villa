/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::de::DeserializeOwned;
use tracing::trace;

use crate::{
  api::{
    api_error::{ApiError, ApiResult},
    villa_bot_api::villa_response::VillaResponse,
  },
  http::response::Response,
};

#[cfg(feature = "unstable_villa_bot_api")]
pub mod get_all_emoticons;
#[cfg(not(feature = "unstable_villa_bot_api"))]
pub(crate) mod get_all_emoticons;
#[cfg(feature = "unstable_villa_bot_api")]
pub mod villa_api;
#[cfg(not(feature = "unstable_villa_bot_api"))]
pub(crate) mod villa_api;
pub mod villa_response;

pub fn parse_villa_res<Data: DeserializeOwned>(response: Response) -> ApiResult<Data> {
  if response.status_code != 200 {
    return Err(ApiError::NonOkResponse(response));
  }

  if let Some(trace_id) = response.headers.get("x-trace-id") {
    trace!("trace_id: {trace_id}");
  }

  serde_json::from_slice::<VillaResponse<Data>>(&response.body)
    .map_err(|err| {
      trace!(
        "deserialize failed, data: {}",
        String::from_utf8_lossy(&response.body)
      );
      err
    })?
    .into_result()
}
