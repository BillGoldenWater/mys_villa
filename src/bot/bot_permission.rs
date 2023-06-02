use crate::bot::bot_event_handler::BotEventHandler;
use crate::bot::Bot;
use crate::error::{VError, VResult};
use crate::request::request_executor::RequestExecutor;

/// permission info about what can do
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
  // /// Audit user content
  // Audit,
}

impl BotPermission {
  /// get all permission
  pub fn all() -> Vec<Self> {
    vec![
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
      // BotPermission::Audit,
    ]
  }

  /// check if bot has this permission
  pub fn check<
    State,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor,
  >(
    &self,
    bot: &Bot<State, EventHandler, ReqExecutor>,
  ) -> bool {
    bot.permission.contains(self)
  }

  /// check if bot has this permission, return a result
  pub fn check_result<
    State,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor,
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
