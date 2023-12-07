/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::sync::Arc;

use tracing::{instrument, trace};

use crate::api::villa_bot_api::villa_api::member_api::audit::AuditResponse;
use crate::api::villa_bot_api::villa_api::member_api::delete_villa_member::DeleteVillaMemberRequest;
use crate::api::villa_bot_api::villa_api::member_api::get_member::{
  GetMemberRequest, GetMemberResponse,
};
use crate::bot::bot_permission::BotPermission;
use crate::bot::villa::member::audit_info::AuditInfo;
use crate::bot::villa::member_info::MemberInfo;
use crate::bot::villa::Villa;
use crate::error::VResult;
use crate::http::request::Request;

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
    BotPermission::ViewMember.check(&self.villa.bot)?;

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
  pub async fn kick(&self) -> VResult<()> {
    BotPermission::ManageMember.check(&self.villa.bot)?;

    self
      .villa
      .execute_bot_req_with_villa::<()>(
        Request::post("/vila/api/bot/platform/deleteVillaMember")
          .with_json(DeleteVillaMemberRequest::new(self.id))?,
      )
      .await
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
