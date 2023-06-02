use serde::Deserialize;

use crate::api_type::room::room_data::RoomData;

/// get room response
#[derive(Debug, Deserialize)]
pub struct GetRoomResponse {
  /// room data
  pub room: RoomData,
}
