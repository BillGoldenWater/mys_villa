/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

use crate::api_type::event::bot_event::bot_event_data::message_identifier::MessageIdentifier;

/// pin message request
#[derive(Debug, Serialize)]
pub struct PinMessageRequest {
  msg_uid: String,
  is_cancel: bool,
  room_id: u64,
  send_at: i64,
}

impl PinMessageRequest {
  /// initialize with room_id, is_cancel and message identifier
  pub fn new(room_id: u64, is_cancel: bool, msg_ident: impl Into<MessageIdentifier>) -> Self {
    let MessageIdentifier { msg_uid, send_at } = msg_ident.into();
    Self {
      room_id,
      is_cancel,
      msg_uid,
      send_at,
    }
  }
}
