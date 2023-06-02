use serde::Serialize;

/// edit room request
#[derive(Debug, Serialize)]
pub struct EditRoomRequest {
  room_id: u64,
  room_name: String,
}

impl EditRoomRequest {
  /// initialize with room_id and room_name
  pub fn new(room_id: u64, room_name: String) -> Self {
    Self { room_id, room_name }
  }
}
