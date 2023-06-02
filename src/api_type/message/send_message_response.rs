use serde::Deserialize;

/// send message response
#[derive(Debug, Deserialize)]
pub struct SendMessageResponse {
  /// bot msg id
  pub bot_msg_id: String,
}
