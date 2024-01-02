/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::sync::Arc;

use serde_json::Value;
use tracing::instrument;

use crate::api::villa_bot_api::villa_api::room_api::message_api::pin_message::PinMessageRequest;
use crate::api::villa_bot_api::villa_api::room_api::message_api::recall_message::RecallMessageRequest;
use crate::bot::villa::room::message_ident::MessageIdent;
use crate::bot::villa::room::Room;
use crate::error::VResult;
use crate::http::request::Request;
use crate::utils::fp_utils::FpUtils;

#[derive(Debug, Clone)]
pub struct Message {
  room: Arc<Room>,

  id: Arc<MessageIdent>,
}

impl Message {
  #[instrument(level = "debug", skip(room))]
  pub fn new(room: Arc<Room>, id: Arc<MessageIdent>) -> Self {
    Self { room, id }
  }

  pub fn id(&self) -> &Arc<MessageIdent> {
    &self.id
  }
}

/// api
impl Message {
  #[instrument(level = "debug", skip(self), fields(id = tracing::field::debug(& self.id)))]
  pub async fn pin(&self) -> VResult<()> {
    self.pin_message(false).await
  }

  #[instrument(level = "debug", skip(self), fields(id = tracing::field::debug(& self.id)))]
  pub async fn unpin(&self) -> VResult<()> {
    self.pin_message(true).await
  }

  async fn pin_message(&self, is_cancel: bool) -> VResult<()> {
    self
      .room
      .villa
      .execute_bot_req_with_villa::<Value>(
        Request::post("/vila/api/bot/platform/pinMessage").with_json(PinMessageRequest::new(
          self.room.id(),
          self.id.id.clone(),
          self.id.send_at,
          is_cancel,
        ))?,
      )
      .await
      .unit_result()
  }

  #[instrument(level = "debug", skip(self), fields(id = tracing::field::debug(& self.id)))]
  pub async fn recall(self) -> VResult<()> {
    self
      .room
      .villa
      .execute_bot_req_with_villa::<Value>(
        Request::post("/vila/api/bot/platform/recallMessage").with_json(
          RecallMessageRequest::new(self.room.id(), self.id.id.clone(), self.id.send_at),
        )?,
      )
      .await
      .unit_result()
  }
}
