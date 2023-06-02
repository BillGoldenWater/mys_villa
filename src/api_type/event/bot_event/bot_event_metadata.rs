use serde::Deserialize;

/// bot event metadata
#[derive(Debug, Deserialize)]
pub struct BotEventMetadata {
  /// event id
  pub id: String,
  /// event create timestamp
  pub created_at: i64,
  /// event send timestamp
  pub send_at: i64,
  /// event type
  pub r#type: i32,
}
