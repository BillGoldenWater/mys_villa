/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::room::room_group_info::RoomGroupInfo;

/// get villa group room list response
#[derive(Debug, Deserialize)]
pub struct GetVillaGroupRoomListResponse {
  /// room group info list
  pub list: Vec<RoomGroupInfo>,
}
