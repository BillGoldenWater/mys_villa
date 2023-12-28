/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use strum::Display;

use crate::bot::Bot;
use crate::error::{VError, VResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum BotPermission {
  /// get information of a villa
  ViewVilla,
  /// get member information and get member list
  ViewMember,
  /// kick a member
  ManageMember,
  /// pin message and recall message
  ManageMessage,
  /// send message
  SendMessage,
  /// create/edit/delete group or room
  ManageRoomAndGroup,
  /// get group/room list and info
  ViewRoomAndGroup,
  /// add/remove a role of a member
  OperateMemberToRole,
  /// create/edit/delete role
  ManageRole,
  /// get role info / get all roles
  ViewRole,
}

impl BotPermission {
  pub fn check(&self, bot: impl AsRef<Bot>) -> VResult<()> {
    bot
      .as_ref()
      .permissions
      .contains(self)
      .then_some(())
      .ok_or(VError::PermissionDenied(*self))
  }
}

/// presets
impl BotPermission {
  pub fn admin() -> [Self; 10] {
    [
      BotPermission::ViewVilla,
      BotPermission::ViewMember,
      BotPermission::ManageMember,
      BotPermission::ManageMessage,
      BotPermission::SendMessage,
      BotPermission::ManageRoomAndGroup,
      BotPermission::ViewRoomAndGroup,
      BotPermission::OperateMemberToRole,
      BotPermission::ManageRole,
      BotPermission::ViewRole,
    ]
  }

  pub fn normal() -> [Self; 5] {
    [
      BotPermission::ViewVilla,
      BotPermission::ViewMember,
      BotPermission::SendMessage,
      BotPermission::ViewRoomAndGroup,
      BotPermission::ViewRole,
    ]
  }
}
