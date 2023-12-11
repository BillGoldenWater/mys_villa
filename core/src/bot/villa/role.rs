/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::sync::Arc;

use serde_json::Value;
use tracing::instrument;

use crate::api::villa_bot_api::villa_api::role_api::delete_member_role::DeleteMemberRoleRequest;
use crate::api::villa_bot_api::villa_api::role_api::edit_member_role::EditMemberRoleRequest;
use crate::api::villa_bot_api::villa_api::role_api::get_member_role_info::{
  GetMemberRoleInfoRequest, GetMemberRoleInfoResponse,
};
use crate::api::villa_bot_api::villa_api::role_api::operate_member_to_role::OperateMemberToRoleRequest;
use crate::bot::bot_permission::BotPermission;
use crate::bot::villa::role_info::role_color::RoleColor;
use crate::bot::villa::role_info::role_permission_info::role_permission_key::RolePermissionKey;
use crate::bot::villa::role_info::RoleInfo;
use crate::bot::villa::Villa;
use crate::error::VResult;
use crate::http::request::Request;
use crate::utils::fp_utils::FpUtils;

#[derive(Debug, Clone)]
pub struct Role {
  villa: Arc<Villa>,

  id: u64,
}

impl Role {
  #[instrument(level = "debug", skip(villa))]
  pub fn new(villa: Arc<Villa>, id: u64) -> Self {
    Self { villa, id }
  }

  pub fn id(&self) -> u64 {
    self.id
  }
}

/// api
impl Role {
  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn get_info(&self) -> VResult<RoleInfo> {
    BotPermission::ViewRole.check(&self.villa.bot)?;

    self
      .villa
      .execute_bot_req_with_villa::<GetMemberRoleInfoResponse>(
        Request::get("/vila/api/bot/platform/getMemberRoleInfo")
          .with_json(GetMemberRoleInfoRequest::new(self.id))?,
      )
      .await
      .map(|it| it.role.into())
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn edit_info(
    &self,
    name: String,
    color: RoleColor,
    permissions: Vec<RolePermissionKey>,
  ) -> VResult<()> {
    BotPermission::ManageRole.check(&self.villa.bot)?;

    self
      .villa
      .execute_bot_req_with_villa::<Value>(
        Request::post("/vila/api/bot/platform/editMemberRole").with_json(
          EditMemberRoleRequest::new(
            self.id,
            name,
            color.into(),
            permissions.into_iter().map(Into::into).collect(),
          ),
        )?,
      )
      .await
      .unit_result()
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn add_member(&self, uid: u64) -> VResult<()> {
    self.operate_member(uid, true).await
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn remove_member(&self, uid: u64) -> VResult<()> {
    self.operate_member(uid, false).await
  }

  async fn operate_member(&self, uid: u64, is_add: bool) -> VResult<()> {
    BotPermission::OperateMemberToRole.check(&self.villa.bot)?;

    self
      .villa
      .execute_bot_req_with_villa::<Value>(
        Request::post("/vila/api/bot/platform/operateMemberToRole")
          .with_json(OperateMemberToRoleRequest::new(self.id, uid, is_add))?,
      )
      .await
      .unit_result()
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn delete(self) -> VResult<()> {
    BotPermission::ManageRole.check(&self.villa.bot)?;

    self
      .villa
      .execute_bot_req_with_villa::<Value>(
        Request::post("/vila/api/bot/platform/deleteMemberRole")
          .with_json(DeleteMemberRoleRequest::new(self.id))?,
      )
      .await
      .unit_result()
  }
}
