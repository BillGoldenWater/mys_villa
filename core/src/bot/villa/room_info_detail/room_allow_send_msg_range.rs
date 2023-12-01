/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api::villa_bot_api::villa_api::send_msg_auth_range::SendMsgAuthRange;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RoomAllowSendMsgRange {
  All,
  Roles(Vec<u64>),
}

impl From<SendMsgAuthRange> for RoomAllowSendMsgRange {
  fn from(value: SendMsgAuthRange) -> Self {
    if value.is_all_send_msg {
      RoomAllowSendMsgRange::All
    } else {
      RoomAllowSendMsgRange::Roles(value.roles)
    }
  }
}

impl From<RoomAllowSendMsgRange> for SendMsgAuthRange {
  fn from(value: RoomAllowSendMsgRange) -> Self {
    Self {
      is_all_send_msg: matches!(value, RoomAllowSendMsgRange::All),
      roles: match value {
        RoomAllowSendMsgRange::All => vec![],
        RoomAllowSendMsgRange::Roles(roles) => roles,
      },
    }
  }
}
