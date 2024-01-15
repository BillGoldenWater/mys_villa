/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
  api::villa_bot_api::villa_api::{group::Group, group_room::GroupRoom},
  bot::villa::room_info_detail::room_info::RoomInfo,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GroupInfo {
  pub id: u64,
  pub name: String,
  pub rooms: Vec<RoomInfo>,
}

impl From<Group> for GroupInfo {
  fn from(value: Group) -> Self {
    Self {
      id: value.group_id,
      name: value.group_name,
      rooms: vec![],
    }
  }
}

impl From<GroupInfo> for Group {
  fn from(value: GroupInfo) -> Self {
    Self {
      group_id: value.id,
      group_name: value.name,
    }
  }
}

impl From<GroupRoom> for GroupInfo {
  fn from(value: GroupRoom) -> Self {
    Self {
      id: value.group_id,
      name: value.group_name,
      rooms: value.room_list.into_iter().map(|it| it.into()).collect(),
    }
  }
}

impl From<GroupInfo> for GroupRoom {
  fn from(value: GroupInfo) -> Self {
    Self {
      group_id: value.id,
      group_name: value.name,
      room_list: value.rooms.into_iter().map(|it| it.into()).collect(),
    }
  }
}
