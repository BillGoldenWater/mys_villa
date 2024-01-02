/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RecallMessageRequest {
  pub room_id: u64,
  pub msg_uid: String,
  pub msg_time: i64,
}

impl RecallMessageRequest {
  pub fn new(room_id: u64, msg_uid: String, msg_time: i64) -> Self {
    Self {
      room_id,
      msg_uid,
      msg_time,
    }
  }
}
