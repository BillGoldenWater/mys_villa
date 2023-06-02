use serde::Serialize;

use crate::api_type::message::message_object::MessageObject;

/// send message request
#[derive(Debug, Serialize)]
pub struct SendMessageRequest {
  room_id: u64,
  #[serde(flatten)]
  msg_object: MessageObject,
}

impl SendMessageRequest {
  /// initialize with room_id and msg_object
  pub fn new(room_id: u64, msg_object: MessageObject) -> Self {
    Self {
      room_id,
      msg_object,
    }
  }
}
