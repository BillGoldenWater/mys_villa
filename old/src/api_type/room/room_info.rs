/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string as from_string;

use crate::api_type::room::room_type::RoomType;

/// room info
#[derive(Debug, Deserialize)]
pub struct RoomInfo {
  /// room id
  #[serde(rename = "room_id", deserialize_with = "from_string")]
  pub id: u64,
  /// room name
  #[serde(rename = "room_name")]
  pub name: String,
  /// type
  #[serde(rename = "room_type")]
  pub r#type: RoomType,
  /// group id
  #[serde(deserialize_with = "from_string")]
  pub group_id: u64,
}
