/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::sync::Arc;

use itertools::Itertools;
use serde::de::DeserializeOwned;
use tracing::instrument;

use crate::api::api_error::ApiError;
use crate::api::villa_bot_api::villa_api::check_member_bot_access_token::{
  CheckMemberBotAccessTokenRequest, CheckMemberBotAccessTokenResponse,
};
use crate::api::villa_bot_api::villa_api::create_group::{CreateGroupRequest, CreateGroupResponse};
use crate::api::villa_bot_api::villa_api::create_member_role::{
  CreateMemberRoleRequest, CreateMemberRoleResponse,
};
use crate::api::villa_bot_api::villa_api::get_group_list::GetGroupListResponse;
use crate::api::villa_bot_api::villa_api::get_villa::GetVillaResponse;
use crate::api::villa_bot_api::villa_api::get_villa_group_room_list::GetVillaGroupRoomListResponse;
use crate::api::villa_bot_api::villa_api::get_villa_member_roles::GetVillaMemberRolesResponse;
use crate::api::villa_bot_api::villa_api::transfer_image::{
  TransferImageRequest, TransferImageResponse,
};
use crate::api::villa_bot_api::villa_response::retcode::RetCode;
use crate::bot::bot_permission::BotPermission;
use crate::bot::villa::group_info::GroupInfo;
use crate::bot::villa::member::Member;
use crate::bot::villa::member_info::MemberInfo;
use crate::bot::villa::member_stream::MemberStream;
use crate::bot::villa::role::Role;
use crate::bot::villa::role_info::role_color::RoleColor;
use crate::bot::villa::role_info::role_permission_info::role_permission_key::RolePermissionKey;
use crate::bot::villa::role_info::RoleInfo;
use crate::bot::villa::villa_info::VillaInfo;
use crate::bot::Bot;
use crate::error::VResult;
use crate::http::request::Request;

pub mod group_info;
pub mod member;
pub mod member_info;
pub mod member_stream;
pub mod role;
pub mod role_info;
pub mod room_info_detail;
pub mod villa_info;

#[derive(Debug, Clone)]
pub struct Villa {
  bot: Arc<Bot>,

  id: u64,
}

impl Villa {
  #[instrument(level = "debug", skip(bot))]
  pub fn new(bot: Arc<Bot>, id: u64) -> Self {
    Self { bot, id }
  }

  pub fn id(&self) -> u64 {
    self.id
  }

  pub async fn execute_bot_req_with_villa<T: DeserializeOwned>(
    &self,
    request: Request,
  ) -> VResult<T> {
    self
      .bot
      .execute_bot_req_authed(request.with_villa_id(self.id))
      .await
  }

  pub fn member(&self, id: u64) -> Member {
    Member::new(self.clone().into(), id)
  }

  pub fn role(&self, id: u64) -> Role {
    Role::new(self.clone().into(), id)
  }
}

/// api
impl Villa {
  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn get_info(&self) -> VResult<VillaInfo> {
    BotPermission::ViewVilla.check(&self.bot)?;

    self
      .execute_bot_req_with_villa::<GetVillaResponse>(Request::get(
        "/vila/api/bot/platform/getVilla",
      ))
      .await
      .map(|it| it.villa.into())
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub fn get_members(&self) -> VResult<MemberStream> {
    BotPermission::ViewMember.check(&self.bot)?;

    Ok(MemberStream::new(self.clone()))
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn get_roles(&self) -> VResult<Vec<RoleInfo>> {
    BotPermission::ViewRole.check(&self.bot)?;

    self
      .execute_bot_req_with_villa::<GetVillaMemberRolesResponse>(Request::get(
        "/vila/api/bot/platform/getVillaMemberRoles",
      ))
      .await
      .map(|it| it.list.into_iter().map(Into::into).collect_vec())
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn create_role(
    &self,
    name: String,
    role_color: RoleColor,
    permissions: Vec<RolePermissionKey>,
  ) -> VResult<u64> {
    BotPermission::ManageRole.check(&self.bot)?;

    self
      .execute_bot_req_with_villa::<CreateMemberRoleResponse>(
        Request::post("/vila/api/bot/platform/createMemberRole").with_json(
          CreateMemberRoleRequest::new(
            name,
            role_color.into(),
            permissions.into_iter().map(Into::into).collect_vec(),
          ),
        )?,
      )
      .await
      .map(|it| it.id)
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn get_groups(&self) -> VResult<Vec<GroupInfo>> {
    BotPermission::ViewRoomAndGroup.check(&self.bot)?;

    self
      .execute_bot_req_with_villa::<GetGroupListResponse>(Request::get(
        "/vila/api/bot/platform/getGroupList",
      ))
      .await
      .map(|it| it.list.into_iter().map(Into::into).collect_vec())
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn create_group(&self, name: String) -> VResult<u64> {
    BotPermission::ManageRoomAndGroup.check(&self.bot)?;

    self
      .execute_bot_req_with_villa::<CreateGroupResponse>(
        Request::post("/vila/api/bot/platform/createGroup")
          .with_json(CreateGroupRequest::new(name))?,
      )
      .await
      .map(|it| it.group_id)
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn get_grouped_rooms(&self) -> VResult<Vec<GroupInfo>> {
    BotPermission::ViewRoomAndGroup.check(&self.bot)?;

    self
      .execute_bot_req_with_villa::<GetVillaGroupRoomListResponse>(Request::get(
        "/vila/api/bot/platform/getVillaGroupRoomList",
      ))
      .await
      .map(|it| it.list.into_iter().map(Into::into).collect_vec())
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn check_access_token(&self, token: String) -> VResult<MemberInfo> {
    let response = self
      .execute_bot_req_with_villa::<CheckMemberBotAccessTokenResponse>(
        Request::post("/vila/api/bot/platform/checkMemberBotAccessToken")
          .with_json(CheckMemberBotAccessTokenRequest::new(token))?,
      )
      .await?;

    // result_flatten is unstable
    if let Some(member) = response.member {
      Ok(member.into())
    } else {
      Err(
        ApiError::Villa {
          retcode: RetCode::InvalidMemberBotAccessToken,
          message: "empty data".into(),
        }
        .into(),
      )
    }
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn transfer_image(&self, image_url: String) -> VResult<String> {
    self
      .execute_bot_req_with_villa::<TransferImageResponse>(
        Request::post("/vila/api/bot/platform/transferImage")
          .with_json(TransferImageRequest::new(image_url))?,
      )
      .await
      .map(|it| it.new_url)
  }
}
