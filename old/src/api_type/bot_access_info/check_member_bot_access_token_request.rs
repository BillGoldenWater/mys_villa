/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

/// check member bot access token request
#[derive(Debug, Serialize)]
pub struct CheckMemberBotAccessTokenRequest {
  /// bot access token
  pub token: String,
}

impl CheckMemberBotAccessTokenRequest {
  /// initialize with token
  pub fn new(token: String) -> Self {
    Self { token }
  }
}
