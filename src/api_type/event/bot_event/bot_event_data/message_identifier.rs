use serde::Deserialize;

/// message identifier
#[derive(Debug, Clone, Deserialize)]
pub struct MessageIdentifier {
  /// message uid
  pub msg_uid: String,
  /// message send time
  pub send_at: i64,
}

impl From<&MessageIdentifier> for MessageIdentifier {
  fn from(value: &MessageIdentifier) -> Self {
    value.clone()
  }
}