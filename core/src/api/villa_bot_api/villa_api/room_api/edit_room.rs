/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct EditRoomRequest {
  pub room_id: u64,
  pub room_name: String,
}

impl EditRoomRequest {
  pub fn new(room_id: u64, room_name: String) -> Self {
    Self { room_id, room_name }
  }
}
