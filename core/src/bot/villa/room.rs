/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::sync::Arc;

use serde_json::Value;
use tracing::instrument;

use crate::api::villa_bot_api::villa_api::room_api::delete_room::DeleteRoomRequest;
use crate::api::villa_bot_api::villa_api::room_api::edit_room::EditRoomRequest;
use crate::api::villa_bot_api::villa_api::room_api::get_room::{GetRoomRequest, GetRoomResponse};
use crate::api::villa_bot_api::villa_api::room_api::send_message::{
  SendMessageRequest, SendMessageResponse,
};
use crate::bot::bot_permission::BotPermission;
use crate::bot::villa::room::message::Message;
use crate::bot::villa::room::message_ident::MessageIdent;
use crate::bot::villa::room_info_detail::RoomInfoDetail;
use crate::bot::villa::Villa;
use crate::bot::Bot;
use crate::error::VResult;
use crate::http::request::Request;
use crate::utils::fp_utils::FpUtils;

pub mod message;
pub mod message_ident;

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
