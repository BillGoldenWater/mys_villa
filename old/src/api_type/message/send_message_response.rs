/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

/// send message response
#[derive(Debug, Deserialize)]
pub struct SendMessageResponse {
  /// bot msg id
  pub bot_msg_id: String,
}
