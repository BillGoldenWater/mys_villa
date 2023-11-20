/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

/// transfer image request
#[derive(Debug, Serialize)]
pub struct TransferImageRequest {
  url: String,
}

impl TransferImageRequest {
  /// initialize with url
  pub fn new(url: impl Into<String>) -> Self {
    Self { url: url.into() }
  }
}
