/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::event::bot_event::bot_event_data::BotEventData;

/// bot event extend data
#[derive(Debug, Deserialize)]
pub enum BotEventExtendData {
  /// event data
  EventData(BotEventData),
}
