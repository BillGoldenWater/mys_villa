/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

/// mention bot
#[derive(Debug, Clone, PartialEq)]
pub struct MentionBot {
  /// bot name
  pub bot_name: String,
  /// bot id
  pub bot_id: String,
}

impl MentionBot {
  /// initialize with bot_name and bot id
  pub fn new(bot_name: impl Into<String>, bot_id: impl Into<String>) -> Self {
    Self {
      bot_name: bot_name.into(),
      bot_id: bot_id.into(),
    }
  }
}
