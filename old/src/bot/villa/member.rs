/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use log::debug;

use crate::{
  api_type::{
    member::{
      delete_villa_member_request::DeleteVillaMemberRequest, get_member_request::GetMemberRequest,
      get_member_response::GetMemberResponse, member_data::MemberData,
    },
    room::{audit_request::AuditRequest, audit_response::AuditResponse},
  },
  bot::{bot_event_handler::BotEventHandler, bot_permission::BotPermission, villa::Villa},
  error::VResult,
  request::request_executor::RequestExecutor,
};

/// for execute api under member context
/// - [Member::get_data] get member data
/// - [Member::kick] remove this member from villa
#[derive(Debug)]
pub struct Member<
  'villa,
  State: Sync,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor + Sync,
> {
  villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>,
  uid: u64,
}

impl<
    'villa,
    State: Sync,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor + Sync,
  > Member<'villa, State, EventHandler, ReqExecutor>
{
  /// create a instance with villa and member uid
  pub fn new(villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>, uid: u64) -> Self {
    debug!("initializing member instance with uid: {uid}");

    Self { villa, uid }
  }

  /// get member uid
  pub fn uid(&self) -> u64 {
    self.uid
  }

  /// get member data
  pub async fn get_data(&self) -> VResult<MemberData> {
    BotPermission::ViewMember.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_get_with_body(
        "/vila/api/bot/platform/getMember",
        GetMemberRequest::new(self.uid),
      )
      .execute_result::<GetMemberResponse, _>(&self.villa.bot.request_executor)
      .await
      .map(|it| it.member)
  }

  /// kick
  pub async fn kick(&self) -> VResult<()> {
    BotPermission::ManageMember.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/deleteVillaMember",
        DeleteVillaMemberRequest::new(self.uid),
      )
      .execute_no_return(&self.villa.bot.request_executor)
      .await
  }

  /// audit
  pub async fn audit(
    &self,
    audit_content: impl Into<String>,
    pass_through: Option<impl Into<String>>,
    room_id: Option<u64>,
  ) -> VResult<String> {
    BotPermission::Audit.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/audit",
        AuditRequest::new(audit_content, pass_through, room_id, self.uid),
      )
      .execute_result::<AuditResponse, _>(&self.villa.bot.request_executor)
      .await
      .map(|it| it.audit_id)
  }
}
