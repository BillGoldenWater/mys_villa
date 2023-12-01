/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TransferImageRequest {
  pub url: String,
}

impl TransferImageRequest {
  pub fn new(url: String) -> Self {
    Self { url }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TransferImageResponse {
  pub new_url: String,
}
