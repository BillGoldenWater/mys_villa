/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("invalid header name {0}")]
  InvalidHeaderName(#[from] InvalidHeaderName),
  #[error("invalid header value {0}")]
  InvalidHeaderValue(#[from] InvalidHeaderValue),

  #[error("reqwest err: {0}")]
  Reqwest(#[from] reqwest::Error),
}
