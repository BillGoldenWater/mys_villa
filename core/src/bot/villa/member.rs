/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::sync::Arc;

use serde_json::Value;
use tracing::{instrument, trace};

use crate::{
  api::villa_bot_api::villa_api::member_api::{
    audit::AuditResponse,
    delete_villa_member::DeleteVillaMemberRequest,
    get_member::{GetMemberRequest, GetMemberResponse},
  },
  bot::{
    bot_permission::BotPermission,
    villa::{member::audit_info::AuditInfo, member_info::MemberInfo, Villa},
    Bot,
  },
  error::VResult,
  http::request::Request,
  utils::fp_utils::FpUtils,
};

pub mod audit_info;

#[derive(Debug, Clone)]
pub struct Member {
  villa: Arc<Villa>,

  id: u64,
}

impl Member {
  #[instrument(level = "debug", skip(villa))]
  pub fn new(villa: Arc<Villa>, id: u64) -> Self {
    Self { villa, id }
  }

  pub fn id(&self) -> u64 {
    self.id
  }
}

/// api
impl Member {
  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn get_info(&self) -> VResult<MemberInfo> {
    BotPermission::ViewMember.check(self)?;

    self
      .villa
      .execute_bot_req_with_villa::<GetMemberResponse>(
        Request::get("/vila/api/bot/platform/getMember")
          .with_json(GetMemberRequest::new(self.id))?,
      )
      .await
      .map(|it| it.member.into())
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn kick(self) -> VResult<()> {
    BotPermission::ManageMember.check(&self)?;

    self
      .villa
      .execute_bot_req_with_villa::<Value>(
        Request::post("/vila/api/bot/platform/deleteVillaMember")
          .with_json(DeleteVillaMemberRequest::new(self.id))?,
      )
      .await
      .unit_result()
  }

  #[instrument(level = "debug", skip(self, audit_info), fields(id = self.id))]
  pub async fn audit(&self, audit_info: AuditInfo) -> VResult<String> {
    trace!("audit info: {audit_info:?}");
    self
      .villa
      .execute_bot_req_with_villa::<AuditResponse>(
        Request::post("/vila/api/bot/platform/audit")
          .with_json(audit_info.construct_req(self.id))?,
      )
      .await
      .map(|it| it.audit_id)
  }
}

impl AsRef<Bot> for Member {
  fn as_ref(&self) -> &Bot {
    &self.villa.bot
  }
}
