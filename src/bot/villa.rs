/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::collections::HashMap;

use log::debug;

use crate::api_type::bot_access_info::check_member_bot_access_token_request::CheckMemberBotAccessTokenRequest;
use crate::api_type::bot_access_info::check_member_bot_access_token_response::CheckMemberBotAccessTokenResponse;
use crate::api_type::bot_access_info::BotAccessData;
use crate::api_type::group::create_group_request::CreateGroupRequest;
use crate::api_type::group::create_group_response::CreateGroupResponse;
use crate::api_type::group::get_group_list_response::GetGroupListResponse;
use crate::api_type::group::group_info::GroupInfo;
use crate::api_type::group::sort_group_list_request::SortGroupListRequest;
use crate::api_type::role::create_member_role_request::CreateMemberRoleRequest;
use crate::api_type::role::create_member_role_response::CreateMemberRoleResponse;
use crate::api_type::role::get_villa_member_roles_response::GetVillaMemberRoles;
use crate::api_type::role::role_color::RoleColor;
use crate::api_type::role::role_info::RoleInfo;
use crate::api_type::role::role_permission::RolePermission;
use crate::api_type::room::get_villa_group_room_list_response::GetVillaGroupRoomListResponse;
use crate::api_type::room::room_group_info::RoomGroupInfo;
use crate::api_type::room::room_order_item::RoomOrderItem;
use crate::api_type::room::sort_room_list_request::SortRoomListRequest;
use crate::api_type::villa::get_villa_members_request::GetVillaMembersRequest;
use crate::api_type::villa::get_villa_members_response::GetVillaMembersResponse;
use crate::api_type::villa::get_villa_response::GetVillaResponse;
use crate::api_type::villa::villa_info::VillaInfo;
use crate::bot::bot_event_handler::BotEventHandler;
use crate::bot::bot_permission::BotPermission;
use crate::bot::villa::group::Group;
use crate::bot::villa::member::Member;
use crate::bot::villa::role::Role;
use crate::bot::villa::room::Room;
use crate::bot::Bot;
use crate::error::{VError, VResult};
use crate::request::header_builder::HeaderBuilder;
use crate::request::request_builder::RequestBuilder;
use crate::request::request_executor::RequestExecutor;
use crate::response::retcode::RetCode;

/// group related logic
pub mod group;
/// member related logic
pub mod member;
/// role related logic
pub mod role;
/// room related logic, include message related operation
pub mod room;

/// for execute api under villa context
/// - [Villa::get_info] get information of villa
/// - [Villa::get_members_data] get all member of villa
/// - [Villa::get_all_roles_info] get all role of villa
/// - [Villa::create_role] create a new role
/// - [Villa::get_all_group_info] get all group of villa, doesn't include room info
/// - [Villa::create_group] create a new group
/// - [Villa::reorder_group] reorder all group of villa
/// - [Villa::get_all_room_group_info] get all group and room of villa
/// - [Villa::reorder_room] reorder all room of villa
/// - [Villa::check_access_token] check bot access token of member
/// - [Villa::member] create member instance by uid
/// - [Villa::role] create role instance by id
/// - [Villa::group] create group instance by id
/// - [Villa::room] create room instance by id
#[derive(Debug)]
pub struct Villa<
  'bot,
  State,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor,
> {
  bot: &'bot Bot<State, EventHandler, ReqExecutor>,
  villa_id: u64,

  req_builder: RequestBuilder,
}

