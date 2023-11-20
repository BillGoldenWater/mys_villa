/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

/// add quick emoticon metadata
#[derive(Debug, Clone, Deserialize)]
pub struct AddQuickEmoticonMetadata {
  /// villa id
  pub villa_id: u64,
  /// room id
  pub room_id: u64,
  /// sender uid
  #[serde(rename = "uid")]
  pub sender_uid: u64,
  /// react target message id
  pub msg_uid: String,
  /// bot msg id ([None] if the target message isn't send by a bot)
  #[serde(default)]
  pub bot_msg_id: Option<String>,
}
