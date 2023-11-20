/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::room::room_data::RoomData;

/// get room response
#[derive(Debug, Deserialize)]
pub struct GetRoomResponse {
  /// room data
  pub room: RoomData,
}
