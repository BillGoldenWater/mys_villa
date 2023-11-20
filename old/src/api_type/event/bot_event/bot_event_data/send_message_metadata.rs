/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::event::bot_event::bot_event_data::message_identifier::MessageIdentifier;
use serde::Deserialize;

/// send message metadata
#[derive(Debug, Clone, Deserialize)]
pub struct SendMessageMetadata {
  /// sender uid
  #[serde(rename = "from_user_id")]
  pub sender_uid: u64,
  /// room id
  pub room_id: u64,
  /// message identifier
  #[serde(flatten)]
  pub msg_ident: MessageIdentifier,
  /// bot msg id ([None] if this message isn't send by a bot)
  #[serde(default)]
  pub bot_msg_id: Option<String>,
  /// sender nickname
  pub nickname: String,
}
