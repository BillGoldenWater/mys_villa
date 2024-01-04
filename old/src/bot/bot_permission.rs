/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
  bot::{bot_event_handler::BotEventHandler, Bot},
  error::{VError, VResult},
  request::request_executor::RequestExecutor,
};

/// permission info about what bot can do
#[derive(Debug, Clone, Eq, PartialEq)]
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
  /// get group list / get room list
  ViewRoomAndGroup,
  /// add/remove a role of a member
  OperateMemberToRole,
  /// create/edit/delete role
  ManageRole,
  /// get role info / get all roles
  ViewRole,
  /// audit user content
  Audit,
  /// transfer image
  TransferImg,
}

impl BotPermission {
  /// get all permission
  pub fn all() -> Vec<Self> {
    vec![
      Self::ViewVilla,
      Self::ViewMember,
      Self::ManageMember,
      Self::ManageMessage,
      Self::SendMessage,
      Self::ManageRoomAndGroup,
      Self::ViewRoomAndGroup,
      Self::OperateMemberToRole,
      Self::ManageRole,
      Self::ViewRole,
      Self::Audit,
      Self::TransferImg,
    ]
  }

  /// get all permission except [BotPermission::Audit]
  pub fn all_except_audit() -> Vec<Self> {
    Self::all()
      .into_iter()
      .filter(|it| !matches!(it, Self::Audit))
      .collect()
  }

  /// check if bot has this permission
  pub fn check<
    State: Sync,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor + Sync,
  >(
    &self,
    bot: &Bot<State, EventHandler, ReqExecutor>,
  ) -> bool {
    bot.permission.contains(self)
  }

  /// check if bot has this permission, return a result
  pub fn check_result<
    State: Sync,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor + Sync,
  >(
    &self,
    bot: &Bot<State, EventHandler, ReqExecutor>,
  ) -> VResult<()> {
    if bot.permission.contains(self) {
      Ok(())
    } else {
      Err(VError::PermissionDenied(self.clone()))
    }
  }
}
