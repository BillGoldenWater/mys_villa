use log::debug;

use crate::api_type::role::delete_member_role_request::DeleteMemberRoleRequest;
use crate::api_type::role::edit_member_role_request::EditMemberRoleRequest;
use crate::api_type::role::get_member_role_info_request::GetMemberRoleInfoRequest;
use crate::api_type::role::get_member_role_info_response::GetMemberRoleInfoResponse;
use crate::api_type::role::operate_member_to_role_request::OperateMemberToRoleRequest;
use crate::api_type::role::role_color::RoleColor;
use crate::api_type::role::role_info::RoleInfo;
use crate::api_type::role::role_permission::RolePermission;
use crate::bot::bot_event_handler::BotEventHandler;
use crate::bot::bot_permission::BotPermission;
use crate::bot::villa::Villa;
use crate::error::VResult;
use crate::request::request_executor::RequestExecutor;

/// role instance, provide role related access
#[derive(Debug)]
pub struct Role<
  'villa,
  State,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor,
> {
  villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>,
  id: u64,
}

impl<
    'villa,
    State,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor,
  > Role<'villa, State, EventHandler, ReqExecutor>
{
  /// create a instance with villa and role id
  pub fn new(villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>, id: u64) -> Self {
    debug!("initializing role instance with id: {id}");

    Self { villa, id }
  }

  /// get role id
  pub fn id(&self) -> u64 {
    self.id
  }

  /// get role info
  pub async fn get_info(&self) -> VResult<RoleInfo> {
    BotPermission::ViewRole.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_get_with_body(
        "/vila/api/bot/platform/getMemberRoleInfo",
        GetMemberRoleInfoRequest::new(self.id),
      )
      .execute_result::<GetMemberRoleInfoResponse, _>(&self.villa.bot.request_executor)
      .await
      .map(|it| it.role)
  }

  /// edit role info
  pub async fn edit_info(
    &self,
    name: impl Into<String>,
    color: RoleColor,
    permissions: Vec<RolePermission>,
  ) -> VResult<()> {
    BotPermission::ManageRole.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/editMemberRole",
        EditMemberRoleRequest::new(self.id, name.into(), color, permissions),
      )
      .execute_no_return(&self.villa.bot.request_executor)
      .await
  }

  /// add user to this role
  pub async fn add_user(&self, uid: u64) -> VResult<()> {
    self.operate_member(uid, true).await
  }

  /// remove user to this role
  pub async fn remove_user(&self, uid: u64) -> VResult<()> {
    self.operate_member(uid, false).await
  }

  async fn operate_member(&self, uid: u64, is_add: bool) -> VResult<()> {
    BotPermission::OperateMemberToRole.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/operateMemberToRole",
        OperateMemberToRoleRequest::new(self.id, uid, is_add),
      )
      .execute_no_return(&self.villa.bot.request_executor)
      .await
  }

  /// delete this role
  pub async fn delete(&self) -> VResult<()> {
    BotPermission::ManageRole.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/deleteMemberRole",
        DeleteMemberRoleRequest::new(self.id),
      )
      .execute_no_return(&self.villa.bot.request_executor)
      .await
  }
}
