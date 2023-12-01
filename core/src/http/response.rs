/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::http::header_map::HeaderMap;
use crate::http::response::status_code::StatusCode;

pub mod status_code;

#[derive(Debug, Clone)]
pub struct Response {
  pub status_code: StatusCode,
  pub headers: HeaderMap,
  pub body: Vec<u8>,
}