impl<
    'bot,
    State,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor,
  > Villa<'bot, State, EventHandler, ReqExecutor>
{
  /// create a instance with bot and villa id
  pub fn new(bot: &'bot Bot<State, EventHandler, ReqExecutor>, villa_id: u64) -> Self {
    debug!("initializing villa instance with id: {villa_id}");

    Self {
      bot,
      villa_id,
      req_builder: RequestBuilder::new(
        HeaderBuilder::from_auth_info(&bot.auth_info).with_villa_id(villa_id),
      ),
    }
  }

  /// get villa id
  pub fn id(&self) -> u64 {
    self.villa_id
  }

  /// create a member instance with member uid
  pub fn member(&self, member_uid: u64) -> Member<'_, State, EventHandler, ReqExecutor> {
    Member::new(self, member_uid)
  }

  /// create a role instance with role id
  pub fn role(&self, role_id: u64) -> Role<'_, State, EventHandler, ReqExecutor> {
    Role::new(self, role_id)
  }

  /// create a group instance with group id
  pub fn group(&self, group_id: u64) -> Group<'_, State, EventHandler, ReqExecutor> {
    Group::new(self, group_id)
  }

  /// create a room instance with group id
  pub fn room(&self, room_id: u64) -> Room<'_, State, EventHandler, ReqExecutor> {
    Room::new(self, room_id)
  }

  /// get villa info
  pub async fn get_info(&self) -> VResult<VillaInfo> {
    BotPermission::ViewVilla.check_result(self.bot)?;

    self
      .req_builder
      .build_get("/vila/api/bot/platform/getVilla")
      .execute_result::<GetVillaResponse, _>(&self.bot.request_executor)
      .await
      .map(|it| it.villa)
  }

  /// get members data
  ///
  /// when set next_offset to empty string, will return first page of member list
  pub async fn get_members_data(
    &self,
    page_size: u64,
    next_offset: impl Into<String>,
  ) -> VResult<GetVillaMembersResponse> {
    BotPermission::ViewMember.check_result(self.bot)?;

    self
      .req_builder
      .build_get_with_body(
        "/vila/api/bot/platform/getVillaMembers",
        GetVillaMembersRequest::new(next_offset, page_size),
      )
      .execute_result::<GetVillaMembersResponse, _>(&self.bot.request_executor)
      .await
  }

  // region role
  /// get all role's info in this villa
  pub async fn get_all_roles_info(&self) -> VResult<HashMap<u64, RoleInfo>> {
    BotPermission::ViewRole.check_result(self.bot)?;

    self
      .req_builder
      .build_get("/vila/api/bot/platform/getVillaMemberRoles")
      .execute_result::<GetVillaMemberRoles, _>(&self.bot.request_executor)
      .await
      .map(|it| it.list.into_iter().map(|it| (it.id, it)).collect())
  }

  /// create role with name, color and permissions
  pub async fn create_role(
    &self,
    name: impl Into<String>,
    color: RoleColor,
    permissions: Vec<RolePermission>,
  ) -> VResult<u64> {
    BotPermission::ManageRole.check_result(self.bot)?;

    self
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/createMemberRole",
        CreateMemberRoleRequest::new(name.into(), color, permissions),
      )
      .execute_result::<CreateMemberRoleResponse, _>(&self.bot.request_executor)
      .await
      .map(|it| it.id)
  }
  // endregion

  // region group
  /// get all group info
  pub async fn get_all_group_info(&self) -> VResult<Vec<GroupInfo>> {
    BotPermission::ViewRoomAndGroup.check_result(self.bot)?;

    self
      .req_builder
      .build_get("/vila/api/bot/platform/getGroupList")
      .execute_result::<GetGroupListResponse, _>(&self.bot.request_executor)
      .await
      .map(|it| it.list)
  }

  /// create group with name
  pub async fn create_group(&self, name: impl Into<String>) -> VResult<u64> {
    BotPermission::ManageRoomAndGroup.check_result(self.bot)?;

    self
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/createGroup",
        CreateGroupRequest::new(name.into()),
      )
      .execute_result::<CreateGroupResponse, _>(&self.bot.request_executor)
      .await
      .map(|it| it.group_id)
  }

  /// reorder group
  pub async fn reorder_group(&self, ordered_group_id: Vec<u64>) -> VResult<()> {
    BotPermission::ManageRoomAndGroup.check_result(self.bot)?;

    self
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/sortGroupList",
        SortGroupListRequest::new(self.villa_id, ordered_group_id),
      )
      .execute_no_return(&self.bot.request_executor)
      .await
    // todo: wait bug fix: input unchecked, and no effect
  }
  // endregion

  // region room
  /// get all room group info
  pub async fn get_all_room_group_info(&self) -> VResult<Vec<RoomGroupInfo>> {
    BotPermission::ViewRoomAndGroup.check_result(self.bot)?;

    self
      .req_builder
      .build_get("/vila/api/bot/platform/getVillaGroupRoomList")
      .execute_result::<GetVillaGroupRoomListResponse, _>(&self.bot.request_executor)
      .await
      .map(|it| it.list)
  }

  /// reorder room
  pub async fn reorder_room(&self, ordered_room_list: Vec<RoomOrderItem>) -> VResult<()> {
    BotPermission::ManageRoomAndGroup.check_result(self.bot)?;

    self
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/sortRoomList",
        SortRoomListRequest::new(self.villa_id, ordered_room_list),
      )
      .execute_no_return(&self.bot.request_executor)
      .await
    // todo: wait bug fix: input unchecked, and no effect
  }
  // endregion

  /// check bot access token of a member
  pub async fn check_access_token(&self, token: String) -> VResult<BotAccessData> {
    let res = self
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/checkMemberBotAccessToken",
        CheckMemberBotAccessTokenRequest::new(token),
      )
      .execute_result::<CheckMemberBotAccessTokenResponse, _>(&self.bot.request_executor)
      .await?;

    // result_flatten is unstable
    if let (Some(access_info), Some(member)) = (res.access_info, res.member) {
      Ok(BotAccessData::new(access_info, member))
    } else {
      Err(VError::ApiError {
        retcode: RetCode::InvalidMemberBotAccessToken,
        message: "empty data".to_string(),
      })
    }
  }
}
