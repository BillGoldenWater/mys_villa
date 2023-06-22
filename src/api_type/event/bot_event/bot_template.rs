/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::event::bot_event::bot_command::BotCommand;

/// bot template
#[derive(Debug, Clone, Deserialize)]
pub struct BotTemplate {
  /// id
  pub id: String,
  /// bot name
  pub name: String,
  /// description
  #[serde(rename = "desc")]
  pub description: String,
  /// bot icon
  pub icon: String,
  /// commands info
  pub commands: Vec<BotCommand>,
}
