use serde::Serialize;

/// delete room request
#[derive(Debug, Serialize)]
pub struct DeleteRoomRequest {
  room_id: u64,
}

impl DeleteRoomRequest {
  /// initialize with room_id
  pub fn new(room_id: u64) -> Self {
    Self { room_id }
  }
}
