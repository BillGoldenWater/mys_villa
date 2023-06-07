/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

/// bot command
#[derive(Debug, Clone, Deserialize)]
pub struct BotCommand {
  /// name
  pub name: String,
  /// description
  #[serde(rename = "desc")]
  pub description: String,
}
