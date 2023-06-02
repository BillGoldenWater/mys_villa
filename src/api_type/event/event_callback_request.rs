use serde::Deserialize;

use crate::api_type::event::bot_event::BotEvent;

/// event callback request
#[derive(Debug, Deserialize)]
pub struct EventCallbackRequest {
  /// event data
  pub event: BotEvent,
}
