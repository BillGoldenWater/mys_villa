/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
  api_type::event::bot_event::{bot_event_metadata::BotEventMetadata, bot_template::BotTemplate},
  bot::event::event_data::EventData,
};

/// definition of event data
pub mod event_data;

/// event
#[derive(Debug, Clone)]
pub struct Event {
  /// bot info
  pub bot_info: BotTemplate,
  /// event metadata
  pub metadata: BotEventMetadata,
  /// event data
  pub data: EventData,
}
