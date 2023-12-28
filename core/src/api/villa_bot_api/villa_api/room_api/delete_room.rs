/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DeleteRoomRequest {
  pub room_id: u64,
}

impl DeleteRoomRequest {
  pub fn new(room_id: u64) -> Self {
    Self { room_id }
  }
}
