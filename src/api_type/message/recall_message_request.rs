use serde::Serialize;

/// recall message request
#[derive(Debug, Serialize)]
pub struct RecallMessageRequest {
  msg_uid: String,
  room_id: u64,
  msg_time: i64,
}

impl RecallMessageRequest {
  /// initialize with msg_uid, room_id and msg_time
  pub fn new(msg_uid: impl Into<String>, room_id: u64, msg_time: i64) -> Self {
    Self {
      msg_uid: msg_uid.into(),
      room_id,
      msg_time,
    }
  }
}
