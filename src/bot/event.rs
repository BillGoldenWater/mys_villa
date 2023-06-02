use crate::api_type::event::bot_event::bot_event_metadata::BotEventMetadata;
use crate::api_type::event::bot_event::bot_template::BotTemplate;
use crate::bot::event::event_data::EventData;

/// definition of event data
pub mod event_data;

/// event
#[derive(Debug)]
pub struct Event {
  /// bot info
  pub bot_info: BotTemplate,
  /// event metadata
  pub metadata: BotEventMetadata,
  /// event data
  pub data: EventData,
}
