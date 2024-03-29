/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use log::debug;
use serde_json::json;

use crate::api_type::event::bot_event::bot_event_data::message_identifier::MessageIdentifier;
use crate::api_type::message::message_object::MessageObject;
use crate::api_type::message::send_message_request::SendMessageRequest;
use crate::api_type::message::send_message_response::SendMessageResponse;
use crate::api_type::room::delete_room_request::DeleteRoomRequest;
use crate::api_type::room::edit_room_request::EditRoomRequest;
use crate::api_type::room::get_room_request::GetRoomRequest;
use crate::api_type::room::get_room_response::GetRoomResponse;
use crate::api_type::room::room_data::RoomData;
use crate::bot::bot_event_handler::BotEventHandler;
use crate::bot::bot_permission::BotPermission;
use crate::bot::villa::room::message::message_builder::MessageBuilder;
use crate::bot::villa::room::message::Message;
use crate::bot::villa::Villa;
use crate::error::VResult;
use crate::request::request_executor::RequestExecutor;

/// message related logic, includes builders
pub mod message;

/// for execute api under room context
/// - [Room::get_data] get room data
/// - [Room::set_name] change name of room
/// - [Room::delete] delete room
/// - [Room::send_message] send message to room
/// - [Room::message_builder] create a message builder
/// - [Room::message] create message instance by id and send time
#[derive(Debug)]
pub struct Room<
  'villa,
  State: Sync,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor + Sync,
> {
  villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>,
  id: u64,
}

impl<
    'villa,
    State: Sync,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor + Sync,
  > Room<'villa, State, EventHandler, ReqExecutor>
{
  /// create a instance with villa and room id
  pub fn new(villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>, id: u64) -> Self {
    debug!("initializing room instance with id: {id}");

    Self { villa, id }
  }

  /// get room id
  pub fn id(&self) -> u64 {
    self.id
  }

  /// create a message instance with message identifier
  pub fn message(
    &self,
    msg_ident: impl Into<MessageIdentifier>,
  ) -> Message<'_, State, EventHandler, ReqExecutor> {
    Message::new(self, msg_ident.into())
  }

  /// get room data
  pub async fn get_data(&self) -> VResult<RoomData> {
    BotPermission::ViewRoomAndGroup.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_get_with_body(
        "/vila/api/bot/platform/getRoom",
        GetRoomRequest::new(self.id),
      )
      .execute_result::<GetRoomResponse, _>(&self.villa.bot.request_executor)
      .await
      .map(|it| it.room)
  }

  /// set room name
  pub async fn set_name(&self, name: impl Into<String>) -> VResult<()> {
    BotPermission::ManageRoomAndGroup.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/editRoom",
        EditRoomRequest::new(self.id, name.into()),
      )
      .execute_no_return(&self.villa.bot.request_executor)
      .await
  }

  /// delete room
  pub async fn delete(&self) -> VResult<()> {
    BotPermission::ManageRoomAndGroup.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/deleteRoom",
        DeleteRoomRequest::new(self.id),
      )
      .execute_no_return(&self.villa.bot.request_executor)
      .await
  }

  /// send message raw
  pub async fn send_message_raw(&self, message: MessageObject) -> VResult<String> {
    BotPermission::SendMessage.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/sendMessage",
        SendMessageRequest::new(self.id, message),
      )
      .execute_result::<SendMessageResponse, _>(&self.villa.bot.request_executor)
      .await
      .map(|it| it.bot_msg_id)
  }

  /// send message raw
  pub async fn send_message_raw_raw(
    &self,
    object_name: String,
    message: String,
  ) -> VResult<String> {
    BotPermission::SendMessage.check_result(self.villa.bot)?;

    self
      .villa
      .req_builder
      .build_post_with_body(
        "/vila/api/bot/platform/sendMessage",
        json!({
          "room_id": self.id,
          "object_name": object_name,
          "msg_content": message,
        }),
      )
      .execute_result::<SendMessageResponse, _>(&self.villa.bot.request_executor)
      .await
      .map(|it| it.bot_msg_id)
  }

  /// send complex message easily with the help from message builder
  /// ```no_run
  /// #  use mys_villa::error::VResult;
  /// #
  /// #  #[tokio::main]
  /// #  async fn main() -> VResult<()> {
  /// #    let bot = mys_villa::bot::default::default();
  /// #    let villa = bot.villa(0);
  /// #    let room = villa.room(0);
  /// room
  ///   .send_message(
  ///     room
  ///       .message_builder()
  ///       .mhy_text(|m| m.mention_all().text("Hello world!")),
  ///   )
  ///   .await?; // @全体成员 Hello world!
  /// #    Ok(())
  /// #  }
  /// ```
  pub async fn send_message(&self, builder: MessageBuilder) -> VResult<String> {
    self.send_message_raw(builder.build()).await
  }

  /// send simple pure text message
  pub async fn send_text(&self, text: impl Into<String>) -> VResult<String> {
    self
      .send_message(self.message_builder().mhy_text(|m| m.text(text)))
      .await
  }

  /// send simple text message with quote
  pub async fn send_reply(
    &self,
    text: impl Into<String>,
    reply_target: impl Into<MessageIdentifier>,
  ) -> VResult<String> {
    self
      .send_message(
        self
          .message_builder()
          .with_quote(reply_target)
          .mhy_text(|m| m.text(text)),
      )
      .await
  }

  /// create a message builder
  pub fn message_builder(&self) -> MessageBuilder {
    MessageBuilder::default()
  }
}
