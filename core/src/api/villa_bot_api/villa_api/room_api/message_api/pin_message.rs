/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PinMessageRequest {
  pub room_id: u64,
  pub msg_uid: String,
  pub send_at: i64,
  pub is_cancel: bool,
}

impl PinMessageRequest {
  pub fn new(room_id: u64, msg_uid: String, send_at: i64, is_cancel: bool) -> Self {
    Self {
      room_id,
      msg_uid,
      send_at,
      is_cancel,
    }
  }
}
