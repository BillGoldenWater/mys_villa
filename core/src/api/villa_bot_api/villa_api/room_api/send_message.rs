/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SendMessageRequest {
  pub room_id: u64,
  pub object_name: String,
  pub msg_content: String,
}

impl SendMessageRequest {
  pub fn new(room_id: u64, object_name: String, msg_content: String) -> Self {
    Self {
      room_id,
      object_name,
      msg_content,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SendMessageResponse {
  pub bot_msg_id: String,
}
