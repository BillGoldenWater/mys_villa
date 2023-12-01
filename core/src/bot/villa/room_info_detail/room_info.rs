/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api::villa_bot_api::villa_api::list_room::ListRoom;
use crate::bot::villa::room_info_detail::room_info::room_type::RoomType;

pub mod room_type;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RoomInfo {
  pub id: u64,
  pub name: String,
  pub r#type: RoomType,
  pub group_id: u64,
}

impl From<ListRoom> for RoomInfo {
  fn from(value: ListRoom) -> Self {
    Self {
      id: value.room_id,
      name: value.room_name,
      r#type: value.room_type.into(),
      group_id: value.group_id,
    }
  }
}

impl From<RoomInfo> for ListRoom {
  fn from(value: RoomInfo) -> Self {
    Self {
      room_id: value.id,
      room_name: value.name,
      room_type: value.r#type.into(),
      group_id: value.group_id,
    }
  }
}
