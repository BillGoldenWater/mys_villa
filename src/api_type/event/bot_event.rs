/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::event::bot_event::bot_context::BotContext;
use crate::api_type::event::bot_event::bot_event_extend_data::BotEventExtendData;
use crate::api_type::event::bot_event::bot_event_metadata::BotEventMetadata;

/// definition of bot command
pub mod bot_command;
/// definition of bot context
pub mod bot_context;
/// definition of bot event data
pub mod bot_event_data;
/// definition of bot event extend data
pub mod bot_event_extend_data;
/// definition of bot event metadata
pub mod bot_event_metadata;
/// definition of bot template
pub mod bot_template;

/// bot event
#[derive(Debug, Deserialize)]
pub struct BotEvent {
  /// bot context
  pub robot: BotContext,
  /// event metadata
  #[serde(flatten)]
  pub metadata: BotEventMetadata,
  /// event extend tata
  pub extend_data: BotEventExtendData,
}
