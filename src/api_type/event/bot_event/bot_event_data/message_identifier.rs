/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

/// message identifier
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MessageIdentifier {
  /// message uid
  pub msg_uid: String,
  /// message send time
  pub send_at: i64,
}

impl MessageIdentifier {
  /// initialize with message uid and send time
  pub fn new(msg_uid: impl Into<String>, send_at: i64) -> Self {
    Self {
      msg_uid: msg_uid.into(),
      send_at,
    }
  }
}

impl From<&MessageIdentifier> for MessageIdentifier {
  fn from(value: &MessageIdentifier) -> Self {
    value.clone()
  }
}
