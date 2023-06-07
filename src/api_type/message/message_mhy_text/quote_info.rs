/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use crate::api_type::event::bot_event::bot_event_data::message_identifier::MessageIdentifier;

/// quote info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteInfo {
  /// quoted message id
  pub quoted_message_id: String,
  /// quoted message send time
  pub quoted_message_send_time: i64,
  original_message_id: String,
  original_message_send_time: i64,
}

impl QuoteInfo {
  /// initialize with quoted_message_id and quoted_message_send_time
  pub fn new(quoted_message_id: impl Into<String>, quoted_message_send_time: i64) -> Self {
    let id = quoted_message_id.into();

    Self {
      quoted_message_id: id.clone(),
      quoted_message_send_time,
      original_message_id: id,
      original_message_send_time: quoted_message_send_time,
    }
  }
}

impl From<MessageIdentifier> for QuoteInfo {
  fn from(value: MessageIdentifier) -> Self {
    Self::new(value.msg_uid,value.send_at)
  }
}
