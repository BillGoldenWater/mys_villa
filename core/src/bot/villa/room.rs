/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::sync::Arc;

use serde_json::Value;
use tracing::instrument;

use crate::{
  api::{
    api_error::ApiError,
    villa_bot_api::villa_api::room_api::{
      delete_room::DeleteRoomRequest,
      edit_room::EditRoomRequest,
      get_room::{GetRoomRequest, GetRoomResponse},
      msg_content_info::{MsgContentInfo, MsgContentInfoForSendAndRecv},
      send_message::{SendMessageRequest, SendMessageResponse},
    },
  },
  bot::{
    bot_permission::BotPermission,
    villa::{
      room::{message::Message, message_ident::MessageIdent, message_object::MessageObject},
      room_info_detail::RoomInfoDetail,
      Villa,
    },
    Bot,
  },
  error::VResult,
  http::request::Request,
  utils::fp_utils::FpUtils,
};

pub mod message;
pub mod message_ident;
pub mod message_object;

#[derive(Debug, Clone)]
pub struct Room {
  villa: Arc<Villa>,

  id: u64,
}

impl Room {
  #[instrument(level = "debug", skip(villa))]
  pub fn new(villa: Arc<Villa>, id: u64) -> Self {
    Self { villa, id }
  }

  pub fn id(&self) -> u64 {
    self.id
  }

  pub fn message(&self, id: Arc<MessageIdent>) -> Message {
    Message::new(self.clone().into(), id)
  }
}

/// message api
impl Room {
  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn send_message(&self, message_object: MessageObject) -> VResult<String> {
    let msg_content_info: MsgContentInfo = message_object.into();
    let object_name = msg_content_info.content.as_str_name().to_string();
    let msg_content_info_for_send_and_recv = MsgContentInfoForSendAndRecv::from(msg_content_info);
    let message =
      serde_json::to_string(&msg_content_info_for_send_and_recv).map_err(Into::<ApiError>::into)?;

    self.send_message_raw(object_name, message).await
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn send_message_raw(&self, object_name: String, message: String) -> VResult<String> {
    BotPermission::SendMessage.check(self)?;

    self
      .villa
      .execute_bot_req_with_villa::<SendMessageResponse>(
        Request::post("/vila/api/bot/platform/sendMessage").with_json(SendMessageRequest::new(
          self.id,
          object_name,
          message,
        ))?,
      )
      .await
      .map(|it| it.bot_msg_id)
  }

  // todo: wrappers for easier construction
}

/// other api
impl Room {
  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn get_info(&self) -> VResult<RoomInfoDetail> {
    BotPermission::ViewRoomAndGroup.check(self)?;

    self
      .villa
      .execute_bot_req_with_villa::<GetRoomResponse>(
        Request::get("/vila/api/bot/platform/getRoom").with_json(GetRoomRequest::new(self.id))?,
      )
      .await
      .map(|it| it.room.into())
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn set_name(&self, name: String) -> VResult<()> {
    BotPermission::ManageRoomAndGroup.check(self)?;

    self
      .villa
      .execute_bot_req_with_villa::<Value>(
        Request::post("/vila/api/bot/platform/editRoom")
          .with_json(EditRoomRequest::new(self.id, name))?,
      )
      .await
      .unit_result()
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn delete(self) -> VResult<()> {
    BotPermission::ManageRoomAndGroup.check(&self)?;

    self
      .villa
      .execute_bot_req_with_villa::<Value>(
        Request::post("/vila/api/bot/platform/deleteRoom")
          .with_json(DeleteRoomRequest::new(self.id))?,
      )
      .await
      .unit_result()
  }
}

impl AsRef<Bot> for Room {
  fn as_ref(&self) -> &Bot {
    &self.villa.bot
  }
}
