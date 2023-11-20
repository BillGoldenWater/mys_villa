/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

/// get room request
#[derive(Debug, Serialize)]
pub struct GetRoomRequest {
  room_id: u64,
}

impl GetRoomRequest {
  /// initialize with room_id
  pub fn new(room_id: u64) -> Self {
    Self { room_id }
  }
}
