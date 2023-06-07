/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

/// room order item
#[derive(Debug, Serialize)]
pub struct RoomOrderItem {
  group_id: u64,
  room_id: u64,
}

impl RoomOrderItem {
  /// initialize with group_id and room_id
  pub fn new(group_id: u64, room_id: u64) -> Self {
    Self { group_id, room_id }
  }
}
