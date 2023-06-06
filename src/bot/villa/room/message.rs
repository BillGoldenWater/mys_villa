use crate::api_type::event::bot_event::bot_event_data::message_identifier::MessageIdentifier;
use log::debug;

use crate::api_type::message::pin_message_request::PinMessageRequest;
use crate::api_type::message::recall_message_request::RecallMessageRequest;
use crate::bot::bot_event_handler::BotEventHandler;
use crate::bot::bot_permission::BotPermission;
use crate::bot::villa::room::Room;
use crate::error::VResult;
use crate::request::request_executor::RequestExecutor;

/// message builders, for easily build message object
pub mod message_builder;

/// for execute api under message context
/// - [Message::pin] to pin the message in room
/// - [Message::unpin] to unpin the message
/// - [Message::recall] to recall the message
#[derive(Debug)]
pub struct Message<
  'room,
  State,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor,
> {
  room: &'room Room<'room, State, EventHandler, ReqExecutor>,
  msg_ident: MessageIdentifier,
}

impl<
    'room,
    State,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor,
  > Message<'room, State, EventHandler, ReqExecutor>
{
  /// create a instance with room and message identifier
  pub fn new(
    room: &'room Room<'room, State, EventHandler, ReqExecutor>,
    msg_ident: MessageIdentifier,
  ) -> Self {
    debug!("initializing message instance with {msg_ident:?}");

    Self { room, msg_ident }
  }

  /// get message identifier
  pub fn ident(&self) -> &MessageIdentifier {
    &self.msg_ident
  }

  /// pin message
  pub async fn pin(&self) -> VResult<()> {
    self.pin_message(false).await
  }

  /// unpin message
  pub async fn unpin(&self) -> VResult<()> {
    self.pin_message(true).await
  }

  async fn pin_message(&self, is_cancel: bool) -> VResult<()> {
    BotPermission::ManageMessage.check_result(self.room.villa.bot)?;

    self
      .room
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/pinMessage",
        PinMessageRequest::new(self.room.id, is_cancel, self.ident()),
      )
      .execute_no_return(&self.room.villa.bot.request_executor)
      .await
  }

  /// recall this message
  pub async fn recall(&self) -> VResult<()> {
    BotPermission::ManageMessage.check_result(self.room.villa.bot)?;

    self
      .room
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/recallMessage",
        RecallMessageRequest::new(self.room.id(), self.ident()),
      )
      .execute_no_return(&self.room.villa.bot.request_executor)
      .await
  }
}
