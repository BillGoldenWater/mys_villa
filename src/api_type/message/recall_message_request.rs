use crate::api_type::event::bot_event::bot_event_data::message_identifier::MessageIdentifier;
use serde::Serialize;

/// recall message request
#[derive(Debug, Serialize)]
pub struct RecallMessageRequest {
  msg_uid: String,
  room_id: u64,
  msg_time: i64,
}

impl RecallMessageRequest {
  /// initialize with room_id and message identifier
  pub fn new(room_id: u64, msg_ident: impl Into<MessageIdentifier>) -> Self {
    let MessageIdentifier { msg_uid, send_at } = msg_ident.into();
    Self {
      room_id,
      msg_uid,
      msg_time: send_at,
    }
  }
}
