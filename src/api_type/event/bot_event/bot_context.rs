/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::event::bot_event::bot_template::BotTemplate;

/// bot context
#[derive(Debug, Deserialize)]
pub struct BotContext {
  /// bot template info
  pub template: BotTemplate,
  /// current villa id
  pub villa_id: u64,
}
