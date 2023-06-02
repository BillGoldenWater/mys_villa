use serde::Deserialize;

/// send message metadata
#[derive(Debug, Clone, Deserialize)]
pub struct SendMessageMetadata {
  /// sender uid
  #[serde(rename = "from_user_id")]
  pub sender_uid: u64,
  /// room id
  pub room_id: u64,
  /// msg uid
  pub msg_uid: String,
  /// send time
  pub send_at: i64,
  /// bot msg id ([None] if this message isn't send by a bot)
  #[serde(default)]
  pub bot_msg_id: Option<String>,
  /// sender nickname
  pub nickname: String,
}
