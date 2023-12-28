/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use crate::api::villa_bot_api::villa_api::room::Room;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetRoomRequest {
  pub room_id: u64,
}

impl GetRoomRequest {
  pub fn new(room_id: u64) -> Self {
    Self { room_id }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetRoomResponse {
  pub room: Room,
}
