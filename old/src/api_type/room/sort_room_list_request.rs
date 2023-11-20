/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

use crate::api_type::room::room_order_item::RoomOrderItem;

/// sort room list request
#[derive(Debug, Serialize)]
pub struct SortRoomListRequest {
  villa_id: u64,
  room_list: Vec<RoomOrderItem>,
}

impl SortRoomListRequest {
  /// initialize with villa_id and room_list
  pub fn new(villa_id: u64, room_list: Vec<RoomOrderItem>) -> Self {
    Self {
      villa_id,
      room_list,
    }
  }
}
