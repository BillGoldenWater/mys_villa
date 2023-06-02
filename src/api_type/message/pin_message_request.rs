use serde::Serialize;

/// pin message request
#[derive(Debug, Serialize)]
pub struct PinMessageRequest {
  msg_uid: String,
  is_cancel: bool,
  room_id: u64,
  send_at: i64,
}

impl PinMessageRequest {
  /// initialize with msg_uid, is_cancel, room_id and send_at
  pub fn new(msg_uid: impl Into<String>, is_cancel: bool, room_id: u64, send_at: i64) -> Self {
    Self {
      msg_uid: msg_uid.into(),
      is_cancel,
      room_id,
      send_at,
    }
  }
}
