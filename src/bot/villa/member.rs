use log::debug;

use crate::api_type::member::delete_villa_member_request::DeleteVillaMemberRequest;
use crate::api_type::member::get_member_request::GetMemberRequest;
use crate::api_type::member::get_member_response::GetMemberResponse;
use crate::api_type::member::member_data::MemberData;
use crate::bot::bot_event_handler::BotEventHandler;
use crate::bot::bot_permission::BotPermission;
use crate::bot::villa::Villa;
use crate::error::VResult;
use crate::request::request_executor::RequestExecutor;

/// member instance, provide member related access
#[derive(Debug)]
pub struct Member<
  'villa,
  State,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor,
> {
  villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>,
  uid: u64,
}

impl<
    'villa,
    State,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor,
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
}
