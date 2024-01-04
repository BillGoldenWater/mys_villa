/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::fmt::Debug;

use crate::{
  api::villa_bot_api::villa_api::room::Room,
  bot::villa::room_info_detail::{
    room_allow_send_msg_range::RoomAllowSendMsgRange, room_info::RoomInfo,
    room_notify_type::RoomNotifyType,
  },
};

pub mod room_allow_send_msg_range;
pub mod room_info;
pub mod room_notify_type;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RoomInfoDetail {
  pub info: RoomInfo,

  pub notify_type: RoomNotifyType,
  pub allow_send_msg_range: RoomAllowSendMsgRange,
}

impl From<Room> for RoomInfoDetail {
  fn from(value: Room) -> Self {
    Self {
      info: RoomInfo {
        id: value.room_id,
        name: value.room_name,
        r#type: value.room_type.into(),
        group_id: value.group_id,
      },

      notify_type: value.room_default_notify_type.into(),
      allow_send_msg_range: value.send_msg_auth_range.into(),
    }
  }
}

impl From<RoomInfoDetail> for Room {
  fn from(value: RoomInfoDetail) -> Self {
    Self {
      room_id: value.info.id,
      room_name: value.info.name,
      room_type: value.info.r#type.into(),
      group_id: value.info.group_id,
      room_default_notify_type: value.notify_type.into(),
      send_msg_auth_range: value.allow_send_msg_range.into(),
    }
  }
}
