/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

/// return code of api
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum RetCode {
  /// unknown server side error
  UnknownServerError,
  /// invalid request
  InvalidRequest,
  /// no error
  Ok,
  /// can't operate member that not in this villa
  MemberNotInVilla,
  /// target role isn't exists
  RoleNotExists,
  /// insufficient permission (for operation that not allow bot to call)
  InsufficientPermission,
  /// cannot kick villa owner
  CannotKickVillaOwner,
  /// cannot kick self
  CannotKickSelf,
  /// bot isn't exists in the target villa
  BotNotAdded,
  /// api need a permission that the bot doesn't have
  PermissionDenied,
  /// invalid bot access token of a member
  InvalidMemberBotAccessToken,
  /// invalid auth info or villa id
  InvalidBotAuthInfo,
  /// unsupported message object type
  UnsupportedMsgType,
  /// unknown retcode
  Unknown(i32),
}

// todo: 10320005, 当前权限不足，无法操作

impl From<i32> for RetCode {
  fn from(value: i32) -> Self {
    match value {
      -502 => Self::UnknownServerError,
      -1 => Self::InvalidRequest,
      0 => Self::Ok,
      10313002 => Self::MemberNotInVilla,
      10318000 => Self::RoleNotExists,
      10318001 => Self::InsufficientPermission,
      10318004 => Self::CannotKickVillaOwner,
      10320004 => Self::CannotKickSelf,
      10322002 => Self::BotNotAdded,
      10322003 => Self::PermissionDenied,
      10322004 => Self::InvalidMemberBotAccessToken,
      10322005 => Self::InvalidBotAuthInfo,
      10322006 => Self::UnsupportedMsgType,
      _ => Self::Unknown(value),
    }
  }
}

impl From<RetCode> for i32 {
  fn from(value: RetCode) -> Self {
    match value {
      RetCode::UnknownServerError => -502,
      RetCode::InvalidRequest => -1,
      RetCode::Ok => 0,
      RetCode::MemberNotInVilla => 10313002,
      RetCode::RoleNotExists => 10318000,
      RetCode::InsufficientPermission => 10318001,
      RetCode::CannotKickVillaOwner => 10318004,
      RetCode::CannotKickSelf => 10320004,
      RetCode::BotNotAdded => 10322002,
      RetCode::PermissionDenied => 10322003,
      RetCode::InvalidMemberBotAccessToken => 10322004,
      RetCode::InvalidBotAuthInfo => 10322005,
      RetCode::UnsupportedMsgType => 10322006,
      RetCode::Unknown(retcode) => retcode,
    }
  }
}
