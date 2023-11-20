/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

/// transfer image response
#[derive(Debug, Deserialize)]
pub struct TransferImageResponse {
  /// new url
  pub new_url: String,
}
