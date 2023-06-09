/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use log::debug;

use crate::api_type::group::delete_group_request::DeleteGroupRequest;
use crate::api_type::group::edit_group_request::EditGroupRequest;
use crate::bot::bot_event_handler::BotEventHandler;
use crate::bot::bot_permission::BotPermission;
use crate::bot::villa::Villa;
use crate::error::VResult;
use crate::request::request_executor::RequestExecutor;

/// for execute api under group context
/// - [Group::set_name] change name of group
/// - [Group::delete] delete group
#[derive(Debug)]
pub struct Group<
  'villa,
  State: Sync,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor + Sync,
> {
  villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>,
  id: u64,
}

impl<
    'villa,
    State: Sync,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor + Sync,
  > Group<'villa, State, EventHandler, ReqExecutor>
{
  /// create a instance with villa and group id
  pub fn new(villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>, id: u64) -> Self {
    debug!("initializing group instance with id: {id}");

    Self { villa, id }
  }

  /// get group id
  pub fn id(&self) -> u64 {
    self.id
  }

  /// set group name
  pub async fn set_name(&self, name: impl Into<String>) -> VResult<()> {
    BotPermission::ManageRoomAndGroup.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/editGroup",
        EditGroupRequest::new(self.id, name.into()),
      )
      .execute_no_return(&self.villa.bot.request_executor)
      .await
  }

  /// delete group
  pub async fn delete(&self) -> VResult<()> {
    BotPermission::ManageRoomAndGroup.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/deleteGroup",
        DeleteGroupRequest::new(self.id),
      )
      .execute_no_return(&self.villa.bot.request_executor)
      .await
  }
}
