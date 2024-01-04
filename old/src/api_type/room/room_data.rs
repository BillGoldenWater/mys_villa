/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::room::{
  room_allow_send_range::RoomAllowSendRange, room_default_notify_type::RoomDefaultNotifyType,
  room_info::RoomInfo,
};

/// room data
#[derive(Debug, Deserialize)]
pub struct RoomData {
  /// room info
  #[serde(flatten)]
  pub info: RoomInfo,
  /// default notify type
  #[serde(rename = "room_default_notify_type")]
  pub default_notify_type: RoomDefaultNotifyType,
  /// allow send range
  #[serde(rename = "send_msg_auth_range")]
  pub allow_send_range: RoomAllowSendRange,
}
